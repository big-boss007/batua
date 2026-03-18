<script lang="ts">
  import { RelativeTime } from '@juspay/svelte-ui-components';
  import { formatCurrencyINR } from '$lib/client/modules/foundation';
  import { formatBucketLabel, formatMovementLabel, getMovementPrefix } from '../utils';
  import type { TransactionEntry } from '../types';

  let { entry }: { entry: TransactionEntry } = $props();

  let icon = $derived.by(() => {
    switch (entry.movement_type) {
      case 'In': return '+';
      case 'Out': return '-';
      case 'Held': return '\u23F3';
      case 'Across': return '\u21C4';
      case 'Released': return '+';
      case 'Expired': return '-';
      default: return '\u2022';
    }
  });

  let colorClass = $derived.by(() => {
    switch (entry.movement_type) {
      case 'In':
      case 'Released':
        return 'amount-positive';
      case 'Out':
      case 'Expired':
        return 'amount-negative';
      case 'Held':
        return 'amount-held';
      default:
        return 'amount-neutral';
    }
  });

  let prefix = $derived(getMovementPrefix(entry.movement_type));
</script>

<div class="tx-card">
  <div class="tx-icon {colorClass}">{icon}</div>
  <div class="tx-details">
    <span class="tx-type">{formatMovementLabel(entry.movement_type)}</span>
    <span class="tx-bucket">{formatBucketLabel(entry.bucket_type)}</span>
  </div>
  <div class="tx-right">
    <span class="tx-amount {colorClass}">
      {prefix}{formatCurrencyINR(Math.abs(entry.currency_equivalent))}
    </span>
    <span class="tx-time">
      <RelativeTime date={entry.created_at} format="short" />
    </span>
  </div>
</div>

<style>
  .tx-card {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-3) var(--space-4);
    background: var(--color-surface);
    border-radius: var(--radius-md);
    transition: background var(--transition-fast);
  }

  .tx-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    border-radius: var(--radius-full);
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-bold);
    flex-shrink: 0;
  }

  .tx-icon.amount-positive {
    background: #dcfce7;
    color: #166534;
  }

  .tx-icon.amount-negative {
    background: #fef2f2;
    color: #991b1b;
  }

  .tx-icon.amount-held {
    background: #fefce8;
    color: #854d0e;
  }

  .tx-icon.amount-neutral {
    background: var(--color-surface-2);
    color: var(--color-text-muted);
  }

  .tx-details {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-width: 0;
  }

  .tx-type {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-medium);
    color: var(--color-text);
  }

  .tx-bucket {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .tx-right {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    flex-shrink: 0;
  }

  .tx-amount {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-semibold);
  }

  .tx-amount.amount-positive {
    color: var(--color-success);
  }

  .tx-amount.amount-negative {
    color: var(--color-error);
  }

  .tx-amount.amount-held {
    color: var(--color-warning);
  }

  .tx-amount.amount-neutral {
    color: var(--color-text-muted);
  }

  .tx-time {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    --relative-time-font-size: var(--font-size-xs);
    --relative-time-color: var(--color-text-muted);
  }
</style>
