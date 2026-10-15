<script lang="ts">
  import LayoutTree from './LayoutTree.svelte';
  import TerminalPane from './TerminalPane.svelte';
  import type { Conversation, Layout, SessionState, Workspace } from './contracts';

  type Props = {
    node: Layout;
    path: string;
    workspace: Workspace;
    conversations: Conversation[];
    focusedPanelId: string | null;
    onFocus: (panelId: string) => void;
    onClose: (panelId: string) => void;
    onMaximize: (panelId: string) => void;
    onState: (conversationId: string, state: SessionState) => void;
    onError: (message: string) => void;
    onResizeSplit: (path: string, ratio: number) => void;
  };

  let {
    node,
    path,
    workspace,
    conversations,
    focusedPanelId,
    onFocus,
    onClose,
    onMaximize,
    onState,
    onError,
    onResizeSplit,
  }: Props = $props();

  let splitElement: HTMLDivElement;
  const visualAxis = $derived(node.type === 'split' ? node.axis : 'horizontal');

  function resizeFromPointer(event: PointerEvent) {
    if (node.type !== 'split' || !splitElement) return;
    event.preventDefault();
    const handle = event.currentTarget as HTMLElement;
    const bounds = splitElement.getBoundingClientRect();
    const update = (move: PointerEvent) => {
      const size = visualAxis === 'horizontal' ? bounds.width : bounds.height;
      const origin = visualAxis === 'horizontal' ? bounds.left : bounds.top;
      const position = visualAxis === 'horizontal' ? move.clientX : move.clientY;
      if (size <= 0) return;
      onResizeSplit(path, Math.min(0.85, Math.max(0.15, (position - origin) / size)));
    };
    const finish = () => {
      handle.removeEventListener('pointermove', update);
      handle.removeEventListener('pointerup', finish);
      handle.removeEventListener('pointercancel', finish);
    };
    handle.setPointerCapture(event.pointerId);
    handle.addEventListener('pointermove', update);
    handle.addEventListener('pointerup', finish, { once: true });
    handle.addEventListener('pointercancel', finish, { once: true });
  }

  function resizeFromKeyboard(event: KeyboardEvent) {
    if (node.type !== 'split') return;
    const horizontal = visualAxis === 'horizontal';
    const increase = horizontal ? event.key === 'ArrowRight' : event.key === 'ArrowDown';
    const decrease = horizontal ? event.key === 'ArrowLeft' : event.key === 'ArrowUp';
    if (!increase && !decrease) return;
    event.preventDefault();
    const adjustment = increase ? 0.025 : -0.025;
    onResizeSplit(path, Math.min(0.85, Math.max(0.15, node.ratio + adjustment)));
  }
</script>

{#if node.type === 'leaf'}
  {@const panel = workspace.panels.find((item) => item.id === node.panel_id)}
  {@const conversation = panel && conversations.find((item) => item.id === panel.conversation_id)}
  {#if panel && conversation}
    <TerminalPane
      {panel}
      {conversation}
      focused={focusedPanelId === panel.id}
      maximized={false}
      {onFocus}
      {onClose}
      {onMaximize}
      {onState}
      {onError}
    />
  {:else}
    <section class="missing-session" aria-live="polite">
      <span>Conversa indisponível</span>
      <p>Este painel aponta para uma conversa que não está no catálogo local.</p>
    </section>
  {/if}
{:else}
  <div
    class:split-horizontal={visualAxis === 'horizontal'}
    class:split-vertical={visualAxis === 'vertical'}
    class="layout-split"
    bind:this={splitElement}
  >
    <div class="split-child" style={`flex-grow: ${node.ratio}; flex-basis: 0`}>
      <LayoutTree
        node={node.first}
        path={`${path}0`}
        {workspace}
        {conversations}
        {focusedPanelId}
        {onFocus}
        {onClose}
        {onMaximize}
        {onState}
        {onError}
        {onResizeSplit}
      />
    </div>
    <div
      class={`split-divider ${visualAxis}`}
      role="separator"
      aria-orientation={visualAxis === 'horizontal' ? 'vertical' : 'horizontal'}
      aria-label="Ajustar tamanho dos painéis"
      aria-valuemin="15"
      aria-valuemax="85"
      aria-valuenow={Math.round(node.ratio * 100)}
      tabindex="0"
      onpointerdown={resizeFromPointer}
      onkeydown={resizeFromKeyboard}
    >
      <span aria-hidden="true"></span>
    </div>
    <div class="split-child" style={`flex-grow: ${1 - node.ratio}; flex-basis: 0`}>
      <LayoutTree
        node={node.second}
        path={`${path}1`}
        {workspace}
        {conversations}
        {focusedPanelId}
        {onFocus}
        {onClose}
        {onMaximize}
        {onState}
        {onError}
        {onResizeSplit}
      />
    </div>
  </div>
{/if}
