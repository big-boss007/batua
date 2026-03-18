<script lang="ts">
  import type { ReferralProgram } from '$lib/client/modules/referrals';

  let {
    program = null,
    onSave
  }: {
    program?: ReferralProgram | null;
    onSave: (data: Omit<ReferralProgram, 'id'>) => void;
  } = $props();

  let referrerReward = $state(program?.referrer_reward_amount ?? 0);
  let refereeReward = $state(program?.referee_reward_amount ?? 0);
  let maxReferrals = $state(program?.max_referrals_per_customer ?? 0);
  let hasMaxLimit = $state(program?.max_referrals_per_customer !== null);
  let isActive = $state(program?.is_active ?? true);
  let isSubmitting = $state(false);

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    if (isSubmitting) return;
    isSubmitting = true;
    onSave({
      referrer_reward_amount: referrerReward,
      referee_reward_amount: refereeReward,
      max_referrals_per_customer: hasMaxLimit ? maxReferrals : null,
      is_active: isActive
    });
    isSubmitting = false;
  }
</script>

<form class="form" onsubmit={handleSubmit}>
  <div class="form-row">
    <div class="form-field">
      <label class="label" for="rp-referrer">Referrer Reward (₹)</label>
      <input
        id="rp-referrer"
        class="input"
        type="number"
        min="0"
        bind:value={referrerReward}
        required
      />
    </div>
    <div class="form-field">
      <label class="label" for="rp-referee">Referee Reward (₹)</label>
      <input
        id="rp-referee"
        class="input"
        type="number"
        min="0"
        bind:value={refereeReward}
        required
      />
    </div>
  </div>

  <div class="form-field">
    <label class="checkbox-label">
      <input type="checkbox" bind:checked={hasMaxLimit} />
      <span>Limit referrals per customer</span>
    </label>
    {#if hasMaxLimit}
      <input
        class="input"
        type="number"
        min="1"
        bind:value={maxReferrals}
        placeholder="Max referrals"
      />
    {/if}
  </div>

  <div class="form-field">
    <label class="checkbox-label">
      <input type="checkbox" bind:checked={isActive} />
      <span>Program active</span>
    </label>
  </div>

  <button class="btn btn-primary" type="submit" disabled={isSubmitting}>
    {program ? 'Update Program' : 'Create Program'}
  </button>
</form>

<style>
  .form {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
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
