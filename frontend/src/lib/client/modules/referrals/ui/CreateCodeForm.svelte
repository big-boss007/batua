<script lang="ts">
  let {
    onCreateCode
  }: {
    onCreateCode: (params: {
      customer_id: string;
      code: string | null;
      is_vanity: boolean;
      is_creator: boolean;
      commission_rate: number | null;
    }) => void;
  } = $props();

  let customerId = $state('');
  let vanityCode = $state('');
  let isVanity = $state(false);
  let isCreator = $state(false);
  let commissionRate = $state(0);
  let hasCommission = $derived(isCreator);
  let isSubmitting = $state(false);

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    if (!customerId || isSubmitting) return;
    isSubmitting = true;
    onCreateCode({
      customer_id: customerId,
      code: isVanity && vanityCode ? vanityCode : null,
      is_vanity: isVanity,
      is_creator: isCreator,
      commission_rate: isCreator ? commissionRate : null
    });
    isSubmitting = false;
    customerId = '';
    vanityCode = '';
    isVanity = false;
    isCreator = false;
    commissionRate = 0;
  }
</script>

<form class="form" onsubmit={handleSubmit}>
  <div class="form-field">
    <label class="label" for="rc-customer">Customer ID</label>
    <input
      id="rc-customer"
      class="input"
      type="text"
      bind:value={customerId}
      placeholder="Customer ID"
      required
    />
  </div>

  <div class="form-field">
    <label class="checkbox-label">
      <input type="checkbox" bind:checked={isVanity} />
      <span>Vanity code</span>
    </label>
    {#if isVanity}
      <input
        class="input"
        type="text"
        bind:value={vanityCode}
        placeholder="Custom code (e.g. JOHN10)"
        pattern="[A-Za-z0-9_-]+"
      />
    {/if}
  </div>

  <div class="form-field">
    <label class="checkbox-label">
      <input type="checkbox" bind:checked={isCreator} />
      <span>Creator / influencer code</span>
    </label>
    {#if hasCommission}
      <div class="commission-field">
        <label class="label" for="rc-commission">Commission rate (%)</label>
        <input
          id="rc-commission"
          class="input"
          type="number"
          min="0"
          max="100"
          step="0.1"
          bind:value={commissionRate}
        />
      </div>
    {/if}
  </div>

  <button class="btn btn-primary" type="submit" disabled={!customerId || isSubmitting}>
    Create Code
  </button>
</form>

<style>
  .form {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .form-field {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-text-muted);
  }

  .input {
    padding: var(--space-2) var(--space-3);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background: var(--color-bg);
    color: var(--color-text);
    font-size: var(--font-size-base);
    transition: border-color var(--transition-fast);
  }

  .input:focus {
    outline: none;
    border-color: var(--color-primary);
    box-shadow: 0 0 0 2px color-mix(in srgb, var(--color-primary) 20%, transparent);
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    font-size: var(--font-size-sm);
    color: var(--color-text);
    cursor: pointer;
  }

  .commission-field {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: var(--space-2) var(--space-4);
    border: none;
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    transition: background-color var(--transition-fast);
    align-self: flex-start;
  }

  .btn-primary {
    background: var(--color-primary);
    color: #ffffff;
  }

  .btn-primary:hover:not(:disabled) {
    background: var(--color-primary-hover);
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
