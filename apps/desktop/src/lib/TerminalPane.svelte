<script lang="ts">
  import { onMount } from 'svelte';
  import LucideIcon from './LucideIcon.svelte';
  import { listen } from '@tauri-apps/api/event';
  import { FitAddon } from '@xterm/addon-fit';
  import { Terminal } from '@xterm/xterm';
  import '@xterm/xterm/css/xterm.css';
  import type { Conversation, Panel, SessionState, TerminalEvent } from './contracts';
  import {
    isDesktop,
    loadSnapshot,
    resizeTerminal,
    subscribeTerminal,
    unsubscribeTerminal,
    writeTerminal,
  } from './api';

  type Props = {
    panel: Panel;
    conversation: Conversation;
    focused: boolean;
    maximized: boolean;
    onFocus: (panelId: string) => void;
    onClose: (panelId: string) => void;
    onMaximize: (panelId: string) => void;
    onState: (conversationId: string, state: SessionState) => void;
    onError: (message: string) => void;
  };

  let {
    panel,
    conversation,
    focused,
    maximized,
    onFocus,
    onClose,
    onMaximize,
    onState,
    onError,
  }: Props = $props();

  let terminalElement: HTMLDivElement;
  function statusLabel(state: SessionState) {
    switch (state) {
      case 'running': return 'Em execução';
      case 'starting': return 'Iniciando';
      case 'exited': return 'Processo encerrado';
      case 'stopped': return 'Interrompida';
      case 'unavailable': return 'Desconectada';
    }
  }

  onMount(() => {
    const terminalBackground = getComputedStyle(document.documentElement)
      .getPropertyValue('--bg')
      .trim();
    const terminal = new Terminal({
      allowTransparency: false,
      cursorBlink: true,
      cursorStyle: 'bar',
      convertEol: false,
      fontFamily: '"SFMono-Regular", "Cascadia Code", Consolas, monospace',
      fontSize: 12,
      lineHeight: 1.35,
      scrollback: 5000,
      theme: {
        background: terminalBackground,
        foreground: '#d4d4d4',
        cursor: '#d2d2d2',
        cursorAccent: terminalBackground,
        selectionBackground: '#80808055',
        black: '#1f1f1f',
        red: '#d17b6e',
        green: '#97ad8b',
        yellow: '#d8ad74',
        blue: '#8b9caf',
        magenta: '#a78eab',
        cyan: '#86a6a1',
        white: '#d4d4d4',
        brightBlack: '#77766f',
        brightRed: '#e28e7e',
        brightGreen: '#a8bf9d',
        brightYellow: '#e7bd84',
        brightBlue: '#9aafc4',
        brightMagenta: '#b9a1bc',
        brightCyan: '#95b6b0',
        brightWhite: '#f2f2f2',
      },
    });
    const fit = new FitAddon();
    terminal.loadAddon(fit);
    terminal.open(terminalElement);
    terminal.focus();

    let cancelled = false;
    let unlisten: (() => void) | undefined;
    let resizeFrame = 0;
    let resizeTimer: ReturnType<typeof setTimeout> | undefined;
    const subscriptionId = crypto.randomUUID();
    const decoder = new TextDecoder();
    let inputQueue = Promise.resolve();
    let queuedInputSize = 0;
    const dataListener = terminal.onData((data) => {
      if (conversation.state !== 'running' || !isDesktop()) return;
      if (queuedInputSize + data.length > 2 * 1024 * 1024) {
        onError('A fila de entrada do terminal atingiu o limite. Aguarde o processo responder e tente novamente.');
        return;
      }
      queuedInputSize += data.length;
      inputQueue = inputQueue
        .then(async () => {
          for (let offset = 0; offset < data.length;) {
            let end = Math.min(offset + 8000, data.length);
            if (end < data.length && /[\uD800-\uDBFF]/.test(data.charAt(end - 1))) end -= 1;
            await writeTerminal(conversation.id, data.slice(offset, end));
            offset = end;
          }
        })
        .catch((error: unknown) => {
          onError(error instanceof Error ? error.message : 'Não foi possível escrever no terminal.');
        })
        .finally(() => queuedInputSize -= data.length);
    });

    function fitToContainer() {
      if (cancelled || !terminalElement.isConnected || terminalElement.clientWidth < 2) return;
      try {
        fit.fit();
        if (isDesktop() && conversation.state === 'running') {
          void resizeTerminal(conversation.id, terminal.cols, terminal.rows).catch(() => undefined);
        }
      } catch {
        // xterm can briefly have no measurable dimensions while a split is animating.
      }
    }

    const observer = new ResizeObserver(() => {
      cancelAnimationFrame(resizeFrame);
      if (resizeTimer) clearTimeout(resizeTimer);
      resizeTimer = setTimeout(() => resizeFrame = requestAnimationFrame(fitToContainer), 70);
    });
    observer.observe(terminalElement);
    resizeFrame = requestAnimationFrame(fitToContainer);

    if (isDesktop() && conversation.state === 'running') {
      void (async () => {
        try {
          unlisten = await listen<TerminalEvent>(`eros-terminal-${subscriptionId}`, ({ payload }) => {
            if (payload.type === 'output' && payload.conversation_id === conversation.id) {
              terminal.write(decoder.decode(Uint8Array.from(payload.data), { stream: true }));
            } else if (payload.type === 'state' && payload.conversation_id === conversation.id) {
              onState(conversation.id, payload.state);
            } else if (payload.type === 'error') {
              onError(payload.message);
            }
          });
          if (cancelled) {
            unlisten();
            return;
          }
          await subscribeTerminal(conversation.id, subscriptionId);
          if (cancelled) await unsubscribeTerminal(subscriptionId);
        } catch (error) {
          try {
            const latest = await loadSnapshot();
            const current = latest.conversations.find((item) => item.id === conversation.id);
            if (current && current.state !== 'running') {
              onState(conversation.id, current.state);
            } else {
              onError(error instanceof Error ? error.message : 'Não foi possível reconectar ao terminal.');
            }
          } catch {
            onError(error instanceof Error ? error.message : 'Não foi possível reconectar ao terminal.');
          }
        }
      })();
    } else if (!isDesktop()) {
      terminal.write('Abra esta tela no aplicativo desktop do Eros para conectar um processo real.\r\n');
    } else {
      terminal.write('Esta conversa não tem um processo ativo para reconectar.\r\n');
    }

    return () => {
      cancelled = true;
      observer.disconnect();
      cancelAnimationFrame(resizeFrame);
      if (resizeTimer) clearTimeout(resizeTimer);
      dataListener.dispose();
      unlisten?.();
      void unsubscribeTerminal(subscriptionId);
      terminal.dispose();
    };
  });
</script>

<section
  class:focused
  class:maximized
  class="terminal-pane"
  aria-label={`Terminal ${conversation.name}`}
  onfocusin={() => onFocus(panel.id)}
  onclick={() => onFocus(panel.id)}
>
  <header class="pane-header" title={`${conversation.cwd} · ${statusLabel(conversation.state)}`}>
    <div class="pane-title-group" role="group" aria-label="Sessão do terminal">
      <span class:codex={conversation.kind === 'codex' || conversation.kind === 'claude'} class="pane-glyph" aria-hidden="true">
        <LucideIcon name={conversation.kind === 'shell' ? 'terminal' : 'bot'} size={14} />
      </span>
      <div class="pane-title-copy">
        <strong>{conversation.name}</strong>
        <span class={`state-dot ${conversation.state}`} role="img" aria-label={statusLabel(conversation.state)} title={statusLabel(conversation.state)}></span>
      </div>
    </div>
    <div class="pane-actions" aria-label="Ações do painel">
      <button
        class="icon-button"
        type="button"
        aria-label={maximized ? 'Restaurar tamanho do painel' : 'Focar terminal'}
        title={maximized ? 'Restaurar painel' : 'Focar terminal'}
        onclick={(event) => { event.stopPropagation(); onMaximize(panel.id); }}
      >
        <LucideIcon name="maximize" size={17} strokeWidth={2} />
      </button>
      <button
        class="icon-button close-button"
        type="button"
        aria-label={conversation.kind === 'shell' ? 'Encerrar terminal e seus processos' : 'Fechar painel; a conversa continua ativa'}
        title={conversation.kind === 'shell' ? 'Encerrar terminal e seus processos' : 'Fechar painel. A conversa continua ativa.'}
        onclick={(event) => { event.stopPropagation(); onClose(panel.id); }}
      >
        <LucideIcon name="x" size={17} strokeWidth={2} />
      </button>
    </div>
  </header>

  <div class="terminal-screen" bind:this={terminalElement} role="group" aria-label="Saída do terminal">
    {#if conversation.state === 'unavailable'}
      <div class="terminal-notice">
        <span class="notice-mark">!</span>
        <div>
          <strong>Esta sessão não está mais conectada</strong>
          <p>O processo local terminou antes de o Eros conseguir reconectar.</p>
        </div>
      </div>
    {:else if conversation.state === 'exited' || conversation.state === 'stopped'}
      <div class="terminal-ended">{conversation.state === 'stopped' ? 'Processo interrompido.' : 'O processo encerrou.'}</div>
    {/if}
  </div>


</section>
