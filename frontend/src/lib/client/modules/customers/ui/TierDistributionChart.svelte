<script lang="ts">
  import type { TierDistribution } from '$lib/client/modules/customers';
  import { getTierColor } from '$lib/client/modules/customers';

  let { distribution }: { distribution: Array<TierDistribution> } = $props();

  let maxCount = $derived(
    distribution.length > 0 ? Math.max(...distribution.map((d) => d.count)) : 1
  );

  let total = $derived(distribution.reduce((sum, d) => sum + d.count, 0));
</script>

<div class="distribution-chart">
  <h3 class="chart-title">Tier Distribution</h3>

  {#if distribution.length === 0}
    <p class="chart-empty">No tier distribution data available</p>
  {:else}
    <div class="chart-summary">
      <span class="summary-total">{total.toLocaleString('en-IN')} customers</span>
    </div>

    <div class="chart-bars">
      {#each distribution as item, index (item.tier_name)}
        <div class="bar-row">
          <span class="bar-label" style:color={getTierColor(index + 1)}>
            {item.tier_name}
          </span>
          <div class="bar-track">
            <div
              class="bar-fill"
              style:width="{maxCount > 0 ? (item.count / maxCount) * 100 : 0}%"
              style:background={getTierColor(index + 1)}
            ></div>
          </div>
          <span class="bar-count">{item.count.toLocaleString('en-IN')}</span>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .distribution-chart {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: var(--space-6);
  }

  .chart-title {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
    margin-bottom: var(--space-4);
  }

  .chart-empty {
    padding: var(--space-8) 0;
    text-align: center;
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
  }

  .chart-summary {
    margin-bottom: var(--space-4);
  }

  .summary-total {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .chart-bars {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .bar-row {
    display: grid;
    grid-template-columns: 100px 1fr 60px;
    gap: var(--space-3);
    align-items: center;
  }

  .bar-label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .bar-track {
    height: 24px;
    background: var(--color-surface-2);
    border-radius: var(--radius-sm);
    overflow: hidden;
  }

  .bar-fill {
    height: 100%;
    border-radius: var(--radius-sm);
    transition: width var(--transition-slow);
    min-width: 2px;
  }

  .bar-count {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-text);
    text-align: right;
    font-family: var(--font-mono);
  }
</style>
