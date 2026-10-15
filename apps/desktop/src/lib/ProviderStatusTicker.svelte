<script lang="ts">
  import CodexMark from './CodexMark.svelte';
  import LucideIcon from './LucideIcon.svelte';
  import type { AuthenticatedProvider } from './contracts';

  type Props = {
    providers: AuthenticatedProvider[];
  };

  let { providers }: Props = $props();
  let activeIndex = $state(0);

  const activeProvider = $derived(
    providers.length > 0 ? providers[activeIndex % providers.length] : null,
  );

  $effect(() => {
    const providerCount = providers.length;
    if (providerCount < 2) {
      activeIndex = 0;
      return;
    }

    const timer = window.setInterval(() => {
      activeIndex = (activeIndex + 1) % providerCount;
    }, 6000);

    return () => window.clearInterval(timer);
  });
</script>

{#if activeProvider}
  <div
    class="provider-status-ticker"
    role="img"
    aria-label={`${activeProvider.label} conectado; dados de uso indisponíveis`}
    title={`${activeProvider.label} conectado; dados de uso indisponíveis`}
  >
    <span class="provider-status-mark" aria-hidden="true">
      {#if activeProvider.id === 'codex'}
        <CodexMark size={16} />
      {:else}
        <LucideIcon name="bot" size={16} />
      {/if}
    </span>
    <span class="provider-status-name">{activeProvider.label}</span>
    <span class="provider-status-usage">Sem dados</span>
    <span class="provider-status-dot" aria-hidden="true"></span>
  </div>
{/if}
