<script lang="ts">
  import { Progress } from '@juspay/svelte-ui-components';
  import { formatCurrencyINR } from '$lib/client/modules/foundation';
  import { formatBucketLabel } from '../utils';
  import type { CustomerBalance } from '../types';

  let { balance }: { balance: CustomerBalance } = $props();

  let totalDisplayed = $derived(
    balance.buckets.length > 0
      ? balance.buckets.reduce((sum, b) => sum + b.displayed, 0)
      : balance.displayed_balance
  );
</script>

<div class="balance-card">
  <div class="balance-header">
    <span class="balance-label">Your Balance</span>
  </div>
  <div class="balance-amount">
    {formatCurrencyINR(balance.spendable_balance)}
  </div>
  <p class="balance-subtitle">
    Spendable balance
  </p>
  {#if balance.displayed_balance !== balance.spendable_balance}
    <p class="balance-displayed">
      {formatCurrencyINR(balance.displayed_balance)} total (includes pending)
    </p>
  {/if}

  {#if balance.buckets.length > 0}
    <div class="bucket-list">
      {#each balance.buckets as bucket}
        <div class="bucket-row">
          <div class="bucket-info">
            <span class="bucket-name">{formatBucketLabel(bucket.bucket_type)}</span>
            <span class="bucket-amount">{formatCurrencyINR(bucket.spendable)}</span>
          </div>
          <div class="bucket-bar">
            <Progress
              value={totalDisplayed > 0 ? (bucket.displayed / totalDisplayed) * 100 : 0}
              classes="bucket-progress"
            />
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .balance-card {
    background: linear-gradient(135deg, var(--color-primary) 0%, var(--color-primary-hover) 100%);
    border-radius: var(--radius-lg);
    padding: var(--space-6);
    color: #ffffff;
    box-shadow: var(--shadow-md);
  }

  .balance-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: var(--space-2);
  }

  .balance-label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    opacity: 0.85;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .balance-amount {
    font-size: 32px;
    font-weight: var(--font-weight-bold);
    line-height: var(--line-height-tight);
    margin-bottom: var(--space-1);
  }

  .balance-subtitle {
    font-size: var(--font-size-sm);
    opacity: 0.8;
  }

  .balance-displayed {
    font-size: var(--font-size-xs);
    opacity: 0.65;
    margin-top: var(--space-1);
  }

  .bucket-list {
    margin-top: var(--space-5);
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
    padding-top: var(--space-4);
    border-top: 1px solid rgba(255, 255, 255, 0.2);
  }

  .bucket-row {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .bucket-info {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .bucket-name {
    font-size: var(--font-size-sm);
    opacity: 0.9;
  }

  .bucket-amount {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
  }

  .bucket-bar {
    --progress-track-background: rgba(255, 255, 255, 0.2);
    --progress-bar-background: rgba(255, 255, 255, 0.85);
    --progress-track-height: 4px;
  }
</style>
