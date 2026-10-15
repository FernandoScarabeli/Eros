use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub const PROTOCOL: u32 = 4;
pub const MAX_FRAME: usize = 2 * 1024 * 1024;
pub const HISTORY_BYTES: usize = 512 * 1024;
pub const OUTPUT_BATCH: usize = 32 * 1024;
pub const MAX_SESSIONS: usize = 24;
pub const MAX_PANELS: usize = 16;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub path: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum SessionKind {
    Shell,
    Codex,
    Claude,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum SessionState {
    Starting,
    Running,
    Exited,
    Stopped,
    Unavailable,
}

/// Task activity is independent of the lifetime of the CLI process.
/// Only explicit provider lifecycle events may change this status.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum ChatStatus {
    #[default]
    Normal,
    Working,
    CompletedUnread,
    AwaitingDecision,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct Conversation {
    pub id: String,
    pub project_id: Option<String>,
    pub name: String,
    pub cwd: String,
    pub kind: SessionKind,
    pub state: SessionState,
    pub provider_thread_id: Option<String>,
    #[serde(default)]
    pub pinned: bool,
    #[serde(default)]
    pub chat_status: ChatStatus,
    #[serde(default)]
    #[ts(type = "number")]
    pub activity_revision: u64,
}

/// Metadata only: prompts, tool arguments, responses and transcript paths never cross IPC.
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
pub struct ProviderHook {
    pub session_id: String,
    pub hook_event_name: String,
    #[serde(default)]
    pub turn_id: Option<String>,
    #[serde(default)]
    pub tool_name: Option<String>,
    #[serde(default)]
    pub tool_use_id: Option<String>,
    #[serde(default)]
    pub agent_id: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub notification_type: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
pub struct ChatActivityEvent {
    pub conversation_id: String,
    pub chat_status: ChatStatus,
    #[ts(type = "number")]
    pub activity_revision: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct Panel {
    pub id: String,
    pub conversation_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TS)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Layout {
    Leaf {
        panel_id: String,
    },
    Split {
        axis: Axis,
        ratio: f64,
        first: Box<Layout>,
        second: Box<Layout>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, TS)]
pub struct Workspace {
    pub scope: String,
    pub panels: Vec<Panel>,
    pub tree: Option<Layout>,
    pub focused_panel_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct AuthenticatedProvider {
    pub id: String,
    pub label: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
pub struct Snapshot {
    pub protocol: u32,
    pub projects: Vec<Project>,
    pub conversations: Vec<Conversation>,
    pub workspaces: Vec<Workspace>,
    #[serde(default)]
    pub sidebar_orders: SidebarOrders,
    #[serde(default)]
    pub authenticated_providers: Vec<AuthenticatedProvider>,
    pub active_scope: String,
    #[serde(default)]
    pub chat_default_folder: String,
    #[serde(default)]
    pub chat_default_folder_unavailable: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, TS)]
pub struct SidebarOrders {
    pub projects: Vec<String>,
    pub chats: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum SidebarGroup {
    Projects,
    Chats,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum Request {
    Snapshot,
    SubscribeActivity,
    ProviderHook {
        conversation_id: String,
        token: String,
        event: ProviderHook,
    },
    MarkConversationRead {
        conversation_id: String,
        activity_revision: u64,
    },
    AddProject {
        path: String,
    },
    RenameProject {
        id: String,
        name: String,
    },
    SetSidebarOrder {
        group: SidebarGroup,
        ids: Vec<String>,
    },
    CreateConversation {
        id: String,
        project_id: Option<String>,
        name: String,
        cwd: String,
        kind: SessionKind,
    },
    SetChatDefaultFolder {
        path: String,
    },
    SetConversationPinned {
        id: String,
        pinned: bool,
    },
    DeleteConversation {
        id: String,
    },
    SetActiveScope {
        scope: String,
    },
    SaveWorkspace {
        workspace: Workspace,
    },
    Write {
        conversation_id: String,
        text: String,
    },
    Resize {
        conversation_id: String,
        cols: u16,
        rows: u16,
    },
    Stop {
        conversation_id: String,
    },
    Subscribe {
        conversation_id: String,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
pub struct Reply {
    pub ok: bool,
    pub data: serde_json::Value,
    pub error: Option<String>,
}

impl Reply {
    pub fn success(value: impl Serialize) -> Self {
        Self {
            ok: true,
            data: serde_json::to_value(value).expect("serializable reply"),
            error: None,
        }
    }

    pub fn error(message: impl ToString) -> Self {
        Self {
            ok: false,
            data: serde_json::Value::Null,
            error: Some(message.to_string()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TerminalEvent {
    Output {
        conversation_id: String,
        data: Vec<u8>,
    },
    State {
        conversation_id: String,
        state: SessionState,
    },
    Error {
        message: String,
    },
}

pub fn validate_workspace(
    workspace: &Workspace,
    conversations: &[Conversation],
) -> Result<(), String> {
    let conversation_ids: Vec<&str> = conversations.iter().map(|item| item.id.as_str()).collect();
    let mut panel_ids = Vec::with_capacity(workspace.panels.len());
    for panel in &workspace.panels {
        let conversation = conversations
            .iter()
            .find(|conversation| conversation.id == panel.conversation_id);
        if panel.id.is_empty()
            || panel_ids.contains(&panel.id.as_str())
            || !conversation_ids.contains(&panel.conversation_id.as_str())
            || conversation.is_none_or(|conversation| {
                if let Some(conversation_id) = workspace.scope.strip_prefix("conversation:") {
                    conversation.id != conversation_id
                        || conversation.kind == SessionKind::Shell
                        || workspace.panels.len() > 1
                } else if workspace.scope == "chats" {
                    conversation.project_id.is_some()
                } else {
                    conversation.project_id.as_deref() != Some(workspace.scope.as_str())
                }
            })
        {
            return Err("A área de trabalho contém um painel inválido.".into());
        }
        panel_ids.push(panel.id.as_str());
    }
    if workspace.panels.len() > MAX_PANELS {
        return Err(format!(
            "Abra até {MAX_PANELS} painéis por área de trabalho."
        ));
    }
    if workspace
        .focused_panel_id
        .as_ref()
        .is_some_and(|focused| !panel_ids.contains(&focused.as_str()))
    {
        return Err("O painel em foco não pertence a esta área de trabalho.".into());
    }

    fn visit(
        node: &Layout,
        depth: usize,
        seen: &mut Vec<String>,
        allowed: &[&str],
    ) -> Result<(), String> {
        if depth > 8 {
            return Err("O layout excede oito níveis de divisão.".into());
        }
        match node {
            Layout::Leaf { panel_id } => {
                if !allowed.contains(&panel_id.as_str()) || seen.contains(panel_id) {
                    return Err("O layout contém um painel inválido ou repetido.".into());
                }
                seen.push(panel_id.clone());
            }
            Layout::Split {
                ratio,
                first,
                second,
                ..
            } => {
                if !ratio.is_finite() || !(0.15..=0.85).contains(ratio) {
                    return Err("A proporção da divisão é inválida.".into());
                }
                visit(first, depth + 1, seen, allowed)?;
                visit(second, depth + 1, seen, allowed)?;
            }
        }
        Ok(())
    }

    if let Some(tree) = &workspace.tree {
        let mut seen = Vec::new();
        visit(tree, 0, &mut seen, &panel_ids)?;
        if seen.len() != panel_ids.len() {
            return Err("Todos os painéis precisam aparecer no layout.".into());
        }
    } else if !panel_ids.is_empty() {
        return Err("O layout não pode ficar vazio enquanto houver painéis.".into());
    }
    Ok(())
}
