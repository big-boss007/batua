<script lang="ts">
  import { Pill, Table } from '@juspay/svelte-ui-components';

  import type {
    ReferralProgram,
    ReferralCode,
    ReferralAnalytics,
    ReferralConversion
  } from '$lib/client/modules/referrals';
  import {
    createProgram,
    createCode,
    fetchMerchantCodes,
    referralProgram,
    referralCodes
  } from '$lib/client/modules/referrals';
  import {
    ReferralProgramForm,
    CreateCodeForm,
    ReferralAnalyticsCard,
    ConversionsList
  } from '$lib/client/modules/referrals/ui';
  import { currentMerchantId } from '$lib/client/modules/admin';
  import { toastStore } from '$lib/client/modules/foundation';

  let { data } = $props();

  let program = $state<ReferralProgram | null>(data.program);
  let codes = $state<Array<ReferralCode>>([]);
  let analytics = $state<ReferralAnalytics | null>(data.analytics);
  let conversions = $state<Array<ReferralConversion>>(data.conversions);
  let activeTab = $state<'program' | 'codes' | 'analytics' | 'conversions'>('program');
  let merchantId = $state<string | null>(null);
  let codesLoading = $state(false);

  currentMerchantId.subscribe((id) => {
    const prevId = merchantId;
    merchantId = id;
    if (id !== null && id !== prevId) {
      loadCodes(id);
    }
  });

  async function loadCodes(mId: string) {
    codesLoading = true;
    const result = await fetchMerchantCodes(mId, 1, 50);
    if (result.tag === 'success') {
      codes = result.data;
    }
    codesLoading = false;
  }

  function setTab(tab: 'program' | 'codes' | 'analytics' | 'conversions') {
    activeTab = tab;
  }

  async function handleSaveProgram(programData: Omit<ReferralProgram, 'id'>) {
    const result = await createProgram(programData);
    if (result.tag === 'success') {
      program = result.data;
      referralProgram.set(result.data);
      toastStore.push({ message: 'Referral program saved', level: 'success' });
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }
  }

  async function handleCreateCode(params: {
    customer_id: string;
    code: string | null;
    is_vanity: boolean;
    is_creator: boolean;
    commission_rate: number | null;
  }) {
    const result = await createCode(params);
    if (result.tag === 'success') {
      codes = [result.data, ...codes];
      referralCodes.add(result.data);
      toastStore.push({ message: 'Referral code created', level: 'success' });
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }
  }

  let codesTableData = $derived(
    codes.map((c) => [
      c.code,
      c.customer_id,
      c.is_vanity ? 'Vanity' : '',
      c.is_creator ? 'Creator' : '',
      String(c.total_conversions),
      c.is_active ? 'Active' : 'Inactive'
    ])
  );
</script>

<div class="page">
  <div class="page-header">
    <h1 class="page-title">Referrals</h1>
  </div>

  <div class="tabs">
    <button
      class="tab"
      class:tab-active={activeTab === 'program'}
      onclick={() => setTab('program')}
    >
      Program
    </button>
    <button class="tab" class:tab-active={activeTab === 'codes'} onclick={() => setTab('codes')}>
      Codes
    </button>
    <button
      class="tab"
      class:tab-active={activeTab === 'analytics'}
      onclick={() => setTab('analytics')}
    >
      Analytics
    </button>
    <button
      class="tab"
      class:tab-active={activeTab === 'conversions'}
      onclick={() => setTab('conversions')}
    >
      Conversions
    </button>
  </div>

  {#if activeTab === 'program'}
    <div class="form-section">
      <ReferralProgramForm {program} onSave={handleSaveProgram} />
    </div>
  {:else if activeTab === 'codes'}
    <div class="codes-section">
      <div class="form-section">
        <h2 class="section-title">Create Code</h2>
        <CreateCodeForm onCreateCode={handleCreateCode} />
      </div>
      <div class="list-section">
        <h2 class="section-title">All Codes</h2>
        {#if merchantId === null}
          <p class="empty-text">Select a merchant to view codes</p>
        {:else if codesLoading}
          <p class="empty-text">Loading codes...</p>
        {:else}
          <Table
            tableHeaders={['Code', 'Customer ID', 'Vanity', 'Creator', 'Conversions', 'Status']}
            tableData={codesTableData}
            sortable={false}
          >
            {#snippet cell(value, rowIndex, colIndex)}
              {#if colIndex === 0}
                <code class="code-cell">{value}</code>
              {:else if colIndex === 2}
                {#if String(value)}
                  <Pill text={String(value)} classes="pill-info" />
                {/if}
              {:else if colIndex === 3}
                {#if String(value)}
                  <Pill text={String(value)} classes="pill-creator" />
                {/if}
              {:else if colIndex === 5}
                <Pill
                  text={String(value)}
                  classes={String(value) === 'Active' ? 'pill-success' : 'pill-muted'}
                />
              {:else}
                {value}
              {/if}
            {/snippet}
            {#snippet empty()}
              <p class="empty-text">No referral codes found</p>
            {/snippet}
          </Table>
        {/if}
      </div>
    </div>
  {:else if activeTab === 'analytics'}
    {#if analytics}
      <ReferralAnalyticsCard {analytics} />
    {:else}
      <p class="empty-text">No analytics data available</p>
    {/if}
  {:else if activeTab === 'conversions'}
    <ConversionsList {conversions} />
  {/if}
</div>

<style>
  .page {
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
    max-width: 960px;
    padding: var(--space-8);
  }

  .page-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .page-title {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
  }

  .tabs {
    display: flex;
    gap: var(--space-1);
    border-bottom: 1px solid var(--color-border);
  }

  .tab {
    padding: var(--space-2) var(--space-4);
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-text-muted);
    background: none;
    border: none;
    border-bottom: 2px solid transparent;
    transition:
      color var(--transition-fast),
      border-color var(--transition-fast);
    margin-bottom: -1px;
  }

  .tab:hover {
    color: var(--color-text);
  }

  .tab-active {
    color: var(--color-primary);
    border-bottom-color: var(--color-primary);
  }

  .form-section {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    max-width: 480px;
  }

  .codes-section {
    display: flex;
    flex-direction: column;
    gap: var(--space-8);
  }

  .list-section {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .section-title {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .code-cell {
    font-family: var(--font-mono);
    font-size: var(--font-size-xs);
    padding: var(--space-1) var(--space-2);
    background: var(--color-surface-2);
    border-radius: var(--radius-sm);
  }

  .empty-text {
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
    padding: var(--space-12);
    text-align: center;
  }

  :global(.pill-info) {
    --pill-color: var(--color-info);
    --pill-bg: color-mix(in srgb, var(--color-info) 12%, transparent);
  }

  :global(.pill-creator) {
    --pill-color: var(--color-primary);
    --pill-bg: color-mix(in srgb, var(--color-primary) 12%, transparent);
  }

  :global(.pill-success) {
    --pill-color: var(--color-success);
    --pill-bg: color-mix(in srgb, var(--color-success) 12%, transparent);
  }

  :global(.pill-muted) {
    --pill-color: var(--color-text-muted);
    --pill-bg: var(--color-surface-2);
  }
</style>
