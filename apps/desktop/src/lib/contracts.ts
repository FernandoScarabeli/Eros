// Generated from eros-core. Run cargo run -p eros-core --bin contracts.
export const MAX_PANELS = 16;
export type Project = { id: string, name: string, path: string, };
export type SessionKind = "shell" | "codex" | "claude";
export type SessionState = "starting" | "running" | "exited" | "stopped" | "unavailable";
export type ChatStatus = "normal" | "working" | "completed_unread" | "awaiting_decision";
export type ChatActivityEvent = { conversation_id: string, chat_status: ChatStatus, activity_revision: number, };
export type Conversation = { id: string, project_id: string | null, name: string, cwd: string, kind: SessionKind, state: SessionState, provider_thread_id: string | null, pinned: boolean, chat_status: ChatStatus, activity_revision: number, };
export type Panel = { id: string, conversation_id: string, };
export type Axis = "horizontal" | "vertical";
export type Layout = { "type": "leaf", panel_id: string, } | { "type": "split", axis: Axis, ratio: number, first: Layout, second: Layout, };
export type Workspace = { scope: string, panels: Array<Panel>, tree: Layout | null, focused_panel_id: string | null, };
export type AuthenticatedProvider = { id: string, label: string, };
export type SidebarOrders = { projects: Array<string>, chats: Array<string>, };
export type SidebarGroup = "projects" | "chats";
export type Snapshot = { protocol: number, projects: Array<Project>, conversations: Array<Conversation>, workspaces: Array<Workspace>, sidebar_orders: SidebarOrders, authenticated_providers: Array<AuthenticatedProvider>, active_scope: string, chat_default_folder: string, chat_default_folder_unavailable: boolean, };
export type TerminalEvent = { "type": "output", conversation_id: string, data: Array<number>, } | { "type": "state", conversation_id: string, state: SessionState, } | { "type": "error", message: string, };
