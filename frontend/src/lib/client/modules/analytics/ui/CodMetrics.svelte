<script lang="ts">
  import type { CodAnalytics } from '$lib/client/modules/analytics';
  import { formatCurrencyINR } from '$lib/client/modules/foundation';

  let { analytics }: { analytics: CodAnalytics } = $props();

  let formattedPendingAmount = $derived(formatCurrencyINR(analytics.pending_amount));
  let formattedReleasedAmount = $derived(formatCurrencyINR(analytics.released_amount));
  let formattedCancelledAmount = $derived(formatCurrencyINR(analytics.cancelled_amount));
</script>

<div class="cod-metrics">
  <div class="metric-card metric-pending">
    <span class="metric-count">{analytics.total_pending}</span>
    <span class="metric-label">Pending</span>
    <span class="metric-amount">{formattedPendingAmount}</span>
  </div>

  <div class="metric-card metric-delivered">
    <span class="metric-count">{analytics.total_delivered}</span>
    <span class="metric-label">Delivered</span>
    <span class="metric-amount">{formattedReleasedAmount}</span>
  </div>

  <div class="metric-card metric-rto">
    <span class="metric-count">{analytics.total_rto}</span>
    <span class="metric-label">RTO</span>
    <span class="metric-amount">{formattedCancelledAmount}</span>
  </div>
</div>

<style>
  .cod-metrics {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: var(--space-4);
  }

  .metric-card {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    padding: var(--space-6);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-sm);
    transition: box-shadow var(--transition-fast);
  }

  .metric-card:hover {
    box-shadow: var(--shadow-md);
  }

  .metric-pending {
    border-left: 3px solid var(--color-warning);
  }

  .metric-delivered {
    border-left: 3px solid var(--color-success);
  }

  .metric-rto {
    border-left: 3px solid var(--color-error);
  }

  .metric-count {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    line-height: var(--line-height-tight);
  }

  .metric-label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .metric-pending .metric-label {
    color: var(--color-warning);
  }

  .metric-delivered .metric-label {
    color: var(--color-success);
  }

  .metric-rto .metric-label {
    color: var(--color-error);
  }

  .metric-amount {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-medium);
    color: var(--color-text);
  }
</style>
