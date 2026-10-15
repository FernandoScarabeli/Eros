<script module lang="ts">
  export const chatStatusLabels = {
    normal: 'Normal',
    working: 'Trabalhando',
    completed_unread: 'Concluído · ainda não visto',
    awaiting_decision: 'Aguardando decisão',
  };
</script>

<script lang="ts">
  import type { ChatStatus } from './contracts';

  let { status = 'normal' }: { status?: ChatStatus } = $props();
</script>

<span class={`chat-status-dot ${status}`} role="img" aria-label={chatStatusLabels[status]} title={chatStatusLabels[status]}></span>

<style>
  .chat-status-dot {
    display: inline-block;
    width: 7px;
    height: 7px;
    flex: 0 0 7px;
    border-radius: 50%;
    background: #f2f2f2;
  }
  .completed_unread { background: #69d58a; }
  .working {
    background: #f0c94e;
    animation: working-pulse 1.4s ease-in-out infinite;
  }
  .awaiting_decision { background: #f16d6d; }
  @keyframes working-pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: .35; }
  }
  @media (prefers-reduced-motion: reduce) {
    .working { animation: none; }
  }
</style>
