<script lang="ts">
  import { goto } from '$app/navigation';
  import { Button, Toggle, Select } from '@juspay/svelte-ui-components';
  import type { PageData } from './$types';
  import type { PlatformMerchant, MerchantStats } from '$lib/client/modules/platform';
  import { updateMerchant, updateMerchantPlan } from '$lib/client/modules/platform';
  import { setCurrentMerchantId } from '$lib/client/modules/admin';
  import { toastStore, formatDate } from '$lib/client/modules/foundation';
  import { StatsGrid } from '$lib/client/modules/platform/ui';

  let { data }: { data: PageData } = $props();

  let merchant: PlatformMerchant | null = $derived(data.merchant ?? null);
  let stats: MerchantStats | null = $derived(data.stats ?? null);

  // svelte-ignore state_referenced_locally
  let currentPlan = $state(data.merchant?.plan_tier ?? 'free');
  // svelte-ignore state_referenced_locally
  let isActive = $state(data.merchant?.is_active ?? false);

  const planItems = [
    { id: 'free', label: 'Free' },
    { id: 'grow', label: 'Grow' },
    { id: 'scale', label: 'Scale' },
    { id: 'enterprise', label: 'Enterprise' }
  ];

  let statItems = $derived(
    stats !== null
      ? [
          {
            label: 'Total wallets',
            value: stats.total_wallets,
            metricType: 'number' as const,
            icon: 'W'
          },
          {
            label: 'Customers',
            value: stats.total_customers,
            metricType: 'number' as const,
            icon: 'C'
          },
          {
            label: 'Ledger entries',
            value: stats.total_ledger_entries,
            metricType: 'number' as const,
            icon: '#'
          },
          {
            label: 'Active credits',
            value: stats.active_credits,
            metricType: 'currency' as const,
            icon: '$'
          },
          {
            label: 'Total redeemed',
            value: stats.total_redeemed,
            metricType: 'currency' as const,
            icon: 'R'
          }
        ]
      : []
  );

  async function handlePlanChange(value: string[]) {
    if (merchant === null || value.length === 0) return;
    const plan = value[0];
    if (plan === currentPlan) return;

    const result = await updateMerchantPlan(merchant.id, plan);
    if (result.tag === 'success') {
      currentPlan = plan;
      toastStore.push({ message: `Plan updated to ${plan}`, level: 'success' });
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }
  }

  async function handleStatusToggle(checked: boolean) {
    if (merchant === null) return;
    const result = await updateMerchant(merchant.id, { is_active: checked } as Record<
      string,
      unknown
    >);
    if (result.tag === 'success') {
      isActive = checked;
      toastStore.push({
        message: checked ? 'Merchant activated' : 'Merchant deactivated',
        level: 'success'
      });
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }
  }
</script>

<svelte:head>
  <title>{merchant?.name ?? 'Merchant'} - Breeze Platform</title>
</svelte:head>

{#if merchant === null}
  <div class="not-found">
    <h2>Merchant not found</h2>
    <a href="/platform/merchants">Back to merchants</a>
  </div>
{:else}
  <div class="merchant-detail">
    <div class="page-header">
      <div class="header-text">
        <div class="breadcrumb">
          <a href="/platform/merchants">Merchants</a>
          <span class="sep">/</span>
          <span>{merchant.name}</span>
        </div>
        <h1>{merchant.name}</h1>
      </div>
      <Button
        text="Open as Merchant"
        classes="btn-primary"
        onclick={() => {
          if (merchant !== null) {
            setCurrentMerchantId(merchant.id);
            goto('/admin');
          }
        }}
      />
    </div>

    <div class="info-card">
      <div class="info-grid">
        <div class="info-item">
          <span class="info-label">Name</span>
          <span class="info-value">{merchant.name}</span>
        </div>
        <div class="info-item">
          <span class="info-label">Slug</span>
          <span class="info-value">{merchant.slug ?? '--'}</span>
        </div>
        <div class="info-item">
          <span class="info-label">Domain</span>
          <span class="info-value">{merchant.domain ?? '--'}</span>
        </div>
        <div class="info-item">
          <span class="info-label">External ID</span>
          <span class="info-value mono">{merchant.external_id}</span>
        </div>
        <div class="info-item">
          <span class="info-label">Currency</span>
          <span class="info-value">{merchant.currency}</span>
        </div>
        <div class="info-item">
          <span class="info-label">Timezone</span>
          <span class="info-value">{merchant.timezone}</span>
        </div>
        <div class="info-item">
          <span class="info-label">Created</span>
          <span class="info-value">{formatDate(merchant.created_at)}</span>
        </div>
      </div>

      <div class="info-controls">
        <div class="control-item">
          <span class="control-label">Plan tier</span>
          <div class="plan-select">
            <Select
              items={planItems}
              value={[currentPlan]}
              onchange={handlePlanChange}
              placeholder="Select plan"
            />
          </div>
        </div>
        <div class="control-item">
          <span class="control-label">Status</span>
          <Toggle
            checked={isActive}
            text={isActive ? 'Active' : 'Inactive'}
            onclick={handleStatusToggle}
          />
        </div>
      </div>
    </div>

    {#if statItems.length > 0}
      <section class="stats-section">
        <h3 class="section-title">Statistics</h3>
        <StatsGrid items={statItems} />
      </section>
    {/if}
  </div>
{/if}

<style>
  .merchant-detail {
    display: flex;
    flex-direction: column;
    gap: var(--space-8);
  }

  .not-found {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-4);
    padding: var(--space-16);
  }

  .not-found h2 {
    font-size: var(--font-size-xl);
    color: var(--color-text-muted);
  }

  .page-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: var(--space-4);
  }

  .header-text {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .breadcrumb {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .breadcrumb a {
    color: var(--color-primary);
    text-decoration: none;
  }

  .breadcrumb a:hover {
    text-decoration: underline;
  }

  .sep {
    color: var(--color-border);
  }

  .page-header h1 {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    line-height: var(--line-height-tight);
  }

  .info-card {
    background: var(--color-surface);
    border-radius: var(--radius-lg);
    padding: var(--space-6);
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
    box-shadow: var(--shadow-card);
  }

  .info-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: var(--space-5);
  }

  .info-item {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .info-label {
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-medium);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .info-value {
    font-size: var(--font-size-base);
    color: var(--color-text);
  }

  .info-value.mono {
    font-family: var(--font-mono);
    font-size: var(--font-size-sm);
  }

  .info-controls {
    display: flex;
    align-items: flex-end;
    gap: var(--space-8);
    padding-top: var(--space-4);
    border-top: 1px solid var(--color-border);
  }

  .control-item {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .control-label {
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-medium);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .plan-select {
    --select-width: 180px;
    --select-min-width: 180px;
  }

  .stats-section {
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
