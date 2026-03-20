<script lang="ts">
  import { goto } from '$app/navigation';
  import { Button, Input, Slider, Stepper, Toggle } from '@juspay/svelte-ui-components';
  import type { StepperProperties } from '@juspay/svelte-ui-components';
  import type { Merchant } from '$lib/client/modules/admin';
  import { currentMerchant, currentMerchantId } from '$lib/client/modules/admin';
  import { createRule, fetchRules } from '$lib/client/modules/rules';
  import type { CreateRuleRequest } from '$lib/client/modules/rules';
  import { apiCaller } from '$lib/client/modules/foundation';
  import { toastStore } from '$lib/client/modules/foundation';

  let merchantId = $state<string | null>(null);
  let merchant = $state<Merchant | null>(null);
  let currentStep = $state(0);
  let loading = $state(false);
  let alreadySetUp = $state(false);
  let checkingSetup = $state(true);

  currentMerchantId.subscribe((id) => {
    if (id !== null && id !== merchantId) {
      merchantId = id;
      checkExistingSetup(id);
    } else if (id === null) {
      merchantId = null;
      checkingSetup = false;
    }
  });

  currentMerchant.subscribe((m) => {
    merchant = m;
  });

  async function checkExistingSetup(id: string) {
    checkingSetup = true;
    const result = await fetchRules(id);
    if (result.tag === 'success' && result.data.length > 0) {
      alreadySetUp = true;
    }
    checkingSetup = false;
  }

  const steps: StepperProperties['steps'] = [
    { label: 'Welcome' },
    { label: 'Rewards' },
    { label: 'Policy' },
    { label: 'Tiers' },
    { label: 'Referrals' },
    { label: 'Done' }
  ];

  let prepaidPercent = $state(5);
  let minOrderAmount = $state('500');
  let maxCashbackPerOrder = $state('200');
  let enableCod = $state(false);
  let codPercent = $state(3);

  let minRedemption = $state('10');
  let maxPerOrderPct = $state('50');
  let allowWithDiscounts = $state(true);

  let enableTiers = $state(false);
  let enableReferrals = $state(false);
  let referrerReward = $state('50');
  let refereeReward = $state('25');

  let completedSteps = $state({
    rewards: false,
    policy: false,
    tiers: false,
    referrals: false
  });

  let previewOrderAmount = $derived(Number(minOrderAmount) > 0 ? Number(minOrderAmount) : 500);

  let previewCashback = $derived(() => {
    const raw = (previewOrderAmount * prepaidPercent) / 100;
    const cap = Number(maxCashbackPerOrder) > 0 ? Number(maxCashbackPerOrder) : Infinity;
    return Math.min(raw, cap);
  });

  function handleStepClick(event: { selectedIndex: number }) {
    if (event.selectedIndex <= currentStep) {
      currentStep = event.selectedIndex;
    }
  }

  function goNext() {
    if (currentStep < steps.length - 1) {
      currentStep += 1;
    }
  }

  function goBack() {
    if (currentStep > 0) {
      currentStep -= 1;
    }
  }

  async function submitRewards() {
    if (merchantId === null) return;
    loading = true;

    const prepaidRequest: CreateRuleRequest = {
      merchant_id: merchantId,
      rule_type: 'reward',
      name: 'Prepaid Order Cashback',
      config: {
        event_type: 'order.paid',
        conditions: [
          { field: 'payment_method', operator: 'eq', value: 'prepaid' },
          { field: 'total_amount', operator: 'gte', value: Number(minOrderAmount) }
        ],
        action: {
          bucket_type: 'cashback',
          calculation: 'percentage',
          value: prepaidPercent,
          max_amount: Number(maxCashbackPerOrder) > 0 ? Number(maxCashbackPerOrder) : null,
          conversion_rate: null,
          expiry_days: 90
        }
      }
    };

    const prepaidResult = await createRule(prepaidRequest);
    if (prepaidResult.tag === 'error') {
      toastStore.push({ message: prepaidResult.message, level: 'error' });
      loading = false;
      return;
    }

    if (enableCod) {
      const codRequest: CreateRuleRequest = {
        merchant_id: merchantId,
        rule_type: 'reward',
        name: 'COD Order Cashback',
        config: {
          event_type: 'order.delivered',
          conditions: [
            { field: 'payment_method', operator: 'eq', value: 'cod' },
            { field: 'total_amount', operator: 'gte', value: Number(minOrderAmount) }
          ],
          action: {
            bucket_type: 'cashback',
            calculation: 'percentage',
            value: codPercent,
            max_amount: Number(maxCashbackPerOrder) > 0 ? Number(maxCashbackPerOrder) : null,
            conversion_rate: null,
            expiry_days: 90
          }
        }
      };

      const codResult = await createRule(codRequest);
      if (codResult.tag === 'error') {
        toastStore.push({ message: codResult.message, level: 'error' });
        loading = false;
        return;
      }
    }

    completedSteps.rewards = true;
    loading = false;
    goNext();
  }

  async function submitPolicy() {
    if (merchantId === null) return;
    loading = true;

    const result = await apiCaller.post(
      '/admin/wallet-policies',
      {
        merchant_id: merchantId,
        bucket_type: 'earned_credit',
        min_redemption: Number(minRedemption),
        step_size: 1,
        max_per_order_pct: Number(maxPerOrderPct),
        max_per_order_fixed: null,
        stackable_with_discounts: allowWithDiscounts,
        default_expiry_days: 365,
        is_transferable: false
      },
      (raw: unknown) => raw
    );

    if (result.tag === 'error') {
      toastStore.push({ message: result.message, level: 'error' });
      loading = false;
      return;
    }

    completedSteps.policy = true;
    loading = false;
    goNext();
  }

  async function submitTiers() {
    if (merchantId === null) return;

    if (!enableTiers) {
      completedSteps.tiers = false;
      loading = false;
      goNext();
      return;
    }

    loading = true;

    const programResult = await apiCaller.post(
      '/loyalty/programs',
      { merchant_id: merchantId, name: 'Loyalty Program', evaluation_criteria: 'spend' },
      (raw: unknown) => raw as Record<string, unknown>
    );

    if (programResult.tag === 'error') {
      toastStore.push({ message: programResult.message, level: 'error' });
      loading = false;
      return;
    }

    const programId = programResult.data['id'] as string;

    const tierDefs = [
      { name: 'Bronze', rank: 1, threshold: 0, earn_rate_multiplier: 1.0 },
      { name: 'Silver', rank: 2, threshold: 2000, earn_rate_multiplier: 1.25 },
      { name: 'Gold', rank: 3, threshold: 5000, earn_rate_multiplier: 1.5 },
      { name: 'Platinum', rank: 4, threshold: 15000, earn_rate_multiplier: 2.0 }
    ];

    for (const tier of tierDefs) {
      const tierResult = await apiCaller.post(
        '/loyalty/tiers',
        { program_id: programId, ...tier, benefits: {} },
        (raw: unknown) => raw
      );
      if (tierResult.tag === 'error') {
        toastStore.push({ message: tierResult.message, level: 'error' });
        loading = false;
        return;
      }
    }

    completedSteps.tiers = true;
    loading = false;
    goNext();
  }

  async function submitReferrals() {
    if (!enableReferrals) {
      completedSteps.referrals = false;
      loading = false;
      goNext();
      return;
    }

    loading = true;

    const result = await apiCaller.post(
      '/referrals/programs',
      {
        merchant_id: merchantId,
        referrer_reward_amount: Number(referrerReward),
        referee_reward_amount: Number(refereeReward),
        max_referrals_per_customer: null
      },
      (raw: unknown) => raw
    );

    if (result.tag === 'error') {
      toastStore.push({ message: result.message, level: 'error' });
      loading = false;
      return;
    }

    completedSteps.referrals = true;
    loading = false;
    goNext();
  }

  function goToDashboard() {
    goto('/admin');
  }

  function openStorefront() {
    if (merchant !== null && merchant.slug !== null) {
      window.open(`/s/${merchant.slug}`, '_blank');
    } else if (merchant !== null) {
      window.open(`/s/${merchant.external_id}`, '_blank');
    }
  }

  let merchantName = $derived(merchant !== null ? merchant.name : 'Merchant');
</script>

<svelte:head>
  <title>Setup - Batua Admin</title>
</svelte:head>

<div class="setup-page">
  {#if merchantId === null}
    <div class="empty-state">
      <p class="empty-text">Select a merchant first to begin setup.</p>
    </div>
  {:else if checkingSetup}
    <div class="empty-state">
      <p class="empty-text">Checking setup status...</p>
    </div>
  {:else if alreadySetUp}
    <div class="already-done">
      <div class="already-done-icon">&#10003;</div>
      <h2>Setup Complete</h2>
      <p class="already-done-text">This merchant already has reward rules configured.</p>
      <div class="already-done-actions">
        <Button text="Go to Dashboard" classes="btn-primary" onclick={goToDashboard} />
      </div>
    </div>
  {:else}
    <div class="stepper-wrapper">
      <Stepper {steps} currentStepIndex={currentStep} onhandleStepClick={handleStepClick} />
    </div>

    <div class="step-content">
      {#if currentStep === 0}
        <div class="step-panel welcome-panel">
          <div class="welcome-icon">&#9733;</div>
          <h1 class="welcome-title">Welcome to Batua!</h1>
          <p class="welcome-subtitle">Let's set up your loyalty program in 5 minutes.</p>
          <p class="welcome-merchant">Setting up for <strong>{merchantName}</strong></p>
          <div class="step-actions">
            <Button text="Get Started" classes="btn-primary" onclick={goNext} />
          </div>
        </div>
      {:else if currentStep === 1}
        <div class="step-panel">
          <h2 class="step-title">Create Your First Reward Rule</h2>
          <p class="step-description">How much cashback on prepaid orders?</p>

          <div class="form-group">
            <span class="form-label">Cashback percentage</span>
            <div class="slider-row">
              <Slider
                value={prepaidPercent}
                min={1}
                max={20}
                step={1}
                showValue={true}
                onchange={(v) => {
                  prepaidPercent = v;
                }}
              />
              <span class="slider-value">{prepaidPercent}%</span>
            </div>
          </div>

          <div class="form-row">
            <div class="form-group">
              <Input
                value={minOrderAmount}
                label="Minimum order amount"
                placeholder="500"
                onInput={(v) => {
                  minOrderAmount = v;
                }}
              />
            </div>
            <div class="form-group">
              <Input
                value={maxCashbackPerOrder}
                label="Maximum cashback per order"
                placeholder="200"
                onInput={(v) => {
                  maxCashbackPerOrder = v;
                }}
              />
            </div>
          </div>

          <div class="form-group">
            <Toggle
              text="Also reward COD orders?"
              checked={enableCod}
              onclick={(checked) => {
                enableCod = checked;
              }}
            />
          </div>

          {#if enableCod}
            <div class="form-group cod-section">
              <span class="form-label">COD cashback percentage</span>
              <div class="slider-row">
                <Slider
                  value={codPercent}
                  min={1}
                  max={20}
                  step={1}
                  showValue={true}
                  onchange={(v) => {
                    codPercent = v;
                  }}
                />
                <span class="slider-value">{codPercent}%</span>
              </div>
            </div>
          {/if}

          <div class="preview-card">
            <span class="preview-label">Preview</span>
            <p class="preview-text">
              Customers will earn
              <strong>Rs. {previewCashback()}</strong>
              on a <strong>Rs. {previewOrderAmount}</strong> prepaid order
            </p>
          </div>

          <div class="step-actions">
            <Button text="Back" classes="btn-secondary" onclick={goBack} />
            <Button
              text="Next"
              classes="btn-primary"
              showLoader={loading}
              onclick={submitRewards}
            />
          </div>
        </div>
      {:else if currentStep === 2}
        <div class="step-panel">
          <h2 class="step-title">Set Up Wallet Policy</h2>
          <p class="step-description">How should customers redeem their credits?</p>

          <div class="form-group">
            <Input
              value={minRedemption}
              label="Minimum redemption amount"
              placeholder="10"
              onInput={(v) => {
                minRedemption = v;
              }}
            />
          </div>

          <div class="form-group">
            <Input
              value={maxPerOrderPct}
              label="Maximum per order (%)"
              placeholder="50"
              onInput={(v) => {
                maxPerOrderPct = v;
              }}
            />
          </div>

          <div class="form-group">
            <Toggle
              text="Allow redemption with discount codes"
              checked={allowWithDiscounts}
              onclick={(checked) => {
                allowWithDiscounts = checked;
              }}
            />
          </div>

          <div class="step-actions">
            <Button text="Back" classes="btn-secondary" onclick={goBack} />
            <Button text="Next" classes="btn-primary" showLoader={loading} onclick={submitPolicy} />
          </div>
        </div>
      {:else if currentStep === 3}
        <div class="step-panel">
          <h2 class="step-title">Loyalty Tiers</h2>
          <p class="step-description">Want to reward your best customers with tiers?</p>

          <div class="form-group">
            <Toggle
              text="Enable loyalty tiers"
              checked={enableTiers}
              onclick={(checked) => {
                enableTiers = checked;
              }}
            />
          </div>

          {#if enableTiers}
            <div class="tiers-preset">
              <p class="preset-label">Recommended tiers</p>
              <div class="tiers-list">
                <div class="tier-item">
                  <span class="tier-badge tier-bronze">Bronze</span>
                  <span class="tier-detail">Rs. 0+ spend &middot; 1x rewards</span>
                </div>
                <div class="tier-item">
                  <span class="tier-badge tier-silver">Silver</span>
                  <span class="tier-detail">Rs. 2,000+ spend &middot; 1.25x rewards</span>
                </div>
                <div class="tier-item">
                  <span class="tier-badge tier-gold">Gold</span>
                  <span class="tier-detail">Rs. 5,000+ spend &middot; 1.5x rewards</span>
                </div>
                <div class="tier-item">
                  <span class="tier-badge tier-platinum">Platinum</span>
                  <span class="tier-detail">Rs. 15,000+ spend &middot; 2x rewards</span>
                </div>
              </div>
            </div>
          {/if}

          <div class="step-actions">
            <Button text="Back" classes="btn-secondary" onclick={goBack} />
            {#if !enableTiers}
              <Button text="Skip" classes="btn-secondary" onclick={submitTiers} />
            {/if}
            <Button
              text={enableTiers ? 'Next' : 'Next'}
              classes="btn-primary"
              showLoader={loading}
              onclick={submitTiers}
            />
          </div>
        </div>
      {:else if currentStep === 4}
        <div class="step-panel">
          <h2 class="step-title">Referral Program</h2>
          <p class="step-description">Let customers refer their friends?</p>

          <div class="form-group">
            <Toggle
              text="Enable referral program"
              checked={enableReferrals}
              onclick={(checked) => {
                enableReferrals = checked;
              }}
            />
          </div>

          {#if enableReferrals}
            <div class="form-row">
              <div class="form-group">
                <Input
                  value={referrerReward}
                  label="Referrer reward (Rs.)"
                  placeholder="50"
                  onInput={(v) => {
                    referrerReward = v;
                  }}
                />
              </div>
              <div class="form-group">
                <Input
                  value={refereeReward}
                  label="Referee reward (Rs.)"
                  placeholder="25"
                  onInput={(v) => {
                    refereeReward = v;
                  }}
                />
              </div>
            </div>
          {/if}

          <div class="step-actions">
            <Button text="Back" classes="btn-secondary" onclick={goBack} />
            {#if !enableReferrals}
              <Button text="Skip" classes="btn-secondary" onclick={submitReferrals} />
            {/if}
            <Button
              text="Next"
              classes="btn-primary"
              showLoader={loading}
              onclick={submitReferrals}
            />
          </div>
        </div>
      {:else if currentStep === 5}
        <div class="step-panel done-panel">
          <div class="done-icon">&#10003;</div>
          <h1 class="done-title">You're all set!</h1>
          <p class="done-subtitle">
            Your loyalty program for <strong>{merchantName}</strong> is ready to go.
          </p>

          <div class="summary-card">
            <h3 class="summary-heading">Setup Summary</h3>
            <ul class="summary-list">
              <li class="summary-item summary-done">
                <span class="check">&#10003;</span>
                Reward rule created ({prepaidPercent}% cashback on prepaid orders{enableCod
                  ? `, ${codPercent}% on COD`
                  : ''})
              </li>
              <li class="summary-item summary-done">
                <span class="check">&#10003;</span>
                Wallet policy set (min Rs. {minRedemption}, max {maxPerOrderPct}% per order)
              </li>
              <li
                class="summary-item"
                class:summary-done={completedSteps.tiers}
                class:summary-skipped={!completedSteps.tiers}
              >
                <span class="check">{completedSteps.tiers ? '✓' : '—'}</span>
                {completedSteps.tiers
                  ? 'Loyalty tiers configured (Bronze, Silver, Gold, Platinum)'
                  : 'Loyalty tiers skipped'}
              </li>
              <li
                class="summary-item"
                class:summary-done={completedSteps.referrals}
                class:summary-skipped={!completedSteps.referrals}
              >
                <span class="check">{completedSteps.referrals ? '✓' : '—'}</span>
                {completedSteps.referrals
                  ? `Referral program set up (Rs. ${referrerReward} referrer / Rs. ${refereeReward} referee)`
                  : 'Referral program skipped'}
              </li>
            </ul>
          </div>

          <div class="step-actions done-actions">
            <Button text="Go to Dashboard" classes="btn-primary" onclick={goToDashboard} />
            <Button text="View Your Storefront" classes="btn-secondary" onclick={openStorefront} />
          </div>
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .setup-page {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-8);
    max-width: 640px;
    margin: 0 auto;
    padding: var(--space-4) 0;
  }

  .stepper-wrapper {
    width: 100%;
  }

  .step-content {
    width: 100%;
  }

  .step-panel {
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: var(--space-8);
  }

  .welcome-panel {
    align-items: center;
    text-align: center;
    padding: var(--space-12) var(--space-8);
  }

  .welcome-icon {
    font-size: 48px;
    color: var(--color-primary);
    line-height: 1;
  }

  .welcome-title {
    font-size: var(--font-size-3xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    line-height: var(--line-height-tight);
  }

  .welcome-subtitle {
    font-size: var(--font-size-lg);
    color: var(--color-text-muted);
  }

  .welcome-merchant {
    font-size: var(--font-size-md);
    color: var(--color-text);
  }

  .step-title {
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    line-height: var(--line-height-tight);
  }

  .step-description {
    font-size: var(--font-size-base);
    color: var(--color-text-muted);
    margin-top: calc(-1 * var(--space-3));
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .form-label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-text-muted);
  }

  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-4);
  }

  .slider-row {
    display: flex;
    align-items: center;
    gap: var(--space-4);
  }

  .slider-value {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
    color: var(--color-primary);
    min-width: 48px;
    text-align: right;
  }

  .cod-section {
    padding: var(--space-4);
    background: var(--color-surface-2);
    border-radius: var(--radius-md);
  }

  .preview-card {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    padding: var(--space-4);
    background: var(--color-surface-2);
    border-radius: var(--radius-md);
    border-left: 3px solid var(--color-primary);
  }

  .preview-label {
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .preview-text {
    font-size: var(--font-size-md);
    color: var(--color-text);
  }

  .step-actions {
    display: flex;
    justify-content: flex-end;
    gap: var(--space-3);
    padding-top: var(--space-4);
    border-top: 1px solid var(--color-border);
  }

  .tiers-preset {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
    padding: var(--space-4);
    background: var(--color-surface-2);
    border-radius: var(--radius-md);
  }

  .preset-label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-muted);
  }

  .tiers-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .tier-item {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-3) var(--space-4);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
  }

  .tier-badge {
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-semibold);
    padding: 2px 10px;
    border-radius: var(--radius-full);
    white-space: nowrap;
  }

  .tier-bronze {
    background: #f5e6d3;
    color: #8b5e3c;
  }

  .tier-silver {
    background: #e8e8e8;
    color: #555555;
  }

  .tier-gold {
    background: #fdf3d7;
    color: #92742e;
  }

  .tier-platinum {
    background: #e8eaf6;
    color: #3949ab;
  }

  :global([data-theme='dark']) .tier-bronze {
    background: #3d2b1a;
    color: #d4a574;
  }

  :global([data-theme='dark']) .tier-silver {
    background: #2a2a2a;
    color: #b0b0b0;
  }

  :global([data-theme='dark']) .tier-gold {
    background: #3d3414;
    color: #ddc35e;
  }

  :global([data-theme='dark']) .tier-platinum {
    background: #1a1e3d;
    color: #7986cb;
  }

  .tier-detail {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .done-panel {
    align-items: center;
    text-align: center;
    padding: var(--space-12) var(--space-8);
  }

  .done-icon {
    width: 72px;
    height: 72px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    background: var(--color-success);
    color: #ffffff;
    font-size: 36px;
    font-weight: var(--font-weight-bold);
  }

  .done-title {
    font-size: var(--font-size-3xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    line-height: var(--line-height-tight);
  }

  .done-subtitle {
    font-size: var(--font-size-md);
    color: var(--color-text-muted);
  }

  .summary-card {
    width: 100%;
    max-width: 480px;
    text-align: left;
    background: var(--color-surface-2);
    border-radius: var(--radius-md);
    padding: var(--space-6);
  }

  .summary-heading {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
    margin-bottom: var(--space-4);
  }

  .summary-list {
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .summary-item {
    display: flex;
    align-items: flex-start;
    gap: var(--space-3);
    font-size: var(--font-size-base);
    color: var(--color-text);
    line-height: var(--line-height-base);
  }

  .summary-done .check {
    color: var(--color-success);
    font-weight: var(--font-weight-bold);
  }

  .summary-skipped .check {
    color: var(--color-text-muted);
  }

  .summary-skipped {
    color: var(--color-text-muted);
  }

  .done-actions {
    justify-content: center;
  }

  .empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--space-16);
    background: var(--color-surface);
    border: 1px dashed var(--color-border);
    border-radius: var(--radius-lg);
    width: 100%;
  }

  .empty-text {
    color: var(--color-text-muted);
    font-size: var(--font-size-base);
  }

  .already-done {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-4);
    padding: var(--space-12);
    text-align: center;
  }

  .already-done-icon {
    width: 64px;
    height: 64px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    background: var(--color-success);
    color: #ffffff;
    font-size: 32px;
    font-weight: var(--font-weight-bold);
  }

  .already-done h2 {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
  }

  .already-done-text {
    font-size: var(--font-size-base);
    color: var(--color-text-muted);
  }

  .already-done-actions {
    margin-top: var(--space-4);
  }

  @media (max-width: 640px) {
    .setup-page {
      padding: var(--space-2) 0;
    }

    .step-panel {
      padding: var(--space-6);
    }

    .welcome-panel,
    .done-panel {
      padding: var(--space-8) var(--space-4);
    }

    .form-row {
      grid-template-columns: 1fr;
    }
  }
</style>
