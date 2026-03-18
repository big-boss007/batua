<script lang="ts">
  import type { WalletPolicy, UpdateWalletPolicyRequest } from '$lib/client/modules/settings';

  let {
    policy,
    onSave
  }: {
    policy: WalletPolicy;
    onSave: (policyId: string, data: UpdateWalletPolicyRequest) => void;
  } = $props();

  let minRedemption = $state(policy.min_redemption);
  let stepSize = $state(policy.step_size);
  let maxPerOrderPct = $state(policy.max_per_order_pct);
  let maxPerOrderFixed = $state(policy.max_per_order_fixed);
  let stackable = $state(policy.stackable_with_discounts);
  let expiryDays = $state(policy.default_expiry_days);
  let transferable = $state(policy.is_transferable);

  function handleSubmit() {
    onSave(policy.id, {
      min_redemption: minRedemption,
      step_size: stepSize,
      max_per_order_pct: maxPerOrderPct,
      max_per_order_fixed: maxPerOrderFixed,
      stackable_with_discounts: stackable,
      default_expiry_days: expiryDays,
      is_transferable: transferable
    });
  }
</script>

<form class="policy-form" onsubmit={handleSubmit}>
  <h4 class="form-title">{policy.bucket_type}</h4>

  <div class="form-grid">
    <div class="form-field">
      <label class="field-label" for="min-redemption-{policy.id}">Min Redemption (₹)</label>
      <input
        id="min-redemption-{policy.id}"
        class="field-input"
        type="number"
        bind:value={minRedemption}
        placeholder="No minimum"
      />
    </div>

    <div class="form-field">
      <label class="field-label" for="step-size-{policy.id}">Step Size (₹)</label>
      <input
        id="step-size-{policy.id}"
        class="field-input"
        type="number"
        bind:value={stepSize}
        placeholder="No step"
      />
    </div>

    <div class="form-field">
      <label class="field-label" for="max-pct-{policy.id}">Max Per Order (%)</label>
      <input
        id="max-pct-{policy.id}"
        class="field-input"
        type="number"
        min="0"
        max="100"
        bind:value={maxPerOrderPct}
        placeholder="No limit"
      />
    </div>

    <div class="form-field">
      <label class="field-label" for="max-fixed-{policy.id}">Max Per Order Fixed (₹)</label>
      <input
        id="max-fixed-{policy.id}"
        class="field-input"
        type="number"
        bind:value={maxPerOrderFixed}
        placeholder="No limit"
      />
    </div>

    <div class="form-field">
      <label class="field-label" for="expiry-{policy.id}">Default Expiry (days)</label>
      <input
        id="expiry-{policy.id}"
        class="field-input"
        type="number"
        min="0"
        bind:value={expiryDays}
        placeholder="No expiry"
      />
    </div>
  </div>

  <div class="toggle-group">
    <label class="toggle-field">
      <input type="checkbox" class="toggle-input" bind:checked={stackable} />
      <span class="toggle-label">Stackable with discounts</span>
    </label>

    <label class="toggle-field">
      <input type="checkbox" class="toggle-input" bind:checked={transferable} />
      <span class="toggle-label">Transferable</span>
    </label>
  </div>

  <button type="submit" class="save-button">Save Policy</button>
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
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
    margin-bottom: var(--space-5);
    text-transform: capitalize;
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
    gap: var(--space-1);
  }

  .field-label {
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-medium);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .field-input {
    padding: var(--space-2) var(--space-3);
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    color: var(--color-text);
    font-size: var(--font-size-sm);
    transition: border-color var(--transition-fast);
  }

  .field-input:focus {
    outline: none;
    border-color: var(--color-primary);
  }

  .field-input::placeholder {
    color: var(--color-text-muted);
  }

  .toggle-group {
    display: flex;
    gap: var(--space-6);
    margin-bottom: var(--space-5);
  }

  .toggle-field {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    cursor: pointer;
  }

  .toggle-input {
    width: 16px;
    height: 16px;
    accent-color: var(--color-primary);
    cursor: pointer;
  }

  .toggle-label {
    font-size: var(--font-size-sm);
    color: var(--color-text);
    font-weight: var(--font-weight-medium);
  }

  .save-button {
    padding: var(--space-2) var(--space-5);
    background: var(--color-primary);
    color: #ffffff;
    border: none;
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    transition: background var(--transition-fast);
  }

  .save-button:hover {
    background: var(--color-primary-hover);
  }
</style>
