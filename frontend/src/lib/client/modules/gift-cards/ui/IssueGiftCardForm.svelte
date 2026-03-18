<script lang="ts">
  let { onIssue }: { onIssue: (amount: number, expiresAt: string | null) => void } = $props();

  let amount = $state(0);
  let expiresAt = $state('');
  let isSubmitting = $state(false);

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    if (amount <= 0 || isSubmitting) return;
    isSubmitting = true;
    onIssue(amount, expiresAt || null);
    isSubmitting = false;
    amount = 0;
    expiresAt = '';
  }
</script>

<form class="form" onsubmit={handleSubmit}>
  <div class="form-field">
    <label class="label" for="gc-amount">Amount (₹)</label>
    <input
      id="gc-amount"
      class="input"
      type="number"
      min="1"
      bind:value={amount}
      placeholder="e.g. 10000 for 100.00"
      required
    />
  </div>

  <div class="form-field">
    <label class="label" for="gc-expires">Expiry Date (optional)</label>
    <input id="gc-expires" class="input" type="date" bind:value={expiresAt} />
  </div>

  <button class="btn btn-primary" type="submit" disabled={amount <= 0 || isSubmitting}>
    Issue Gift Card
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
    gap: var(--space-1);
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
