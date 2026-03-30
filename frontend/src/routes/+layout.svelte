<script lang="ts">
  import type { Snippet } from 'svelte';
  import { Toast } from '@juspay/svelte-ui-components';
  import { toastStore } from '$lib/client/modules/foundation';
  import '../app.css';

  let { children }: { children: Snippet } = $props();

  let toasts = $state<Array<{ id: string; message: string; level: string; durationMs: number }>>([]);

  toastStore.subscribe((value) => {
    toasts = value;
  });

  function mapLevel(level: string): 'success' | 'error' | 'info' | 'warn' {
    if (level === 'warning') return 'warn';
    return level as 'success' | 'error' | 'info' | 'warn';
  }
</script>

{@render children()}

<div class="toast-container">
  {#each toasts as toast (toast.id)}
    <Toast
      message={toast.message}
      type={mapLevel(toast.level)}
      duration={toast.durationMs}
      onToastHide={() => toastStore.dismiss(toast.id)}
    />
  {/each}
</div>

<style>
  .toast-container {
    position: fixed;
    top: var(--space-4, 16px);
    right: var(--space-4, 16px);
    z-index: 9999;
    display: flex;
    flex-direction: column;
    gap: var(--space-2, 8px);
    pointer-events: none;
  }

  .toast-container :global(*) {
    pointer-events: auto;
  }
</style>
