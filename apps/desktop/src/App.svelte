<script lang="ts">
  import { onMount } from 'svelte';
  import ChatStatusDot, { chatStatusLabels } from './lib/ChatStatusDot.svelte';
  import HomeScreen from './lib/HomeScreen.svelte';
  import LucideIcon from './lib/LucideIcon.svelte';
  import LayoutTree from './lib/LayoutTree.svelte';
  import ProviderStatusTicker from './lib/ProviderStatusTicker.svelte';
  import SessionCreateMenu from './lib/SessionCreateMenu.svelte';
  import TerminalPane from './lib/TerminalPane.svelte';
  import {
    MAX_PANELS,
    type ChatActivityEvent,
    type Conversation,
    type Layout,
    type Project,
    type SidebarGroup,
    type SessionKind,
    type SessionState,
    type Snapshot,
    type Workspace,
  } from './lib/contracts';
  import {
    addProject,
    chooseProjectFolder,
    createConversation,
    deleteConversation,
    isDesktop,
    loadSnapshot,
    markConversationRead,
    watchChatActivity,
    renameProject,
    saveWorkspace,
    stopConversation,
    setSidebarOrder,
    setActiveScope,
    setChatDefaultFolder,
    setConversationPinned,
  } from './lib/api';

  function errorMessageFrom(error: unknown, fallback: string): string {
    if (error instanceof Error && error.message.trim()) return error.message;
    if (typeof error === 'string' && error.trim()) return error;
    if (typeof error === 'object' && error !== null && 'message' in error) {
      const message = (error as { message?: unknown }).message;
      if (typeof message === 'string' && message.trim()) return message;
    }
    return fallback;
  }

  const emptyWorkspace = (scope: string): Workspace => ({
    scope,
    panels: [],
    tree: null,
    focused_panel_id: null,
  });

  let snapshot = $state<Snapshot | null>(null);
  const activityCache = new Map<string, ChatActivityEvent>();
  const acknowledgements = new Set<string>();
  let windowFocused = $state(false);
  let activityMessage = $state('');
  let activeScope = $state('chats');
  let activeConversationId = $state<string | null>(null);
  let visitedTabScopes = $state<string[]>([]);
  let desktopReady = $state(false);
  let loading = $state(true);
  let busy = $state(false);
  let errorMessage = $state('');
  let searchQuery = $state('');
  let expandedProjectId = $state<string | null>(null);
  let contextMenu = $state<string | null>(null);
  let contextMenuPosition = $state({ top: 0, left: 0 });
  let showProjectSettings = $state(false);
  let showChatSettings = $state(false);
  let showRenameProject = $state(false);
  let showReorderDialog = $state(false);
  let deleteChatTarget = $state<Conversation | null>(null);
  let deleteChatBusy = $state(false);
  let deleteChatError = $state('');
  let settingsProject = $state<Project | null>(null);
  let renameProjectId = $state('');
  let renameProjectName = $state('');
  let renameProjectError = $state('');
  let projectActionBusy = $state(false);
  let reorderGroup = $state<SidebarGroup>('projects');
  let reorderRows = $state<Array<{ id: string; name: string }>>([]);
  let reorderError = $state('');
  let maximizedPanelId = $state<string | null>(null);
  let sidebarCollapsed = $state(false);
  let showHome = $state(true);
  let projectsExpanded = $state(true);
  let chatsExpanded = $state(true);
  let searchInput = $state<HTMLInputElement>();
  let renameNameInput = $state<HTMLInputElement>();
  let contextMenuElement = $state<HTMLDivElement>();
  let projectSettingsCloseButton = $state<HTMLButtonElement>();
  let reorderCloseButton = $state<HTMLButtonElement>();
  let deleteChatCancelButton = $state<HTMLButtonElement>();
  let lastDialogTrigger: HTMLElement | null = null;
  let lastContextMenuTrigger: HTMLElement | null = null;
  let workspaceSaveTimer: ReturnType<typeof setTimeout> | undefined;

  const activeProject = $derived(
    snapshot?.projects.find((project) => project.id === activeScope) ?? null,
  );
  const activeWorkspaceScope = $derived(activeConversationId ? `conversation:${activeConversationId}` : activeScope);
  const activeWorkspace = $derived(
    snapshot?.workspaces.find((workspace) => workspace.scope === activeWorkspaceScope) ??
      emptyWorkspace(activeWorkspaceScope),
  );
  const selectedTabScope = $derived(showHome ? '__home__' : activeScope);
  const workspaceTabs = $derived.by(() => {
    const scopes = [...new Set([
      ...(snapshot?.workspaces.filter((workspace) => workspace.panels.length > 0).map((workspace) => workspace.scope) ?? []),
      ...visitedTabScopes,
      activeScope,
    ])];
    return [{ scope: '__home__', name: 'Início' }, ...scopes.flatMap((scope) => {
      const project = snapshot?.projects.find((item) => item.id === scope);
      if (scope !== 'chats' && !project) return [];
      return [{ scope, name: project?.name ?? 'Chats' }];
    })];
  });
  $effect(() => {
    if (!showHome && !visitedTabScopes.includes(activeScope)) {
      visitedTabScopes = [...visitedTabScopes, activeScope];
    }
  });

  async function selectWorkspaceTab(scope: string) {
    if (scope === '__home__') goHome();
    else await selectScope(scope);
  }

  function navigateWorkspaceTabs(event: KeyboardEvent) {
    const index = workspaceTabs.findIndex((tab) => tab.scope === selectedTabScope);
    let next = index;
    if (event.key === 'ArrowRight') next = (index + 1) % workspaceTabs.length;
    else if (event.key === 'ArrowLeft') next = (index - 1 + workspaceTabs.length) % workspaceTabs.length;
    else if (event.key === 'Home') next = 0;
    else if (event.key === 'End') next = workspaceTabs.length - 1;
    else return;
    event.preventDefault();
    const scope = workspaceTabs[next]?.scope;
    if (!scope || busy) return;
    void selectWorkspaceTab(scope).then(() => {
      document.getElementById(`workspace-tab-${selectedTabScope}`)?.focus();
    });
  }

  const scopedConversations = $derived(
    (snapshot?.conversations ?? []).filter((conversation) => {
      return activeScope === 'chats'
        ? conversation.project_id === null
        : conversation.project_id === activeScope;
    }),
  );
  const matchingScopedConversations = $derived(
    scopedConversations.filter((conversation) => conversation.kind !== 'shell').filter(matchesConversation),
  );
  const matchingProjects = $derived(
    orderedBySavedIds(
      snapshot?.projects ?? [],
      snapshot?.sidebar_orders.projects ?? [],
    ).filter((project) => {
      const query = searchQuery.trim().toLowerCase();
      return (
        !query ||
        project.name.toLowerCase().includes(query) ||
        project.path.toLowerCase().includes(query) ||
        projectConversations(project).some(matchesConversation)
      );
    }),
  );
  const matchingStandaloneChats = $derived(
    orderedBySavedIds(
      (snapshot?.conversations ?? []).filter(
        (conversation) => conversation.project_id === null && conversation.kind !== 'shell',
      ),
      snapshot?.sidebar_orders.chats ?? [],
    ).sort((left, right) => Number(right.pinned) - Number(left.pinned)).filter(matchesConversation),
  );
  const selectedMaximizedPanel = $derived(
    maximizedPanelId
      ? (activeWorkspace.panels.find(
          (panel) => panel.id === maximizedPanelId,
        ) ?? null)
      : null,
  );
  const selectedMaximizedConversation = $derived(
    selectedMaximizedPanel
      ? (snapshot?.conversations.find(
          (conversation) =>
            conversation.id === selectedMaximizedPanel.conversation_id,
        ) ?? null)
      : null,
  );

  $effect(() => {
    if (maximizedPanelId && !activeWorkspace.panels.some((panel) => panel.id === maximizedPanelId)) {
      maximizedPanelId = null;
    }
  });

  $effect(() => {
    if (!isDesktop() || !desktopReady) return;
    return watchChatActivity(receiveChatActivity, (message) => { activityMessage = message; });
  });

  function receiveChatActivity(event: ChatActivityEvent) {
    const known = activityCache.get(event.conversation_id);
    if (known && known.activity_revision > event.activity_revision) return;
    activityCache.set(event.conversation_id, event);
    activityMessage = '';
    if (snapshot) {
      snapshot = { ...snapshot, conversations: snapshot.conversations.map((conversation) =>
        conversation.id === event.conversation_id && (conversation.activity_revision ?? 0) <= event.activity_revision
          ? { ...conversation, chat_status: event.chat_status, activity_revision: event.activity_revision }
          : conversation,
      ) };
    }
  }

  $effect(() => {
    if (!windowFocused || showHome || !activeConversationId || !desktopReady) return;
    if (!activeWorkspace.tree || !activeWorkspace.panels.some(panel => panel.conversation_id === activeConversationId)) return;
    const conversation = snapshot?.conversations.find((item) => item.id === activeConversationId);
    if (!conversation || conversation.chat_status !== 'completed_unread') return;
    const revision = conversation.activity_revision ?? 0;
    const key = `${conversation.id}:${revision}`;
    if (acknowledgements.has(key)) return;
    acknowledgements.add(key);
    void markConversationRead(conversation.id, revision)
      .then(receiveChatActivity)
      .catch(() => { activityMessage = 'Não foi possível marcar a conclusão como vista. Reabra o chat para tentar novamente.'; })
      .finally(() => acknowledgements.delete(key));
  });

  async function refresh(restoreSelectedChat = false) {
    const loaded = await loadSnapshot();
    snapshot = normalizeSnapshot(loaded);
    const conversationId = loaded.active_scope.startsWith('conversation:')
      ? loaded.active_scope.slice('conversation:'.length)
      : null;
    const activeConversation = conversationId
      ? snapshot.conversations.find((conversation) => conversation.id === conversationId && conversation.kind !== 'shell')
      : null;
    activeConversationId = activeConversation?.id ?? null;
    if (restoreSelectedChat && activeConversation) showHome = false;
    activeScope = activeConversation?.project_id ?? (activeConversation ? 'chats' : loaded.active_scope.startsWith('conversation:') ? 'chats' : loaded.active_scope);
    desktopReady = true;
    if (!expandedProjectId && activeScope !== 'chats')
      expandedProjectId = activeScope;
    errorMessage = '';
  }

  onMount(() => {
    const updateWindowFocus = () => { windowFocused = document.visibilityState === 'visible' && document.hasFocus(); };
    updateWindowFocus();
    window.addEventListener('focus', updateWindowFocus);
    window.addEventListener('blur', updateWindowFocus);
    document.addEventListener('visibilitychange', updateWindowFocus);
    const removeFocusListeners = () => {
      window.removeEventListener('focus', updateWindowFocus);
      window.removeEventListener('blur', updateWindowFocus);
      document.removeEventListener('visibilitychange', updateWindowFocus);
    };
    sidebarCollapsed = window.matchMedia('(max-width: 620px)').matches;
    const handleOutsidePointer = (event: PointerEvent) => {
      if (!contextMenu) return;
      const target = event.target;
      if (
        target instanceof Element &&
        target.closest('.sidebar-context-menu, [data-context-trigger]')
      ) return;
      closeContextMenu(false);
    };
    const handleShortcut = (event: KeyboardEvent) => {
      if (deleteChatTarget) {
        if (event.key === 'Escape' && !deleteChatBusy) {
          event.preventDefault();
          closeDeleteChatDialog();
        }
        return;
      }
      if (showRenameProject || showProjectSettings || showReorderDialog) {
        if (event.key === 'Escape' && !projectActionBusy) {
          if (showRenameProject) closeRenameDialog();
          else if (showProjectSettings) closeProjectSettings();
          else closeReorderDialog();
        }
        return;
      }
      if (event.key === 'Escape' && contextMenu) {
        event.preventDefault();
        closeContextMenu();
        return;
      }
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === 'n') {
        event.preventDefault();
        void startTerminal();
      }
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === 'k') {
        event.preventDefault();
        sidebarCollapsed = false;
        requestAnimationFrame(() => searchInput?.focus());
        searchInput?.select();
      }
      if (event.key === 'Escape') {
        maximizedPanelId = null;
        if (window.matchMedia('(max-width: 620px)').matches) sidebarCollapsed = true;
      }
    };
    window.addEventListener('keydown', handleShortcut);
    document.addEventListener('pointerdown', handleOutsidePointer, true);
    if (!isDesktop()) {
      loading = false;
      return () => {
        removeFocusListeners();
        window.removeEventListener('keydown', handleShortcut);
        document.removeEventListener('pointerdown', handleOutsidePointer, true);
      };
    }
    void refresh(true)
      .catch((error: unknown) => {
        errorMessage = errorMessageFrom(error, 'O serviço local não respondeu.');
        showHome = false;
      })
      .finally(() => {
        loading = false;
      });
    return () => {
      removeFocusListeners();
      window.removeEventListener('keydown', handleShortcut);
      document.removeEventListener('pointerdown', handleOutsidePointer, true);
    };
  });

  $effect(() => {
    if (showRenameProject)
      requestAnimationFrame(() => renameNameInput?.focus());
  });

  $effect(() => {
    if (showProjectSettings)
      requestAnimationFrame(() => projectSettingsCloseButton?.focus());
  });

  $effect(() => {
    if (showReorderDialog)
      requestAnimationFrame(() => reorderCloseButton?.focus());
  });

  $effect(() => {
    if (deleteChatTarget)
      requestAnimationFrame(() => deleteChatCancelButton?.focus());
  });

  $effect(() => {
    if (contextMenu)
      requestAnimationFrame(() =>
        contextMenuElement?.querySelector<HTMLButtonElement>(
          '[role="menuitem"]:not(:disabled)',
        )?.focus(),
      );
  });

  function goHome() {
    showHome = true;
    if (window.matchMedia('(max-width: 620px)').matches) sidebarCollapsed = true;
    if (isDesktop())
      void refresh().catch((error: unknown) => {
        errorMessage =
          error instanceof Error
            ? error.message
            : 'Não foi possível atualizar o workspace.';
      });
  }

  function setLocalSnapshot(nextWorkspace: Workspace) {
    if (!snapshot) return;
    const exists = snapshot.workspaces.some(
      (workspace) => workspace.scope === nextWorkspace.scope,
    );
    snapshot = {
      ...snapshot,
      workspaces: exists
        ? snapshot.workspaces.map((workspace) =>
            workspace.scope === nextWorkspace.scope ? nextWorkspace : workspace,
          )
        : [...snapshot.workspaces, nextWorkspace],
    };
  }

  async function persistWorkspace(workspace: Workspace) {
    setLocalSnapshot(workspace);
    try {
      await saveWorkspace(workspace);
      errorMessage = '';
    } catch (error) {
      errorMessage =
        error instanceof Error
          ? error.message
          : 'Não foi possível salvar o layout.';
    }
  }

  async function selectScope(scope: string, revealWorkspace = true) {
    if (!snapshot) return;
    if (scope === activeScope && !activeConversationId) {
      if (revealWorkspace) showHome = false;
      return;
    }
    busy = true;
    try {
      await setActiveScope(scope);
      activeScope = scope;
      activeConversationId = null;
      if (revealWorkspace) showHome = false;
      expandedProjectId = scope === 'chats' ? null : scope;
      maximizedPanelId = null;
      errorMessage = '';
    } catch (error) {
      errorMessage =
        error instanceof Error
          ? error.message
          : 'Não foi possível abrir este espaço.';
    } finally {
      busy = false;
    }
  }

  function selectProject(projectId: string) {
    showHome = false;
    if (activeScope === projectId) {
      expandedProjectId = expandedProjectId === projectId ? null : projectId;
      return;
    }
    void selectScope(projectId);
  }

  function toggleContextMenu(target: string, event: MouseEvent) {
    const trigger = event.currentTarget as HTMLElement;
    if (contextMenu === target) {
      closeContextMenu();
      return;
    }
    lastContextMenuTrigger = trigger;
    const bounds = trigger.getBoundingClientRect();
    const menuWidth = target === 'top-projects' ? 248 : 214;
    const menuHeight = target === 'top-projects'
      ? Math.min(380, 48 + (snapshot?.projects.length ?? 0) * 36)
      : target.startsWith('chat:') ? 84 : 146;
    contextMenuPosition = {
      top: Math.max(8, Math.min(bounds.bottom + 4, window.innerHeight - menuHeight - 8)),
      left: Math.max(8, Math.min(bounds.right - menuWidth, window.innerWidth - menuWidth - 8)),
    };
    contextMenu = target;
    requestAnimationFrame(() =>
      contextMenuElement
        ?.querySelector<HTMLButtonElement>('[role="menuitem"]:not(:disabled)')
        ?.focus(),
    );
  }

  function closeContextMenu(restoreFocus = true) {
    contextMenu = null;
    if (restoreFocus)
      requestAnimationFrame(() => lastContextMenuTrigger?.focus());
  }

  function handleContextMenuKeydown(event: KeyboardEvent) {
    const menu = event.currentTarget as HTMLElement;
    const items = Array.from(
      menu.querySelectorAll<HTMLButtonElement>('[role="menuitem"]:not(:disabled)'),
    );
    if (!items.length) return;
    const current = items.indexOf(document.activeElement as HTMLButtonElement);
    let next = current;
    if (event.key === 'ArrowDown')
      next = current < 0 ? 0 : (current + 1) % items.length;
    else if (event.key === 'ArrowUp')
      next = current < 0 ? items.length - 1 : (current - 1 + items.length) % items.length;
    else if (event.key === 'Home') next = 0;
    else if (event.key === 'End') next = items.length - 1;
    else return;
    event.preventDefault();
    items[next]?.focus();
  }

  async function createInScope(scope: string, kind: SessionKind, name = '') {
    if (!desktopReady) {
      errorMessage = 'Abra o Eros desktop para iniciar uma sessão local.';
      return;
    }
    closeContextMenu(false);
    await selectScope(scope, false);
    if (activeScope !== scope) return;
    await createConversationInCurrentScope(kind, name);
  }

  function openProjectSettings(project: Project) {
    lastDialogTrigger = lastContextMenuTrigger;
    closeContextMenu(false);
    settingsProject = project;
    showProjectSettings = true;
  }

  function renameConfiguredProject() {
    const project = settingsProject;
    if (project) openRenameProject(project, lastDialogTrigger);
  }

  function closeProjectSettings(restoreFocus = true) {
    showProjectSettings = false;
    settingsProject = null;
    if (restoreFocus)
      requestAnimationFrame(() => lastDialogTrigger?.focus());
  }

  function openRenameProject(project: Project, returnFocus = lastContextMenuTrigger) {
    closeContextMenu(false);
    showProjectSettings = false;
    settingsProject = null;
    renameProjectId = project.id;
    renameProjectName = project.name;
    renameProjectError = '';
    lastDialogTrigger = returnFocus;
    showRenameProject = true;
  }

  function closeRenameDialog(restoreFocus = true) {
    if (projectActionBusy) return;
    showRenameProject = false;
    renameProjectError = '';
    if (restoreFocus)
      requestAnimationFrame(() => lastDialogTrigger?.focus());
  }

  async function submitProjectRename(event: SubmitEvent) {
    event.preventDefault();
    if (!snapshot) return;
    projectActionBusy = true;
    renameProjectError = '';
    try {
      const project = await renameProject(renameProjectId, renameProjectName);
      const projects = snapshot.projects.map((item) =>
        item.id === project.id ? project : item,
      );
      if (!snapshot.sidebar_orders.projects.length)
        projects.sort((left, right) => left.name.localeCompare(right.name, undefined, { sensitivity: 'base' }));
      snapshot = { ...snapshot, projects };
      showRenameProject = false;
      requestAnimationFrame(() => lastDialogTrigger?.focus());
    } catch (error) {
      renameProjectError =
        error instanceof Error ? error.message : 'Não foi possível renomear o projeto.';
    } finally {
      projectActionBusy = false;
    }
  }

  function beginReordering(group: SidebarGroup) {
    lastDialogTrigger = lastContextMenuTrigger;
    closeContextMenu(false);
    reorderGroup = group;
    const source = group === 'projects'
      ? orderedBySavedIds(snapshot?.projects ?? [], snapshot?.sidebar_orders.projects ?? [])
      : orderedBySavedIds(
          (snapshot?.conversations ?? []).filter((conversation) => conversation.project_id === null && conversation.kind !== 'shell'),
          snapshot?.sidebar_orders.chats ?? [],
        ).sort((left, right) => Number(right.pinned) - Number(left.pinned));
    reorderRows = source.map(({ id, name }) => ({ id, name }));
    reorderError = '';
    showReorderDialog = true;
  }

  function moveReorderRow(index: number, offset: -1 | 1) {
    const nextIndex = index + offset;
    if (nextIndex < 0 || nextIndex >= reorderRows.length) return;
    const next = [...reorderRows];
    [next[index], next[nextIndex]] = [next[nextIndex], next[index]];
    reorderRows = next;
  }

  function closeReorderDialog(restoreFocus = true) {
    if (projectActionBusy) return;
    showReorderDialog = false;
    reorderError = '';
    if (restoreFocus)
      requestAnimationFrame(() => lastDialogTrigger?.focus());
  }

  async function toggleChatPinned(conversation: Conversation) {
    closeContextMenu(false);
    try {
      await setConversationPinned(conversation.id, !conversation.pinned);
      await refresh();
      errorMessage = '';
    } catch (error) {
      errorMessage = errorMessageFrom(error, 'Não foi possível atualizar este chat.');
    }
  }

  function requestDeleteChat(conversation: Conversation) {
    lastDialogTrigger = lastContextMenuTrigger;
    closeContextMenu(false);
    deleteChatError = '';
    deleteChatTarget = conversation;
  }

  function closeDeleteChatDialog() {
    if (deleteChatBusy) return;
    deleteChatTarget = null;
    deleteChatError = '';
    requestAnimationFrame(() => {
      document.querySelector<HTMLButtonElement>('[aria-label="Chats"] .section-title')?.focus();
    });
  }

  async function confirmDeleteChat() {
    const conversation = deleteChatTarget;
    if (!conversation) return;
    deleteChatBusy = true;
    deleteChatError = '';
    try {
      await deleteConversation(conversation.id);
      deleteChatTarget = null;
      await refresh();
      requestAnimationFrame(() => {
        document.querySelector<HTMLButtonElement>('[aria-label="Chats"] .section-title')?.focus();
      });
    } catch (error) {
      deleteChatError = errorMessageFrom(error, 'Não foi possível excluir este chat.');
    } finally {
      deleteChatBusy = false;
    }
  }

  async function saveReorder() {
    if (!snapshot) return;
    projectActionBusy = true;
    reorderError = '';
    try {
      const sidebar_orders = await setSidebarOrder(
        reorderGroup,
        reorderRows.map((row) => row.id),
      );
      snapshot = { ...snapshot, sidebar_orders };
      showReorderDialog = false;
      requestAnimationFrame(() => lastDialogTrigger?.focus());
    } catch (error) {
      reorderError =
        error instanceof Error ? error.message : 'Não foi possível salvar esta ordem.';
    } finally {
      projectActionBusy = false;
    }
  }

  async function openProjectFolder() {
    errorMessage = '';
    try {
      const path = await chooseProjectFolder();
      if (!path) return;
      const project = await addProject(path);
      await refresh();
      await selectScope(project.id);
      expandedProjectId = project.id;
    } catch (error) {
      errorMessage =
        error instanceof Error
          ? error.message
          : 'Não foi possível abrir essa pasta.';
    }
  }

  async function startFromHome(kind: SessionKind, name = '', projectId = '') {
    showHome = true;
    await createInScope(projectId || 'chats', kind, name);
  }

  async function startTerminal() {
    if (busy || loading) return;
    if (!desktopReady || !snapshot) {
      errorMessage = 'Abra o Eros desktop para criar um terminal conectado ao seu computador.';
      return;
    }
    const shellWorkspace = snapshot.workspaces.find((workspace) => workspace.scope === activeScope) ?? emptyWorkspace(activeScope);
    if (shellWorkspace.panels.length >= MAX_PANELS) {
      errorMessage = `Esta área de trabalho já tem o máximo de ${MAX_PANELS} painéis. Feche um painel antes de abrir outro terminal.`;
      return;
    }
    busy = true;
    errorMessage = '';
    try {
      const conversation = await createConversation({
        id: crypto.randomUUID(),
        projectId: activeProject?.id ?? null,
        name: 'Terminal',
        cwd: activeProject?.path ?? '',
        kind: 'shell',
      });
      await refresh();
      activeConversationId = null;
      await setActiveScope(activeScope);
      await addConversationPanel(conversation);
      maximizedPanelId = null;
      showHome = false;
    } catch (error) {
      errorMessage = errorMessageFrom(error, 'Não foi possível abrir o terminal.');
    } finally {
      busy = false;
    }
  }

  function retryConnection() {
    loading = true;
    errorMessage = '';
    void refresh()
      .catch((error: unknown) => {
        errorMessage = errorMessageFrom(error, 'O serviço local não respondeu.');
        showHome = false;
      })
      .finally(() => (loading = false));
  }

  async function updateChatFolder(path: string) {
    busy = true;
    try {
      await setChatDefaultFolder(path);
      await refresh();
    } catch (error) {
      errorMessage = errorMessageFrom(error, 'Não foi possível salvar a pasta.');
    } finally { busy = false; }
  }

  async function chooseChatFolder() {
    try {
      const path = await chooseProjectFolder();
      if (path) await updateChatFolder(path);
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : 'Não foi possível escolher a pasta.';
    }
  }

  async function createConversationInCurrentScope(kind: SessionKind, name = '') {
    if (kind === 'shell') {
      await startTerminal();
      return;
    }
    if (busy || loading) return;
    if (!desktopReady || !snapshot) {
      errorMessage = 'Abra o Eros desktop para iniciar uma sessão local.';
      return;
    }
    busy = true;
    errorMessage = '';
    try {
      const cwd = activeProject?.path ?? '';
      const conversation = await createConversation({
        id: crypto.randomUUID(),
        projectId: activeProject?.id ?? null,
        name: name.trim().slice(0, 80) || (kind === 'codex' ? 'Codex' : 'Claude Code'),
        cwd,
        kind,
      });
      await refresh();
      activeConversationId = conversation.id;
      await setActiveScope(`conversation:${conversation.id}`);
      await addConversationPanel(conversation);
      showHome = false;
    } catch (error) {
      errorMessage = errorMessageFrom(error, 'Não foi possível iniciar a sessão.');
    } finally {
      busy = false;
    }
  }

  async function addConversationPanel(conversation: Conversation) {
    const scope = conversation.kind === 'shell'
      ? activeScope
      : `conversation:${conversation.id}`;
    const current = snapshot?.workspaces.find((workspace) => workspace.scope === scope) ?? emptyWorkspace(scope);
    if (conversation.kind !== 'shell' && current.panels.length > 0) {
      throw new Error('Este chat já tem um terminal aberto.');
    }
    if (conversation.kind === 'shell' && current.panels.length >= MAX_PANELS) {
      throw new Error(`Esta área de trabalho já tem o máximo de ${MAX_PANELS} painéis.`);
    }
    const panel = { id: crypto.randomUUID(), conversation_id: conversation.id };
    let tree: Layout = { type: 'leaf', panel_id: panel.id };
    if (current.tree) {
      tree = {
        type: 'split', axis: 'horizontal', ratio: 0.5, first: current.tree,
        second: { type: 'leaf', panel_id: panel.id },
      };
    }
    maximizedPanelId = null;
    await persistWorkspace({ ...current, panels: [...current.panels, panel], tree, focused_panel_id: panel.id });
  }

  async function openConversation(conversation: Conversation) {
    try {
      const latest = normalizeSnapshot(await loadSnapshot());
      snapshot = latest;
      desktopReady = true;
      const currentConversation = latest.conversations.find((item) => item.id === conversation.id);
      if (!currentConversation) throw new Error('Esta conversa não está mais no catálogo local.');
      const projectScope = currentConversation.project_id ?? 'chats';
      const workspaceScope = currentConversation.kind === 'shell'
        ? projectScope
        : `conversation:${currentConversation.id}`;
      await setActiveScope(workspaceScope);
      activeScope = projectScope;
      if (projectScope !== 'chats') expandedProjectId = projectScope;
      activeConversationId = currentConversation.kind === 'shell' ? null : currentConversation.id;
      showHome = false;
      const workspace = latest.workspaces.find((item) => item.scope === workspaceScope) ?? emptyWorkspace(workspaceScope);
      const existing = workspace.panels.find((panel) => panel.conversation_id === currentConversation.id);
      if (existing) await focusPanel(existing.id);
      else await addConversationPanel(currentConversation);
    } catch (error) {
      errorMessage = errorMessageFrom(error, 'Não foi possível abrir esta conversa.');
    }
  }

  async function focusPanel(panelId: string) {
    if (panelId === activeWorkspace.focused_panel_id) return;
    await persistWorkspace({ ...activeWorkspace, focused_panel_id: panelId });
  }

  async function endShell(conversation: Conversation) {
    if (busy) return;
    busy = true;
    try {
      await stopConversation(conversation.id);
      await refresh();
    } catch (error) {
      errorMessage = errorMessageFrom(error, 'Não foi possível encerrar o terminal.');
    } finally {
      busy = false;
    }
  }

  async function closePanel(panelId: string) {
    const panel = activeWorkspace.panels.find((item) => item.id === panelId);
    const conversation = snapshot?.conversations.find((item) => item.id === panel?.conversation_id);
    if (conversation?.kind === 'shell') await endShell(conversation);
    else await detachPanel(panelId);
  }

  async function detachPanel(panelId: string) {
    const tree = activeWorkspace.tree
      ? removeLeaf(activeWorkspace.tree, panelId)
      : null;
    const panels = activeWorkspace.panels.filter(
      (panel) => panel.id !== panelId,
    );
    const nextFocus = panels.some(
      (panel) => panel.id === activeWorkspace.focused_panel_id,
    )
      ? activeWorkspace.focused_panel_id
      : (panels.at(-1)?.id ?? null);
    maximizedPanelId = null;
    await persistWorkspace({
      ...activeWorkspace,
      panels,
      tree,
      focused_panel_id: nextFocus,
    });
  }

  function removeLeaf(node: Layout, panelId: string): Layout | null {
    if (node.type === 'leaf') return node.panel_id === panelId ? null : node;
    const first = removeLeaf(node.first, panelId);
    const second = removeLeaf(node.second, panelId);
    if (!first) return second;
    if (!second) return first;
    return { ...node, first, second };
  }

  function resizeSplit(node: Layout, path: string, ratio: number): Layout {
    if (node.type === 'leaf') return node;
    if (!path) return { ...node, ratio };
    const [direction, ...rest] = path;
    const nextPath = rest.join('');
    return direction === '0'
      ? { ...node, first: resizeSplit(node.first, nextPath, ratio) }
      : { ...node, second: resizeSplit(node.second, nextPath, ratio) };
  }

  async function updateSplit(path: string, ratio: number) {
    if (!activeWorkspace.tree) return;
    const workspace = {
      ...activeWorkspace,
      tree: resizeSplit(activeWorkspace.tree, path, ratio),
    };
    setLocalSnapshot(workspace);
    if (workspaceSaveTimer) clearTimeout(workspaceSaveTimer);
    workspaceSaveTimer = setTimeout(() => {
      void saveWorkspace(workspace).catch((error: unknown) => {
        errorMessage =
          error instanceof Error
            ? error.message
            : 'Não foi possível salvar o layout.';
      });
    }, 180);
  }

  async function toggleMaximize(panelId: string) {
    await focusPanel(panelId);
    maximizedPanelId = maximizedPanelId === panelId ? null : panelId;
  }

  function updateConversationState(
    conversationId: string,
    state: SessionState,
  ) {
    if (!snapshot) return;
    snapshot = normalizeSnapshot({
      ...snapshot,
      conversations: snapshot.conversations.map((conversation) =>
        conversation.id === conversationId
          ? { ...conversation, state }
          : conversation,
      ),
    });
  }

  function projectConversations(project: Project) {
    return (snapshot?.conversations ?? []).filter(
      (conversation) => conversation.project_id === project.id && conversation.kind !== 'shell',
    );
  }

  function normalizeSnapshot(loaded: Snapshot): Snapshot {
    loaded = { ...loaded, conversations: loaded.conversations.map((conversation) => {
      const event = activityCache.get(conversation.id);
      return event && event.activity_revision >= (conversation.activity_revision ?? 0)
        ? { ...conversation, chat_status: event.chat_status, activity_revision: event.activity_revision }
        : conversation;
    }) };
    const conversations = loaded.conversations.filter((item) =>
      item.kind !== 'shell' || ['running', 'starting'].includes(item.state),
    );
    const ids = new Set(conversations.map((item) => item.id));
    return {
      ...loaded,
      conversations,
      workspaces: loaded.workspaces.map((workspace) => {
        let tree = workspace.tree;
        for (const panel of workspace.panels) {
          const conversation = conversations.find((item) => item.id === panel.conversation_id);
          const belongsHere = conversation?.kind === 'shell' || workspace.scope === `conversation:${panel.conversation_id}`;
          if ((!ids.has(panel.conversation_id) || !belongsHere) && tree) tree = removeLeaf(tree, panel.id);
        }
        const panels = workspace.panels.filter((panel) => {
          const conversation = conversations.find((item) => item.id === panel.conversation_id);
          return ids.has(panel.conversation_id) && (conversation?.kind === 'shell' || workspace.scope === `conversation:${panel.conversation_id}`);
        });
        return { ...workspace, panels, tree,
          focused_panel_id: panels.some((panel) => panel.id === workspace.focused_panel_id)
            ? workspace.focused_panel_id : panels.at(-1)?.id ?? null };
      }),
      sidebar_orders: loaded.sidebar_orders ?? { projects: [], chats: [] },
    };
  }

  function orderedBySavedIds<T extends { id: string }>(items: T[], ids: string[]) {
    if (!ids.length) return items;
    const rank = new Map(ids.map((id, index) => [id, index]));
    return [...items].sort((left, right) => {
      const leftRank = rank.get(left.id);
      const rightRank = rank.get(right.id);
      if (leftRank === undefined && rightRank === undefined) return 0;
      if (leftRank === undefined) return 1;
      if (rightRank === undefined) return -1;
      return leftRank - rightRank;
    });
  }

  function trapDialogFocus(event: KeyboardEvent) {
    if (event.key !== 'Tab') return;
    const items = Array.from(
      (event.currentTarget as HTMLElement).querySelectorAll<HTMLElement>(
        'button:not(:disabled), input:not(:disabled)',
      ),
    );
    if (!items.length) return;
    const first = items[0];
    const last = items[items.length - 1];
    if (event.shiftKey && document.activeElement === first) {
      event.preventDefault();
      last.focus();
    } else if (!event.shiftKey && document.activeElement === last) {
      event.preventDefault();
      first.focus();
    }
  }

  function matchesConversation(conversation: Conversation) {
    const query = searchQuery.trim().toLowerCase();
    return (
      !query ||
      conversation.name.toLowerCase().includes(query) ||
      conversation.cwd.toLowerCase().includes(query)
    );
  }

  function errorFromTerminal(message: string) {
    errorMessage = message;
  }
</script>

<svelte:head>
  <title
    >{showHome ? 'Início' : (activeProject?.name ?? 'Chats')} · Eros</title
  >
  <meta
    name="description"
    content="Workspace local para conversas com agentes e terminais."
  />
</svelte:head>

<div
  class:sidebar-collapsed={sidebarCollapsed}
  class="app-shell"
  inert={showProjectSettings || showRenameProject || showReorderDialog}
>
  {#if !sidebarCollapsed}<button
      class="sidebar-scrim"
      type="button"
      aria-label="Fechar navegação"
      onclick={() => (sidebarCollapsed = true)}
    ></button>{/if}
  <aside class="sidebar" aria-label="Navegação principal">
    <div class="sidebar-header">
      <button
        class="brand-mark"
        type="button"
        aria-label="Eros, início"
        onclick={goHome}><LucideIcon name="shell" size={30} className="brand-shell-icon" /></button
      >
      <label class="search-field">
        <LucideIcon name="search" size={15} />
        <input
          bind:this={searchInput}
          bind:value={searchQuery}
          type="search"
          placeholder="Buscar..."
          aria-label="Buscar projetos e conversas"
        />
      </label>
      <button
        class="icon-button"
        type="button"
        aria-label={sidebarCollapsed
          ? 'Expandir navegação'
          : 'Recolher navegação'}
        onclick={() => (sidebarCollapsed = !sidebarCollapsed)}
        ><LucideIcon
          name={sidebarCollapsed ? 'panel-right-close' : 'panel-left-close'}
          size={20}
          strokeWidth={2.5}
        /></button
      >
    </div>
    <nav class="primary-nav" aria-label="Ações principais">
      <button
        class="nav-entry new-chat"
        aria-label="Novo terminal"
        title="Novo terminal (Ctrl+N)"
        type="button"
        onclick={() => void startTerminal()}
        ><LucideIcon name="terminal" size={20} /><span>Novo terminal</span></button
      >
      <button
        class="nav-entry"
        type="button"
        disabled
        title="Rotinas: planejado, ainda indisponível"
        ><LucideIcon name="clock" size={20} /><span>Rotinas</span></button
      >
      <button
        class="nav-entry"
        type="button"
        disabled
        title="Mobile: planejado, ainda indisponível"
        ><LucideIcon name="smartphone" size={20} /><span>Mobile</span></button
      >
      <button
        class="nav-entry"
        type="button"
        disabled
        title="Skills: planejado, ainda indisponível"
        ><LucideIcon name="shapes" size={20} /><span>Skills</span></button
      >
    </nav>
    <div class="sidebar-scroll" onscroll={() => closeContextMenu(false)}>
      <section class="sidebar-section" aria-label="Projetos">
        <div class="section-heading">
          <button
            class="section-title"
            type="button"
            aria-expanded={projectsExpanded}
            onclick={() => (projectsExpanded = !projectsExpanded)}
            ><span>Projetos</span><LucideIcon
              name="chevron"
              size={14}
              className={projectsExpanded ? 'expanded-chevron' : ''}
            /></button
          >
          <div class="section-actions">
            <button
              class="context-action"
              type="button"
              aria-label="Novo projeto"
              title="Novo projeto"
              onclick={() => void openProjectFolder()}
              ><LucideIcon name="plus" size={18} /></button
            >
            <button
              class="context-action"
              type="button"
              aria-label="Opções de projetos"
              aria-haspopup="menu"
              aria-expanded={contextMenu === 'projects'}
              data-context-trigger
              onclick={(event) => toggleContextMenu('projects', event)}
              ><LucideIcon name="ellipsis" size={18} /></button
            >
          </div>
          {#if contextMenu === 'projects'}
            <div
              bind:this={contextMenuElement}
              class="sidebar-context-menu"
              role="menu"
              tabindex="-1"
              aria-label="Opções de projetos"
              style={`top:${contextMenuPosition.top}px;left:${contextMenuPosition.left}px`}
              onkeydown={handleContextMenuKeydown}
            >
              <button role="menuitem" type="button" onclick={() => { closeContextMenu(false); void openProjectFolder(); }}>
                <LucideIcon name="plus" size={16} />Novo projeto
              </button>
              <button role="menuitem" type="button" disabled title="Configurações da lista ainda indisponíveis">
                <LucideIcon name="settings" size={16} />Configurações
              </button>
              <button role="menuitem" type="button" onclick={() => beginReordering('projects')}>
                <LucideIcon name="folder" size={16} />Organizar projetos
              </button>
            </div>
          {/if}
        </div>
        {#if projectsExpanded}
          {#each matchingProjects as project (project.id)}
            <div
              class="project-row"
              class:active={!showHome && activeScope === project.id}
            >
              <button
                class="project-entry"
                type="button"
                aria-label={project.name}
                title={project.name}
                aria-current={!showHome && activeScope === project.id ? 'page' : undefined}
                aria-expanded={expandedProjectId === project.id}
                onclick={() => selectProject(project.id)}
              >
                <LucideIcon
                  name={expandedProjectId === project.id
                    ? 'folder-open'
                    : 'folder'}
                  size={18}
                /><span>{project.name}</span><LucideIcon
                  name="chevron"
                  size={14}
                  className={expandedProjectId === project.id
                    ? 'project-chevron expanded-chevron'
                    : 'project-chevron'}
                />
              </button>
              <div class="project-actions">
                <SessionCreateMenu
                  variant="icon"
                  label={`Novo chat em ${project.name}`}
                  disabled={busy || loading}
                  onSelect={(kind) => void createInScope(project.id, kind)}
                />
                <button
                  class="context-action"
                  type="button"
                  aria-label={`Opções de ${project.name}`}
                  aria-haspopup="menu"
                  aria-expanded={contextMenu === `project:${project.id}`}
                  data-context-trigger
                  onclick={(event) => toggleContextMenu(`project:${project.id}`, event)}
                  ><LucideIcon name="ellipsis" size={18} /></button
                >
              </div>
              {#if contextMenu === `project:${project.id}`}
                <div
                  bind:this={contextMenuElement}
                  class="sidebar-context-menu"
                  role="menu"
                  tabindex="-1"
                  aria-label={`Opções de ${project.name}`}
                  style={`top:${contextMenuPosition.top}px;left:${contextMenuPosition.left}px`}
                  onkeydown={handleContextMenuKeydown}
                >
                  <SessionCreateMenu
                    variant="menu-item"
                    label="Novo chat"
                    disabled={busy || loading}
                    onSelect={(kind) => { closeContextMenu(false); void createInScope(project.id, kind); }}
                  />
                  <button role="menuitem" type="button" onclick={() => openProjectSettings(project)}>
                    <LucideIcon name="settings" size={16} />Configurações
                  </button>
                  <button role="menuitem" type="button" onclick={() => openRenameProject(project)}>
                    <LucideIcon name="folder" size={16} />Renomear projeto
                  </button>
                </div>
              {/if}
            </div>
            {#if expandedProjectId === project.id || searchQuery.trim()}
              {#each projectConversations(project).filter(matchesConversation) as conversation (conversation.id)}
                <button
                  class="project-conversation"
                  class:active={!showHome && activeWorkspace.panels.some((p) => p.id === activeWorkspace.focused_panel_id && p.conversation_id === conversation.id)}
                  aria-label={`${conversation.name}, ${chatStatusLabels[conversation.chat_status ?? 'normal']}`}
                  title={conversation.name}
                  type="button"
                  onclick={() => void openConversation(conversation)}
                  ><ChatStatusDot status={conversation.chat_status} /><span
                    >{conversation.name}</span
                  ></button
                >
              {:else}
                <p class="project-empty">{searchQuery ? 'Nenhuma conversa encontrada.' : 'Nenhuma conversa neste projeto.'}</p>
              {/each}
            {/if}
          {:else}
            <p class="sidebar-empty">
              {searchQuery
                ? 'Nenhum projeto encontrado.'
                : 'Seus projetos aparecem aqui.'}
            </p>
          {/each}
        {/if}
      </section>
      <section class="sidebar-section" aria-label="Chats">
        <div class="section-heading">
          <button
            class="section-title"
            type="button"
            aria-expanded={chatsExpanded}
            onclick={() => (chatsExpanded = !chatsExpanded)}
            ><span>Chats</span><LucideIcon
              name="chevron"
              size={14}
              className={chatsExpanded ? 'expanded-chevron' : ''}
            /></button
          >
          <div class="section-actions">
            <SessionCreateMenu
              variant="icon"
              label="Novo chat"
              disabled={busy || loading}
              onSelect={(kind) => void createInScope('chats', kind)}
            />
            <button
              class="context-action"
              type="button"
              aria-label="Opções de chats"
              aria-haspopup="menu"
              aria-expanded={contextMenu === 'chats'}
              data-context-trigger
              onclick={(event) => toggleContextMenu('chats', event)}
              ><LucideIcon name="ellipsis" size={18} /></button
            >
          </div>
          {#if contextMenu === 'chats'}
            <div
              bind:this={contextMenuElement}
              class="sidebar-context-menu"
              role="menu"
              tabindex="-1"
              aria-label="Opções de chats"
              style={`top:${contextMenuPosition.top}px;left:${contextMenuPosition.left}px`}
              onkeydown={handleContextMenuKeydown}
            >
              <SessionCreateMenu
                variant="menu-item"
                label="Novo chat"
                disabled={busy || loading}
                onSelect={(kind) => { closeContextMenu(false); void createInScope('chats', kind); }}
              />
              <button role="menuitem" type="button" onclick={() => { closeContextMenu(false); showChatSettings = true; }}>
                <LucideIcon name="settings" size={16} />Configurações
              </button>
              <button role="menuitem" type="button" onclick={() => beginReordering('chats')}>
                <LucideIcon name="chat" size={16} />Organizar chats
              </button>
            </div>
          {/if}
        </div>
        {#if chatsExpanded}
          {#each matchingStandaloneChats as conversation (conversation.id)}
            <div
              class="chat-row"
              class:active={!showHome && activeWorkspace.panels.some((p) => p.id === activeWorkspace.focused_panel_id && p.conversation_id === conversation.id)}
            >
              <button
                class="chat-entry"
                class:active={!showHome && activeWorkspace.panels.some((p) => p.id === activeWorkspace.focused_panel_id && p.conversation_id === conversation.id)}
                aria-label={`${conversation.name}${conversation.pinned ? ", fixado" : ""}, ${chatStatusLabels[conversation.chat_status ?? 'normal']}`}
                title={conversation.name}
                type="button"
                onclick={() => void openConversation(conversation)}
              ><ChatStatusDot status={conversation.chat_status} /><span>{conversation.name}</span>
                {#if conversation.pinned}<LucideIcon name="pin" size={14} className="chat-pin-indicator" />{/if}
              </button>
              <button
                class="context-action chat-context-action"
                type="button"
                aria-label={`Opções de ${conversation.name}`}
                aria-haspopup="menu"
                aria-expanded={contextMenu === `chat:${conversation.id}`}
                data-context-trigger
                onclick={(event) => toggleContextMenu(`chat:${conversation.id}`, event)}
              ><LucideIcon name="ellipsis-vertical" size={17} /></button>
              {#if contextMenu === `chat:${conversation.id}`}
                <div
                  bind:this={contextMenuElement}
                  class="sidebar-context-menu"
                  role="menu"
                  tabindex="-1"
                  aria-label={`Opções de ${conversation.name}`}
                  style={`top:${contextMenuPosition.top}px;left:${contextMenuPosition.left}px`}
                  onkeydown={handleContextMenuKeydown}
                >
                  <button role="menuitem" type="button" onclick={() => void toggleChatPinned(conversation)}>
                    <LucideIcon name={conversation.pinned ? 'pin-off' : 'pin'} size={16} />
                    {conversation.pinned ? 'Desafixar do topo' : 'Fixar no topo'}
                  </button>
                  <button class="destructive-menu-item" role="menuitem" type="button" onclick={() => requestDeleteChat(conversation)}>
                    <LucideIcon name="trash" size={16} />Excluir chat
                  </button>
                </div>
              {/if}
            </div>
          {:else}
            <p class="sidebar-empty">
              {searchQuery
                ? 'Nenhuma conversa encontrada.'
                : 'Nenhuma conversa ainda.'}
            </p>
          {/each}
        {/if}
      </section>
    </div>
    <div class="sidebar-bottom">
      <div class="profile-row">
        <span class="avatar"><LucideIcon name="user" size={19} /></span>
        <div>
          <strong>Workspace local</strong><small>Workspace pessoal</small>
        </div>
        <button
          class="icon-button"
          type="button"
          disabled
          aria-label="Configurações indisponíveis"
          title="Configurações: planejado"
          ><LucideIcon name="settings" size={18} /></button
        >
      </div>
      <ProviderStatusTicker
        providers={snapshot?.authenticated_providers ?? []}
      />
    </div>
  </aside>
  <main class="workspace" id="workspace">
    <header class="topbar">
      <button
        class="icon-button mobile-menu"
        type="button"
        aria-label="Alternar navegação"
        aria-expanded={!sidebarCollapsed}
        onclick={() => (sidebarCollapsed = !sidebarCollapsed)}
        ><LucideIcon
          name={sidebarCollapsed ? 'panel-right-close' : 'panel-left-close'}
          size={20}
          strokeWidth={2.5}
        /></button
      >
      <div class="workspace-tabs" role="tablist" tabindex="-1" aria-label="Workspaces abertos" onkeydown={navigateWorkspaceTabs}>
        {#each workspaceTabs as tab (tab.scope)}
          <button
            id={`workspace-tab-${tab.scope}`}
            class="workspace-tab"
            class:active={selectedTabScope === tab.scope}
            role="tab"
            type="button"
            aria-selected={selectedTabScope === tab.scope}
            aria-controls="workspace-tab-panel"
            tabindex={selectedTabScope === tab.scope ? 0 : -1}
            disabled={busy}
            title={tab.name}
            onclick={() => void selectWorkspaceTab(tab.scope)}
          >
            <LucideIcon name={tab.scope === '__home__' ? 'shell' : tab.scope === 'chats' ? 'chat' : 'folder'} size={13} />
            <span>{tab.name}</span>
          </button>
        {/each}
      </div>
      <label class="workspace-tab-picker" title="Abrir aba de outro projeto">
        <LucideIcon name="plus" size={16} />
        <select
          aria-label="Abrir aba de workspace"
          value=""
          disabled={busy || loading}
          onchange={(event) => {
            const scope = event.currentTarget.value;
            event.currentTarget.value = '';
            if (scope === '__folder__') void openProjectFolder();
            else if (scope) void selectScope(scope);
          }}
        >
          <option value="" disabled>Abrir aba</option>
          {#each snapshot?.projects ?? [] as project (project.id)}
            <option value={project.id}>{project.name}</option>
          {/each}
          <option value="chats">Chats</option>
          <option value="__folder__">Abrir pasta…</option>
        </select>
      </label>
    </header>
    {#if activityMessage}<p class="app-alert" role="status">{activityMessage}</p>{/if}
    {#if errorMessage}<div class="app-alert" role="alert">
        <span>{errorMessage}</span><button
          type="button"
          aria-label="Fechar aviso"
          onclick={() => (errorMessage = '')}><LucideIcon name="x" size={16} /></button
        >
      </div>{/if}
    <div class="workspace-tab-content" id="workspace-tab-panel" role="tabpanel" tabindex="0" aria-labelledby={`workspace-tab-${selectedTabScope}`}>
    {#if showHome}
      <HomeScreen
        {snapshot}
        {activeProject}
        {loading}
        {busy}
        onCreate={(kind, name, projectId) =>
          void startFromHome(kind, name, projectId)}
        onOpenFolder={() => void openProjectFolder()}
      />
    {:else}
      <div class="terminal-content" id="terminal-view" tabindex="-1">
        {#key activeWorkspaceScope}
        {#if loading}<div class="loading-state" role="status">
            Conectando ao serviço local…
          </div>
        {:else if !desktopReady}<div class="service-unavailable">
            <h2>O serviço local não conectou.</h2>
            <p>Abra o aplicativo desktop para acessar seus terminais.</p>
            <button
              class="secondary-button"
              type="button"
              onclick={retryConnection}>Tentar novamente</button
            >
          </div>
        {:else if activeWorkspace.tree}
          <div
            class:maximized-view={Boolean(selectedMaximizedPanel)}
            class="terminal-workspace"
          >
            {#if selectedMaximizedPanel && selectedMaximizedConversation}
              <TerminalPane
                panel={selectedMaximizedPanel}
                conversation={selectedMaximizedConversation}
                focused={true}
                maximized={true}
                onFocus={(panelId) => void focusPanel(panelId)}
                onClose={(panelId) => void closePanel(panelId)}
                onMaximize={(panelId) => void toggleMaximize(panelId)}
                onState={updateConversationState}
                onError={errorFromTerminal}
              />
            {:else if !maximizedPanelId}
              <LayoutTree
                node={activeWorkspace.tree}
                path=""
                workspace={activeWorkspace}
                conversations={snapshot?.conversations ?? []}
                focusedPanelId={activeWorkspace.focused_panel_id}
                onFocus={(panelId) => void focusPanel(panelId)}
                onClose={(panelId) => void closePanel(panelId)}
                onMaximize={(panelId) => void toggleMaximize(panelId)}
                onState={updateConversationState}
                onError={errorFromTerminal}
                onResizeSplit={(path, ratio) => void updateSplit(path, ratio)}
              />
            {/if}
          </div>
        {:else}
          <div class="workspace-empty">
            <h1>{activeProject?.name ?? 'Chats'}</h1>
            <p>Abra uma conversa existente ou comece um novo terminal.</p>
            <div class="conversation-catalog">
              {#each matchingScopedConversations as conversation (conversation.id)}<button
                  class="quiet-button"
                  type="button"
                  onclick={() => void openConversation(conversation)}
                  ><ChatStatusDot status={conversation.chat_status} />{conversation.name}</button
                >{/each}
            </div>
            <SessionCreateMenu
              variant="primary"
              label="Nova conversa"
              disabled={busy || loading}
              onSelect={(kind) => void createInScope(activeScope, kind)}
            />
          </div>
        {/if}
        {/key}
      </div>
    {/if}
    </div>
  </main>
</div>

{#if showChatSettings}
  <div class="dialog-backdrop" role="presentation">
    <div class="conversation-dialog settings-dialog" role="dialog" aria-modal="true" aria-labelledby="chat-settings-title" tabindex="-1" onkeydown={(event) => { if (event.key === 'Escape') showChatSettings = false; else trapDialogFocus(event); }}>
      <div class="dialog-header">
        <h2 id="chat-settings-title">Configurações de Chats</h2>
        <button class="icon-button" type="button" aria-label="Fechar configurações" onclick={() => showChatSettings = false}><LucideIcon name="x" size={16} /></button>
      </div>
      <div class="form-field">
        <span>Pasta padrão dos chats avulsos</span>
        <button
          class="settings-path settings-path-picker"
          type="button"
          aria-label="Escolher pasta padrão dos chats"
          title="Clique para escolher uma pasta"
          disabled={busy}
          onclick={() => void chooseChatFolder()}
        >{snapshot?.chat_default_folder || 'Pasta pessoal do usuário'}</button>
      </div>
      {#if snapshot?.chat_default_folder_unavailable}
        <p class="field-hint" role="status">A pasta configurada não está disponível. Novos chats vão abrir na pasta pessoal até você escolher outra.</p>
      {/if}
      <div class="dialog-actions">
        <button class="secondary-button" type="button" disabled={busy} onclick={() => void updateChatFolder('')}>Usar pasta pessoal</button>
        <button class="primary-button" type="button" disabled={busy} onclick={() => void chooseChatFolder()}>Escolher pasta</button>
      </div>
    </div>
  </div>
{/if}

{#if deleteChatTarget}
  <div class="dialog-backdrop" role="presentation">
    <div
      class="conversation-dialog delete-chat-dialog"
      role="dialog"
      aria-modal="true"
      aria-labelledby="delete-chat-title"
      aria-describedby="delete-chat-copy"
      tabindex="-1"
      onkeydown={trapDialogFocus}
    >
      <div class="dialog-header">
        <h2 id="delete-chat-title">Excluir chat?</h2>
      </div>
      <p id="delete-chat-copy" class="delete-chat-copy">
        “{deleteChatTarget.name}” será removido do Eros. Se estiver ativo, o terminal será encerrado.
      </p>
      {#if deleteChatError}<p class="dialog-error" role="alert">{deleteChatError}</p>{/if}
      <div class="dialog-actions">
        <button
          bind:this={deleteChatCancelButton}
          class="secondary-button"
          type="button"
          disabled={deleteChatBusy}
          onclick={closeDeleteChatDialog}
        >Cancelar</button>
        <button
          class="destructive-button"
          type="button"
          disabled={deleteChatBusy}
          onclick={() => void confirmDeleteChat()}
        >{deleteChatBusy ? 'Excluindo…' : 'Excluir chat'}</button>
      </div>
    </div>
  </div>
{/if}

{#if showProjectSettings && settingsProject}
  <div class="dialog-backdrop" role="presentation">
    <section
      class="conversation-dialog settings-dialog"
      tabindex="-1"
      role="dialog"
      aria-modal="true"
      aria-labelledby="project-settings-title"
      onkeydown={trapDialogFocus}
    >
      <div class="dialog-header">
        <div>
          <h2 id="project-settings-title">Configurações do projeto</h2>
        </div>
        <button
          bind:this={projectSettingsCloseButton}
          class="icon-button"
          type="button"
          aria-label="Fechar configurações do projeto"
          onclick={() => closeProjectSettings()}
        ><LucideIcon name="x" size={16} /></button>
      </div>
      <label class="form-field">
        <span>Pasta associada</span>
        <code class="settings-path" title={settingsProject.path}>{settingsProject.path}</code>
      </label>
      <p class="field-hint settings-hint">
        As conversas iniciadas neste projeto usam esta pasta local como diretório de trabalho.
      </p>
      <div class="dialog-actions">
        <button class="secondary-button" type="button" onclick={() => closeProjectSettings()}>
          Fechar
        </button>
        <button
          class="primary-button"
          type="button"
          onclick={renameConfiguredProject}
        >Renomear projeto</button>
      </div>
    </section>
  </div>
{/if}

{#if showRenameProject}
  <div class="dialog-backdrop" role="presentation">
    <section
      class="conversation-dialog compact-dialog"
      tabindex="-1"
      role="dialog"
      aria-modal="true"
      aria-labelledby="rename-project-title"
      onkeydown={trapDialogFocus}
    >
      <div class="dialog-header">
        <div>
          <h2 id="rename-project-title">Renomear projeto</h2>
        </div>
        <button
          class="icon-button"
          type="button"
          aria-label="Fechar diálogo"
          onclick={() => closeRenameDialog()}
          disabled={projectActionBusy}
        ><LucideIcon name="x" size={16} /></button>
      </div>
      {#if renameProjectError}<p class="dialog-error" role="alert">{renameProjectError}</p>{/if}
      <form onsubmit={submitProjectRename}>
        <label class="form-field">
          <span>Nome do projeto</span>
          <input
            bind:this={renameNameInput}
            bind:value={renameProjectName}
            maxlength="80"
            required
            aria-label="Nome do projeto"
          />
        </label>
        <div class="dialog-actions">
          <button class="secondary-button" type="button" onclick={() => closeRenameDialog()} disabled={projectActionBusy}>
            Cancelar
          </button>
          <button class="primary-button" type="submit" disabled={projectActionBusy || !desktopReady}>
            {#if projectActionBusy}<span class="button-spinner"></span>{/if}
            {projectActionBusy ? 'Salvando…' : 'Salvar nome'}
          </button>
        </div>
      </form>
    </section>
  </div>
{/if}

{#if showReorderDialog}
  <div class="dialog-backdrop" role="presentation">
    <section
      class="conversation-dialog reorder-dialog"
      tabindex="-1"
      role="dialog"
      aria-modal="true"
      aria-labelledby="reorder-title"
      onkeydown={trapDialogFocus}
    >
      <div class="dialog-header">
        <div>
          <h2 id="reorder-title">Organizar {reorderGroup === 'projects' ? 'projetos' : 'chats'}</h2>
        </div>
        <button
          bind:this={reorderCloseButton}
          class="icon-button"
          type="button"
          aria-label="Fechar organização"
          onclick={() => closeReorderDialog()}
          disabled={projectActionBusy}
        ><LucideIcon name="x" size={16} /></button>
      </div>
      {#if reorderError}<p class="dialog-error" role="alert">{reorderError}</p>{/if}
      {#if reorderRows.length}
        <ol class="reorder-list" aria-label={`Ordem de ${reorderGroup === 'projects' ? 'projetos' : 'chats'}`}>
          {#each reorderRows as row, index (row.id)}
            <li class="reorder-item">
              <span class="reorder-position">{index + 1}</span>
              <span class="reorder-name" title={row.name}>{row.name}</span>
              <div class="reorder-controls">
                <button
                  class="icon-button"
                  type="button"
                  aria-label={`Mover ${row.name} para cima`}
                  title="Mover para cima"
                  disabled={index === 0 || projectActionBusy}
                  onclick={() => moveReorderRow(index, -1)}
                ><LucideIcon name="arrow-up" size={16} /></button>
                <button
                  class="icon-button"
                  type="button"
                  aria-label={`Mover ${row.name} para baixo`}
                  title="Mover para baixo"
                  disabled={index === reorderRows.length - 1 || projectActionBusy}
                  onclick={() => moveReorderRow(index, 1)}
                ><LucideIcon name="arrow-down" size={16} /></button>
              </div>
            </li>
          {/each}
        </ol>
      {:else}
        <p class="sidebar-empty reorder-empty">Não há {reorderGroup === 'projects' ? 'projetos' : 'chats'} para organizar.</p>
      {/if}
      <div class="dialog-actions">
        <button class="secondary-button" type="button" onclick={() => closeReorderDialog()} disabled={projectActionBusy}>
          Cancelar
        </button>
        <button class="primary-button" type="button" onclick={() => void saveReorder()} disabled={projectActionBusy || reorderRows.length < 2}>
          {#if projectActionBusy}<span class="button-spinner"></span>{/if}
          {projectActionBusy ? 'Salvando…' : 'Salvar ordem'}
        </button>
      </div>
    </section>
  </div>
{/if}

