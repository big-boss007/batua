<script lang="ts">
  import { Input, Button, Toggle } from '@juspay/svelte-ui-components';
  import { isPointsBucket } from '$lib/client/modules/foundation';

  import type { WalletPolicy, UpdateWalletPolicyRequest } from '$lib/client/modules/settings';

  const BUCKET_LABELS: Record<string, string> = {
    EarnedCredit: 'Reward Points',
    CodPending: 'COD Pending Points',
    GiftCard: 'Gift Card',
    CustomerFunded: 'Customer Funded',
    ReferralReward: 'Referral Points',
    GoodwillCredit: 'Courtesy Points',
    MembershipBenefit: 'Membership Benefit',
    RefundCredit: 'Store Credit'
  };

  const BUCKET_DESCRIPTIONS: Record<string, string> = {
    EarnedCredit:
      'Controls how reward points can be redeemed at checkout — limits, caps, expiry, and discount stacking.',
    CodPending: 'Policy for points held pending COD delivery confirmation.',
    GiftCard: 'Controls how gift card balances can be spent at checkout.',
    CustomerFunded: 'Policy for customer-funded store credit balances.',
    ReferralReward: 'Controls how referral points can be redeemed.',
    GoodwillCredit: 'Policy for courtesy points issued by your team.',
    MembershipBenefit: 'Policy for membership benefit credits.',
    RefundCredit: 'Policy for store credits (goodwill, refund, token of apology etc.) issued to customers.'
  };

  let {
    policy,
    pointsIcon,
    onSave,
    onCancel
  }: {
    policy: WalletPolicy;
    pointsIcon: string;
    onSave: (policyId: string, data: UpdateWalletPolicyRequest) => void;
    onCancel: () => void;
  } = $props();

  // svelte-ignore state_referenced_locally
  let minRedemption = $state(String(policy.min_redemption ?? ''));
  // svelte-ignore state_referenced_locally
  let maxPerOrderPct = $state(String(policy.max_per_order_pct ?? ''));
  // svelte-ignore state_referenced_locally
  let maxPerOrderFixed = $state(String(policy.max_per_order_fixed ?? ''));
  // svelte-ignore state_referenced_locally
  let stackable = $state(policy.stackable_with_discounts);
  // svelte-ignore state_referenced_locally
  let expiryDays = $state(String(policy.default_expiry_days ?? ''));
  let isPoints = $derived(isPointsBucket(policy.bucket_type));
  let unitIcon = $derived(isPoints ? pointsIcon : '₹');
  let displayName = $derived(BUCKET_LABELS[policy.bucket_type] ?? policy.bucket_type);
  let description = $derived(BUCKET_DESCRIPTIONS[policy.bucket_type] ?? '');

  function handleSubmit() {
    onSave(policy.id, {
      min_redemption: Number(minRedemption) || null,
      max_per_order_pct: Number(maxPerOrderPct) || null,
      max_per_order_fixed: Number(maxPerOrderFixed) || null,
      stackable_with_discounts: stackable,
      default_expiry_days: Number(expiryDays) || null
    });
  }
</script>

<form class="policy-form" onsubmit={handleSubmit}>
  <h4 class="form-title">{displayName}</h4>
  {#if description}
    <p class="form-desc">{description}</p>
  {/if}

  <div class="form-grid">
    <div class="form-field">
      <Input
        value={minRedemption}
        label={'Min Redemption (' + unitIcon + ')'}
        dataType="number"
        placeholder="No minimum"
        onInput={(val) => {
          minRedemption = val;
        }}
      />
      <span class="field-hint">Customer must have at least this many to redeem</span>
    </div>

    <div class="form-field">
      <Input
        value={maxPerOrderPct}
        label="Max Per Order (%)"
        dataType="number"
        placeholder="No limit"
        onInput={(val) => {
          maxPerOrderPct = val;
        }}
      />
      <span class="field-hint">Max % of order value payable with credits</span>
    </div>

    <div class="form-field">
      <Input
        value={maxPerOrderFixed}
        label={'Max Per Order (' + unitIcon + ')'}
        dataType="number"
        placeholder="No limit"
        onInput={(val) => {
          maxPerOrderFixed = val;
        }}
      />
      <span class="field-hint">Absolute cap on credits per order</span>
    </div>

    <div class="form-field">
      <Input
        value={expiryDays}
        label="Default Expiry (days)"
        dataType="number"
        placeholder="No expiry"
        onInput={(val) => {
          expiryDays = val;
        }}
      />
      <span class="field-hint">Credits expire this many days after earning</span>
    </div>
  </div>

  <div class="toggle-group">
    <Toggle
      text=""
      checked={stackable}
      onclick={(checked) => {
        stackable = checked;
      }}
    />
    <div class="toggle-text">
      <span class="toggle-label">Stackable with discounts</span>
      <span class="toggle-desc">Allow credits alongside Shopify discount codes</span>
    </div>
  </div>

  <div class="form-actions">
    <Button text="Cancel" classes="btn-ghost" onclick={onCancel} />
    <Button text="Save Policy" classes="btn-primary" type="submit" />
  </div>
</form>

<style>
  .policy-form {
    padding: var(--space-6);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
  }

  .form-title {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    margin-bottom: var(--space-1);
  }

  .form-desc {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    line-height: 1.5;
    margin-bottom: var(--space-5);
  }

  .form-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: var(--space-4);
    margin-bottom: var(--space-5);
  }

  .form-field {
    display: flex;
    flex-direction: column;
  }

  .field-hint {
    font-size: 10px;
    color: var(--color-text-muted);
    margin-top: 3px;
    opacity: 0.7;
  }

  .toggle-group {
    display: flex;
    align-items: flex-start;
    gap: var(--space-3);
    margin-bottom: var(--space-5);
  }

  .toggle-text {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .toggle-label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-text);
  }

  .toggle-desc {
    font-size: 11px;
    color: var(--color-text-muted);
  }

  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: var(--space-2);
  }
</style>
