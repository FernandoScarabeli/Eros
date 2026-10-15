use eros_core::{
    ChatActivityEvent, Conversation, Project, Request, SidebarGroup, SidebarOrders, Snapshot,
    TerminalEvent, Workspace, PROTOCOL,
};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::os::unix::net::UnixStream;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tauri::Emitter;

#[derive(Default)]
struct AppState {
    subscriptions: Arc<Mutex<HashMap<String, UnixStream>>>,
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            snapshot,
            benchmark_config,
            benchmark_report,
            benchmark_producers,
            benchmark_window,
            subscribe_activity,
            mark_conversation_read,
            add_project,
            rename_project,
            set_sidebar_order,
            set_active_scope,
            set_chat_default_folder,
            set_conversation_pinned,
            delete_conversation,
            create_conversation,
            save_workspace,
            write_terminal,
            resize_terminal,
            stop_conversation,
            subscribe_terminal,
            unsubscribe_terminal,
        ])
        .run(tauri::generate_context!())
        .expect("Eros desktop could not start");
}

#[tauri::command]
async fn snapshot() -> Result<Snapshot, String> {
    run_blocking(|| {
        ensure_service()?;
        eros_service::client::request(Request::Snapshot)
    })
    .await
}

#[tauri::command]
async fn mark_conversation_read(
    conversation_id: String,
    activity_revision: u64,
) -> Result<ChatActivityEvent, String> {
    request(Request::MarkConversationRead {
        conversation_id,
        activity_revision,
    })
    .await
}

#[tauri::command(rename_all = "camelCase")]
async fn subscribe_activity(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    subscription_id: String,
) -> Result<(), String> {
    if subscription_id.is_empty() || subscription_id.len() > 96 {
        return Err("Identificador de atividade inválido.".into());
    }
    let subscriptions = state.subscriptions.clone();
    let mut reader = run_blocking(|| {
        ensure_service()?;
        eros_service::client::open_activity_subscription()
    })
    .await?;
    let control = reader
        .get_ref()
        .try_clone()
        .map_err(|_| "Não foi possível acompanhar a atividade.")?;
    {
        let mut active = subscriptions
            .lock()
            .map_err(|_| "O controle das conexões ficou indisponível.")?;
        if active.contains_key(&subscription_id) {
            return Err("Esta conexão já existe.".into());
        }
        active.insert(subscription_id.clone(), control);
    }
    let thread_id = subscription_id.clone();
    let worker_subscriptions = subscriptions.clone();
    if std::thread::Builder::new()
        .name("eros-chat-activity".into())
        .spawn(move || {
            while let Ok(frame) = eros_service::client::read_frame(&mut reader) {
                let Ok(event) = serde_json::from_slice::<ChatActivityEvent>(&frame) else {
                    break;
                };
                if app
                    .emit(&format!("eros-activity-{thread_id}"), event)
                    .is_err()
                {
                    break;
                }
            }
            let was_active = worker_subscriptions
                .lock()
                .ok()
                .and_then(|mut active| active.remove(&thread_id))
                .is_some();
            if was_active {
                let _ = app.emit(&format!("eros-activity-disconnected-{thread_id}"), ());
            }
        })
        .is_err()
    {
        if let Ok(mut active) = subscriptions.lock() {
            active.remove(&subscription_id);
        }
        return Err("Não foi possível acompanhar a atividade.".into());
    }
    Ok(())
}

#[tauri::command]
async fn add_project(path: String) -> Result<Project, String> {
    request(Request::AddProject { path }).await
}

#[tauri::command]
async fn rename_project(id: String, name: String) -> Result<Project, String> {
    request(Request::RenameProject { id, name }).await
}

#[tauri::command]
async fn set_sidebar_order(group: SidebarGroup, ids: Vec<String>) -> Result<SidebarOrders, String> {
    request(Request::SetSidebarOrder { group, ids }).await
}

#[tauri::command]
async fn set_chat_default_folder(path: String) -> Result<(), String> {
    request(Request::SetChatDefaultFolder { path }).await
}

#[tauri::command]
async fn set_conversation_pinned(id: String, pinned: bool) -> Result<(), String> {
    request(Request::SetConversationPinned { id, pinned }).await
}

#[tauri::command]
async fn delete_conversation(id: String) -> Result<(), String> {
    request(Request::DeleteConversation { id }).await
}

#[tauri::command]
async fn set_active_scope(scope: String) -> Result<(), String> {
    request(Request::SetActiveScope { scope }).await
}

#[tauri::command]
async fn create_conversation(
    id: String,
    project_id: Option<String>,
    name: String,
    cwd: String,
    kind: eros_core::SessionKind,
) -> Result<Conversation, String> {
    request(Request::CreateConversation {
        id,
        project_id,
        name,
        cwd,
        kind,
    })
    .await
}

#[tauri::command]
async fn save_workspace(workspace: Workspace) -> Result<Workspace, String> {
    request(Request::SaveWorkspace { workspace }).await
}

#[tauri::command]
async fn write_terminal(conversation_id: String, text: String) -> Result<(), String> {
    request(Request::Write {
        conversation_id,
        text,
    })
    .await
}

#[tauri::command]
async fn resize_terminal(conversation_id: String, cols: u16, rows: u16) -> Result<(), String> {
    request(Request::Resize {
        conversation_id,
        cols,
        rows,
    })
    .await
}

#[tauri::command]
async fn stop_conversation(conversation_id: String) -> Result<(), String> {
    request(Request::Stop { conversation_id }).await
}

#[tauri::command(rename_all = "camelCase")]
async fn subscribe_terminal(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    conversation_id: String,
    subscription_id: String,
) -> Result<(), String> {
    if subscription_id.is_empty() || subscription_id.len() > 96 {
        return Err("Identificador de terminal inválido.".into());
    }
    let subscriptions = state.subscriptions.clone();
    let mut reader =
        run_blocking(move || eros_service::client::open_subscription(conversation_id)).await?;
    let control = reader
        .get_ref()
        .try_clone()
        .map_err(|_| "Não foi possível acompanhar o terminal.".to_string())?;
    {
        let mut active = subscriptions
            .lock()
            .map_err(|_| "O controle dos terminais ficou indisponível.")?;
        if active.contains_key(&subscription_id) {
            return Err("Esta conexão de terminal já existe.".into());
        }
        active.insert(subscription_id.clone(), control);
    }

    let event_name = format!("eros-terminal-{subscription_id}");
    let thread_id = subscription_id.clone();
    let worker_subscriptions = subscriptions.clone();
    if std::thread::Builder::new()
        .name("eros-terminal-events".into())
        .spawn(move || {
            let mut terminal_finished = false;
            loop {
                let Ok(frame) = eros_service::client::read_frame(&mut reader) else {
                    break;
                };
                let Ok(event) = serde_json::from_slice::<TerminalEvent>(&frame) else {
                    break;
                };
                if matches!(
                    &event,
                    TerminalEvent::State { state, .. } if *state != eros_core::SessionState::Running
                ) {
                    terminal_finished = true;
                }
                if app.emit(&event_name, event).is_err() {
                    break;
                }
            }
            let was_active = if let Ok(mut subscriptions) = worker_subscriptions.lock() {
                if let Some(stream) = subscriptions.remove(&thread_id) {
                    let _ = stream.shutdown(std::net::Shutdown::Both);
                    true
                } else {
                    false
                }
            } else {
                false
            };
            if was_active && !terminal_finished {
                let _ = app.emit(
                    &event_name,
                    TerminalEvent::Error {
                        message: "A conexão local com este terminal foi encerrada. Reabra o painel para reconectar.".into(),
                    },
                );
            }
        })
        .is_err()
    {
        if let Ok(mut active) = subscriptions.lock() {
            if let Some(stream) = active.remove(&subscription_id) {
                let _ = stream.shutdown(std::net::Shutdown::Both);
            }
        }
        return Err("Não foi possível acompanhar o terminal.".into());
    }
    Ok(())
}

#[tauri::command(rename_all = "camelCase")]
async fn unsubscribe_terminal(
    state: tauri::State<'_, AppState>,
    subscription_id: String,
) -> Result<(), String> {
    let subscriptions = state.subscriptions.clone();
    run_blocking(move || {
        if let Some(stream) = subscriptions
            .lock()
            .map_err(|_| "O controle dos terminais ficou indisponível.")?
            .remove(&subscription_id)
        {
            let _ = stream.shutdown(std::net::Shutdown::Both);
        }
        Ok(())
    })
    .await
}

async fn request<T: DeserializeOwned + Send + 'static>(request: Request) -> Result<T, String> {
    run_blocking(move || eros_service::client::request(request)).await
}

async fn run_blocking<T: Send + 'static>(
    task: impl FnOnce() -> Result<T, String> + Send + 'static,
) -> Result<T, String> {
    tauri::async_runtime::spawn_blocking(task)
        .await
        .map_err(|_| "A operação local foi interrompida.".to_string())?
}

static SERVICE_STARTUP: Mutex<()> = Mutex::new(());

fn check_service_protocol(snapshot: &serde_json::Value) -> Result<(), String> {
    if snapshot.get("protocol").and_then(serde_json::Value::as_u64) == Some(PROTOCOL as u64) {
        Ok(())
    } else {
        Err("O serviço local está desatualizado. Encerre o processo Eros em segundo plano e abra o aplicativo novamente.".into())
    }
}

fn ensure_service() -> Result<(), String> {
    let _startup = SERVICE_STARTUP
        .lock()
        .map_err(|_| "A inicialização do serviço foi interrompida.")?;
    if let Ok(snapshot) = eros_service::client::request::<serde_json::Value>(Request::Snapshot) {
        return check_service_protocol(&snapshot);
    }
    spawn_service()?;
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(5) {
        if let Ok(snapshot) = eros_service::client::request::<serde_json::Value>(Request::Snapshot)
        {
            return check_service_protocol(&snapshot);
        }
        std::thread::sleep(Duration::from_millis(80));
    }
    Err(
        "O serviço local do Eros não iniciou. Feche e abra o aplicativo para tentar novamente."
            .into(),
    )
}

#[cfg(unix)]
fn spawn_service() -> Result<(), String> {
    use std::os::unix::process::CommandExt;
    let executable =
        std::env::current_exe().map_err(|_| "Não foi possível localizar o Eros desktop.")?;
    let mut command = Command::new(executable);
    command
        .arg("--eros-service")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    unsafe {
        command.pre_exec(|| {
            if libc::setsid() == -1 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    command
        .spawn()
        .map(|_| ())
        .map_err(|_| "Não foi possível iniciar o serviço local do Eros.".into())
}

#[cfg(not(unix))]
fn spawn_service() -> Result<(), String> {
    let executable =
        std::env::current_exe().map_err(|_| "Não foi possível localizar o Eros desktop.")?;
    Command::new(executable)
        .arg("--eros-service")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map(|_| ())
        .map_err(|_| "Não foi possível iniciar o serviço local do Eros.".into())
}

#[cfg(test)]
mod connection_tests {
    use super::*;

    #[test]
    fn detects_old_protocol_without_requiring_the_new_snapshot_schema() {
        assert!(check_service_protocol(&serde_json::json!({"protocol": PROTOCOL})).is_ok());
        let error = check_service_protocol(&serde_json::json!({"protocol": 3})).unwrap_err();
        assert!(error.contains("desatualizado"));
        assert!(check_service_protocol(&serde_json::json!({})).is_err());
    }
}

// No monitor or file writes unless explicitly launched by the synthetic harness.
#[tauri::command]
fn benchmark_config() -> Result<Option<serde_json::Value>, String> {
    if std::env::var("EROS_BENCHMARK").as_deref() != Ok("1") {
        return Ok(None);
    }
    let raw = std::env::var("EROS_BENCH_CONFIG").map_err(|_| "Missing benchmark configuration")?;
    serde_json::from_str(&raw).map(Some).map_err(|_| "Invalid benchmark configuration".into())
}

#[tauri::command]
fn benchmark_report(value: serde_json::Value) -> Result<(), String> {
    if std::env::var("EROS_BENCHMARK").as_deref() != Ok("1") {
        return Err("Benchmark is disabled".into());
    }
    let root = std::env::var("EROS_RUNTIME_DIR").map_err(|_| "Missing isolated runtime")?;
    let data = serde_json::to_vec(&value).map_err(|_| "Invalid benchmark report")?;
    if data.len() > 65536 {
        return Err("Benchmark report exceeds 64 KiB".into());
    }
    let path = std::path::Path::new(&root).join("benchmark-ui.json");
    let temporary = path.with_extension("tmp");
    std::fs::write(&temporary, data).map_err(|_| "Cannot write benchmark report")?;
    std::fs::rename(temporary, path).map_err(|_| "Cannot publish benchmark report".into())
}


#[tauri::command]
fn benchmark_producers() -> Result<Vec<serde_json::Value>, String> {
    if std::env::var("EROS_BENCHMARK").as_deref() != Ok("1") {
        return Err("Benchmark is disabled".into());
    }
    let root = std::env::var("EROS_DATA_DIR").map_err(|_| "Missing isolated data")?;
    let mut states = Vec::new();
    for entry in std::fs::read_dir(root).map_err(|_| "Cannot read fixture counters")? {
        let entry = entry.map_err(|_| "Cannot read fixture counter")?;
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if !name.starts_with("producer-") || !name.ends_with(".json") { continue; }
        if states.len() >= 64 { return Err("Fixture counter budget exceeded".into()); }
        let metadata = entry.metadata().map_err(|_| "Cannot inspect fixture counter")?;
        if !metadata.is_file() || metadata.len() > 1024 { return Err("Invalid fixture counter".into()); }
        let raw = std::fs::read(entry.path()).map_err(|_| "Cannot read fixture counter")?;
        let value: serde_json::Value = serde_json::from_slice(&raw).map_err(|_| "Invalid fixture counter")?;
        states.push(value);
    }
    Ok(states)
}

#[tauri::command]
async fn benchmark_window(window: tauri::WebviewWindow, minimized: bool) -> Result<bool, String> {
    if std::env::var("EROS_BENCHMARK").as_deref() != Ok("1") {
        return Err("Benchmark is disabled".into());
    }
    tauri::async_runtime::spawn_blocking(move || -> Result<bool, String> {
    if minimized { window.minimize() } else { window.unminimize() }
        .map_err(|_| "Window manager refused benchmark state")?;
    for _ in 0..20 {
        if window.is_minimized().map_err(|_| "Cannot inspect native window state")? == minimized {
            return Ok(minimized);
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    Err("Native window state not confirmed within 1 second".into())
    }).await.map_err(|_| "Benchmark window worker interrupted".to_string())?
}
