<script lang="ts">
  import { Button, Input, Pill } from '@juspay/svelte-ui-components';

  import type { ReferralCode } from '$lib/client/modules/referrals';
  import { createCode, fetchMerchantCodes, resolveCustomerByPhone } from '$lib/client/modules/referrals';
  import type { Merchant } from '$lib/client/modules/admin';
  import { currentMerchant, currentMerchantId } from '$lib/client/modules/admin';
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

  function formatMobile(customerId: string): string {
    if (customerId.length >= 10) {
      const last10 = customerId.slice(-10);
      return `+91 ${last10.slice(0, 5)} ${last10.slice(5)}`;
    }
    return customerId;
  }
</script>

<svelte:head>
  <title>Influencers - Batua Admin</title>
</svelte:head>

<div class="page">
  {#if creatorCodes.length === 0 && !loading}
    <!-- EMPTY STATE -->
    <div class="page-header">
      <div><h1 class="page-title">Influencers</h1><p class="page-subtitle">Manage creator and influencer partnerships</p></div>
    </div>
    <div class="empty-card">
      <div class="empty-hero">
        <div class="empty-icon">🎤</div>
        <div class="empty-title">No influencers yet</div>
        <div class="empty-desc">Add creators and influencers with vanity codes. They'll earn {pointsName} {pointsIcon} for every successful referral through their code.</div>
        <Button text="+ Add Influencer" classes="btn-primary" onclick={openModal} />
      </div>
    </div>
  {:else}
    <!-- INFLUENCERS LIST -->
    <div class="page-header">
      <div><h1 class="page-title">Influencers</h1><p class="page-subtitle">Manage creator and influencer partnerships</p></div>
      <Button text="+ Add Influencer" classes="btn-primary-sm" onclick={openModal} />
    </div>

    <div class="metrics-row">
      <div class="metric-card"><span class="metric-val">{creatorCodes.length}</span><span class="metric-label">Active Influencers</span></div>
      <div class="metric-card"><span class="metric-val">{totalConversions}</span><span class="metric-label">Total Conversions</span></div>
    </div>

    <div class="table-wrapper">
      <table class="data-table">
        <thead><tr><th>Influencer</th><th>Code</th><th>Commission</th><th>Conversions</th><th>Status</th></tr></thead>
        <tbody>
          {#each creatorCodes as code (code.id)}
            <tr>
              <td><div class="influencer-mobile">{formatMobile(code.customer_id)}</div></td>
              <td><code class="code-mono">{code.code}</code></td>
              <td class="text-bold">{code.commission_rate ?? 0}%</td>
              <td class="text-bold">{code.total_conversions}</td>
              <td><Pill text={code.is_active ? 'Active' : 'Inactive'} classes={code.is_active ? 'pill-success' : 'pill-neutral'} /></td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}

  <!-- ADD INFLUENCER MODAL -->
  {#if showModal}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div class="modal-overlay" role="presentation" onclick={closeModal}>
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <div class="modal-panel" role="dialog" aria-label="Add influencer" tabindex="-1" onclick={(e) => e.stopPropagation()}>
        <div class="modal-header">
          <span class="modal-title">Add Influencer</span>
          <button class="modal-close" onclick={closeModal}>&times;</button>
        </div>
        <div class="modal-body">
          <div class="info-banner">
            Influencers get a vanity code and earn {pointsName} {pointsIcon} on every referred order. The customer must already exist in the system.
          </div>

          <div class="field-group">
            <span class="field-label">Mobile Number</span>
            <div class="input-with-prefix">
              <span class="input-prefix">+91</span>
              <Input value={mobileNumber} placeholder="98765 43210" onInput={(val) => { mobileNumber = val; }} />
            </div>
            <span class="field-hint">Must be an existing customer</span>
          </div>

          <div class="field-group">
            <span class="field-label">Vanity Code</span>
            <Input value={vanityCode} placeholder="e.g. PRIYA2024" onInput={(val) => { vanityCode = val; }} />
            <span class="field-hint">Uppercase letters and numbers. The code the influencer will share.</span>
          </div>

          <div class="field-group">
            <span class="field-label">Commission Rate</span>
            <div class="input-with-suffix">
              <Input value={commissionRate} dataType="number" onInput={(val) => { commissionRate = val; }} />
              <span class="input-suffix">%</span>
            </div>
            <span class="field-hint">Percentage of order value earned as {pointsName} {pointsIcon} per referral</span>
          </div>
        </div>
        <div class="modal-footer">
          <Button text="Cancel" classes="btn-ghost" onclick={closeModal} />
          <Button text={saving ? 'Adding...' : 'Add Influencer'} classes="btn-primary" disabled={!mobileNumber || !vanityCode || saving} onclick={handleAddInfluencer} />
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .page { display: flex; flex-direction: column; gap: var(--space-5); max-width: 960px; }

  .page-header { display: flex; justify-content: space-between; align-items: flex-start; }
  .page-title { font-size: var(--font-size-2xl); font-weight: var(--font-weight-bold); color: var(--color-text); }
  .page-subtitle { font-size: var(--font-size-sm); color: var(--color-text-muted); margin-top: var(--space-1); }

  .empty-card { border: 1px solid var(--color-border); border-radius: var(--radius-lg); background: var(--color-surface); }
  .empty-hero { text-align: center; padding: var(--space-12) var(--space-8); }
  .empty-icon { font-size: 44px; margin-bottom: var(--space-4); }
  .empty-title { font-size: var(--font-size-lg); font-weight: var(--font-weight-bold); color: var(--color-text); margin-bottom: var(--space-2); }
  .empty-desc { font-size: var(--font-size-sm); color: var(--color-text-muted); max-width: 380px; margin: 0 auto var(--space-6); line-height: 1.6; }

  .metrics-row { display: grid; grid-template-columns: repeat(2, 1fr); gap: var(--space-4); max-width: 400px; }
  .metric-card { padding: var(--space-4); background: var(--color-surface); border: 1px solid var(--color-border); border-radius: var(--radius-lg); }
  .metric-val { font-size: var(--font-size-xl); font-weight: var(--font-weight-bold); color: var(--color-text); display: block; }
  .metric-label { font-size: var(--font-size-xs); color: var(--color-text-muted); font-weight: var(--font-weight-medium); margin-top: 2px; display: block; }

  .table-wrapper { border: 1px solid var(--color-border); border-radius: var(--radius-lg); overflow: hidden; }
  .data-table { width: 100%; border-collapse: collapse; font-size: var(--font-size-sm); }
  .data-table th { text-align: left; padding: var(--space-3) var(--space-4); font-size: var(--font-size-xs); font-weight: var(--font-weight-semibold); color: var(--color-text-muted); text-transform: uppercase; letter-spacing: 0.04em; background: var(--color-surface); border-bottom: 1px solid var(--color-border); }
  .data-table td { padding: var(--space-3) var(--space-4); border-bottom: 1px solid var(--color-border); }
  .data-table tr:last-child td { border-bottom: none; }

  .code-mono { font-family: var(--font-mono); font-size: var(--font-size-xs); padding: 2px 8px; background: var(--color-surface-2); border-radius: var(--radius-sm); font-weight: var(--font-weight-semibold); }

  .influencer-mobile { font-size: var(--font-size-sm); color: var(--color-text); }
  .text-bold { font-weight: var(--font-weight-semibold); }

  /* Modal */
  .modal-overlay { position: fixed; inset: 0; background: rgba(0, 0, 0, 0.5); display: flex; align-items: flex-start; justify-content: center; padding-top: var(--space-12); z-index: var(--z-modal); overflow-y: auto; }
  .modal-panel { background: var(--color-bg); border-radius: var(--radius-lg); box-shadow: 0 20px 60px rgba(0, 0, 0, 0.12); width: 440px; max-width: 100%; overflow: hidden; }
  .modal-header { display: flex; justify-content: space-between; align-items: center; padding: var(--space-4) var(--space-5); border-bottom: 1px solid var(--color-border); }
  .modal-title { font-size: var(--font-size-lg); font-weight: var(--font-weight-semibold); }
  .modal-close { background: none; border: none; font-size: 20px; color: var(--color-text-muted); cursor: pointer; padding: 2px 6px; border-radius: var(--radius-sm); }
  .modal-body { padding: var(--space-5); display: flex; flex-direction: column; gap: var(--space-4); }
  .modal-footer { display: flex; justify-content: flex-end; gap: var(--space-3); padding: var(--space-3) var(--space-5); border-top: 1px solid var(--color-border); }

  .info-banner { padding: var(--space-3) var(--space-4); background: #eff6ff; border: 1px solid #bfdbfe; border-radius: var(--radius-md); font-size: var(--font-size-xs); color: #1e40af; line-height: 1.5; }

  .field-group { display: flex; flex-direction: column; gap: var(--space-1); }
  .field-label { font-size: var(--font-size-xs); font-weight: var(--font-weight-semibold); color: var(--color-text-muted); }
  .field-hint { font-size: 11px; color: var(--color-text-muted); }

  .input-with-suffix { display: inline-flex; align-items: stretch; border: 1px solid var(--color-border); border-radius: var(--radius-md); overflow: hidden; width: fit-content; }
  .input-with-suffix :global(input) { border: none !important; border-radius: 0 !important; width: 80px !important; }
  .input-suffix { padding: var(--space-2) var(--space-3); background: var(--color-surface-2); border-left: 1px solid var(--color-border); font-size: var(--font-size-sm); color: var(--color-text-muted); font-weight: var(--font-weight-semibold); white-space: nowrap; display: flex; align-items: center; }

  .input-with-prefix { display: inline-flex; align-items: stretch; border: 1px solid var(--color-border); border-radius: var(--radius-md); overflow: hidden; width: fit-content; }
  .input-with-prefix :global(input) { border: none !important; border-radius: 0 !important; width: 160px !important; }
  .input-prefix { padding: var(--space-2) var(--space-3); background: var(--color-surface-2); border-right: 1px solid var(--color-border); font-size: var(--font-size-sm); color: var(--color-text-muted); font-weight: var(--font-weight-medium); white-space: nowrap; display: flex; align-items: center; }

  :global(.btn-primary-sm) {
    --button-color: var(--color-primary);
    --button-text-color: #ffffff;
    --button-border-radius: var(--radius-md);
    --button-padding: var(--space-1) var(--space-4);
    --button-font-size: var(--font-size-sm);
    --button-font-weight: var(--font-weight-semibold);
    --button-hover-color: var(--color-primary-hover);
  }
</style>
