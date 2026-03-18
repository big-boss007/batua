<script lang="ts">
  import type { OverviewMetrics } from '$lib/client/modules/analytics';
  import { formatCurrencyINR } from '$lib/client/modules/foundation';

  let { metrics }: { metrics: OverviewMetrics } = $props();

  let cards = $derived([
    { label: 'Total Wallets', value: metrics.total_wallets.toLocaleString('en-IN') },
    { label: 'Active Credits', value: formatCurrencyINR(metrics.total_active_credits) },
    { label: 'Total Redeemed', value: formatCurrencyINR(metrics.total_redeemed) },
    { label: 'Total Expired', value: formatCurrencyINR(metrics.total_expired) }
  ]);
</script>

<div class="overview-cards">
  {#each cards as card (card.label)}
    <div class="overview-card">
      <span class="card-value">{card.value}</span>
      <span class="card-label">{card.label}</span>
    </div>
  {/each}
</div>

<style>
  .overview-cards {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: var(--space-4);
  }

  .overview-card {
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

  .overview-card:hover {
    box-shadow: var(--shadow-md);
  }

  .card-value {
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    line-height: var(--line-height-tight);
  }

  .card-label {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    font-weight: var(--font-weight-medium);
  }
</style>
