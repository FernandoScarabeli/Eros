import { isTauri, invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { open } from '@tauri-apps/plugin-dialog';
import type { ChatActivityEvent, Conversation, Project, SessionKind, SidebarGroup, SidebarOrders, Snapshot, TerminalEvent, Workspace } from './contracts';

export const isDesktop = () => isTauri();

function requireDesktop() {
  if (!isDesktop()) {
    throw new Error('Abra o Eros desktop para acessar o sistema de arquivos e os terminais locais.');
  }
}

export async function loadSnapshot(): Promise<Snapshot> {
  requireDesktop();
  return invoke<Snapshot>('snapshot');
}

export async function chooseProjectFolder(): Promise<string | null> {
  requireDesktop();
  const selected = await open({
    directory: true,
    multiple: false,
    title: 'Escolha uma pasta para abrir no Eros',
  });
  return typeof selected === 'string' ? selected : null;
}

export async function addProject(path: string): Promise<Project> {
  return invoke<Project>('add_project', { path });
}

export async function renameProject(id: string, name: string): Promise<Project> {
  return invoke<Project>('rename_project', { id, name });
}

export async function setSidebarOrder(
  group: SidebarGroup,
  ids: string[],
): Promise<SidebarOrders> {
  return invoke<SidebarOrders>('set_sidebar_order', { group, ids });
}

export async function setActiveScope(scope: string): Promise<void> {
  return invoke('set_active_scope', { scope });
}

export async function createConversation(input: {
  id: string;
  projectId: string | null;
  name: string;
  cwd: string;
  kind: SessionKind;
}): Promise<Conversation> {
  return invoke<Conversation>('create_conversation', {
    id: input.id,
    projectId: input.projectId,
    name: input.name,
    cwd: input.cwd,
    kind: input.kind,
  });
}

export async function saveWorkspace(workspace: Workspace): Promise<Workspace> {
  return invoke<Workspace>('save_workspace', { workspace });
}

export async function writeTerminal(conversationId: string, text: string): Promise<void> {
  return invoke('write_terminal', { conversationId, text });
}

export async function resizeTerminal(conversationId: string, cols: number, rows: number): Promise<void> {
  return invoke('resize_terminal', { conversationId, cols, rows });
}

export async function stopConversation(conversationId: string): Promise<void> {
  return invoke('stop_conversation', { conversationId });
}

export async function subscribeTerminal(
  conversationId: string,
  subscriptionId: string,
): Promise<void> {
  requireDesktop();
  await invoke('subscribe_terminal', { conversationId, subscriptionId });
}

export async function unsubscribeTerminal(subscriptionId: string): Promise<void> {
  if (isDesktop()) await invoke('unsubscribe_terminal', { subscriptionId });
}

export type TerminalEventListener = (event: TerminalEvent) => void;

export async function setChatDefaultFolder(path: string): Promise<void> {
  return invoke('set_chat_default_folder', { path });
}

export async function setConversationPinned(id: string, pinned: boolean): Promise<void> {
  return invoke('set_conversation_pinned', { id, pinned });
}

export async function deleteConversation(id: string): Promise<void> {
  return invoke('delete_conversation', { id });
}

export async function markConversationRead(conversationId: string, activityRevision: number): Promise<ChatActivityEvent> {
  return invoke('mark_conversation_read', { conversationId, activityRevision });
}

/** Reconnect only after channel failure; never poll the provider or the catalog. */
export function watchChatActivity(onEvent: (event: ChatActivityEvent) => void, onFailure: (message: string) => void): () => void {
  let disposed = false;
  let cleanup: (() => Promise<void>) | undefined;
  let retry: ReturnType<typeof setTimeout> | undefined;
  let delay = 500;
  async function connect() {
    if (disposed) return;
    const id = `activity-${crypto.randomUUID()}`;
    let unlistenEvent: (() => void) | undefined;
    let unlistenDisconnect: (() => void) | undefined;
    let closed = false;
    let retryScheduled = false;
    const close = async () => {
      if (closed) return;
      closed = true;
      unlistenEvent?.();
      unlistenDisconnect?.();
      await unsubscribeTerminal(id).catch(() => {});
    };
    const failed = async () => {
      if (retryScheduled) return;
      retryScheduled = true;
      await close();
      if (disposed) return;
      onFailure('A conexão dos indicadores foi interrompida. Reconectando…');
      retry = setTimeout(() => void connect(), delay);
      delay = Math.min(delay * 2, 8000);
    };
    try {
      unlistenEvent = await listen<ChatActivityEvent>(`eros-activity-${id}`, ({ payload }) => {
        delay = 500;
        onEvent(payload);
      });
      unlistenDisconnect = await listen(`eros-activity-disconnected-${id}`, () => void failed());
      cleanup = close;
      if (disposed) { await close(); return; }
      await invoke('subscribe_activity', { subscriptionId: id });
      if (!disposed && !closed) { delay = 500; onFailure(''); }
      if (disposed || closed) await unsubscribeTerminal(id).catch(() => {});
    } catch {
      await failed();
    }
  }
  void connect();
  return () => {
    disposed = true;
    clearTimeout(retry);
    void cleanup?.();
  };
}
