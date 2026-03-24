<script lang="ts">
  import { goto } from '$app/navigation';
  import { Button, Shimmer } from '@juspay/svelte-ui-components';
  import type { MerchantDashboard, Merchant } from '$lib/client/modules/admin';
  import {
    currentMerchant,
    currentMerchantId,
    fetchMerchantDashboard
  } from '$lib/client/modules/admin';
  import { MetricCard } from '$lib/client/modules/admin/ui';

  let dashboard = $state<MerchantDashboard | null>(null);
  let loading = $state(false);
  let merchantId = $state<string | null>(null);
  let merchant = $state<Merchant | null>(null);

  currentMerchantId.subscribe((id) => {
    if (id !== null && id !== merchantId) {
      merchantId = id;
      loadDashboard(id);
    } else if (id === null) {
      merchantId = null;
      dashboard = null;
    }
  });

  currentMerchant.subscribe((m) => {
    merchant = m;
  });

  async function loadDashboard(id: string) {
    loading = true;
    dashboard = null;
    const result = await fetchMerchantDashboard(id);
    if (result.tag === 'success') {
      dashboard = result.data;
    }
    loading = false;
  }

  let storefrontSlug = $derived(merchant !== null ? merchant.external_id : null);
</script>

<svelte:head>
  <title>Wallet - Batua Admin</title>
</svelte:head>

<div class="dashboard">
  <div class="dashboard-header">
    <h1>Wallet</h1>
    <p class="subtitle">Overview of your wallet program</p>
  </div>

  {#if merchantId === null}
    <div class="empty-state">
      <p class="empty-text">Select a merchant to view dashboard</p>
    </div>
  {:else if loading}
    <div class="stats-grid">
      <Shimmer classes="shimmer-card" />
      <Shimmer classes="shimmer-card" />
      <Shimmer classes="shimmer-card" />
      <Shimmer classes="shimmer-card" />
      <Shimmer classes="shimmer-card" />
      <Shimmer classes="shimmer-card" />
    </div>
  {:else if dashboard !== null}
    {@const pIcon = merchant?.points_icon ?? 'pts'}
    {@const pRate = merchant?.points_to_currency_rate ?? 1.0}
    {@const pName = merchant?.points_name ?? 'Points'}
    <div class="metrics-grid">
      <MetricCard
        label="Active Customers"
        value={dashboard.active_customers}
        metricType="number"
      />
      <MetricCard
        label="Active {pName}"
        value={dashboard.active_credits}
        metricType="points"
        pointsIcon={pIcon}
        pointsRate={pRate}
        valueColor="var(--color-success)"
        subColor="var(--color-primary)"
      />
      <MetricCard
        label="Total Earned"
        value={dashboard.total_earned}
        metricType="points"
        pointsIcon={pIcon}
        pointsRate={pRate}
        subColor="var(--color-text-muted)"
      />
      <MetricCard
        label="Total Redeemed"
        value={dashboard.total_redeemed}
        metricType="points"
        pointsIcon={pIcon}
        pointsRate={pRate}
        subColor="var(--color-text-muted)"
      />
      <MetricCard
        label="COD Pending"
        value={dashboard.total_cod_pending}
        metricType="points"
        pointsIcon={pIcon}
        pointsRate={pRate}
        valueColor="var(--color-warning)"
        subColor="var(--color-text-muted)"
      />
      <MetricCard
        label="Redemptions"
        value={dashboard.redemption_count}
        metricType="number"
      />
    </div>
  {/if}

  <section class="quick-actions">
    <h3 class="section-title">Quick Actions</h3>
    <div class="actions-grid">
      <Button text="Create Rule" classes="btn-primary" onclick={() => goto('/admin/rules')} />
      <Button
        text="View Transactions"
        classes="btn-secondary"
        onclick={() => goto('/admin/transactions')}
      />
      <Button
        text="View Customers"
        classes="btn-secondary"
        onclick={() => goto('/admin/customers')}
      />
      {#if storefrontSlug !== null}
        <Button
          text="View Storefront"
          classes="btn-secondary"
          onclick={() => window.open(`/s/${storefrontSlug}`, '_blank')}
        />
      {/if}
    </div>
  </section>
</div>

<style>
  .dashboard {
    display: flex;
    flex-direction: column;
    gap: var(--space-8);
  }

  .dashboard-header {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .dashboard-header h1 {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    line-height: var(--line-height-tight);
  }

  .subtitle {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .metrics-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: var(--space-4);
  }

  .quick-actions {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .actions-grid {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-3);
  }

  .section-title {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--space-16);
    background: var(--color-surface);
    border: 1px dashed var(--color-border);
    border-radius: var(--radius-lg);
  }

  .empty-text {
    color: var(--color-text-muted);
    font-size: var(--font-size-base);
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
    gap: var(--space-4);
  }

  :global(.shimmer-card) {
    --shimmer-width: 100%;
    --shimmer-height: 100px;
    --shimmer-border-radius: 12px;
  }

  @media (max-width: 768px) {
    .metrics-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
