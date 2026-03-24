<script lang="ts">
  import type { TransactionEntry, StorefrontMerchant } from '../types';
  import { groupEntriesByDate, formatDateLabel } from '../utils';
  import TransactionCard from './TransactionCard.svelte';

  let {
    entries,
    runningBalances = null,
    merchant
  }: {
    entries: Array<TransactionEntry>;
    runningBalances: Map<string, number> | null;
    merchant: StorefrontMerchant;
  } = $props();

  let dateGroups = $derived(groupEntriesByDate(entries));
</script>

<div class="tx-list">
  <div class="tx-list-header">
    <h3 class="tx-list-title">Activity</h3>
  </div>

  {#if entries.length === 0}
    <div class="tx-empty">
      <p class="tx-empty-text">No transactions yet</p>
    </div>
  {:else}
    {#each dateGroups as group}
      <div class="tx-date-group">
        <div class="tx-date-label">{group.label}</div>
        {#each group.entries as entry (entry.id)}
          <TransactionCard
            {entry}
            {merchant}
            runningBalance={runningBalances?.get(entry.id) ?? null}
          />
        {/each}
      </div>
    {/each}
  {/if}
</div>

<style>
  .tx-list {
    display: flex;
    flex-direction: column;
  }

  .tx-list-header {
    margin-bottom: 14px;
  }

  .tx-list-title {
    font-size: 12px;
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.8px;
    color: #9ca3af;
  }

  .tx-date-group {
    margin-bottom: 0;
  }

  .tx-date-group + .tx-date-group {
    border-top: 1px solid #2a2d3a;
    padding-top: 16px;
    margin-top: 8px;
  }

  .tx-date-label {
    font-size: 11px;
    color: #9ca3af;
    text-transform: uppercase;
    letter-spacing: 0.6px;
    font-weight: 500;
    margin-bottom: 4px;
  }

  .tx-empty {
    padding: 32px 16px;
    text-align: center;
  }

  .tx-empty-text {
    font-size: 13px;
    color: #9ca3af;
  }
</style>
