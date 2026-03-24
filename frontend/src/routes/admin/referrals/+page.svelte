<script lang="ts">
  import { Button, Input, Toggle, Pill } from '@juspay/svelte-ui-components';

  import type {
    ReferralProgram,
    ReferralCode,
    ReferralAnalytics,
    ReferralConversion
  } from '$lib/client/modules/referrals';
  import {
    createProgram,
    updateProgram,
    fetchProgram,
    fetchAnalytics,
    fetchConversions,
    fetchMerchantCodes,
    referralProgram
  } from '$lib/client/modules/referrals';
  import type { Merchant } from '$lib/client/modules/admin';
  import { currentMerchant, currentMerchantId } from '$lib/client/modules/admin';
  import { toastStore } from '$lib/client/modules/foundation';

  type ViewMode = 'empty' | 'setup' | 'dashboard';

  let program = $state<ReferralProgram | null>(null);
  let codes = $state<Array<ReferralCode>>([]);
  let analytics = $state<ReferralAnalytics | null>(null);
  let conversions = $state<Array<ReferralConversion>>([]);
  let merchantId = $state<string | null>(null);
  let merchant = $state<Merchant | null>(null);
  let loading = $state(true);
  let showSetup = $state(false);
  let saving = $state(false);

  let referrerReward = $state('100');
  let refereeReward = $state('50');
  let maxReferrals = $state('10');
  let hasMaxLimit = $state(true);
  let isActive = $state(true);

  let pointsName = $derived(merchant?.points_name ?? 'Points');
  let pointsIcon = $derived(merchant?.points_icon ?? 'pts');

  let viewMode = $derived.by((): ViewMode => {
    if (loading) return 'empty';
    if (showSetup) return 'setup';
    if (program !== null) return 'dashboard';
    return 'empty';
  });

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
    const [programResult, analyticsResult, conversionsResult, codesResult] = await Promise.all([
      fetchProgram(mId),
      fetchAnalytics(mId),
      fetchConversions(mId),
      fetchMerchantCodes(mId, 1, 50)
    ]);

    if (programResult.tag === 'success') program = programResult.data;
    else program = null;
    if (analyticsResult.tag === 'success') analytics = analyticsResult.data;
    if (conversionsResult.tag === 'success') conversions = conversionsResult.data;
    if (codesResult.tag === 'success') codes = codesResult.data;
    loading = false;
  }

  function startSetup() {
    if (program !== null) {
      referrerReward = String(program.referrer_reward_amount);
      refereeReward = String(program.referee_reward_amount);
      hasMaxLimit = program.max_referrals_per_customer !== null;
      maxReferrals = String(program.max_referrals_per_customer ?? 10);
      isActive = program.is_active;
    }
    showSetup = true;
  }

  function cancelSetup() {
    showSetup = false;
  }

  async function handleSaveProgram() {
    if (merchantId === null || saving) return;
    saving = true;

    const programData = {
      referrer_reward_amount: Number(referrerReward),
      referee_reward_amount: Number(refereeReward),
      max_referrals_per_customer: hasMaxLimit ? Number(maxReferrals) : null,
      is_active: isActive
    };

    const result = program !== null
      ? await updateProgram(merchantId, programData)
      : await createProgram(merchantId, programData);

    if (result.tag === 'success') {
      program = result.data;
      referralProgram.set(result.data);
      showSetup = false;
      toastStore.push({ message: 'Referral program saved', level: 'success' });
      if (merchantId !== null) loadData(merchantId);
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }
    saving = false;
  }

  async function handleToggleActive() {
    if (merchantId === null || program === null) return;
    const newActive = !program.is_active;
    const result = await updateProgram(merchantId, { is_active: newActive });
    if (result.tag === 'success') {
      program = result.data;
      referralProgram.set(result.data);
      toastStore.push({ message: newActive ? 'Referral program resumed' : 'Referral program paused', level: 'success' });
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }
  }

  function formatMobile(customerId: string): string {
    if (customerId.length >= 10) {
      const last10 = customerId.slice(-10);
      return `+91 ${last10.slice(0, 5)} ${last10.slice(5)}`;
    }
    return customerId;
  }

  function formatDate(iso: string): string {
    return new Date(iso).toLocaleDateString('en-IN', {
      day: 'numeric',
      month: 'short',
      hour: '2-digit',
      minute: '2-digit'
    });
  }
</script>

<svelte:head>
  <title>Referral Program - Batua Admin</title>
</svelte:head>

<div class="page">
  {#if viewMode === 'empty' && !loading}
    <!-- EMPTY STATE -->
    <div class="page-header">
      <div><h1 class="page-title">Referral Program</h1><p class="page-subtitle">Grow your customer base through word-of-mouth</p></div>
    </div>
    <div class="empty-card">
      <div class="setup-hero">
        <div class="setup-icon">🔗</div>
        <div class="setup-title">Set up your referral program</div>
        <div class="setup-desc">Reward customers for bringing in their friends. Configure rewards, share codes, and track conversions.</div>
        <div class="setup-steps">
          <div class="setup-step"><div class="setup-step-num">1</div><div class="setup-step-label">Set Rewards</div><div class="setup-step-hint">Referrer & referee amounts</div></div>
          <div class="setup-step"><div class="setup-step-num">2</div><div class="setup-step-label">Configure</div><div class="setup-step-hint">Limits & activation</div></div>
          <div class="setup-step"><div class="setup-step-num">3</div><div class="setup-step-label">Share</div><div class="setup-step-hint">Customers share from storefront</div></div>
        </div>
        <Button text="Set Up Referral Program" classes="btn-primary" onclick={startSetup} />
      </div>
    </div>

  {:else if viewMode === 'setup'}
    <!-- SETUP / EDIT -->
    <div class="page-header">
      <div><h1 class="page-title">Referral Program</h1><p class="page-subtitle">{program !== null ? 'Edit your referral program settings' : 'Configure your referral program'}</p></div>
    </div>

    {#if program !== null}
      <div class="reconfigure-banner">
        <strong>Reconfigure mode:</strong> Your current settings are pre-filled. Changes take effect immediately on save.
      </div>
    {/if}

    <div class="config-layout">
      <div class="config-card">
        <div class="config-card-header">
          <span class="config-card-title">Program Settings</span>
          {#if program !== null}
            <Pill text={program.is_active ? 'Active' : 'Inactive'} classes={program.is_active ? 'pill-success' : 'pill-neutral'} />
          {/if}
        </div>
        <div class="config-card-body">
          <div class="field-row">
            <div class="field-group">
              <span class="field-label">Referrer Reward</span>
              <div class="input-with-suffix">
                <Input value={referrerReward} dataType="number" onInput={(val) => { referrerReward = val; }} />
                <span class="input-suffix">{pointsIcon} {pointsName}</span>
              </div>
              <span class="field-hint">Earned per successful referral</span>
            </div>
            <div class="field-group">
              <span class="field-label">Referee Reward</span>
              <div class="input-with-suffix">
                <Input value={refereeReward} dataType="number" onInput={(val) => { refereeReward = val; }} />
                <span class="input-suffix">{pointsIcon} {pointsName}</span>
              </div>
              <span class="field-hint">Earned by the new customer</span>
            </div>
          </div>

          <div class="config-divider"></div>

          <Toggle text="Limit referrals per customer" checked={hasMaxLimit} onclick={(checked) => { hasMaxLimit = checked; }} />
          {#if hasMaxLimit}
            <div class="field-group" style="max-width: 200px;">
              <span class="field-label">Max Referrals</span>
              <Input value={maxReferrals} dataType="number" onInput={(val) => { maxReferrals = val; }} />
              <span class="field-hint">0 = unlimited</span>
            </div>
          {/if}

          <div class="config-divider"></div>

          <Toggle text="Program active" checked={isActive} onclick={(checked) => { isActive = checked; }} />

          <div class="config-actions">
            <Button text={saving ? 'Saving...' : program !== null ? 'Save Changes' : 'Activate Program'} classes="btn-primary" disabled={saving} onclick={handleSaveProgram} />
            <Button text="Cancel" classes="btn-ghost" onclick={cancelSetup} />
          </div>
        </div>
      </div>

      <div class="config-preview">
        <span class="preview-label">How it works</span>
        <div class="preview-flow">
          <div class="preview-box"><div class="preview-box-label">Referrer</div><div class="preview-box-val">+{referrerReward}</div></div>
          <span class="preview-arrow">→</span>
          <div class="preview-box"><div class="preview-box-label">Code</div><div class="preview-box-val">🔗</div></div>
          <span class="preview-arrow">→</span>
          <div class="preview-box"><div class="preview-box-label">Referee</div><div class="preview-box-val">+{refereeReward}</div></div>
        </div>
        <div class="preview-note">Both earn {pointsName} when the referee places their first order</div>
      </div>
    </div>

  {:else if viewMode === 'dashboard'}
    <!-- ACTIVE DASHBOARD -->
    <div class="page-header">
      <div><h1 class="page-title">Referral Program</h1><p class="page-subtitle">Manage your referral program</p></div>
      <Button text="Edit Program" classes="btn-secondary-sm" onclick={startSetup} />
    </div>

    <div class="status-bar" class:inactive={!program?.is_active}>
      <span class="status-text">
        <span class="status-dot"></span>
        {program?.is_active ? 'Active' : 'Paused'} — Referrer: {program?.referrer_reward_amount} {pointsIcon} · Referee: {program?.referee_reward_amount} {pointsIcon}
        {#if program?.max_referrals_per_customer !== null}
          · Max {program?.max_referrals_per_customer}/customer
        {/if}
      </span>
      <Button text={program?.is_active ? 'Pause' : 'Resume'} classes={program?.is_active ? 'btn-danger-sm' : 'btn-success-sm'} onclick={handleToggleActive} />
    </div>

    {#if analytics !== null}
      <div class="metrics-row">
        <div class="metric-card"><span class="metric-val">{analytics.total_codes}</span><span class="metric-label">Total Codes</span></div>
        <div class="metric-card"><span class="metric-val">{analytics.total_referrals}</span><span class="metric-label">Referrals</span></div>
        <div class="metric-card"><span class="metric-val">{analytics.total_conversions}</span><span class="metric-label">Conversions</span></div>
        <div class="metric-card"><span class="metric-val">{analytics.conversion_rate > 0 ? `${analytics.conversion_rate.toFixed(1)}%` : '0%'}</span><span class="metric-label">Conversion Rate</span></div>
      </div>
    {/if}

    <div class="section-header">
      <span class="section-title">All Referral Codes</span>
      <span class="section-count">{codes.length} codes</span>
    </div>
    {#if codes.length > 0}
      <div class="table-wrapper">
        <table class="data-table">
          <thead><tr><th>Code</th><th>Mobile</th><th>Type</th><th>Conversions</th><th>Status</th></tr></thead>
          <tbody>
            {#each codes as code (code.id)}
              <tr>
                <td><code class="code-mono">{code.code}</code></td>
                <td class="text-muted">{formatMobile(code.customer_id)}</td>
                <td>
                  {#if code.is_creator}
                    <Pill text="Creator" classes="pill-info" />
                  {:else}
                    <Pill text="Standard" classes="pill-neutral" />
                  {/if}
                </td>
                <td class="text-bold">{code.total_conversions}</td>
                <td><Pill text={code.is_active ? 'Active' : 'Inactive'} classes={code.is_active ? 'pill-success' : 'pill-neutral'} /></td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {:else}
      <div class="empty-section">No referral codes yet. Codes are auto-generated when customers share from the storefront.</div>
    {/if}

    {#if conversions.length > 0}
      <div class="table-divider"></div>
      <div class="section-header"><span class="section-title">Recent Conversions</span></div>
      <div class="table-wrapper">
        <table class="data-table">
          <thead><tr><th>Referrer</th><th>Referee</th><th>Order</th><th>Status</th><th>Date</th></tr></thead>
          <tbody>
            {#each conversions.slice(0, 10) as conv (conv.id)}
              <tr class:suspicious-row={conv.is_suspicious}>
                <td>{formatMobile(conv.referrer_id)}</td>
                <td>{formatMobile(conv.referee_id)}</td>
                <td>{conv.order_id ?? '—'}</td>
                <td>
                  {#if conv.fraud_signals.length > 0}
                    {#each conv.fraud_signals as signal}
                      <Pill text={signal} classes="pill-warning" />
                    {/each}
                  {:else}
                    <Pill text="Clean" classes="pill-success" />
                  {/if}
                </td>
                <td class="text-muted nowrap">{formatDate(conv.created_at)}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  {/if}
</div>

<style>
  .page { display: flex; flex-direction: column; gap: var(--space-5); max-width: 960px; }

  .page-header { display: flex; justify-content: space-between; align-items: flex-start; }
  .page-title { font-size: var(--font-size-2xl); font-weight: var(--font-weight-bold); color: var(--color-text); }
  .page-subtitle { font-size: var(--font-size-sm); color: var(--color-text-muted); margin-top: var(--space-1); }

  .empty-card { border: 1px solid var(--color-border); border-radius: var(--radius-lg); background: var(--color-surface); }

  .setup-hero { text-align: center; padding: var(--space-12) var(--space-8); }
  .setup-icon { font-size: 48px; margin-bottom: var(--space-4); }
  .setup-title { font-size: var(--font-size-xl); font-weight: var(--font-weight-bold); color: var(--color-text); margin-bottom: var(--space-2); }
  .setup-desc { font-size: var(--font-size-sm); color: var(--color-text-muted); max-width: 400px; margin: 0 auto var(--space-6); line-height: 1.6; }
  .setup-steps { display: flex; gap: var(--space-4); justify-content: center; margin-bottom: var(--space-6); max-width: 480px; margin-left: auto; margin-right: auto; }
  .setup-step { flex: 1; text-align: center; padding: var(--space-4) var(--space-3); background: var(--color-surface-2); border: 1px solid var(--color-border); border-radius: var(--radius-lg); }
  .setup-step-num { width: 28px; height: 28px; border-radius: 50%; background: var(--color-primary); color: white; display: flex; align-items: center; justify-content: center; font-size: 12px; font-weight: var(--font-weight-bold); margin: 0 auto var(--space-2); }
  .setup-step-label { font-size: var(--font-size-xs); font-weight: var(--font-weight-semibold); color: var(--color-text); }
  .setup-step-hint { font-size: 10px; color: var(--color-text-muted); margin-top: 2px; }

  .reconfigure-banner { padding: var(--space-3) var(--space-4); background: #fffbeb; border: 1px solid #fde68a; border-radius: var(--radius-md); font-size: var(--font-size-sm); color: #92400e; }

  .config-layout { display: grid; grid-template-columns: 1fr 240px; gap: var(--space-5); }
  .config-card { border: 1px solid var(--color-border); border-radius: var(--radius-lg); overflow: hidden; }
  .config-card-header { padding: var(--space-4) var(--space-5); border-bottom: 1px solid var(--color-border); display: flex; justify-content: space-between; align-items: center; }
  .config-card-title { font-size: var(--font-size-md); font-weight: var(--font-weight-semibold); color: var(--color-text); }
  .config-card-body { padding: var(--space-5); display: flex; flex-direction: column; gap: var(--space-4); }

  .field-row { display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-4); }
  .field-group { display: flex; flex-direction: column; gap: var(--space-1); }
  .field-label { font-size: var(--font-size-xs); font-weight: var(--font-weight-semibold); color: var(--color-text-muted); }
  .field-hint { font-size: 11px; color: var(--color-text-muted); }

  .input-with-suffix { display: inline-flex; align-items: stretch; border: 1px solid var(--color-border); border-radius: var(--radius-md); overflow: hidden; width: fit-content; }
  .input-with-suffix :global(input) { border: none !important; border-radius: 0 !important; width: 80px !important; }
  .input-suffix { padding: var(--space-2) var(--space-3); background: var(--color-surface-2); border-left: 1px solid var(--color-border); font-size: var(--font-size-xs); color: var(--color-text-muted); font-weight: var(--font-weight-medium); white-space: nowrap; display: flex; align-items: center; }

  .config-divider { height: 1px; background: var(--color-border); }

  .config-actions { display: flex; gap: var(--space-3); padding-top: var(--space-2); }

  .config-preview { background: linear-gradient(165deg, #eff6ff, #eef2ff); border: 1px solid #c7d2fe; border-radius: var(--radius-lg); padding: var(--space-5); display: flex; flex-direction: column; align-items: center; justify-content: center; gap: var(--space-3); text-align: center; height: fit-content; }
  .preview-label { font-size: 10px; font-weight: var(--font-weight-bold); text-transform: uppercase; letter-spacing: 0.08em; color: var(--color-primary); }
  .preview-flow { display: flex; align-items: center; gap: var(--space-2); }
  .preview-box { padding: var(--space-2) var(--space-3); background: white; border: 1px solid var(--color-border); border-radius: var(--radius-md); text-align: center; }
  .preview-box-label { font-size: 8px; color: var(--color-text-muted); text-transform: uppercase; }
  .preview-box-val { font-size: var(--font-size-md); font-weight: var(--font-weight-bold); color: var(--color-text); }
  .preview-arrow { color: var(--color-text-muted); font-size: var(--font-size-md); }
  .preview-note { font-size: 10px; color: var(--color-text-muted); }

  .status-bar { display: flex; align-items: center; justify-content: space-between; gap: var(--space-3); padding: var(--space-3) var(--space-4); background: #f0fdf4; border: 1px solid #bbf7d0; border-radius: var(--radius-md); }
  .status-bar.inactive { background: #fef2f2; border-color: #fecaca; }
  .status-text { font-size: var(--font-size-sm); font-weight: var(--font-weight-medium); color: #065f46; display: flex; align-items: center; gap: var(--space-2); flex: 1; }
  .status-bar.inactive .status-text { color: #991b1b; }
  .status-dot { width: 8px; height: 8px; border-radius: 50%; background: #22c55e; flex-shrink: 0; }
  .status-bar.inactive .status-dot { background: #ef4444; }
  .status-bar :global(button) { flex-shrink: 0; cursor: pointer; position: relative; z-index: 1; }

  .metrics-row { display: grid; grid-template-columns: repeat(4, 1fr); gap: var(--space-4); }
  .metric-card { padding: var(--space-4); background: var(--color-surface); border: 1px solid var(--color-border); border-radius: var(--radius-lg); }
  .metric-val { font-size: var(--font-size-xl); font-weight: var(--font-weight-bold); color: var(--color-text); display: block; }
  .metric-label { font-size: var(--font-size-xs); color: var(--color-text-muted); font-weight: var(--font-weight-medium); margin-top: 2px; display: block; }

  .section-header { display: flex; align-items: baseline; gap: var(--space-2); }
  .section-title { font-size: var(--font-size-md); font-weight: var(--font-weight-semibold); }
  .section-count { font-size: var(--font-size-xs); color: var(--color-text-muted); }

  .table-wrapper { border: 1px solid var(--color-border); border-radius: var(--radius-lg); overflow: hidden; }
  .data-table { width: 100%; border-collapse: collapse; font-size: var(--font-size-sm); }
  .data-table th { text-align: left; padding: var(--space-3) var(--space-4); font-size: var(--font-size-xs); font-weight: var(--font-weight-semibold); color: var(--color-text-muted); text-transform: uppercase; letter-spacing: 0.04em; background: var(--color-surface); border-bottom: 1px solid var(--color-border); }
  .data-table td { padding: var(--space-3) var(--space-4); border-bottom: 1px solid var(--color-border); }
  .data-table tr:last-child td { border-bottom: none; }

  .code-mono { font-family: var(--font-mono); font-size: var(--font-size-xs); padding: 2px 8px; background: var(--color-surface-2); border-radius: var(--radius-sm); font-weight: var(--font-weight-semibold); }

  .text-muted { color: var(--color-text-muted); font-size: var(--font-size-xs); }
  .text-bold { font-weight: var(--font-weight-semibold); }
  .nowrap { white-space: nowrap; }

  .suspicious-row { background: color-mix(in srgb, var(--color-warning) 5%, transparent); }

  .table-divider { height: 1px; background: var(--color-border); }

  .empty-section { padding: var(--space-8); text-align: center; color: var(--color-text-muted); font-size: var(--font-size-sm); border: 1px dashed var(--color-border); border-radius: var(--radius-lg); }

  :global(.btn-secondary-sm) {
    --button-color: transparent;
    --button-text-color: var(--color-text);
    --button-border: 1px solid var(--color-border);
    --button-border-radius: var(--radius-md);
    --button-padding: var(--space-1) var(--space-4);
    --button-font-size: var(--font-size-sm);
    --button-font-weight: var(--font-weight-medium);
    --button-hover-color: var(--color-surface-2);
  }

  :global(.btn-danger-sm) {
    --button-color: transparent;
    --button-text-color: #ef4444;
    --button-border: 1px solid #fecaca;
    --button-border-radius: var(--radius-md);
    --button-padding: var(--space-1) var(--space-4);
    --button-font-size: var(--font-size-sm);
    --button-font-weight: var(--font-weight-medium);
    --button-hover-color: #fef2f2;
  }

  :global(.btn-success-sm) {
    --button-color: transparent;
    --button-text-color: #059669;
    --button-border: 1px solid #a7f3d0;
    --button-border-radius: var(--radius-md);
    --button-padding: var(--space-1) var(--space-4);
    --button-font-size: var(--font-size-sm);
    --button-font-weight: var(--font-weight-medium);
    --button-hover-color: #ecfdf5;
  }

  :global(.btn-ghost) {
    --button-color: transparent;
    --button-text-color: var(--color-text-muted);
    --button-border: 1px solid var(--color-border);
    --button-border-radius: var(--radius-md);
    --button-padding: var(--space-2) var(--space-5);
    --button-font-size: var(--font-size-sm);
    --button-font-weight: var(--font-weight-medium);
  }
</style>
