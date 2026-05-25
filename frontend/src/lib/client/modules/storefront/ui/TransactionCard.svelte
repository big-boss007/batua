<script lang="ts">
  import {
    formatCurrencyINR,
    formatPoints,
    isPointsBucket
  } from '$lib/client/modules/foundation';
  import { formatBucketLabel, formatMovementLabel, getMovementPrefix } from '../utils';
  import type { TransactionEntry, StorefrontMerchant } from '../types';

  let {
    entry,
    merchant,
    dateMeta = null,
    runningBalance = null
  }: {
    entry: TransactionEntry;
    merchant: StorefrontMerchant;
    dateMeta?: string | null;
    runningBalance?: number | null;
  } = $props();

  let isPoints = $derived(isPointsBucket(entry.bucket_type));

  let colorClass = $derived.by(() => {
    if (isPoints) {
      return entry.movement_type === 'Out' || entry.movement_type === 'Expired'
        ? 'indicator-debit'
        : 'indicator-credit';
    }
    return 'indicator-cash';
  });

  let amountColorClass = $derived.by(() => {
    if (isPoints) {
      return entry.movement_type === 'Out' || entry.movement_type === 'Expired'
        ? 'amount-debit'
        : 'amount-credit';
    }
    return 'amount-cash';
  });

  let prefix = $derived(getMovementPrefix(entry.movement_type));

  let amountText = $derived.by(() => {
    if (isPoints) {
      return `${prefix}${formatPoints(Math.abs(entry.earning_unit), merchant.points_icon)}`;
    }
    return `${prefix}${formatCurrencyINR(Math.abs(entry.currency_equivalent))}`;
  });

  let hintText = $derived.by(() => {
    if (isPoints) {
      return `≈ ${formatCurrencyINR(Math.abs(entry.currency_equivalent))}`;
    }
    return 'cash';
  });

  let metaText = $derived.by(() => {
    const bucket = formatBucketLabel(entry.bucket_type);
    return dateMeta !== null ? `${bucket} · ${dateMeta}` : bucket;
  });
</script>

<div class="tx-item">
  <div class="tx-indicator {colorClass}"></div>
  <div class="tx-details">
    <span class="tx-label">{formatMovementLabel(entry.movement_type)}</span>
    <span class="tx-meta">{metaText}</span>
  </div>
  <div class="tx-right">
    <span class="tx-amount {amountColorClass}">{amountText}</span>
    <span class="tx-hint">{hintText}</span>
    {#if runningBalance !== null}
      <span class="tx-running">bal {formatCurrencyINR(runningBalance)}</span>
    {/if}
  </div>
</div>

<style>
  .tx-item {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 14px 0;
  }

  .tx-indicator {
    width: 4px;
    height: 32px;
    border-radius: 2px;
    flex-shrink: 0;
  }

  .tx-indicator.indicator-credit {
    background: #4ade80;
  }

  .tx-indicator.indicator-debit {
    background: #f87171;
  }

  .tx-indicator.indicator-cash {
    background: #c4b5fd;
  }

  .tx-details {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-width: 0;
  }

  .tx-label {
    font-size: 13px;
    font-weight: 500;
    color: #ffffff;
  }

  .tx-meta {
    font-size: 11px;
    color: #9ca3af;
    margin-top: 2px;
  }

  .tx-right {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    flex-shrink: 0;
  }

  .tx-amount {
    font-size: 14px;
    font-weight: 600;
    font-variant-numeric: tabular-nums;
  }

  .tx-amount.amount-credit {
    color: #4ade80;
  }

  .tx-amount.amount-debit {
    color: #ffffff;
  }

  .tx-amount.amount-cash {
    color: #c4b5fd;
  }

  .tx-hint {
    font-size: 9px;
    color: #6b7280;
    margin-top: 1px;
    font-variant-numeric: tabular-nums;
  }

  .tx-running {
    font-size: 10px;
    color: #9ca3af;
    margin-top: 2px;
    font-variant-numeric: tabular-nums;
  }
</style>
