<script lang="ts">
  import { MetricCard } from '$lib/client/modules/admin/ui';
  import type { MetricType } from '$lib/client/modules/admin';

  type StatItem = {
    label: string;
    value: number;
    metricType: MetricType;
    icon: string;
  };

  let { items }: { items: StatItem[] } = $props();
</script>

<div class="stats-grid">
  {#each items as item}
    <MetricCard
      label={item.label}
      value={item.value}
      metricType={item.metricType}
      icon={item.icon}
    />
  {/each}
</div>

<style>
  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
    gap: var(--space-4);
  }
</style>
