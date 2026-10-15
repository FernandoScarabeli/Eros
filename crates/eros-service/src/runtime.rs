use crate::{
    activity::{ActivityHub, ProviderActivity},
    provider_hooks,
    store::Store,
};
use eros_core::{
    AuthenticatedProvider, Conversation, SessionKind, SessionState, TerminalEvent, HISTORY_BYTES,
    MAX_SESSIONS, OUTPUT_BATCH,
};
use portable_pty::{native_pty_system, CommandBuilder, MasterPty, PtySize};
use std::collections::{HashMap, VecDeque};
use std::env;
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::sync::{Arc, Mutex};

pub struct Runtime {
    store: Arc<Store>,
    pub activity: ActivityHub,
    creation_lock: Mutex<()>,
    sessions: Mutex<HashMap<String, Arc<LiveSession>>>,
}

struct LiveSession {
    master: Mutex<Box<dyn MasterPty + Send>>,
    writer: Mutex<Box<dyn Write + Send>>,
    child: Mutex<Box<dyn portable_pty::Child + Send + Sync>>,
    subscribers: Mutex<Vec<SyncSender<TerminalEvent>>>,
    history: Mutex<VecDeque<u8>>,
    state: Mutex<SessionState>,
    hook_token: String,
    activity: Mutex<ProviderActivity>,
}

impl Runtime {
    pub fn new(store: Arc<Store>) -> Self {
        Self {
            store,
            activity: ActivityHub::default(),
            creation_lock: Mutex::new(()),
            sessions: Mutex::new(HashMap::new()),
        }
    }

    pub fn authenticated_providers(&self) -> Vec<AuthenticatedProvider> {
        let Ok(codex) = resolve_codex() else {
            return Vec::new();
        };
        let Ok(status) = Command::new(codex).args(["login", "status"]).output() else {
            return Vec::new();
        };
        if !status.status.success() {
            return Vec::new();
        }

        let output = format!(
            "{}\n{}",
            String::from_utf8_lossy(&status.stdout),
            String::from_utf8_lossy(&status.stderr)
        )
        .to_ascii_lowercase();
        let confirms_login = output.contains("logged in") || output.contains("authenticated");
        let denies_login = output.contains("not logged in") || output.contains("not authenticated");
        if !confirms_login || denies_login {
            return Vec::new();
        }

        vec![AuthenticatedProvider {
            id: "codex".into(),
            label: "Codex".into(),
        }]
    }

    pub fn create_conversation(
        self: &Arc<Self>,
        id: String,
        project_id: Option<String>,
        name: String,
        cwd: String,
        kind: SessionKind,
    ) -> Result<Conversation, String> {
        let _creation = self
            .creation_lock
            .lock()
            .map_err(|_| "O runtime local ficou indisponível.")?;
        let name = name.trim();
        if name.is_empty() || name.chars().count() > 80 {
            return Err("Dê um nome de até 80 caracteres para a conversa.".into());
        }
        // Standalone shells start in the user's home; project terminals pass
        // their explicit folder. Resolve this on the process-owning side.
        let cwd = if cwd.is_empty() {
            crate::paths::home_dir()
                .map_err(|error| format!("A pasta pessoal não está disponível: {error}"))?
                .to_string_lossy()
                .into_owned()
        } else {
            cwd
        };
        let cwd = std::fs::canonicalize(&cwd)
            .map_err(|error| format!("A pasta dessa conversa não está disponível: {error}"))?;
        if !cwd.is_dir() {
            return Err("A pasta dessa conversa não está disponível.".into());
        }
        {
            let sessions = self
                .sessions
                .lock()
                .map_err(|_| "O runtime local ficou indisponível.")?;
            if sessions.contains_key(&id) {
                return Err("Já existe uma conversa com esse identificador.".into());
            }
            let running = sessions
                .values()
                .filter(|session| {
                    session
                        .state
                        .lock()
                        .is_ok_and(|state| *state == SessionState::Running)
                })
                .count();
            if running >= MAX_SESSIONS {
                return Err(format!(
                    "O limite de {MAX_SESSIONS} conversas ativas foi atingido."
                ));
            }
        }

        let pty_system = native_pty_system();
        let pair = pty_system
            .openpty(PtySize {
                rows: 24,
                cols: 80,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|error| format!("Não foi possível abrir o terminal: {error}"))?;
        let shell = env::var_os("SHELL")
            .filter(|shell| Path::new(shell).is_file())
            .unwrap_or_else(|| "/bin/bash".into());
        let mut command = CommandBuilder::new(&shell);
        command.env("SHELL", &shell);
        let mut token_bytes = [0_u8; 32];
        std::fs::File::open("/dev/urandom")
            .and_then(|mut file| file.read_exact(&mut token_bytes))
            .map_err(|_| "Não foi possível identificar a sessão local.")?;
        let hook_token: String = token_bytes
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        if kind != SessionKind::Shell {
            let executable =
                std::env::current_exe().map_err(|_| "Não foi possível localizar o Eros.")?;
            let launch = provider_hooks::launch_command(&kind, &executable)?;
            command.args(["-i", "-c", &launch]);
            command.env("EROS_HOOK_CONVERSATION", &id);
            command.env("EROS_HOOK_TOKEN", &hook_token);
            command.env(
                "EROS_HOOK_SOCKET",
                crate::paths::socket_path().map_err(|error| error.to_string())?,
            );
        }
        command.cwd(&cwd);
        command.env("TERM", "xterm-256color");
        command.env("COLORTERM", "truecolor");

        let child = pair
            .slave
            .spawn_command(command)
            .map_err(|error| format!("Não foi possível iniciar o processo: {error}"))?;
        drop(pair.slave);
        let mut reader = match pair.master.try_clone_reader() {
            Ok(reader) => reader,
            Err(error) => {
                let mut child = child;
                let _ = child.kill();
                let _ = child.wait();
                return Err(format!("Não foi possível ler o terminal: {error}"));
            }
        };
        let writer = match pair.master.take_writer() {
            Ok(writer) => writer,
            Err(error) => {
                let mut child = child;
                let _ = child.kill();
                let _ = child.wait();
                return Err(format!("Não foi possível escrever no terminal: {error}"));
            }
        };
        let conversation = Conversation {
            id: id.clone(),
            project_id,
            name: name.to_owned(),
            cwd: cwd.to_string_lossy().into_owned(),
            kind,
            state: SessionState::Running,
            provider_thread_id: None,
            pinned: false,
            chat_status: eros_core::ChatStatus::Normal,
            activity_revision: 0,
        };
        if let Err(error) = self.store.create_conversation(&conversation) {
            let mut child = child;
            let _ = child.kill();
            let _ = child.wait();
            return Err(error);
        }

        let session = Arc::new(LiveSession {
            master: Mutex::new(pair.master),
            writer: Mutex::new(writer),
            child: Mutex::new(child),
            subscribers: Mutex::new(Vec::new()),
            history: Mutex::new(VecDeque::with_capacity(HISTORY_BYTES)),
            state: Mutex::new(SessionState::Running),
            hook_token,
            activity: Mutex::new(ProviderActivity::default()),
        });
        self.sessions
            .lock()
            .map_err(|_| "O runtime local ficou indisponível.")?
            .insert(id.clone(), session.clone());

        let runtime = Arc::downgrade(self);
        let reader_session = session.clone();
        let monitor_id = id.clone();
        let monitor = std::thread::Builder::new()
            .name(format!("eros-pty-{id}"))
            .spawn(move || {
                let mut buffer = vec![0u8; OUTPUT_BATCH];
                loop {
                    match reader.read(&mut buffer) {
                        Ok(0) => break,
                        Ok(size) => {
                            let chunk = buffer[..size].to_vec();
                            publish_output(
                                &reader_session,
                                TerminalEvent::Output {
                                    conversation_id: monitor_id.clone(),
                                    data: chunk,
                                },
                            );
                        }
                        Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
                        Err(_) => break,
                    }
                }
                if let Ok(mut child) = reader_session.child.lock() {
                    let _ = child.wait();
                }
                let exited = if let Ok(mut state) = reader_session.state.lock() {
                    if *state == SessionState::Running {
                        *state = SessionState::Exited;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                };
                if exited {
                    if let Some(runtime) = runtime.upgrade() {
                        let _ = runtime
                            .store
                            .set_conversation_state(&monitor_id, &SessionState::Exited);
                        let _ = runtime.end_activity(&monitor_id, &reader_session);
                    }
                    broadcast(
                        &reader_session,
                        TerminalEvent::State {
                            conversation_id: monitor_id.clone(),
                            state: SessionState::Exited,
                        },
                    );
                }
                if let Some(runtime) = runtime.upgrade() {
                    if let Ok(mut sessions) = runtime.sessions.lock() {
                        if sessions
                            .get(&monitor_id)
                            .is_some_and(|session| Arc::ptr_eq(session, &reader_session))
                        {
                            sessions.remove(&monitor_id);
                        }
                    }
                }
            });
        if let Err(error) = monitor {
            if let Ok(mut child) = session.child.lock() {
                let _ = child.kill();
                let _ = child.wait();
            }
            if let Ok(mut state) = session.state.lock() {
                *state = SessionState::Unavailable;
            }
            let _ = self
                .store
                .set_conversation_state(&id, &SessionState::Unavailable);
            if let Ok(mut sessions) = self.sessions.lock() {
                sessions.remove(&id);
            }
            return Err(format!("Não foi possível observar o terminal: {error}"));
        }

        Ok(conversation)
    }

    pub fn provider_hook(
        &self,
        conversation_id: &str,
        token: &str,
        event: &eros_core::ProviderHook,
    ) -> Result<(), String> {
        provider_hooks::validate_metadata(event)?;
        // Hooks can arrive as soon as spawn succeeds. Wait for registration.
        let _creation = self
            .creation_lock
            .lock()
            .map_err(|_| "O runtime ficou indisponível.")?;
        let session = self.session(conversation_id)?;
        if session.hook_token != token
            || !session
                .state
                .lock()
                .is_ok_and(|state| *state == SessionState::Running)
        {
            return Err("Evento de atividade não pertence a uma sessão ativa.".into());
        }
        let mut activity = session
            .activity
            .lock()
            .map_err(|_| "A atividade ficou indisponível.")?;
        if let Some(status) = activity.apply(event) {
            self.activity.update(
                &self.store,
                conversation_id,
                &status,
                activity.session_id.as_deref(),
            )?;
        }
        Ok(())
    }

    fn end_activity(&self, id: &str, session: &LiveSession) -> Result<(), String> {
        let mut activity = session
            .activity
            .lock()
            .map_err(|_| "A atividade ficou indisponível.")?;
        // Pure shell sessions are not chats and must not emit activity.
        if activity.session_id.is_some() {
            if let Some(status) = activity.end() {
                self.activity.update(&self.store, id, &status, None)?;
            }
        }
        Ok(())
    }

    pub fn write(&self, conversation_id: &str, text: &str) -> Result<(), String> {
        if text.len() > eros_core::MAX_FRAME / 2 {
            return Err("O envio excede o limite de um lote do terminal.".into());
        }
        let session = self.session(conversation_id)?;
        let mut writer = session
            .writer
            .lock()
            .map_err(|_| "O terminal ficou indisponível.")?;
        writer
            .write_all(text.as_bytes())
            .and_then(|_| writer.flush())
            .map_err(|_| "Não foi possível enviar texto ao terminal.".into())
    }

    pub fn resize(&self, conversation_id: &str, cols: u16, rows: u16) -> Result<(), String> {
        if !(2..=500).contains(&cols) || !(2..=300).contains(&rows) {
            return Err("O tamanho informado para o terminal é inválido.".into());
        }
        let session = self.session(conversation_id)?;
        let result = session
            .master
            .lock()
            .map_err(|_| "O terminal ficou indisponível.")?
            .resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|_| "Não foi possível ajustar o tamanho do terminal.".into());
        result
    }

    pub fn stop(&self, conversation_id: &str) -> Result<(), String> {
        let session = self.session(conversation_id)?;
        let mut child = session
            .child
            .lock()
            .map_err(|_| "O processo ficou indisponível.")?;
        let mut groups = Vec::with_capacity(2);
        if let Some(group) = child.process_id() {
            groups.push(group as libc::pid_t);
        }
        if let Some(group) = session
            .master
            .lock()
            .map_err(|_| "O terminal ficou indisponível.")?
            .process_group_leader()
        {
            if !groups.contains(&group) {
                groups.push(group);
            }
        }
        if groups.is_empty() {
            child
                .kill()
                .map_err(|_| "Não foi possível interromper o processo.".to_string())?;
        } else {
            let mut signaled = false;
            for group in &groups {
                signaled |= signal_process_group(*group, libc::SIGHUP)?;
            }
            if signaled {
                std::thread::sleep(std::time::Duration::from_millis(250));
            }
            for group in &groups {
                signal_process_group(*group, libc::SIGKILL)?;
            }
            child
                .wait()
                .map_err(|_| "Não foi possível recolher o processo.".to_string())?;
        }
        if let Ok(mut state) = session.state.lock() {
            *state = SessionState::Stopped;
        }
        self.store
            .set_conversation_state(conversation_id, &SessionState::Stopped)?;
        self.end_activity(conversation_id, &session)?;
        broadcast(
            &session,
            TerminalEvent::State {
                conversation_id: conversation_id.to_owned(),
                state: SessionState::Stopped,
            },
        );
        Ok(())
    }

    pub fn delete_conversation(&self, conversation_id: &str) -> Result<(), String> {
        self.store.ensure_standalone_chat(conversation_id)?;
        if self
            .state(conversation_id)
            .is_some_and(|state| matches!(state, SessionState::Running | SessionState::Starting))
        {
            if let Err(error) = self.stop(conversation_id) {
                if self.state(conversation_id).is_some_and(|state| {
                    matches!(state, SessionState::Running | SessionState::Starting)
                }) {
                    return Err(error);
                }
            }
        }
        self.store.delete_standalone_conversation(conversation_id)
    }

    pub fn subscribe(
        &self,
        conversation_id: &str,
    ) -> Result<(Vec<u8>, Receiver<TerminalEvent>), String> {
        let session = self.session(conversation_id)?;
        let state = session
            .state
            .lock()
            .map_err(|_| "O terminal ficou indisponível.")?;
        if *state != SessionState::Running {
            return Err("Esta conversa não tem um terminal ativo.".into());
        }
        let (sender, receiver) = mpsc::sync_channel(16);
        let history_guard = session
            .history
            .lock()
            .map_err(|_| "O histórico do terminal ficou indisponível.")?;
        let mut subscribers = session
            .subscribers
            .lock()
            .map_err(|_| "O terminal ficou indisponível.")?;
        let history = history_guard.iter().copied().collect::<Vec<_>>();
        while subscribers.len() >= 4 {
            subscribers.remove(0);
        }
        subscribers.push(sender);
        drop(subscribers);
        drop(history_guard);
        drop(state);
        Ok((history, receiver))
    }

    pub fn state(&self, conversation_id: &str) -> Option<SessionState> {
        self.sessions
            .lock()
            .ok()?
            .get(conversation_id)
            .and_then(|session| session.state.lock().ok().map(|state| state.clone()))
    }

    fn session(&self, conversation_id: &str) -> Result<Arc<LiveSession>, String> {
        self.sessions
            .lock()
            .map_err(|_| "O runtime local ficou indisponível.")?
            .get(conversation_id)
            .cloned()
            .ok_or_else(|| "Esta conversa não tem um terminal ativo.".into())
    }
}

fn resolve_codex() -> Result<PathBuf, String> {
    let mut candidates = env::split_paths(&env::var_os("PATH").unwrap_or_default())
        .map(|directory| directory.join("codex"))
        .collect::<Vec<_>>();
    if let Some(home) = env::var_os("HOME") {
        let home = PathBuf::from(home);
        candidates.push(home.join(".local/bin/codex"));
        candidates.push(home.join(".cargo/bin/codex"));
    }
    candidates.extend([
        PathBuf::from("/usr/local/bin/codex"),
        PathBuf::from("/usr/bin/codex"),
        PathBuf::from("/bin/codex"),
    ]);
    candidates
        .into_iter()
        .find(|candidate| {
            std::fs::metadata(candidate).is_ok_and(|metadata| {
                metadata.is_file() && metadata.permissions().mode() & 0o111 != 0
            })
        })
        .ok_or_else(|| {
            "O Codex CLI não foi encontrado. Instale o CLI do Codex e abra uma nova conversa."
                .into()
        })
}

fn signal_process_group(group: libc::pid_t, signal: libc::c_int) -> Result<bool, String> {
    if unsafe { libc::kill(-group, signal) } == 0 {
        return Ok(true);
    }
    let error = std::io::Error::last_os_error();
    if error.raw_os_error() == Some(libc::ESRCH) {
        Ok(false)
    } else {
        Err("Não foi possível interromper o grupo de processos do terminal.".into())
    }
}

fn publish_output(session: &LiveSession, event: TerminalEvent) {
    let TerminalEvent::Output { data, .. } = &event else {
        return;
    };
    // Keep history and subscriber registration ordered so a new subscriber
    // receives each output byte exactly once across its initial replay.
    let Ok(mut history) = session.history.lock() else {
        return;
    };
    if data.len() >= HISTORY_BYTES {
        history.clear();
        history.extend(data[data.len() - HISTORY_BYTES..].iter().copied());
    } else {
        let overflow = history
            .len()
            .saturating_add(data.len())
            .saturating_sub(HISTORY_BYTES);
        for _ in 0..overflow {
            history.pop_front();
        }
        history.extend(data.iter().copied());
    }
    if let Ok(mut subscribers) = session.subscribers.lock() {
        subscribers.retain(|subscriber| subscriber.try_send(event.clone()).is_ok());
    }
}

fn broadcast(session: &LiveSession, event: TerminalEvent) {
    if let Ok(mut subscribers) = session.subscribers.lock() {
        subscribers.retain(|subscriber| subscriber.try_send(event.clone()).is_ok());
    }
}
