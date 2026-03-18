<script lang="ts">
  import type { PageData } from './$types';
  import type { SystemHealth } from '$lib/client/modules/platform';
  import { formatMetricValue } from '$lib/client/modules/admin';
  import { HealthCard } from '$lib/client/modules/platform/ui';

  let { data }: { data: PageData } = $props();

  let health: SystemHealth = $derived(
    data.health ?? {
      unprocessed_events: 0,
      failed_events: 0,
      pending_cod_orders: 0,
      expiring_7d_count: 0,
      expiring_7d_value: 0,
      expiring_30d_count: 0,
      expiring_30d_value: 0
    }
  );

  function eventThreshold(count: number): 'green' | 'amber' | 'red' {
    if (count === 0) return 'green';
    return 'amber';
  }

  function failedThreshold(count: number): 'green' | 'amber' | 'red' {
    if (count === 0) return 'green';
    return 'red';
  }
</script>

<svelte:head>
  <title>System Health - Breeze Platform</title>
</svelte:head>

<div class="system-page">
  <div class="page-header">
    <h1>System Health</h1>
    <p class="subtitle">Operational diagnostics and processing status</p>
  </div>

  <section class="health-section">
    <h3 class="section-title">Processing</h3>
    <div class="health-grid">
      <HealthCard
        label="Unprocessed Events"
        value={health.unprocessed_events}
        threshold={eventThreshold(health.unprocessed_events)}
        subtitle="Events in received state"
      />
      <HealthCard
        label="Failed Events"
        value={health.failed_events}
        threshold={failedThreshold(health.failed_events)}
        subtitle="Events needing attention"
      />
      <HealthCard
        label="Pending COD Orders"
        value={health.pending_cod_orders}
        threshold={eventThreshold(health.pending_cod_orders)}
        subtitle="Awaiting delivery signal"
      />
    </div>
  </section>

  <section class="health-section">
    <h3 class="section-title">Credit Expiry</h3>
    <div class="health-grid">
      <HealthCard
        label="Expiring in 7 Days"
        value={health.expiring_7d_count}
        threshold={health.expiring_7d_count > 0 ? 'amber' : 'green'}
        subtitle={`Value: ${formatMetricValue(health.expiring_7d_value, 'currency')}`}
      />
      <HealthCard
        label="Expiring in 30 Days"
        value={health.expiring_30d_count}
        threshold={health.expiring_30d_count > 0 ? 'amber' : 'green'}
        subtitle={`Value: ${formatMetricValue(health.expiring_30d_value, 'currency')}`}
      />
    </div>
  </section>
</div>

<style>
  .system-page {
    display: flex;
    flex-direction: column;
    gap: var(--space-8);
  }

  .page-header {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .page-header h1 {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    line-height: var(--line-height-tight);
  }

  .subtitle {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .health-section {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .section-title {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .health-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
    gap: var(--space-4);
  }
</style>
