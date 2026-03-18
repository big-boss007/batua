<script lang="ts">
  import type { TransactionEntry } from '../types';
  import TransactionCard from './TransactionCard.svelte';

  let { entries }: { entries: Array<TransactionEntry> } = $props();
</script>

<div class="tx-list">
  <h3 class="tx-list-title">Recent Activity</h3>
  {#if entries.length === 0}
    <div class="tx-empty">
      <p class="tx-empty-text">No transactions yet</p>
    </div>
  {:else}
    <div class="tx-items">
      {#each entries as entry (entry.id)}
        <TransactionCard {entry} />
      {/each}
    </div>
  {/if}
</div>

<style>
  .tx-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .tx-list-title {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .tx-items {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .tx-empty {
    padding: var(--space-8) var(--space-4);
    text-align: center;
  }

  .tx-empty-text {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }
</style>
