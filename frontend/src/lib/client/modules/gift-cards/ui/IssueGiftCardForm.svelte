<script lang="ts">
  import { Input, Button } from '@juspay/svelte-ui-components';

  let { onIssue }: { onIssue: (amount: number, expiresAt: string | null) => void } = $props();

  let amount = $state('');
  let expiresAt = $state('');
  let isSubmitting = $state(false);

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    if (Number(amount) <= 0 || !expiresAt || isSubmitting) return;
    isSubmitting = true;
    onIssue(Number(amount), expiresAt);
    isSubmitting = false;
    amount = '';
    expiresAt = '';
  }
</script>

<form class="form" onsubmit={handleSubmit}>
  <Input
    value={amount}
    label="Amount (₹)"
    dataType="number"
    placeholder="e.g. 500"
    onInput={(val) => {
      amount = val;
    }}
  />

  <div class="form-field">
    <label class="label" for="gc-expires">Expiry date</label>
    <input id="gc-expires" class="date-input" type="date" bind:value={expiresAt} required />
  </div>

  <Button
    text="Issue gift card"
    classes="btn-primary"
    disabled={Number(amount) <= 0 || !expiresAt || isSubmitting}
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
    gap: var(--space-1);
  }

  .label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-text-muted);
  }

  .date-input {
    padding: var(--space-2) var(--space-3);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background: var(--color-bg);
    color: var(--color-text);
    font-size: var(--font-size-base);
    transition: border-color var(--transition-fast);
  }

  .date-input:focus {
    outline: none;
    border-color: var(--color-primary);
  }
</style>
