use crate::store::Store;
use eros_core::{ChatActivityEvent, ChatStatus, ProviderHook};
use std::collections::HashSet;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    mpsc, Mutex,
};

/// One lock orders persistence, replay, acknowledgements and live delivery.
#[derive(Default)]
pub struct ActivityHub {
    subscribers: Mutex<Vec<(u64, mpsc::SyncSender<ChatActivityEvent>)>>,
    next_id: AtomicU64,
}

impl ActivityHub {
    pub fn subscribe<'a>(&'a self, store: &Store) -> Result<ActivitySubscription<'a>, String> {
        let mut subscribers = self
            .subscribers
            .lock()
            .map_err(|_| "O canal de atividade ficou indisponível.")?;
        let snapshot = store.activity_snapshot()?;
        let (sender, receiver) = mpsc::sync_channel(128);
        if subscribers.len() >= 8 {
            return Err("Há muitas conexões de atividade abertas.".into());
        }
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        subscribers.push((id, sender));
        Ok(ActivitySubscription {
            snapshot,
            receiver,
            id,
            hub: self,
        })
    }

    pub fn update(
        &self,
        store: &Store,
        id: &str,
        status: &ChatStatus,
        provider_session: Option<&str>,
    ) -> Result<ChatActivityEvent, String> {
        self.publish(|| store.set_chat_activity(id, status, provider_session))
    }

    pub fn acknowledge(
        &self,
        store: &Store,
        id: &str,
        revision: u64,
    ) -> Result<ChatActivityEvent, String> {
        self.publish(|| store.mark_conversation_read(id, revision))
    }

    fn publish(
        &self,
        change: impl FnOnce() -> Result<ChatActivityEvent, String>,
    ) -> Result<ChatActivityEvent, String> {
        let mut subscribers = self
            .subscribers
            .lock()
            .map_err(|_| "O canal de atividade ficou indisponível.")?;
        let event = change()?;
        subscribers.retain(|(_, sender)| sender.try_send(event.clone()).is_ok());
        Ok(event)
    }
}

pub struct ActivitySubscription<'a> {
    pub snapshot: Vec<ChatActivityEvent>,
    pub receiver: mpsc::Receiver<ChatActivityEvent>,
    id: u64,
    hub: &'a ActivityHub,
}

impl Drop for ActivitySubscription<'_> {
    fn drop(&mut self) {
        if let Ok(mut subscribers) = self.hub.subscribers.lock() {
            subscribers.retain(|(id, _)| *id != self.id);
        }
    }
}

#[derive(Default)]
pub struct ProviderActivity {
    pub session_id: Option<String>,
    turn_id: Option<String>,
    pending: HashSet<String>,
    status: ChatStatus,
}

impl ProviderActivity {
    pub fn apply(&mut self, event: &ProviderHook) -> Option<ChatStatus> {
        // Child completion must never mark the parent chat as completed.
        if event.agent_id.as_ref().is_some_and(|id| !id.is_empty()) {
            return None;
        }
        if event.hook_event_name == "SessionStart" {
            if event.source.as_deref() == Some("compact") {
                return None;
            }
            self.session_id = Some(event.session_id.clone());
            self.turn_id = None;
            self.pending.clear();
            self.status = ChatStatus::Normal;
            return Some(self.status.clone());
        }
        if self.session_id.is_none() && event.hook_event_name == "UserPromptSubmit" {
            self.session_id = Some(event.session_id.clone());
        }
        if self.session_id.as_deref() != Some(event.session_id.as_str()) {
            return None;
        }
        if event.hook_event_name == "UserPromptSubmit" {
            self.turn_id = event.turn_id.clone();
            self.pending.clear();
            self.status = ChatStatus::Working;
            return Some(self.status.clone());
        }
        if let (Some(current), Some(incoming)) = (&self.turn_id, &event.turn_id) {
            if current != incoming {
                return None;
            }
        }
        let tool = event.tool_name.as_deref().unwrap_or("");
        let key = event.tool_use_id.as_deref().unwrap_or(tool).to_owned();
        match event.hook_event_name.as_str() {
            "PermissionRequest" => {
                self.pending.insert(key);
            }
            "PreToolUse"
                if matches!(
                    tool,
                    "AskUserQuestion" | "ExitPlanMode" | "request_user_input"
                ) =>
            {
                self.pending.insert(key);
            }
            "PreToolUse" => {}
            "PostToolUse" | "PostToolUseFailure" | "PermissionDenied" => {
                self.pending.remove(&key);
                // Codex PermissionRequest does not provide tool_use_id.
                self.pending.remove(tool);
            }
            "Elicitation" => {
                self.pending.insert("elicitation".into());
            }
            "ElicitationResult" => {
                self.pending.remove("elicitation");
            }
            "Stop" => {
                self.pending.clear();
                self.status = ChatStatus::CompletedUnread;
                return Some(self.status.clone());
            }
            "Interrupt" | "StopFailure" | "SessionEnd" => return self.end(),
            _ => return None,
        }
        self.status = if self.pending.is_empty() {
            ChatStatus::Working
        } else {
            ChatStatus::AwaitingDecision
        };
        Some(self.status.clone())
    }

    pub fn end(&mut self) -> Option<ChatStatus> {
        self.pending.clear();
        if matches!(
            self.status,
            ChatStatus::Working | ChatStatus::AwaitingDecision
        ) {
            self.status = ChatStatus::Normal;
            Some(self.status.clone())
        } else {
            // Do not recreate an unread completion already acknowledged in SQLite.
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eros_core::{Conversation, SessionKind, SessionState};
    use std::path::Path;

    fn event(name: &str) -> ProviderHook {
        serde_json::from_value(serde_json::json!({
            "session_id": "provider-session", "hook_event_name": name,
            "turn_id": "turn-1", "tool_name": "Bash", "tool_use_id": "call-1",
        }))
        .unwrap()
    }

    fn store() -> Store {
        let store = Store::open(Path::new(":memory:")).unwrap();
        store
            .create_conversation(&Conversation {
                id: "chat".into(),
                project_id: None,
                name: "Chat".into(),
                cwd: "/tmp".into(),
                kind: SessionKind::Codex,
                state: SessionState::Running,
                provider_thread_id: None,
                pinned: false,
                chat_status: ChatStatus::Normal,
                activity_revision: 0,
            })
            .unwrap();
        store
    }

    #[test]
    fn lifecycle_distinguishes_idle_work_decision_and_completed() {
        let mut activity = ProviderActivity::default();
        assert_eq!(
            activity.apply(&event("SessionStart")),
            Some(ChatStatus::Normal)
        );
        assert_eq!(
            activity.apply(&event("UserPromptSubmit")),
            Some(ChatStatus::Working)
        );
        assert_eq!(
            activity.apply(&event("PermissionRequest")),
            Some(ChatStatus::AwaitingDecision)
        );
        assert_eq!(
            activity.apply(&event("PostToolUse")),
            Some(ChatStatus::Working)
        );
        assert_eq!(
            activity.apply(&event("Stop")),
            Some(ChatStatus::CompletedUnread)
        );
        assert_eq!(activity.end(), None);
    }

    #[test]
    fn codex_permission_without_call_id_resolves_and_unrelated_tools_do_not_clear_decisions() {
        let mut activity = ProviderActivity::default();
        activity.apply(&event("SessionStart"));
        activity.apply(&event("UserPromptSubmit"));
        let mut approval = event("PermissionRequest");
        approval.tool_use_id = None;
        activity.apply(&approval);
        let mut other = event("PostToolUse");
        other.tool_name = Some("Read".into());
        assert_eq!(activity.apply(&other), Some(ChatStatus::AwaitingDecision));
        assert_eq!(
            activity.apply(&event("PostToolUse")),
            Some(ChatStatus::Working)
        );
    }

    #[test]
    fn questions_and_elicitations_wait_until_answered() {
        let mut activity = ProviderActivity::default();
        activity.apply(&event("UserPromptSubmit"));
        for tool in ["AskUserQuestion", "ExitPlanMode", "request_user_input"] {
            let mut question = event("PreToolUse");
            question.tool_name = Some(tool.into());
            assert_eq!(
                activity.apply(&question),
                Some(ChatStatus::AwaitingDecision)
            );
            question.hook_event_name = "PostToolUse".into();
            assert_eq!(activity.apply(&question), Some(ChatStatus::Working));
        }
        assert_eq!(
            activity.apply(&event("Elicitation")),
            Some(ChatStatus::AwaitingDecision)
        );
        assert_eq!(
            activity.apply(&event("ElicitationResult")),
            Some(ChatStatus::Working)
        );
    }

    #[test]
    fn old_turns_other_sessions_and_children_cannot_complete_parent() {
        let mut activity = ProviderActivity::default();
        activity.apply(&event("UserPromptSubmit"));
        let mut stale = event("Stop");
        stale.turn_id = Some("old-turn".into());
        assert_eq!(activity.apply(&stale), None);
        stale = event("Stop");
        stale.session_id = "unrelated-session".into();
        assert_eq!(activity.apply(&stale), None);
        stale = event("Stop");
        stale.agent_id = Some("child".into());
        assert_eq!(activity.apply(&stale), None);
        assert_eq!(activity.status, ChatStatus::Working);
    }

    #[test]
    fn interruption_error_and_exit_never_create_completion() {
        for ending in ["Interrupt", "StopFailure", "SessionEnd"] {
            let mut activity = ProviderActivity::default();
            activity.apply(&event("UserPromptSubmit"));
            assert_eq!(activity.apply(&event(ending)), Some(ChatStatus::Normal));
        }
        let mut activity = ProviderActivity::default();
        activity.apply(&event("UserPromptSubmit"));
        assert_eq!(activity.end(), Some(ChatStatus::Normal));
    }

    #[test]
    fn completion_is_persisted_and_read_ack_is_revision_specific() {
        let store = store();
        let hub = ActivityHub::default();
        let first = hub
            .update(
                &store,
                "chat",
                &ChatStatus::CompletedUnread,
                Some("provider-session"),
            )
            .unwrap();
        let next = hub
            .update(&store, "chat", &ChatStatus::CompletedUnread, None)
            .unwrap();
        let stale_ack = hub
            .acknowledge(&store, "chat", first.activity_revision)
            .unwrap();
        assert_eq!(stale_ack.chat_status, ChatStatus::CompletedUnread);
        assert_eq!(stale_ack.activity_revision, next.activity_revision);
        store.set_active_scope("conversation:chat").unwrap();
        assert_eq!(
            store.activity_snapshot().unwrap()[0].chat_status,
            ChatStatus::CompletedUnread,
            "Selecting a stored route must not acknowledge an unseen window"
        );
        let read = hub
            .acknowledge(&store, "chat", next.activity_revision)
            .unwrap();
        assert_eq!(read.chat_status, ChatStatus::Normal);
        assert!(read.activity_revision > next.activity_revision);
        let decision = hub
            .update(&store, "chat", &ChatStatus::AwaitingDecision, None)
            .unwrap();
        assert_eq!(
            hub.acknowledge(&store, "chat", decision.activity_revision)
                .unwrap()
                .chat_status,
            ChatStatus::AwaitingDecision
        );
        let snapshot = store.snapshot().unwrap();
        assert_eq!(
            snapshot.conversations[0].provider_thread_id.as_deref(),
            Some("provider-session")
        );
        assert_eq!(snapshot.conversations[0].state, SessionState::Running);
    }

    #[test]
    fn global_stream_replays_and_updates_without_any_terminal_subscription() {
        let store = store();
        let hub = ActivityHub::default();
        let stream = hub.subscribe(&store).unwrap();
        assert_eq!(stream.snapshot[0].chat_status, ChatStatus::Normal);
        hub.update(&store, "chat", &ChatStatus::Working, None)
            .unwrap();
        assert_eq!(
            stream.receiver.try_recv().unwrap().chat_status,
            ChatStatus::Working
        );
        drop(stream);
        hub.update(&store, "chat", &ChatStatus::CompletedUnread, None)
            .unwrap();
        assert_eq!(
            hub.subscribe(&store).unwrap().snapshot[0].chat_status,
            ChatStatus::CompletedUnread
        );
    }

    #[test]
    fn slow_stream_disconnects_and_reconnect_replays_latest_revision() {
        let store = store();
        let hub = ActivityHub::default();
        let stream = hub.subscribe(&store).unwrap();
        for _ in 0..130 {
            hub.update(&store, "chat", &ChatStatus::Working, None)
                .unwrap();
        }
        assert!(hub.subscribers.lock().unwrap().is_empty());
        while stream.receiver.try_recv().is_ok() {}
        assert!(matches!(
            stream.receiver.try_recv(),
            Err(mpsc::TryRecvError::Disconnected)
        ));
        assert_eq!(
            hub.subscribe(&store).unwrap().snapshot[0].activity_revision,
            130
        );
        // Closing an idle stream immediately releases its bounded subscriber slot.
        for _ in 0..20 {
            drop(hub.subscribe(&store).unwrap());
        }
    }
}
