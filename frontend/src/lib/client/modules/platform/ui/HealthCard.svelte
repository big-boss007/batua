<script lang="ts">
  import { formatMetricValue } from '$lib/client/modules/admin';
  import type { MetricType } from '$lib/client/modules/admin';

  let {
    label,
    value,
    metricType = 'number',
    threshold = 'green',
    subtitle = null
  }: {
    label: string;
    value: number;
    metricType?: MetricType;
    threshold?: 'green' | 'amber' | 'red';
    subtitle?: string | null;
  } = $props();

  let formattedValue = $derived(formatMetricValue(value, metricType));
</script>

<div class="health-card" data-threshold={threshold}>
  <div class="health-indicator"></div>
  <div class="health-content">
    <span class="health-value">{formattedValue}</span>
    <span class="health-label">{label}</span>
    {#if subtitle}
      <span class="health-subtitle">{subtitle}</span>
    {/if}
  </div>
</div>

<style>
  .health-card {
    display: flex;
    align-items: stretch;
    background: var(--color-surface);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-card);
    overflow: hidden;
    transition: box-shadow var(--transition-fast);
  }

  .health-card:hover {
    box-shadow: var(--shadow-md);
  }

  .health-indicator {
    width: 4px;
    flex-shrink: 0;
  }

  .health-card[data-threshold='green'] .health-indicator {
    background: var(--color-success);
  }

  .health-card[data-threshold='amber'] .health-indicator {
    background: var(--color-warning);
  }

  .health-card[data-threshold='red'] .health-indicator {
    background: var(--color-error);
  }

  .health-content {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    padding: var(--space-5) var(--space-6);
    min-width: 0;
  }

  .health-value {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    line-height: var(--line-height-tight);
  }

  .health-label {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    font-weight: var(--font-weight-medium);
  }

  .health-subtitle {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }
</style>
