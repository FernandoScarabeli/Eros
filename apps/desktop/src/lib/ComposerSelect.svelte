<script lang="ts">
  import { onMount, tick } from 'svelte';
  import CodexMark from './CodexMark.svelte';
  import LucideIcon from './LucideIcon.svelte';

  type IconName = 'codex' | 'bot' | 'terminal' | 'folder-open';
  type Option = { value: string; label: string; icon?: IconName };
  type Props = {
    className: string;
    icon: IconName;
    label: string;
    onChange: (value: string) => void;
    options: Option[];
    value: string;
  };

  let { className, icon, label, onChange, options, value }: Props = $props();
  let root: HTMLDivElement;
  let trigger: HTMLButtonElement;
  let menuOpen = $state(false);
  let activeIndex = $state(0);
  const selectedOption = $derived(options.find((option) => option.value === value) ?? options[0]);
  const selectedIcon = $derived(selectedOption?.icon ?? icon);
  const menuId = $derived(`composer-select-${className}`);

  async function focusOption(index: number) {
    activeIndex = Math.max(0, Math.min(index, options.length - 1));
    menuOpen = true;
    await tick();
    root.querySelectorAll<HTMLButtonElement>('[role="option"]')[activeIndex]?.focus();
  }

  function closeMenu(restoreFocus = false) {
    menuOpen = false;
    if (restoreFocus) void tick().then(() => trigger.focus());
  }

  function chooseOption(option: Option) {
    onChange(option.value);
    closeMenu(true);
  }

  function handleTriggerKeydown(event: KeyboardEvent) {
    if (event.key === 'ArrowDown' || event.key === 'ArrowUp') {
      event.preventDefault();
      const selectedIndex = options.findIndex((option) => option.value === value);
      void focusOption(selectedIndex < 0 ? 0 : selectedIndex + (event.key === 'ArrowDown' ? 0 : -1));
    } else if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      if (menuOpen) closeMenu();
      else void focusOption(options.findIndex((option) => option.value === value));
    } else if (event.key === 'Escape' && menuOpen) {
      event.preventDefault();
      closeMenu();
    }
  }

  function handleOptionKeydown(event: KeyboardEvent, index: number) {
    if (event.key === 'ArrowDown') {
      event.preventDefault();
      void focusOption((index + 1) % options.length);
    } else if (event.key === 'ArrowUp') {
      event.preventDefault();
      void focusOption((index - 1 + options.length) % options.length);
    } else if (event.key === 'Home') {
      event.preventDefault();
      void focusOption(0);
    } else if (event.key === 'End') {
      event.preventDefault();
      void focusOption(options.length - 1);
    } else if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      chooseOption(options[index]);
    } else if (event.key === 'Escape') {
      event.preventDefault();
      closeMenu(true);
    } else if (event.key === 'Tab') {
      closeMenu();
    }
  }

  onMount(() => {
    const handlePointerDown = (event: PointerEvent) => {
      if (menuOpen && !root.contains(event.target as Node)) closeMenu();
    };
    window.addEventListener('pointerdown', handlePointerDown);
    return () => window.removeEventListener('pointerdown', handlePointerDown);
  });
</script>

<div class={`composer-select ${className}`} bind:this={root}>
  <button
    bind:this={trigger}
    type="button"
    class="composer-select-trigger"
    aria-label={label}
    aria-haspopup="listbox"
    aria-expanded={menuOpen}
    aria-controls={menuId}
    onclick={() => menuOpen ? closeMenu() : void focusOption(options.findIndex((option) => option.value === value))}
    onkeydown={handleTriggerKeydown}
  >
    {#if selectedIcon === 'codex'}
      <CodexMark size={16} />
    {:else}
      <LucideIcon name={selectedIcon} size={selectedIcon === 'folder-open' ? 15 : 16} />
    {/if}
    <span>{selectedOption?.label ?? label}</span>
    <LucideIcon name="chevron" size={13} />
  </button>
  {#if menuOpen}
    <div id={menuId} class="composer-select-menu" role="listbox" aria-label={label}>
      {#each options as option, index (option.value)}
        <button
          type="button"
          role="option"
          aria-selected={option.value === value}
          tabindex={activeIndex === index ? 0 : -1}
          class="composer-select-option"
          class:selected={option.value === value}
          class:active={activeIndex === index}
          onpointermove={() => activeIndex = index}
          onclick={() => chooseOption(option)}
          onkeydown={(event) => handleOptionKeydown(event, index)}
        >
          {#if option.icon === 'codex'}
            <CodexMark size={15} />
          {:else if option.icon}
            <LucideIcon name={option.icon} size={15} />
          {/if}
          <span>{option.label}</span>
        </button>
      {/each}
    </div>
  {/if}
</div>
