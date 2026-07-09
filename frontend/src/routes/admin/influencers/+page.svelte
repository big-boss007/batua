<script lang="ts">
  import { Button, Input, Pill, Table } from '@juspay/svelte-ui-components';

  import type { ReferralCode } from '$lib/client/modules/referrals';
  import {
    createCode,
    fetchMerchantCodes,
    resolveCustomerByPhone
  } from '$lib/client/modules/referrals';
  import type { Merchant } from '$lib/client/modules/admin';
  import { currentMerchant, currentMerchantId } from '$lib/client/modules/admin';
  import { MetricCard } from '$lib/client/modules/admin/ui';
  import { toastStore } from '$lib/client/modules/foundation';

  let codes = $state<Array<ReferralCode>>([]);
  let merchantId = $state<string | null>(null);
  let merchant = $state<Merchant | null>(null);
  let loading = $state(true);
  let showModal = $state(false);
  let saving = $state(false);

  let mobileNumber = $state('');
  let vanityCode = $state('');
  let commissionRate = $state('5');

  let pointsName = $derived(merchant?.points_name ?? 'Points');
  let pointsIcon = $derived(merchant?.points_icon ?? 'pts');

  let creatorCodes = $derived(codes.filter((c) => c.is_creator));

  let totalConversions = $derived(creatorCodes.reduce((sum, c) => sum + c.total_conversions, 0));

  const TABLE_HEADERS = ['Influencer', 'Code', 'Commission', 'Conversions', 'Status'];
  let tableData = $derived(
    creatorCodes.map((code) => [
      formatInfluencer(code.customer_name, code.customer_phone),
      code.code,
      `${code.commission_rate ?? 0}%`,
      code.total_conversions,
      code.is_active ? 'Active' : 'Inactive'
    ])
  );

  currentMerchant.subscribe((m) => {
    merchant = m;
  });

  currentMerchantId.subscribe((id) => {
    const prevId = merchantId;
    merchantId = id;
    if (id !== null && id !== prevId) {
      loadData(id);
    }
  });

  async function loadData(mId: string) {
    loading = true;
    const result = await fetchMerchantCodes(mId, 1, 100);
    if (result.tag === 'success') codes = result.data;
    loading = false;
  }

  function openModal() {
    mobileNumber = '';
    vanityCode = '';
    commissionRate = '5';
    showModal = true;
  }

  function closeModal() {
    showModal = false;
  }

  async function handleAddInfluencer() {
    if (!mobileNumber || !vanityCode || !merchantId || saving) return;
    saving = true;

    const phone = mobileNumber.replace(/\s+/g, '');
    const resolveResult = await resolveCustomerByPhone(phone);
    if (resolveResult.tag !== 'success') {
      toastStore.push({ message: `Customer not found: ${resolveResult.message}`, level: 'error' });
      saving = false;
      return;
    }

    const result = await createCode({
      merchant_id: merchantId,
      customer_id: resolveResult.data.customer_id,
      code: vanityCode.toUpperCase(),
      is_vanity: true,
      is_creator: true,
      commission_rate: Number(commissionRate)
    });
    if (result.tag === 'success') {
      codes = [result.data, ...codes];
      showModal = false;
      toastStore.push({ message: 'Influencer added', level: 'success' });
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }
    saving = false;
  }

  function formatPhone(phone: string): string {
    const digits = phone.replace(/\D/g, '');
    if (digits.length === 12 && digits.startsWith('91')) {
      const local = digits.slice(2);
      return `+91 ${local.slice(0, 5)} ${local.slice(5)}`;
    }
    return phone;
  }

  function formatInfluencer(name: string | null, phone: string | null): string {
    if (name && phone) return `${name} (${formatPhone(phone)})`;
    if (name) return name;
    if (phone) return formatPhone(phone);
    return 'Unknown';
  }
</script>

<svelte:head>
  <title>Influencers - Batua Admin</title>
</svelte:head>

<div class="page">
  {#if creatorCodes.length === 0 && !loading}
    <!-- EMPTY STATE -->
    <div class="page-header">
      <div>
        <h1 class="page-title">Influencers</h1>
        <p class="page-subtitle">Manage creator and influencer partnerships</p>
      </div>
    </div>
    <div class="empty-card">
      <div class="empty-hero">
        <div class="empty-icon">🎤</div>
        <div class="empty-title">No influencers yet</div>
        <div class="empty-desc">
          Add creators and influencers with vanity codes. They'll earn {pointsName}
          {pointsIcon} for every successful referral through their code.
        </div>
        <Button text="+ Add influencer" classes="btn-primary" onclick={openModal} />
      </div>
    </div>
  {:else}
    <!-- INFLUENCERS LIST -->
    <div class="page-header">
      <div>
        <h1 class="page-title">Influencers</h1>
        <p class="page-subtitle">Manage creator and influencer partnerships</p>
      </div>
      <Button text="+ Add influencer" classes="btn-primary" onclick={openModal} />
    </div>

    <div class="metrics-row">
      <MetricCard label="Active influencers" value={creatorCodes.length} size="compact" />
      <MetricCard label="Total conversions" value={totalConversions} size="compact" />
    </div>

    <Table
      tableHeaders={TABLE_HEADERS}
      {tableData}
      sortable={false}
      --table-row-hover-background="var(--color-surface-2)"
      --table-content-font-size="var(--font-size-sm)"
    >
      {#snippet cell(value, _rowIndex, colIndex)}
        {#if colIndex === 1}
          <code class="code-mono">{value}</code>
        {:else if colIndex === 2 || colIndex === 3}
          <span class="text-bold">{value}</span>
        {:else if colIndex === 4}
          <Pill
            text={String(value)}
            classes={value === 'Active' ? 'pill-success' : 'pill-neutral'}
          />
        {:else}
          {value}
        {/if}
      {/snippet}
    </Table>
  {/if}

  <!-- ADD INFLUENCER MODAL -->
  {#if showModal}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div class="modal-overlay" role="presentation" onclick={closeModal}>
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <div
        class="modal-panel"
        role="dialog"
        aria-label="Add influencer"
        tabindex="-1"
        onclick={(e) => e.stopPropagation()}
      >
        <div class="modal-header">
          <span class="modal-title">Add influencer</span>
          <button class="modal-close" onclick={closeModal}>&times;</button>
        </div>
        <div class="modal-body">
          <div class="info-banner">
            Influencers get a vanity code and earn {pointsName}
            {pointsIcon} on every referred order. The customer must already exist in the system.
          </div>

          <div class="field-group">
            <span class="field-label">Mobile number</span>
            <div class="input-with-prefix">
              <span class="input-prefix">+91</span>
              <Input
                value={mobileNumber}
                placeholder="98765 43210"
                onInput={(val) => {
                  mobileNumber = val;
                }}
              />
            </div>
            <span class="field-hint">Must be an existing customer</span>
          </div>

          <div class="field-group">
            <span class="field-label">Vanity code</span>
            <Input
              value={vanityCode}
              placeholder="e.g. PRIYA2024"
              onInput={(val) => {
                vanityCode = val;
              }}
            />
            <span class="field-hint"
              >Uppercase letters and numbers. The code the influencer will share.</span
            >
          </div>

          <div class="field-group">
            <span class="field-label">Commission rate</span>
            <div class="input-with-suffix">
              <Input
                value={commissionRate}
                dataType="number"
                onInput={(val) => {
                  commissionRate = val;
                }}
              />
              <span class="input-suffix">%</span>
            </div>
            <span class="field-hint"
              >Percentage of order value earned as {pointsName} {pointsIcon} per referral</span
            >
          </div>
        </div>
        <div class="modal-footer">
          <Button text="Cancel" classes="btn-ghost" onclick={closeModal} />
          <Button
            text={saving ? 'Adding...' : 'Add influencer'}
            classes="btn-primary"
            disabled={!mobileNumber || !vanityCode || saving}
            onclick={handleAddInfluencer}
          />
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .page {
    display: flex;
    flex-direction: column;
    gap: var(--space-5);
    max-width: 960px;
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
  }
  .page-title {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
  }
  .page-subtitle {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    margin-top: var(--space-1);
  }

  .empty-card {
    border-radius: var(--radius-lg);
    background: var(--color-surface);
    box-shadow: var(--shadow-card);
  }
  .empty-hero {
    text-align: center;
    padding: var(--space-12) var(--space-8);
  }
  .empty-icon {
    font-size: 44px;
    margin-bottom: var(--space-4);
  }
  .empty-title {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    margin-bottom: var(--space-2);
  }
  .empty-desc {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    max-width: 380px;
    margin: 0 auto var(--space-6);
    line-height: 1.6;
  }

  .metrics-row {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: var(--space-4);
    max-width: 400px;
  }

  .code-mono {
    font-family: var(--font-mono);
    font-size: var(--font-size-xs);
    padding: 2px 8px;
    background: var(--color-surface-2);
    border-radius: var(--radius-sm);
    font-weight: var(--font-weight-semibold);
  }

  .text-bold {
    font-weight: var(--font-weight-semibold);
  }

  /* Modal */
  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: flex-start;
    justify-content: center;
    padding-top: var(--space-12);
    z-index: var(--z-modal);
    overflow-y: auto;
  }
  .modal-panel {
    background: var(--color-bg);
    border-radius: var(--radius-lg);
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.12);
    width: 440px;
    max-width: 100%;
    overflow: hidden;
  }
  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--space-4) var(--space-5);
    border-bottom: 1px solid var(--color-border);
  }
  .modal-title {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
  }
  .modal-close {
    background: none;
    border: none;
    font-size: var(--font-size-xl);
    color: var(--color-text-muted);
    cursor: pointer;
    padding: 2px 6px;
    border-radius: var(--radius-sm);
  }
  .modal-body {
    padding: var(--space-5);
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }
  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: var(--space-3);
    padding: var(--space-3) var(--space-5);
    border-top: 1px solid var(--color-border);
  }

  .info-banner {
    padding: var(--space-3) var(--space-4);
    background: var(--p-100);
    border: 1px solid var(--p-100);
    border-radius: var(--radius-md);
    font-size: var(--font-size-xs);
    color: var(--p-700);
    line-height: 1.5;
  }

  .field-group {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }
  .field-label {
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-muted);
  }
  .field-hint {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .input-with-suffix {
    display: inline-flex;
    align-items: stretch;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    overflow: hidden;
    width: fit-content;
  }
  .input-with-suffix :global(input) {
    border: none !important;
    border-radius: 0 !important;
    width: 80px !important;
  }
  .input-suffix {
    padding: var(--space-2) var(--space-3);
    background: var(--color-surface-2);
    border-left: 1px solid var(--color-border);
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    font-weight: var(--font-weight-semibold);
    white-space: nowrap;
    display: flex;
    align-items: center;
  }

  .input-with-prefix {
    display: inline-flex;
    align-items: stretch;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    overflow: hidden;
    width: fit-content;
  }
  .input-with-prefix :global(input) {
    border: none !important;
    border-radius: 0 !important;
    width: 160px !important;
  }
  .input-prefix {
    padding: var(--space-2) var(--space-3);
    background: var(--color-surface-2);
    border-right: 1px solid var(--color-border);
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    font-weight: var(--font-weight-medium);
    white-space: nowrap;
    display: flex;
    align-items: center;
  }
</style>
