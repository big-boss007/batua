<script lang="ts">
  import type { MerchantAnalytics } from '$lib/client/modules/analytics';
  import { fetchMerchantAnalytics } from '$lib/client/modules/analytics';
  import { currentMerchantId } from '$lib/client/modules/admin';
  import { formatCurrencyINR } from '$lib/client/modules/foundation';

  let analytics = $state<MerchantAnalytics | null>(null);
  let loading = $state(false);
  let merchantId = $state<string | null>(null);

  currentMerchantId.subscribe((id) => {
    const prevId = merchantId;
    merchantId = id;
    if (id !== null && id !== prevId) {
      loadAnalytics(id);
    }
  });

  async function loadAnalytics(mId: string) {
    loading = true;
    const result = await fetchMerchantAnalytics(mId);
    if (result.tag === 'success') {
      analytics = result.data;
    }
    loading = false;
  }

  let overviewCards = $derived(
    analytics !== null
      ? [
          { label: 'Total Earned', value: formatCurrencyINR(analytics.total_earned) },
          { label: 'Total Redeemed', value: formatCurrencyINR(analytics.total_redeemed) },
          { label: 'Active Credits', value: formatCurrencyINR(analytics.active_credits) },
          { label: 'Total Expired', value: formatCurrencyINR(analytics.total_expired) }
        ]
      : []
  );

  let codCards = $derived(
    analytics !== null
      ? [
          { label: 'COD Pending', value: analytics.cod_pending.toLocaleString('en-IN'), accent: 'warning' },
          { label: 'COD Delivered', value: analytics.cod_delivered.toLocaleString('en-IN'), accent: 'success' },
          { label: 'COD RTO', value: analytics.cod_rto.toLocaleString('en-IN'), accent: 'error' }
        ]
      : []
  );

  let prepaidPct = $derived(
    analytics !== null && analytics.total_orders > 0
      ? Math.round((analytics.prepaid_orders / analytics.total_orders) * 100)
      : 0
  );

  let codPct = $derived(
    analytics !== null && analytics.total_orders > 0
      ? Math.round((analytics.cod_orders / analytics.total_orders) * 100)
      : 0
  );

  let rtoMaxRate = $derived(
    analytics !== null
      ? Math.max(analytics.loyalty_rto_rate, analytics.non_loyalty_rto_rate, 1)
      : 1
  );

  let loyaltyWidth = $derived(
    analytics !== null ? (analytics.loyalty_rto_rate / rtoMaxRate) * 100 : 0
  );

  let nonLoyaltyWidth = $derived(
    analytics !== null ? (analytics.non_loyalty_rto_rate / rtoMaxRate) * 100 : 0
  );

  let rtoReduction = $derived(
    analytics !== null && analytics.non_loyalty_rto_rate > 0
      ? Math.round(
          ((analytics.non_loyalty_rto_rate - analytics.loyalty_rto_rate) /
            analytics.non_loyalty_rto_rate) *
            100
        )
      : 0
  );
</script>

<svelte:head>
  <title>Analytics - Batua</title>
</svelte:head>

<div class="analytics-page">
  <header class="page-header">
    <h1 class="page-title">Analytics</h1>
  </header>

  {#if merchantId === null}
    <div class="empty-state">
      <p class="empty-text">Select a merchant to view analytics</p>
    </div>
  {:else if loading}
    <div class="empty-state">
      <p class="empty-text">Loading analytics...</p>
    </div>
  {:else if analytics !== null}
    <section class="section">
      <h2 class="section-title">Overview</h2>
      <div class="overview-cards">
        {#each overviewCards as card (card.label)}
          <div class="metric-card">
            <span class="card-value">{card.value}</span>
            <span class="card-label">{card.label}</span>
          </div>
        {/each}
      </div>
    </section>

    <section class="section">
      <h2 class="section-title">COD Analytics</h2>
      <div class="cod-cards">
        {#each codCards as card (card.label)}
          <div class="metric-card cod-card cod-{card.accent}">
            <span class="card-value">{card.value}</span>
            <span class="card-label">{card.label}</span>
          </div>
        {/each}
      </div>

      <div class="order-split">
        <h3 class="split-title">Prepaid vs COD Split</h3>
        <div class="split-bars">
          <div class="split-bar-group">
            <div class="split-header">
              <span class="split-label">Prepaid</span>
              <span class="split-value">{analytics.prepaid_orders.toLocaleString('en-IN')} ({prepaidPct}%)</span>
            </div>
            <div class="split-track">
              <div class="split-fill split-prepaid" style="width: {prepaidPct}%"></div>
            </div>
          </div>
          <div class="split-bar-group">
            <div class="split-header">
              <span class="split-label">COD</span>
              <span class="split-value">{analytics.cod_orders.toLocaleString('en-IN')} ({codPct}%)</span>
            </div>
            <div class="split-track">
              <div class="split-fill split-cod" style="width: {codPct}%"></div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <section class="section">
      <h2 class="section-title">ROI Metrics</h2>
      <div class="rto-comparison">
        <h3 class="comparison-subtitle">RTO Rate Comparison</h3>
        {#if rtoReduction > 0}
          <p class="reduction-badge">{rtoReduction}% lower RTO with loyalty</p>
        {/if}
        <div class="comparison-bars">
          <div class="bar-group">
            <div class="bar-header">
              <span class="bar-label">Loyalty Members</span>
              <span class="bar-value">{analytics.loyalty_rto_rate.toFixed(1)}%</span>
            </div>
            <div class="bar-track">
              <div class="bar-fill bar-loyalty" style="width: {loyaltyWidth}%"></div>
            </div>
          </div>
          <div class="bar-group">
            <div class="bar-header">
              <span class="bar-label">Non-Members</span>
              <span class="bar-value">{analytics.non_loyalty_rto_rate.toFixed(1)}%</span>
            </div>
            <div class="bar-track">
              <div class="bar-fill bar-non-loyalty" style="width: {nonLoyaltyWidth}%"></div>
            </div>
          </div>
        </div>
      </div>

      <div class="repeat-rate-card">
        <span class="repeat-rate-value">{analytics.repeat_purchase_rate.toFixed(1)}%</span>
        <span class="repeat-rate-label">Repeat Purchase Rate</span>
      </div>
    </section>
  {/if}
</div>

<style>
  .analytics-page {
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
    padding: var(--space-8);
    max-width: 1200px;
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .page-title {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
  }

  .section {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .section-title {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .overview-cards {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: var(--space-4);
  }

  .metric-card {
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

  .metric-card:hover {
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

  .cod-cards {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: var(--space-4);
  }

  .cod-card {
    border-left: 3px solid var(--color-border);
  }

  .cod-warning {
    border-left-color: var(--color-warning);
  }

  .cod-success {
    border-left-color: var(--color-success);
  }

  .cod-error {
    border-left-color: var(--color-error);
  }

  .order-split {
    padding: var(--space-6);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-sm);
  }

  .split-title {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
    margin-bottom: var(--space-4);
  }

  .split-bars {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .split-bar-group {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .split-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .split-label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-text);
  }

  .split-value {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
  }

  .split-track {
    height: 10px;
    background: var(--color-surface-2);
    border-radius: var(--radius-full);
    overflow: hidden;
  }

  .split-fill {
    height: 100%;
    border-radius: var(--radius-full);
    transition: width var(--transition-slow);
    min-width: 2px;
  }

  .split-prepaid {
    background: var(--color-primary);
  }

  .split-cod {
    background: var(--color-warning);
  }

  .rto-comparison {
    padding: var(--space-6);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-sm);
  }

  .comparison-subtitle {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
    margin-bottom: var(--space-4);
  }

  .reduction-badge {
    display: inline-block;
    padding: var(--space-1) var(--space-3);
    background: color-mix(in srgb, var(--color-success) 12%, transparent);
    color: var(--color-success);
    border-radius: var(--radius-full);
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    margin-bottom: var(--space-6);
  }

  .comparison-bars {
    display: flex;
    flex-direction: column;
    gap: var(--space-5);
  }

  .bar-group {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .bar-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .bar-label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-text);
  }

  .bar-value {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
  }

  .bar-track {
    height: 12px;
    background: var(--color-surface-2);
    border-radius: var(--radius-full);
    overflow: hidden;
  }

  .bar-fill {
    height: 100%;
    border-radius: var(--radius-full);
    transition: width var(--transition-slow);
    min-width: 2px;
  }

  .bar-loyalty {
    background: var(--color-success);
  }

  .bar-non-loyalty {
    background: var(--color-error);
  }

  .repeat-rate-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-8);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-sm);
  }

  .repeat-rate-value {
    font-size: var(--font-size-3xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-primary);
    line-height: var(--line-height-tight);
  }

  .repeat-rate-label {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    font-weight: var(--font-weight-medium);
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
</style>
