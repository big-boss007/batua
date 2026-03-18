<script lang="ts">
  import { Button } from '@juspay/svelte-ui-components';

  let { onSubmit }: { onSubmit: (phone: string) => void } = $props();

  let phoneValue = $state('');
  let isValid = $derived(phoneValue.replace(/\D/g, '').length === 10);

  function handleInput(event: Event) {
    const target = event.target as HTMLInputElement;
    phoneValue = target.value.replace(/\D/g, '').slice(0, 10);
  }

  function handleKeyDown(event: KeyboardEvent) {
    if (event.key === 'Enter' && isValid) {
      onSubmit(phoneValue);
    }
  }

  function handleSubmit() {
    if (isValid) {
      onSubmit(phoneValue);
    }
  }
</script>

<div class="phone-input-container">
  <p class="phone-input-label">Enter your phone number to view rewards</p>
  <div class="phone-input-row">
    <span class="country-code">+91</span>
    <input
      type="tel"
      class="phone-field"
      placeholder="9876543210"
      maxlength={10}
      inputmode="numeric"
      value={phoneValue}
      oninput={handleInput}
      onkeydown={handleKeyDown}
    />
  </div>
  <div class="phone-submit">
    <Button
      text="View Rewards"
      onclick={handleSubmit}
      disabled={!isValid}
      classes="btn-primary phone-submit-btn"
    />
  </div>
</div>

<style>
  .phone-input-container {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    padding: var(--space-6);
    background: var(--color-surface);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-sm);
  }

  .phone-input-label {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-medium);
    color: var(--color-text);
    text-align: center;
  }

  .phone-input-row {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .country-code {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 48px;
    padding: 0 var(--space-3);
    background: var(--color-surface-2);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-medium);
    color: var(--color-text-muted);
    white-space: nowrap;
  }

  .phone-field {
    flex: 1;
    height: 48px;
    padding: 0 var(--space-4);
    font-size: var(--font-size-lg);
    font-family: var(--font-sans);
    color: var(--color-text);
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    outline: none;
    transition: border-color var(--transition-fast);
    letter-spacing: 0.05em;
  }

  .phone-field:focus {
    border-color: var(--color-primary);
  }

  .phone-field::placeholder {
    color: var(--color-text-muted);
  }

  .phone-submit {
    --button-width: 100%;
    --button-height: 48px;
    --button-font-size: var(--font-size-md);
    --button-font-weight: var(--font-weight-semibold);
  }

  :global(.phone-submit-btn) {
    width: 100%;
  }
</style>
