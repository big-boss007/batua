<script lang="ts">
  import { Input, Button, Toggle } from '@juspay/svelte-ui-components';

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
  let commissionRate = $state('0');
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
      commission_rate: isCreator ? Number(commissionRate) : null
    });
    isSubmitting = false;
    customerId = '';
    vanityCode = '';
    isVanity = false;
    isCreator = false;
    commissionRate = '0';
  }
</script>

<form class="form" onsubmit={handleSubmit}>
  <Input
    value={customerId}
    label="Customer ID"
    placeholder="Customer ID"
    onInput={(val) => {
      customerId = val;
    }}
  />

  <div class="form-field">
    <Toggle
      text="Vanity code"
      checked={isVanity}
      onclick={(checked) => {
        isVanity = checked;
      }}
    />
    {#if isVanity}
      <Input
        value={vanityCode}
        placeholder="Custom code (e.g. JOHN10)"
        onInput={(val) => {
          vanityCode = val;
        }}
      />
    {/if}
  </div>

  <div class="form-field">
    <Toggle
      text="Creator / influencer code"
      checked={isCreator}
      onclick={(checked) => {
        isCreator = checked;
      }}
    />
    {#if hasCommission}
      <Input
        value={commissionRate}
        label="Commission rate (%)"
        dataType="number"
        onInput={(val) => {
          commissionRate = val;
        }}
      />
    {/if}
  </div>

  <Button
    text="Create code"
    classes="btn-primary"
    disabled={!customerId || isSubmitting}
    type="submit"
  />
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
</style>
