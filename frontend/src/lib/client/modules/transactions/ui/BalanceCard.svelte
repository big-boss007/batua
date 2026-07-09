<script lang="ts">
  import type { WalletBalance } from '$lib/client/modules/transactions';
  import { formatCurrencyINR, formatPoints, isPointsBucket } from '$lib/client/modules/foundation';
  import { formatBucketType } from '$lib/client/modules/transactions';
  import { Progress } from '@juspay/svelte-ui-components';

  let { balance }: { balance: WalletBalance } = $props();

  let maxBucket = $derived(
    balance.buckets.length > 0 ? Math.max(...balance.buckets.map((b) => b.displayed)) : 1
  );
</script>

<div class="balance-card">
  <div class="balance-header">
    <span class="balance-label">Displayed balance</span>
    <span class="balance-amount">{formatCurrencyINR(balance.displayed_balance)}</span>
  </div>

  <div class="balance-sub">
    <span class="sub-label">Spendable</span>
    <span class="sub-amount">{formatCurrencyINR(balance.spendable_balance)}</span>
  </div>

  <div class="wallet-id">
    <span class="id-label">Wallet</span>
    <span class="id-value">{balance.wallet_id}</span>
  </div>

  {#if balance.buckets.length > 0}
    <div class="bucket-section">
      <span class="bucket-heading">Bucket breakdown</span>
      <div class="bucket-list">
        {#each balance.buckets as bucket}
          <div class="bucket-row">
            <div class="bucket-info">
              <span class="bucket-name">{formatBucketType(bucket.bucket_type)}</span>
              <span class="bucket-values">
                {isPointsBucket(bucket.bucket_type)
                  ? formatPoints(bucket.displayed, '★') +
                    ' / ' +
                    formatPoints(bucket.spendable, '★')
                  : formatCurrencyINR(bucket.displayed) +
                    ' / ' +
                    formatCurrencyINR(bucket.spendable)}
              </span>
            </div>
            <Progress value={maxBucket > 0 ? (bucket.displayed / maxBucket) * 100 : 0} />
            <span class="bucket-count">{bucket.count} entries</span>
          </div>
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
  .balance-card {
    background: var(--color-surface);
    border-radius: var(--radius-lg);
    padding: var(--space-6);
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    box-shadow: var(--shadow-card);
  }

  .balance-header {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
  }

  .balance-label {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    font-weight: var(--font-weight-medium);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .balance-amount {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
  }

  .balance-sub {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    padding-top: var(--space-2);
    border-top: 1px solid var(--color-border);
  }

  .sub-label {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .sub-amount {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .wallet-id {
    display: flex;
    gap: var(--space-2);
    align-items: center;
  }

  .id-label {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .id-value {
    font-size: var(--font-size-xs);
    font-family: var(--font-mono);
    color: var(--color-text-muted);
  }

  .bucket-section {
    padding-top: var(--space-4);
    border-top: 1px solid var(--color-border);
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .bucket-heading {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .bucket-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .bucket-row {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .bucket-info {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
  }

  .bucket-name {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-text);
  }

  .bucket-values {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .bucket-count {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    text-align: right;
  }
</style>
