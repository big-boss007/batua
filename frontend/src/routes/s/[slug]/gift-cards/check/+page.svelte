<script lang="ts">
  import { goto } from '$app/navigation';

  import { Input, Button, Shimmer } from '@juspay/svelte-ui-components';

  import {
    fetchGiftCard,
    customerPhone,
    lookupCustomer
  } from '$lib/client/modules/storefront';
  import type { GiftCardInfo } from '$lib/client/modules/storefront';
  import { normalizePhoneE164 } from '$lib/client/modules/foundation';
  import { GiftCardStatus } from '$lib/client/modules/storefront/ui';

  let { data }: { data: { prefilledCode: string | null } } = $props();

  let codeValue = $state(data.prefilledCode ?? '');
  let loading = $state(false);
  let error: string | null = $state(null);
  let card: GiftCardInfo | null = $state(null);
  let isValid = $state(false);
  let customerId: string | null = $state(null);

  customerPhone.subscribe((stored) => {
    if (stored !== null && customerId === null) {
      resolveCustomerId(stored);
    }
  });

  async function resolveCustomerId(phone: string) {
    const result = await lookupCustomer(phone);
    if (result.tag === 'success' && result.data.length > 0) {
      customerId = result.data[0].id;
    }
  }

  function handleStateChange(state: string) {
    isValid = state === 'Valid' || codeValue.trim().length >= 4;
  }

  function handleKeyDown(event: KeyboardEvent) {
    if (event.key === 'Enter' && codeValue.trim().length >= 4) {
      handleCheck();
    }
  }

  async function handleCheck() {
    const code = codeValue.trim();
    if (code.length < 4) return;

    loading = true;
    error = null;
    card = null;

    const result = await fetchGiftCard(code);
    if (result.tag === 'success') {
      card = result.data;
    } else {
      error = 'Gift card not found. Please check the code and try again.';
    }

    loading = false;
  }

  function handleReset() {
    codeValue = '';
    card = null;
    error = null;
  }

  function handleClaimed() {
    goto('../');
  }
</script>

<div class="gift-check-page">
  <div class="gift-check-hero">
    <h2 class="gift-check-title">Check Gift Card</h2>
    <p class="gift-check-subtitle">Enter your gift card code to check the balance</p>
  </div>

  {#if card === null}
    <div class="gift-check-form">
      <div class="gift-code-input">
        <Input
          bind:value={codeValue}
          placeholder="BRZE-XXXX-XXXX-XXXX"
          label="Gift Card Code"
          maxLength={30}
          onStateChange={handleStateChange}
          onKeyDown={handleKeyDown}
          classes="gift-input-field"
        />
      </div>
      <div class="gift-check-submit">
        <Button
          text={loading ? 'Checking...' : 'Check Balance'}
          onclick={handleCheck}
          disabled={codeValue.trim().length < 4 || loading}
          showLoader={loading}
          loaderType="Circular"
          classes="btn-primary gift-check-btn"
        />
      </div>

      {#if error !== null}
        <div class="gift-check-error">
          <p class="gift-error-text">{error}</p>
        </div>
      {/if}
    </div>
  {:else}
    <GiftCardStatus {card} {customerId} onClaimed={handleClaimed} />
    <div class="gift-check-actions">
      <button class="check-another" onclick={handleReset}> Check another card </button>
    </div>
  {/if}

  {#if loading && card === null && error === null}
    <div class="loading-skeleton">
      <Shimmer />
    </div>
  {/if}
</div>

<style>
  .gift-check-page {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .gift-check-hero {
    text-align: center;
    padding: var(--space-6) 0 var(--space-2);
  }

  .gift-check-title {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    margin-bottom: var(--space-2);
  }

  .gift-check-subtitle {
    font-size: var(--font-size-base);
    color: var(--color-text-muted);
  }

  .gift-check-form {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    padding: var(--space-6);
    background: var(--color-surface);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-sm);
  }

  .gift-code-input {
    --input-font-size: var(--font-size-md);
    --input-height: 48px;
    --input-margin: 0;
    --input-width: 100%;
    --input-text-align: center;
  }

  .gift-check-submit {
    --button-width: 100%;
    --button-height: 48px;
    --button-font-size: var(--font-size-md);
    --button-font-weight: var(--font-weight-semibold);
  }

  .gift-check-error {
    text-align: center;
    padding: var(--space-3);
    background: #fef2f2;
    border-radius: var(--radius-md);
  }

  :global([data-theme='dark']) .gift-check-error {
    background: #450a0a;
  }

  .gift-error-text {
    font-size: var(--font-size-sm);
    color: var(--color-error);
  }

  .gift-check-actions {
    text-align: center;
    padding: var(--space-4) 0;
  }

  .check-another {
    background: none;
    border: none;
    color: var(--color-primary);
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-medium);
    padding: var(--space-2) var(--space-4);
    border-radius: var(--radius-md);
    transition: background var(--transition-fast);
  }

  .check-another:hover {
    background: var(--color-surface);
  }

  .loading-skeleton {
    --shimmer-height: 200px;
    --shimmer-border-radius: var(--radius-lg);
    --shimmer-background: var(--color-surface-2);
  }
</style>
