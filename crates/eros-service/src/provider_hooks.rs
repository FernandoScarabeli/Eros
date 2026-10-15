//! Observation-only lifecycle hooks. No prompt, approval or tool output is returned.
use eros_core::{ProviderHook, Reply, Request};
use std::io::{BufReader, Read};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::time::Duration;

const EVENTS: &[&str] = &[
    "SessionStart",
    "SessionEnd",
    "UserPromptSubmit",
    "PreToolUse",
    "PermissionRequest",
    "PostToolUse",
    "Stop",
];
const CLAUDE_EVENTS: &[&str] = &[
    "PostToolUseFailure",
    "PermissionDenied",
    "StopFailure",
    "Elicitation",
    "ElicitationResult",
];

pub fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

pub fn launch_command(kind: &eros_core::SessionKind, executable: &Path) -> Result<String, String> {
    let hook = format!(
        "{} --eros-provider-hook",
        shell_quote(executable.to_str().ok_or("Caminho do Eros inválido.")?)
    );
    let command = match kind {
        eros_core::SessionKind::Codex => {
            let mut args = vec!["codex".to_owned()];
            for event in EVENTS.iter().copied().chain(["Interrupt"]) {
                // Session overrides are additional hook sources. Existing provider settings
                // and hook trust remain in force; no global files are modified.
                let value = format!(
                    "hooks.{event}=[{{hooks=[{{type=\"command\",command={},timeout=2}}]}}]",
                    serde_json::to_string(&hook).unwrap()
                );
                args.push("-c".into());
                args.push(shell_quote(&value));
            }
            args.join(" ")
        }
        eros_core::SessionKind::Claude => {
            let mut hooks = serde_json::Map::new();
            for event in EVENTS.iter().chain(CLAUDE_EVENTS) {
                hooks.insert((*event).into(), serde_json::json!([{"hooks": [{"type": "command", "command": hook, "timeout": 2}]}]));
            }
            format!(
                "claude --settings {}",
                shell_quote(&serde_json::json!({"hooks": hooks}).to_string())
            )
        }
        eros_core::SessionKind::Shell => return Err("Shells não usam hooks de agente.".into()),
    };
    Ok(format!("{command}; exec \"$SHELL\" -i"))
}

/// Runs before Tauri initialization. A failure must not block the provider's
/// action, auto-approve anything, or leak a payload into terminal/context.
pub fn run() {
    let _ = report(std::io::stdin().lock());
    println!("{{}}");
}

fn report(input: impl Read) -> Result<(), String> {
    let mut bytes = Vec::new();
    input
        .take(eros_core::MAX_FRAME as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "Hook input unavailable")?;
    if bytes.len() > eros_core::MAX_FRAME {
        return Err("Hook input too large".into());
    }
    // Deserialize just metadata; all unrecognized provider fields are discarded.
    let event: ProviderHook =
        serde_json::from_slice(&bytes).map_err(|_| "Invalid hook metadata")?;
    validate_metadata(&event)?;
    let conversation_id =
        std::env::var("EROS_HOOK_CONVERSATION").map_err(|_| "Missing hook identity")?;
    let token = std::env::var("EROS_HOOK_TOKEN").map_err(|_| "Missing hook token")?;
    let socket = std::env::var_os("EROS_HOOK_SOCKET").ok_or("Missing hook socket")?;
    let mut stream = UnixStream::connect(socket).map_err(|_| "Hook channel unavailable")?;
    stream
        .set_read_timeout(Some(Duration::from_millis(1200)))
        .map_err(|_| "Hook timeout unavailable")?;
    stream
        .set_write_timeout(Some(Duration::from_millis(1200)))
        .map_err(|_| "Hook timeout unavailable")?;
    crate::client::write_frame(
        &mut stream,
        &Request::ProviderHook {
            conversation_id,
            token,
            event,
        },
    )?;
    let frame = crate::client::read_frame(&mut BufReader::new(stream))
        .map_err(|_| "Hook reply unavailable")?;
    let reply: Reply = serde_json::from_slice(&frame).map_err(|_| "Invalid hook reply")?;
    if reply.ok {
        Ok(())
    } else {
        Err("Hook event rejected".into())
    }
}

pub fn validate_metadata(event: &ProviderHook) -> Result<(), String> {
    let fields = [
        Some(event.session_id.as_str()),
        Some(event.hook_event_name.as_str()),
        event.turn_id.as_deref(),
        event.tool_name.as_deref(),
        event.tool_use_id.as_deref(),
        event.agent_id.as_deref(),
        event.source.as_deref(),
        event.notification_type.as_deref(),
    ];
    if event.session_id.is_empty()
        || fields
            .iter()
            .flatten()
            .any(|field| field.len() > 256 || field.chars().any(char::is_control))
    {
        return Err("Metadados de atividade inválidos.".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use eros_core::SessionKind;

    #[test]
    fn hook_payload_discards_prompts_tool_data_and_transcript_paths() {
        let event: ProviderHook = serde_json::from_value(serde_json::json!({
            "session_id": "fixture-session", "hook_event_name": "PostToolUse",
            "prompt": "private prompt", "tool_input": {"credential": "private value"},
            "tool_response": "private response", "transcript_path": "/private/transcript",
        }))
        .unwrap();
        let metadata = serde_json::to_value(event).unwrap();
        for field in ["prompt", "tool_input", "tool_response", "transcript_path"] {
            assert!(metadata.get(field).is_none());
        }
    }

    #[test]
    fn launch_preserves_interactive_cli_and_does_not_bypass_provider_safeguards() {
        for kind in [SessionKind::Codex, SessionKind::Claude] {
            let command = launch_command(&kind, Path::new("/opt/Eros app/eros")).unwrap();
            assert!(command.contains("--eros-provider-hook"));
            assert!(command.contains("PermissionRequest"));
            assert!(command.contains("UserPromptSubmit"));
            assert!(!command.contains("dangerously"));
            assert!(!command.contains("--print"));
            assert!(!command.contains("bypassPermissions"));
            assert!(command.ends_with("; exec \"$SHELL\" -i"));
        }
        assert!(launch_command(&SessionKind::Shell, Path::new("/opt/eros")).is_err());
    }

    #[test]
    fn shell_quoting_keeps_paths_literal() {
        let input = "/tmp/a b'$(printf injected)`printf injected`";
        let output = std::process::Command::new("/bin/sh")
            .args(["-c", &format!("printf %s {}", shell_quote(input))])
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(String::from_utf8(output.stdout).unwrap(), input);
    }

    #[test]
    fn metadata_is_bounded_and_malformed_input_is_not_an_event() {
        let mut event: ProviderHook =
            serde_json::from_str(r#"{"session_id":"s","hook_event_name":"Stop"}"#).unwrap();
        event.session_id = "x".repeat(257);
        assert!(validate_metadata(&event).is_err());
        event.session_id = "bad\nidentity".into();
        assert!(validate_metadata(&event).is_err());
        assert!(report(&b"not json"[..]).is_err());
        assert!(report(std::io::repeat(b'x').take(eros_core::MAX_FRAME as u64 + 1)).is_err());
    }
}
