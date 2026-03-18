<script lang="ts">
  import type { MetricType } from '$lib/client/modules/admin';
  import { formatMetricValue } from '$lib/client/modules/admin';

  let {
    label,
    value,
    metricType = 'number',
    icon = null
  }: {
    label: string;
    value: number;
    metricType?: MetricType;
    icon?: string | null;
  } = $props();

  let formattedValue = $derived(formatMetricValue(value, metricType));
</script>

<div class="metric-card">
  {#if icon}
    <span class="metric-icon">{icon}</span>
  {/if}
  <div class="metric-content">
    <span class="metric-value">{formattedValue}</span>
    <span class="metric-label">{label}</span>
  </div>
</div>

<style>
  .metric-card {
    display: flex;
    align-items: flex-start;
    gap: var(--space-4);
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

  .metric-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 40px;
    height: 40px;
    font-size: var(--font-size-xl);
    background: var(--color-surface-2);
    border-radius: var(--radius-md);
    flex-shrink: 0;
  }

  .metric-content {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    min-width: 0;
  }

  .metric-value {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    line-height: var(--line-height-tight);
  }

  .metric-label {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    font-weight: var(--font-weight-medium);
  }
</style>
