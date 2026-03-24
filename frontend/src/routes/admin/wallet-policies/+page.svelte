<script lang="ts">
  import { Button, Input } from '@juspay/svelte-ui-components';
  import type { Merchant } from '$lib/client/modules/admin';
  import { currentMerchant, currentMerchantId } from '$lib/client/modules/admin';
  import type { WalletPolicy, UpdateWalletPolicyRequest } from '$lib/client/modules/settings';
  import {
    fetchWalletPolicies,
    updateWalletPolicy,
    walletPoliciesStore,
    updateMerchantProfile
  } from '$lib/client/modules/settings';
  import { toastStore } from '$lib/client/modules/foundation';
  import { WalletPoliciesList, WalletPolicyForm } from '$lib/client/modules/settings/ui';
  import type { Rule } from '$lib/client/modules/rules';
  import { fetchRules } from '$lib/client/modules/rules';

  let merchant = $state<Merchant | null>(null);
  let mId = $state<string | null>(null);
  let policies = $state<Array<WalletPolicy>>([]);
  let editingPolicy = $state<WalletPolicy | null>(null);
  let rules = $state<Array<Rule>>([]);
  let pointsName = $state('Points');
  let pointsIcon = $state('pts');
  let pointsRate = $state('1.0');
  let savingPoints = $state(false);

  currentMerchant.subscribe((m) => {
    merchant = m;
    if (m !== null) {
      pointsName = m.points_name;
      pointsIcon = m.points_icon;
      pointsRate = String(m.points_to_currency_rate);
    }
  });

  let rateNum = $derived(parseFloat(pointsRate) || 1.0);
  let rateInverse = $derived(rateNum > 0 ? Math.round(1 / rateNum) : 1);

  let earnRules = $derived(
    rules
      .filter((r) => r.is_active && r.rule_type === 'reward')
      .map((r) => {
        const action = r.config?.action;
        if (action === undefined) return null;
        const isPercentage = action.calculation === 'percentage';
        const expiryDays = action.expiry_days;

        let text: string;
        let sub: string;

        if (isPercentage) {
          const pts = Math.round((1000 * action.value) / 100);
          const worth = Math.round(pts * rateNum);
          text = `On a ₹1,000 order → customer earns ${pts.toLocaleString('en-IN')} ${pointsIcon}`;
          sub = `worth ₹${worth}`;
          if (action.max_amount !== null && action.max_amount > 0) {
            const maxWorth = Math.round(action.max_amount * rateNum);
            sub += ` · capped at ${action.max_amount.toLocaleString('en-IN')} ${pointsIcon} (₹${maxWorth})`;
          }
        } else {
          const worth = Math.round(action.value * rateNum);
          text = `Customer earns ${action.value.toLocaleString('en-IN')} ${pointsIcon}`;
          sub = `worth ₹${worth}`;
        }

        if (expiryDays !== null && expiryDays !== undefined && expiryDays > 0) {
          sub += ` · expires in ${expiryDays} days`;
        }

        return { name: r.name, text, sub };
      })
      .filter((r) => r !== null)
  );

  currentMerchantId.subscribe((id) => {
    const prevId = mId;
    mId = id;
    if (id !== null && id !== prevId) {
      loadData(id);
    }
  });

  async function loadData(id: string) {
    const [policiesResult, rulesResult] = await Promise.all([
      fetchWalletPolicies(id),
      fetchRules(id)
    ]);
    if (policiesResult.tag === 'success') policies = policiesResult.data;
    if (rulesResult.tag === 'success') rules = rulesResult.data;
  }

  async function handleSavePoints() {
    if (merchant === null) return;
    savingPoints = true;
    const result = await updateMerchantProfile(merchant.id, {
      points_name: pointsName,
      points_icon: pointsIcon,
      points_to_currency_rate: parseFloat(pointsRate) || 1.0
    });
    if (result.tag === 'success') {
      currentMerchant.set(result.data);
      toastStore.push({ message: 'Points configuration saved', level: 'success' });
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }
    savingPoints = false;
  }

  function handleEditPolicy(policy: WalletPolicy) {
    editingPolicy = policy;
  }

  async function handleSavePolicy(policyId: string, body: UpdateWalletPolicyRequest) {
    const result = await updateWalletPolicy(policyId, body);
    if (result.tag === 'success') {
      walletPoliciesStore.updatePolicy(result.data);
      editingPolicy = null;
      toastStore.push({ message: 'Policy updated', level: 'success' });
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }
  }
</script>

<svelte:head>
  <title>Wallet Policies - Batua Admin</title>
</svelte:head>

<div class="wallet-policies-page">
  <header class="page-header">
    <h1 class="page-title">Wallet Policies</h1>
    <p class="subtitle">Configure your points currency and bucket policies</p>
  </header>

  {#if merchant !== null}
    <div class="section-label">Points Configuration</div>
    <div class="points-card">
      <div class="points-form">
        <div class="points-row">
          <div class="points-field" style="flex: 1;">
            <label class="points-field-label">Points Name</label>
            <Input
              value={pointsName}
              placeholder="Stars"
              onInput={(val) => {
                pointsName = val;
              }}
            />
          </div>
          <div class="points-field" style="flex: 0 0 90px;">
            <label class="points-field-label">Icon</label>
            <Input
              value={pointsIcon}
              placeholder="★"
              onInput={(val) => {
                pointsIcon = val;
              }}
            />
          </div>
        </div>
        <div>
          <label class="points-field-label">Value per Point (₹)</label>
          <Input
            value={pointsRate}
            dataType="number"
            placeholder="0.25"
            onInput={(val) => {
              pointsRate = val;
            }}
          />
        </div>
        <div class="points-actions">
          <Button
            text={savingPoints ? 'Saving...' : 'Save'}
            classes="btn-primary"
            onclick={handleSavePoints}
          />
        </div>
      </div>
      <div class="points-preview">
        <span class="points-preview-label">Preview</span>
        <div class="points-preview-icon">{pointsIcon}</div>
        <div class="points-preview-name">{pointsName}</div>
        <div class="points-preview-rate">{rateInverse} {pointsIcon} = ₹1</div>
        <div class="points-preview-eq">1 {pointsName} = ₹{pointsRate}</div>
      </div>
    </div>

    {#if earnRules.length > 0}
      <div class="earn-rates">
        <span class="earn-rates-label">Active Earn Rates</span>
        <div class="earn-rates-list">
          {#each earnRules as rule}
            <div class="earn-preview-card">
              <div class="earn-preview-name">{rule.name}</div>
              <div class="earn-preview-text">{rule.text}</div>
              <div class="earn-preview-sub">{rule.sub}</div>
            </div>
          {/each}
        </div>
      </div>
    {:else}
      <div class="earn-rates-empty">
        No active earn rules. <a href="/admin/rules">Configure rules →</a>
      </div>
    {/if}

    <div class="section-divider"></div>
    <div class="section-label">Global Bucket Policies</div>
  {/if}

  <div class="policies-constrained">
    {#if editingPolicy !== null}
      <WalletPolicyForm
        policy={editingPolicy}
        {pointsIcon}
        onSave={handleSavePolicy}
        onCancel={() => {
          editingPolicy = null;
        }}
      />
    {:else}
      <WalletPoliciesList {policies} onEdit={handleEditPolicy} />
    {/if}
  </div>
</div>

<style>
  .wallet-policies-page {
    display: flex;
    flex-direction: column;
    gap: var(--space-5);
    max-width: 800px;
  }

  .page-header {
    margin-bottom: var(--space-2);
  }

  .page-title {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
  }

  .subtitle {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    margin-top: var(--space-1);
  }

  .section-label {
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.8px;
    margin-bottom: var(--space-1);
  }

  .points-card {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    overflow: hidden;
    display: grid;
    grid-template-columns: 1fr 210px;
    max-width: 720px;
  }

  .points-form {
    padding: var(--space-5) var(--space-6);
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .points-field-label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-muted);
    display: block;
    margin-bottom: 4px;
  }

  .points-actions {
    display: flex;
    justify-content: flex-end;
    padding-top: var(--space-1);
  }

  .points-preview {
    background: linear-gradient(165deg, #f0fdf4, #dcfce7);
    border-left: 1px solid #bbf7d0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: var(--space-6) var(--space-5);
    gap: 4px;
  }

  .points-preview-label {
    font-size: 10px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 1px;
    color: #16a34a;
    margin-bottom: var(--space-2);
  }

  .points-preview-icon {
    width: 52px;
    height: 52px;
    background: #fff;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 26px;
    box-shadow: 0 2px 8px rgba(22, 163, 74, 0.12);
  }

  .points-preview-name {
    font-size: 15px;
    font-weight: var(--font-weight-bold);
    color: #14532d;
    margin-top: 6px;
  }

  .points-preview-rate {
    font-size: 20px;
    font-weight: 800;
    color: #059669;
    margin-top: 2px;
  }

  .points-preview-eq {
    font-size: 11px;
    color: #16a34a;
    opacity: 0.7;
  }

  .points-row {
    display: flex;
    gap: var(--space-3);
  }

  .points-field {
    display: flex;
    flex-direction: column;
  }

  .earn-rates {
    padding: var(--space-4);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    max-width: 720px;
  }

  .earn-rates-label {
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    display: block;
    margin-bottom: var(--space-3);
  }

  .earn-rates-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .earn-preview-card {
    padding: var(--space-3) var(--space-4);
    background: color-mix(in srgb, var(--color-success) 6%, transparent);
    border: 1px solid color-mix(in srgb, var(--color-success) 20%, transparent);
    border-radius: var(--radius-md);
  }

  .earn-preview-name {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
    margin-bottom: var(--space-1);
  }

  .earn-preview-text {
    font-size: var(--font-size-sm);
    color: var(--color-text);
  }

  .earn-preview-sub {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    margin-top: 2px;
  }

  .earn-rates-empty {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .earn-rates-empty a {
    color: var(--color-primary);
    font-weight: var(--font-weight-medium);
  }

  .section-divider {
    height: 1px;
    background: var(--color-border);
    margin: var(--space-2) 0;
    max-width: 720px;
  }

  .policies-constrained {
    max-width: 720px;
  }

  :global(.btn-primary) {
    --button-color: var(--color-primary);
    --button-text-color: #ffffff;
    --button-border-radius: var(--radius-md);
    --button-padding: var(--space-2) var(--space-5);
    --button-font-size: var(--font-size-sm);
    --button-font-weight: var(--font-weight-semibold);
    --button-hover-color: var(--color-primary-hover);
  }
</style>
