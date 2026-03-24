<script lang="ts">
  import type { OverviewMetrics } from '$lib/client/modules/analytics';
  import { formatCurrencyINR, formatPoints } from '$lib/client/modules/foundation';

  let {
    metrics,
    pointsIcon = 'pts',
    pointsRate = 1.0
  }: {
    metrics: OverviewMetrics;
    pointsIcon?: string;
    pointsRate?: number;
  } = $props();

  let cards = $derived([
    { label: 'Total Wallets', value: metrics.total_wallets.toLocaleString('en-IN'), sub: null },
    {
      label: 'Active Credits',
      value: formatPoints(Math.round(metrics.total_active_credits / pointsRate), pointsIcon),
      sub: '≈ ' + formatCurrencyINR(metrics.total_active_credits)
    },
    {
      label: 'Total Redeemed',
      value: formatPoints(Math.round(metrics.total_redeemed / pointsRate), pointsIcon),
      sub: '≈ ' + formatCurrencyINR(metrics.total_redeemed)
    },
    {
      label: 'Total Expired',
      value: formatPoints(Math.round(metrics.total_expired / pointsRate), pointsIcon),
      sub: '≈ ' + formatCurrencyINR(metrics.total_expired)
    }
  ]);
</script>

<div class="overview-cards">
  {#each cards as card (card.label)}
    <div class="overview-card">
      <span class="card-value">{card.value}</span>
      {#if card.sub !== null}
        <span class="card-sub">{card.sub}</span>
      {/if}
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

  .card-sub {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .card-label {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    font-weight: var(--font-weight-medium);
  }
</style>
