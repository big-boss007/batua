<script lang="ts">
  import type { MetricType } from '$lib/client/modules/admin';
  import { formatMetricValue } from '$lib/client/modules/admin';

  let {
    label,
    value = 0,
    displayValue = null,
    metricType = 'number',
    size = 'default',
    icon = null,
    pointsIcon = 'pts',
    pointsRate = 1.0,
    sub = null,
    valueColor = null,
    subColor = null
  }: {
    label: string;
    value?: number;
    displayValue?: string | null;
    metricType?: MetricType;
    size?: 'default' | 'compact';
    icon?: string | null;
    pointsIcon?: string;
    pointsRate?: number;
    sub?: string | null;
    valueColor?: string | null;
    subColor?: string | null;
  } = $props();

  let formattedValue = $derived.by(() => {
    if (displayValue !== null) return displayValue;
    if (metricType === 'points') {
      const pts = pointsRate > 0 ? Math.round(value / pointsRate) : value;
      return `${pts.toLocaleString('en-IN')} ${pointsIcon}`;
    }
    return formatMetricValue(value, metricType);
  });

  let subText = $derived.by(() => {
    if (sub !== null) return sub;
    if (metricType === 'points') return `≈ ${formatMetricValue(value, 'currency')}`;
    return null;
  });
</script>

<div class="metric-card" class:compact={size === 'compact'}>
  {#if icon}
    <span class="metric-icon">{icon}</span>
  {/if}
  <div class="metric-content">
    <span class="metric-value" style={valueColor ? `color: ${valueColor}` : ''}>{formattedValue}</span>
    {#if subText !== null}
      <span class="metric-sub" style={subColor ? `color: ${subColor}` : ''}>{subText}</span>
    {/if}
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

  .metric-card.compact {
    padding: var(--space-4);
    gap: var(--space-3);
  }

  .metric-card.compact .metric-value {
    font-size: var(--font-size-xl);
  }

  .metric-card.compact .metric-label {
    font-size: var(--font-size-xs);
  }

  .metric-card.compact .metric-icon {
    width: 32px;
    height: 32px;
    font-size: var(--font-size-md);
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

  .metric-sub {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .metric-label {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    font-weight: var(--font-weight-medium);
  }
</style>
