use crate::{client, paths, runtime::Runtime, store::Store};
use eros_core::{validate_workspace, Reply, Request, SessionState, Snapshot, TerminalEvent};
use fs2::FileExt;
use serde::Serialize;
use std::fs::{self, OpenOptions};
use std::io::BufReader;
use std::os::fd::AsRawFd;
use std::os::unix::fs::{FileTypeExt, PermissionsExt};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;
use std::sync::mpsc::RecvTimeoutError;
use std::sync::Arc;
use std::time::Duration;

pub struct Server {
    store: Arc<Store>,
    runtime: Arc<Runtime>,
}

impl Server {
    fn open() -> Result<Self, String> {
        let data_dir = paths::data_dir().map_err(|error| error.to_string())?;
        let store = Arc::new(Store::open(&data_dir.join("eros.sqlite3"))?);
        for conversation in store.snapshot()?.conversations {
            if conversation.state == SessionState::Running
                || conversation.state == SessionState::Starting
            {
                store.set_conversation_state(&conversation.id, &SessionState::Unavailable)?;
                if matches!(
                    conversation.chat_status,
                    eros_core::ChatStatus::Working | eros_core::ChatStatus::AwaitingDecision
                ) {
                    store.set_chat_activity(
                        &conversation.id,
                        &eros_core::ChatStatus::Normal,
                        None,
                    )?;
                }
            }
        }
        Ok(Self {
            runtime: Arc::new(Runtime::new(store.clone())),
            store,
        })
    }

    fn handle(self: &Arc<Self>, mut stream: UnixStream) {
        let request = match client::read_frame(&mut BufReader::new(&stream)).and_then(|frame| {
            serde_json::from_slice::<Request>(&frame)
                .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))
        }) {
            Ok(request) => request,
            Err(_) => {
                let _ = write_reply(&mut stream, &Reply::error("A mensagem local é inválida."));
                return;
            }
        };

        let result = match request {
            Request::SubscribeActivity => {
                self.stream_activity(stream);
                return;
            }
            Request::Subscribe { conversation_id } => {
                self.stream_subscription(stream, conversation_id);
                return;
            }
            request => self.execute(request),
        };
        match result {
            Ok(value) => {
                let _ = write_reply(&mut stream, &Reply::success(value));
            }
            Err(error) => {
                let _ = write_reply(&mut stream, &Reply::error(error));
            }
        }
    }

    fn stream_subscription(&self, mut stream: UnixStream, conversation_id: String) {
        let (history, receiver) = match self.runtime.subscribe(&conversation_id) {
            Ok(subscription) => subscription,
            Err(error) => {
                let _ = write_reply(&mut stream, &Reply::error(error));
                return;
            }
        };
        if write_reply(&mut stream, &Reply::success(true)).is_err() {
            return;
        }
        for chunk in history.chunks(eros_core::OUTPUT_BATCH) {
            if write_frame(
                &mut stream,
                &TerminalEvent::Output {
                    conversation_id: conversation_id.clone(),
                    data: chunk.to_vec(),
                },
            )
            .is_err()
            {
                return;
            }
        }
        let _ = stream.set_write_timeout(Some(Duration::from_secs(30)));
        loop {
            match receiver.recv_timeout(Duration::from_secs(1)) {
                Ok(event) => {
                    if write_frame(&mut stream, &event).is_err() {
                        break;
                    }
                    if matches!(
                        &event,
                        TerminalEvent::State { state, .. } if *state != SessionState::Running
                    ) {
                        break;
                    }
                }
                Err(RecvTimeoutError::Timeout) => {
                    if peer_disconnected(&stream) {
                        break;
                    }
                }
                Err(RecvTimeoutError::Disconnected) => break,
            }
        }
    }

    fn stream_activity(&self, mut stream: UnixStream) {
        let subscription = match self.runtime.activity.subscribe(&self.store) {
            Ok(subscription) => subscription,
            Err(error) => {
                let _ = write_reply(&mut stream, &Reply::error(error));
                return;
            }
        };
        let _ = stream.set_write_timeout(Some(Duration::from_secs(5)));
        if write_reply(&mut stream, &Reply::success(true)).is_err() {
            return;
        }
        for event in &subscription.snapshot {
            if write_frame(&mut stream, &event).is_err() {
                return;
            }
        }
        loop {
            match subscription.receiver.recv_timeout(Duration::from_secs(1)) {
                Ok(event) => {
                    if write_frame(&mut stream, &event).is_err() {
                        break;
                    }
                }
                Err(RecvTimeoutError::Timeout) => {
                    if peer_disconnected(&stream) {
                        break;
                    }
                }
                Err(RecvTimeoutError::Disconnected) => break,
            }
        }
    }

    fn execute(&self, request: Request) -> Result<serde_json::Value, String> {
        match request {
            Request::Snapshot => to_value(self.snapshot()?),
            Request::ProviderHook {
                conversation_id,
                token,
                event,
            } => {
                self.runtime
                    .provider_hook(&conversation_id, &token, &event)?;
                to_value(())
            }
            Request::MarkConversationRead {
                conversation_id,
                activity_revision,
            } => to_value(self.runtime.activity.acknowledge(
                &self.store,
                &conversation_id,
                activity_revision,
            )?),
            Request::AddProject { path } => to_value(self.store.add_project(&path)?),
            Request::RenameProject { id, name } => to_value(self.store.rename_project(&id, &name)?),
            Request::SetSidebarOrder { group, ids } => {
                to_value(self.store.set_sidebar_order(group, &ids)?)
            }
            Request::CreateConversation {
                id,
                project_id,
                name,
                cwd,
                kind,
            } => {
                let cwd = if project_id.is_none() && cwd.is_empty() {
                    self.store.resolve_chat_default_folder()?
                } else {
                    cwd
                };
                to_value(
                    self.runtime
                        .create_conversation(id, project_id, name, cwd, kind)?,
                )
            }
            Request::SetChatDefaultFolder { path } => {
                self.store.set_chat_default_folder(&path)?;
                to_value(())
            }
            Request::SetConversationPinned { id, pinned } => {
                self.store.set_conversation_pinned(&id, pinned)?;
                to_value(())
            }
            Request::DeleteConversation { id } => {
                self.runtime.delete_conversation(&id)?;
                to_value(())
            }
            Request::SetActiveScope { scope } => {
                self.store.set_active_scope(&scope)?;
                to_value(())
            }
            Request::SaveWorkspace { workspace } => {
                let snapshot = self.snapshot()?;
                validate_workspace(&workspace, &snapshot.conversations)?;
                let project_scope = snapshot
                    .projects
                    .iter()
                    .any(|project| project.id == workspace.scope);
                let conversation_scope =
                    workspace
                        .scope
                        .strip_prefix("conversation:")
                        .is_some_and(|id| {
                            snapshot.conversations.iter().any(|conversation| {
                                conversation.id == id
                                    && conversation.kind != eros_core::SessionKind::Shell
                            })
                        });
                if workspace.scope != "chats" && !project_scope && !conversation_scope {
                    return Err("O projeto desta área de trabalho não existe mais.".into());
                }
                self.store.save_workspace(&workspace)?;
                to_value(workspace)
            }
            Request::Write {
                conversation_id,
                text,
            } => {
                self.runtime.write(&conversation_id, &text)?;
                to_value(())
            }
            Request::Resize {
                conversation_id,
                cols,
                rows,
            } => {
                self.runtime.resize(&conversation_id, cols, rows)?;
                to_value(())
            }
            Request::Stop { conversation_id } => {
                self.runtime.stop(&conversation_id)?;
                to_value(())
            }
            Request::SubscribeActivity | Request::Subscribe { .. } => {
                Err("Assinatura inválida.".into())
            }
        }
    }

    fn snapshot(&self) -> Result<Snapshot, String> {
        let mut snapshot = self.store.snapshot()?;
        for conversation in &mut snapshot.conversations {
            if let Some(state) = self.runtime.state(&conversation.id) {
                conversation.state = state;
            }
        }
        snapshot.authenticated_providers = self.runtime.authenticated_providers();
        Ok(snapshot)
    }
}

fn peer_disconnected(stream: &UnixStream) -> bool {
    let events = libc::POLLHUP | libc::POLLERR | libc::POLLNVAL;
    let mut descriptor = libc::pollfd {
        fd: stream.as_raw_fd(),
        events,
        revents: 0,
    };
    let result = unsafe { libc::poll(&mut descriptor, 1, 0) };
    result > 0 && descriptor.revents & events != 0
}

pub fn serve() -> Result<(), String> {
    let runtime_dir = paths::runtime_dir().map_err(|error| error.to_string())?;
    let lock_path = runtime_dir.join("service.lock");
    let lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(lock_path)
        .map_err(|error| error.to_string())?;
    if lock.try_lock_exclusive().is_err() {
        return Ok(());
    }

    let socket_path = paths::socket_path().map_err(|error| error.to_string())?;
    remove_stale_socket(&socket_path)?;
    let listener = UnixListener::bind(&socket_path).map_err(|error| error.to_string())?;
    fs::set_permissions(&socket_path, fs::Permissions::from_mode(0o600))
        .map_err(|error| error.to_string())?;
    let server = Arc::new(Server::open()?);

    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                let server = server.clone();
                let _ = std::thread::Builder::new()
                    .name("eros-ipc-client".into())
                    .spawn(move || server.handle(stream));
            }
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(error) => return Err(error.to_string()),
        }
    }
    Ok(())
}

fn remove_stale_socket(path: &PathBuf) -> Result<(), String> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_socket() => {
            fs::remove_file(path).map_err(|error| error.to_string())
        }
        Ok(_) => Err("O caminho do socket local contém um arquivo inesperado.".into()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

fn to_value(value: impl Serialize) -> Result<serde_json::Value, String> {
    serde_json::to_value(value).map_err(|error| error.to_string())
}

fn write_reply(stream: &mut UnixStream, reply: &Reply) -> Result<(), String> {
    write_frame(stream, reply)
}

fn write_frame(stream: &mut UnixStream, value: &impl Serialize) -> Result<(), String> {
    client::write_frame(stream, value)
}
