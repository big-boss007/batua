<script lang="ts">
  import { Shimmer } from '@juspay/svelte-ui-components';
  import { normalizePhoneE164 } from '$lib/client/modules/foundation';
  import {
    customerPhone,
    lookupCustomer,
    lookupWallet,
    fetchBalance
  } from '$lib/client/modules/storefront';
  import type { StorefrontMerchant, CustomerBalance } from '$lib/client/modules/storefront';
  import { PhoneInput, BalanceCard } from '$lib/client/modules/storefront/ui';

  let { data }: { data: { merchant: StorefrontMerchant | null } } = $props();

  let merchant = $derived(data.merchant);
  let phone: string | null = $state(null);
  let loading = $state(false);
  let error: string | null = $state(null);
  let balance: CustomerBalance | null = $state(null);

  customerPhone.subscribe((stored) => {
    if (stored !== null && phone === null) {
      phone = stored;
      if (merchant !== null) {
        loadBalance(stored);
      }
    }
  });

  async function handlePhoneSubmit(rawPhone: string) {
    phone = rawPhone;
    const normalized = normalizePhoneE164(rawPhone);
    customerPhone.set(normalized);
    await loadBalance(normalized);
  }

  async function loadBalance(phoneNumber: string) {
    if (merchant === null) return;

    loading = true;
    error = null;
    balance = null;

    const customerResult = await lookupCustomer(phoneNumber);
    if (customerResult.tag === 'error' || customerResult.data.length === 0) {
      error = 'No rewards found for this phone number.';
      loading = false;
      return;
    }

    const customer = customerResult.data[0];
    const walletResult = await lookupWallet(merchant.id, customer.id);
    if (walletResult.tag === 'error' || walletResult.data.id === '') {
      error = 'No wallet found for this account.';
      loading = false;
      return;
    }

    const balanceResult = await fetchBalance(walletResult.data.id);
    if (balanceResult.tag === 'success') {
      balance = balanceResult.data;
    } else {
      error = 'Could not load balance.';
    }

    loading = false;
  }

  function handleChangePhone() {
    phone = null;
    balance = null;
    error = null;
    customerPhone.clear();
  }
</script>

<div class="balance-page">
  {#if phone === null}
    <div class="balance-hero">
      <h2 class="balance-title">Check Your Balance</h2>
      <p class="balance-subtitle">Enter your phone number to view your wallet balance</p>
    </div>
    <PhoneInput onSubmit={handlePhoneSubmit} />
  {:else if loading}
    <div class="loading-skeleton">
      <div class="skeleton-card">
        <Shimmer />
      </div>
    </div>
  {:else if error !== null}
    <div class="balance-error">
      <div class="error-icon">!</div>
      <p class="error-text">{error}</p>
      <button class="error-retry" onclick={handleChangePhone}> Try a different number </button>
    </div>
  {:else if balance !== null}
    <div class="balance-phone-bar">
      <span class="phone-label">Your balance</span>
      <button class="phone-change" onclick={handleChangePhone}>Change number</button>
    </div>
    <BalanceCard {balance} />
    <div class="balance-footer">
      <a href="." class="view-full-link">View full rewards dashboard</a>
    </div>
  {/if}
</div>

<style>
  .balance-page {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .balance-hero {
    text-align: center;
    padding: var(--space-6) 0 var(--space-2);
  }

  .balance-title {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    margin-bottom: var(--space-2);
  }

  .balance-subtitle {
    font-size: var(--font-size-base);
    color: var(--color-text-muted);
  }

  .balance-phone-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .phone-label {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .phone-change {
    background: none;
    border: none;
    color: var(--color-primary);
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    padding: var(--space-1) var(--space-2);
    border-radius: var(--radius-sm);
    transition: background var(--transition-fast);
  }

  .phone-change:hover {
    background: var(--color-surface);
  }

  .balance-footer {
    text-align: center;
    padding: var(--space-4) 0;
  }

  .view-full-link {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-medium);
    color: var(--color-primary);
  }

  .view-full-link:hover {
    color: var(--color-primary-hover);
  }

  .balance-error {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    padding: var(--space-10) var(--space-4);
    gap: var(--space-3);
  }

  .error-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 48px;
    height: 48px;
    border-radius: var(--radius-full);
    background: #fef2f2;
    color: var(--color-error);
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-bold);
  }

  .error-text {
    font-size: var(--font-size-base);
    color: var(--color-text-muted);
  }

  .error-retry {
    background: none;
    border: none;
    color: var(--color-primary);
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-medium);
    padding: var(--space-2) var(--space-4);
    border-radius: var(--radius-md);
    transition: background var(--transition-fast);
  }

  .error-retry:hover {
    background: var(--color-surface);
  }

  .loading-skeleton {
    padding-top: var(--space-4);
  }

  .skeleton-card {
    --shimmer-height: 180px;
    --shimmer-border-radius: var(--radius-lg);
    --shimmer-background: var(--color-surface-2);
  }
</style>
