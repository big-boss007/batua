<script lang="ts">
  import { Input, Button, Toggle } from '@juspay/svelte-ui-components';

  import type { ReferralProgram } from '$lib/client/modules/referrals';

  let {
    program = null,
    onSave
  }: {
    program?: ReferralProgram | null;
    onSave: (data: Omit<ReferralProgram, 'id'>) => void;
  } = $props();

  let referrerReward = $state(String(program?.referrer_reward_amount ?? 0));
  let refereeReward = $state(String(program?.referee_reward_amount ?? 0));
  let maxReferrals = $state(String(program?.max_referrals_per_customer ?? 0));
  let hasMaxLimit = $state(program?.max_referrals_per_customer !== null);
  let isActive = $state(program?.is_active ?? true);
  let isSubmitting = $state(false);

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    if (isSubmitting) return;
    isSubmitting = true;
    onSave({
      referrer_reward_amount: Number(referrerReward),
      referee_reward_amount: Number(refereeReward),
      max_referrals_per_customer: hasMaxLimit ? Number(maxReferrals) : null,
      is_active: isActive,
      code_creation_trigger: program?.code_creation_trigger ?? 'on_registration'
    });
    isSubmitting = false;
  }
</script>

<form class="form" onsubmit={handleSubmit}>
  <div class="form-row">
    <Input
      value={referrerReward}
      label="Referrer Reward (★)"
      dataType="number"
      onInput={(val) => {
        referrerReward = val;
      }}
    />
    <Input
      value={refereeReward}
      label="Referee Reward (★)"
      dataType="number"
      onInput={(val) => {
        refereeReward = val;
      }}
    />
  </div>

  <div class="form-field">
    <Toggle
      text="Limit referrals per customer"
      checked={hasMaxLimit}
      onclick={(checked) => {
        hasMaxLimit = checked;
      }}
    />
    {#if hasMaxLimit}
      <Input
        value={maxReferrals}
        dataType="number"
        placeholder="Max referrals"
        onInput={(val) => {
          maxReferrals = val;
        }}
      />
    {/if}
  </div>

  <Toggle
    text="Program active"
    checked={isActive}
    onclick={(checked) => {
      isActive = checked;
    }}
  />

  <Button
    text={program ? 'Update Program' : 'Create Program'}
    classes="btn-primary"
    disabled={isSubmitting}
    type="submit"
  />
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
</style>
