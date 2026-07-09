<script lang="ts">
  import type { PageData } from './$types';
  import type { DashboardStats, RecentEvent } from '$lib/client/modules/platform';
  import { StatsGrid, EventsTable } from '$lib/client/modules/platform/ui';

  let { data }: { data: PageData } = $props();

  let stats: DashboardStats = $derived(
    data.stats ?? {
      total_merchants: 0,
      total_wallets: 0,
      total_ledger_entries: 0,
      total_value_in_system: 0
    }
  );

  let events: RecentEvent[] = $derived(data.events ?? []);

  let metricItems = $derived([
    {
      label: 'Total merchants',
      value: stats.total_merchants,
      metricType: 'number' as const,
      icon: 'M'
    },
    {
      label: 'Total wallets',
      value: stats.total_wallets,
      metricType: 'number' as const,
      icon: 'W'
    },
    {
      label: 'Ledger entries',
      value: stats.total_ledger_entries,
      metricType: 'number' as const,
      icon: '#'
    },
    {
      label: 'Active credits',
      value: stats.total_value_in_system,
      metricType: 'currency' as const,
      icon: '$'
    }
  ]);
</script>

<svelte:head>
  <title>Dashboard - Breeze Platform</title>
</svelte:head>

<div class="dashboard">
  <div class="page-header">
    <h1>Platform dashboard</h1>
    <p class="subtitle">System-wide overview of the Breeze platform</p>
  </div>

  <StatsGrid items={metricItems} />

  <section class="events-section">
    <h3 class="section-title">Recent events</h3>
    <EventsTable {events} />
  </section>
</div>

<style>
  .dashboard {
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

  .events-section {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .section-title {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }
</style>
