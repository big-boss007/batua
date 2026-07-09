<script lang="ts">
  import type { RedemptionRequest } from '$lib/client/modules/transactions';
  import { formatState } from '$lib/client/modules/transactions';
  import { formatCurrencyINR, formatDateTime } from '$lib/client/modules/foundation';
  import { Pill } from '@juspay/svelte-ui-components';

  let { redemptions }: { redemptions: Array<RedemptionRequest> } = $props();

  function statePillClass(state: string): string {
    const map: Record<string, string> = {
      completed: 'pill-success',
      approved: 'pill-success',
      pending: 'pill-warning',
      processing: 'pill-info',
      reversed: 'pill-info',
      failed: 'pill-error',
      rejected: 'pill-error',
      cancelled: 'pill-neutral'
    };
    return map[state.toLowerCase()] ?? 'pill-neutral';
  }
</script>

<div class="redemption-list">
  <h3 class="redemption-heading">Redemption history</h3>

  {#if redemptions.length === 0}
    <div class="empty-state">No redemptions found</div>
  {:else}
    <div class="redemption-items">
      {#each redemptions as redemption (redemption.id)}
        <div class="redemption-item">
          <div class="redemption-top">
            <div class="redemption-order">
              <span class="order-label">Order</span>
              <span class="order-id">{redemption.order_id}</span>
            </div>
            <Pill
              text={formatState(redemption.state).label}
              classes={statePillClass(redemption.state)}
            />
          </div>

          <div class="redemption-details">
            <div class="detail-pair">
              <span class="detail-label">Requested</span>
              <span class="detail-value">{formatCurrencyINR(redemption.requested_amount)}</span>
            </div>
            <div class="detail-pair">
              <span class="detail-label">Applied</span>
              <span class="detail-value">
                {redemption.applied_amount !== null
                  ? formatCurrencyINR(redemption.applied_amount)
                  : '\u2014'}
              </span>
            </div>
            <div class="detail-pair">
              <span class="detail-label">Date</span>
              <span class="detail-value date">{formatDateTime(redemption.created_at)}</span>
            </div>
          </div>

          <div class="redemption-meta">
            <span class="meta-id">{redemption.id}</span>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .redemption-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .redemption-heading {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .empty-state {
    padding: var(--space-8);
    text-align: center;
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
    border: 1px dashed var(--color-border);
    border-radius: var(--radius-md);
  }

  .redemption-items {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .redemption-item {
    background: var(--color-surface);
    border-radius: var(--radius-lg);
    padding: var(--space-4);
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
    transition: box-shadow var(--transition-fast);
    box-shadow: var(--shadow-card);
  }

  .redemption-item:hover {
    box-shadow: var(--shadow-sm);
  }

  .redemption-top {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .redemption-order {
    display: flex;
    gap: var(--space-2);
    align-items: center;
  }

  .order-label {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .order-id {
    font-size: var(--font-size-sm);
    font-family: var(--font-mono);
    font-weight: var(--font-weight-medium);
    color: var(--color-text);
  }

  .redemption-details {
    display: flex;
    gap: var(--space-6);
    flex-wrap: wrap;
  }

  .detail-pair {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .detail-label {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .detail-value {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-text);
  }

  .detail-value.date {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    font-weight: var(--font-weight-normal);
  }

  .redemption-meta {
    padding-top: var(--space-2);
    border-top: 1px solid var(--color-border);
  }

  .meta-id {
    font-size: var(--font-size-xs);
    font-family: var(--font-mono);
    color: var(--color-text-muted);
  }
</style>
