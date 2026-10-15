<script lang="ts">
  import { onMount, tick } from 'svelte';
  import CodexMark from './CodexMark.svelte';
  import LucideIcon from './LucideIcon.svelte';
  import type { SessionKind } from './contracts';

  type Variant = 'composer' | 'primary' | 'icon' | 'menu-item';
  type Option = {
    kind: SessionKind;
    name: string;
    detail: string;
  };
  type Props = {
    variant: Variant;
    label?: string;
    disabled?: boolean;
    onSelect: (kind: SessionKind) => void;
  };

  let {
    variant,
    label = 'Nova conversa',
    disabled = false,
    onSelect,
  }: Props = $props();

  let root = $state<HTMLDivElement>();
  let trigger = $state<HTMLButtonElement>();
  let menuOpen = $state(false);
  let menuPosition = $state({ top: 0, left: 0 });

  const options: Option[] = [
    { kind: 'codex', name: 'Codex', detail: 'Iniciar no terminal' },
    { kind: 'claude', name: 'Claude Code', detail: 'Iniciar no terminal' },
    { kind: 'shell', name: 'Terminal local', detail: 'Shell interativo' },
  ];

  function updateMenuPosition() {
    const bounds = trigger?.getBoundingClientRect();
    if (!bounds) return;
    const width = Math.min(252, window.innerWidth - 16);
    const height = 190;
    const left = variant === 'composer' ? bounds.right - width : bounds.left;
    let top = bounds.bottom + 8;
    if (top + height > window.innerHeight) top = bounds.top - height - 8;
    menuPosition = {
      left: Math.max(8, Math.min(left, window.innerWidth - width - 8)),
      top: Math.max(8, top),
    };
  }

  async function openMenu(focusFirst = false) {
    updateMenuPosition();
    menuOpen = true;
    if (focusFirst) {
      await tick();
      root?.querySelector<HTMLButtonElement>('[role="menuitem"]')?.focus();
    }
  }

  function closeMenu(restoreFocus = false) {
    menuOpen = false;
    if (restoreFocus) void tick().then(() => trigger?.focus());
  }

  function choose(option: Option) {
    closeMenu();
    onSelect(option.kind);
  }

  function handleTriggerKeydown(event: KeyboardEvent) {
    if (event.key === 'ArrowDown' || event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      event.stopPropagation();
      if (!menuOpen) void openMenu(true);
    } else if (event.key === 'Escape' && menuOpen) {
      event.preventDefault();
      event.stopPropagation();
      closeMenu(true);
    }
  }

  function handleMenuKeydown(event: KeyboardEvent) {
    const items = Array.from(
      root?.querySelectorAll<HTMLButtonElement>('[role="menuitem"]') ?? [],
    );
    const current = items.indexOf(document.activeElement as HTMLButtonElement);
    let next = current;
    if (event.key === 'ArrowDown') next = (current + 1 + items.length) % items.length;
    else if (event.key === 'ArrowUp') next = (current - 1 + items.length) % items.length;
    else if (event.key === 'Home') next = 0;
    else if (event.key === 'End') next = items.length - 1;
    else if (event.key === 'Escape') {
      event.preventDefault();
      event.stopPropagation();
      closeMenu(true);
      return;
    } else if (event.key === 'Tab') {
      closeMenu();
      return;
    } else return;

    event.preventDefault();
    event.stopPropagation();
    items[next]?.focus();
  }

  onMount(() => {
    const onPointerDown = (event: PointerEvent) => {
      if (menuOpen && !root?.contains(event.target as Node)) closeMenu();
    };
    const onViewportChange = () => {
      if (menuOpen) updateMenuPosition();
    };
    document.addEventListener('pointerdown', onPointerDown);
    window.addEventListener('resize', onViewportChange);
    window.addEventListener('scroll', onViewportChange, true);
    return () => {
      document.removeEventListener('pointerdown', onPointerDown);
      window.removeEventListener('resize', onViewportChange);
      window.removeEventListener('scroll', onViewportChange, true);
    };
  });
</script>

<div
  bind:this={root}
  class="session-create-picker"
  class:session-create-picker-composer={variant === 'composer'}
  class:session-create-picker-primary={variant === 'primary'}
  class:session-create-picker-icon={variant === 'icon'}
  class:session-create-picker-menu-item={variant === 'menu-item'}
>
  {#if variant === 'composer'}
    <button
      bind:this={trigger}
      class="send-button"
      type="button"
      aria-label="Iniciar sessão"
      title="Escolher IA ou terminal"
      aria-haspopup="menu"
      aria-expanded={menuOpen}
      disabled={disabled}
      onclick={() => menuOpen ? closeMenu() : void openMenu()}
      onkeydown={handleTriggerKeydown}
    ><LucideIcon name="arrow-right" size={17} /></button>
  {:else if variant === 'primary'}
    <button
      bind:this={trigger}
      class="primary-button"
      type="button"
      aria-haspopup="menu"
      aria-expanded={menuOpen}
      disabled={disabled}
      onclick={() => menuOpen ? closeMenu() : void openMenu()}
      onkeydown={handleTriggerKeydown}
    ><LucideIcon name="plus" size={16} />{label}</button>
  {:else if variant === 'icon'}
    <button
      bind:this={trigger}
      class="context-action"
      type="button"
      aria-label={label}
      title={label}
      aria-haspopup="menu"
      aria-expanded={menuOpen}
      disabled={disabled}
      onclick={() => menuOpen ? closeMenu() : void openMenu()}
      onkeydown={handleTriggerKeydown}
    ><LucideIcon name="plus" size={18} /></button>
  {:else}
    <button
      bind:this={trigger}
      class="session-create-menu-trigger"
      type="button"
      role="menuitem"
      aria-haspopup="menu"
      aria-expanded={menuOpen}
      disabled={disabled}
      onclick={() => menuOpen ? closeMenu() : void openMenu()}
      onkeydown={handleTriggerKeydown}
    ><LucideIcon name="plus" size={16} />{label}</button>
  {/if}

  {#if menuOpen}
    <div
      class="session-create-menu"
      class:session-create-menu-composer={variant === 'composer'}
      role="menu"
      tabindex="-1"
      aria-label="Iniciar com"
      style={`top:${menuPosition.top}px;left:${menuPosition.left}px`}
      onkeydown={handleMenuKeydown}
    >
      {#each options as option (option.kind)}
        <button
          class="session-create-option"
          type="button"
          role="menuitem"
          onclick={() => choose(option)}
        >
          <span class="session-create-option-icon">
            {#if option.kind === 'codex'}
              <CodexMark size={16} />
            {:else}
              <LucideIcon name={option.kind === 'shell' ? 'terminal' : 'bot'} size={15} />
            {/if}
          </span>
          <span class="session-create-option-copy">
            <strong>{option.name}</strong>
            <small>{option.detail}</small>
          </span>
        </button>
      {/each}
    </div>
  {/if}
</div>
