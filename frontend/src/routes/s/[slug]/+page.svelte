<script lang="ts">
  import { Shimmer } from '@juspay/svelte-ui-components';
  import { normalizePhoneE164 } from '$lib/client/modules/foundation';
  import {
    customerPhone,
    merchantContext,
    lookupCustomer,
    lookupWallet,
    fetchBalance,
    fetchEntries,
    fetchCustomerTier,
    fetchCustomerReferralCode,
    fetchReferralProgram
  } from '$lib/client/modules/storefront';
  import type {
    StorefrontMerchant,
    CustomerBalance,
    CustomerTierInfo,
    TransactionEntry,
    ReferralCodeInfo,
    ReferralProgramInfo
  } from '$lib/client/modules/storefront';
  import {
    PhoneInput,
    BalanceCard,
    TierCard,
    TransactionList,
    ReferralCard
  } from '$lib/client/modules/storefront/ui';

  let { data }: { data: { merchant: StorefrontMerchant | null } } = $props();

  let merchant = $derived(data.merchant);
  let phone: string | null = $state(null);
  let loading = $state(false);
  let error: string | null = $state(null);

  let balance: CustomerBalance | null = $state(null);
  let tier: CustomerTierInfo | null = $state(null);
  let entries: Array<TransactionEntry> = $state([]);
  let referralCode: ReferralCodeInfo | null = $state(null);
  let referralProgram: ReferralProgramInfo | null = $state(null);

  customerPhone.subscribe((stored) => {
    if (stored !== null && phone === null) {
      phone = stored;
      if (merchant !== null) {
        loadCustomerData(stored);
      }
    }
  });

  async function handlePhoneSubmit(rawPhone: string) {
    phone = rawPhone;
    const normalized = normalizePhoneE164(rawPhone);
    customerPhone.set(normalized);
    await loadCustomerData(normalized);
  }

  async function loadCustomerData(phoneNumber: string) {
    if (merchant === null) return;

    loading = true;
    error = null;
    balance = null;
    tier = null;
    entries = [];
    referralCode = null;
    referralProgram = null;

    const customerResult = await lookupCustomer(phoneNumber);
    if (customerResult.tag === 'error' || customerResult.data.length === 0) {
      error = 'No rewards found for this phone number.';
      loading = false;
      return;
    }

    const customer = customerResult.data[0];
    const walletResult = await lookupWallet(merchant.id, customer.id);

    if (walletResult.tag === 'error') {
      error = 'Could not load wallet information.';
      loading = false;
      return;
    }

    const walletId = walletResult.data.id;
    if (walletId === '') {
      error = 'No wallet found for this account.';
      loading = false;
      return;
    }

    const [balanceResult, entriesResult, tierResult, refCodeResult, refProgramResult] =
      await Promise.all([
        fetchBalance(walletId),
        fetchEntries(walletId, 10),
        fetchCustomerTier(merchant.id, customer.id),
        fetchCustomerReferralCode(merchant.id, customer.id),
        fetchReferralProgram(merchant.id)
      ]);

    if (balanceResult.tag === 'success') {
      balance = balanceResult.data;
    }
    if (entriesResult.tag === 'success') {
      entries = entriesResult.data;
    }
    if (tierResult.tag === 'success') {
      tier = tierResult.data;
    }
    if (refCodeResult.tag === 'success') {
      referralCode = refCodeResult.data;
    }
    if (refProgramResult.tag === 'success') {
      referralProgram = refProgramResult.data;
    }

    loading = false;
  }

  function handleChangePhone() {
    phone = null;
    balance = null;
    tier = null;
    entries = [];
    referralCode = null;
    referralProgram = null;
    error = null;
    customerPhone.clear();
  }
</script>

<div class="loyalty-hub">
  {#if phone === null}
    <div class="hub-hero">
      <h2 class="hub-title">Your Rewards</h2>
      <p class="hub-subtitle">
        Check your balance, tier status, and recent activity
      </p>
    </div>
    <PhoneInput onSubmit={handlePhoneSubmit} />

    <div class="hub-quick-links">
      <a href="gift-cards/check" class="quick-link">
        <span class="quick-link-icon">&#x1F381;</span>
        <span class="quick-link-text">Check Gift Card</span>
      </a>
    </div>
  {:else if loading}
    <div class="loading-skeleton">
      <Shimmer classes="shimmer-balance" />
      <Shimmer classes="shimmer-tier" />
      <Shimmer classes="shimmer-tx" />
      <Shimmer classes="shimmer-tx" />
      <Shimmer classes="shimmer-tx" />
    </div>
  {:else if error !== null}
    <div class="hub-error">
      <div class="hub-error-icon">!</div>
      <p class="hub-error-text">{error}</p>
      <button class="hub-error-retry" onclick={handleChangePhone}>
        Try a different number
      </button>
    </div>
  {:else}
    <div class="hub-phone-bar">
      <span class="hub-phone-current">Showing rewards for your account</span>
      <button class="hub-phone-change" onclick={handleChangePhone}>Change</button>
    </div>

    {#if balance !== null}
      <section class="hub-section">
        <BalanceCard {balance} />
      </section>
    {/if}

    {#if tier !== null}
      <section class="hub-section">
        <TierCard {tier} />
      </section>
    {/if}

    {#if entries.length > 0}
      <section class="hub-section">
        <TransactionList {entries} />
      </section>
    {/if}

    {#if referralCode !== null && referralProgram !== null && merchant !== null}
      <section class="hub-section">
        <ReferralCard
          code={referralCode}
          referralReward={referralProgram.referrer_reward_amount}
          merchantName={merchant.name}
        />
      </section>
    {/if}

    <div class="hub-quick-links">
      <a href="balance" class="quick-link">
        <span class="quick-link-icon">&#x1F4B0;</span>
        <span class="quick-link-text">Balance Details</span>
      </a>
      <a href="gift-cards/check" class="quick-link">
        <span class="quick-link-icon">&#x1F381;</span>
        <span class="quick-link-text">Check Gift Card</span>
      </a>
      <a href="refer" class="quick-link">
        <span class="quick-link-icon">&#x1F91D;</span>
        <span class="quick-link-text">My Referrals</span>
      </a>
    </div>
  {/if}
</div>

<style>
  .loyalty-hub {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .hub-hero {
    text-align: center;
    padding: var(--space-6) 0 var(--space-2);
  }

  .hub-title {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    margin-bottom: var(--space-2);
  }

  .hub-subtitle {
    font-size: var(--font-size-base);
    color: var(--color-text-muted);
  }

  .hub-section {
    margin-bottom: var(--space-1);
  }

  .hub-phone-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-3) var(--space-4);
    background: var(--color-surface);
    border-radius: var(--radius-md);
    margin-bottom: var(--space-2);
  }

  .hub-phone-current {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .hub-phone-change {
    background: none;
    border: none;
    color: var(--color-primary);
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    padding: var(--space-1) var(--space-2);
    border-radius: var(--radius-sm);
    transition: background var(--transition-fast);
  }

  .hub-phone-change:hover {
    background: var(--color-surface-2);
  }

  .hub-quick-links {
    display: flex;
    gap: var(--space-3);
    flex-wrap: wrap;
    padding-top: var(--space-2);
  }

  .quick-link {
    flex: 1;
    min-width: 120px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-4);
    background: var(--color-surface);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-sm);
    text-decoration: none;
    color: var(--color-text);
    transition: box-shadow var(--transition-fast), transform var(--transition-fast);
  }

  .quick-link:hover {
    box-shadow: var(--shadow-md);
    transform: translateY(-1px);
    color: var(--color-text);
  }

  .quick-link-icon {
    font-size: 24px;
    line-height: 1;
  }

  .quick-link-text {
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-medium);
    text-align: center;
  }

  .hub-error {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    padding: var(--space-10) var(--space-4);
    gap: var(--space-3);
  }

  .hub-error-icon {
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

  .hub-error-text {
    font-size: var(--font-size-base);
    color: var(--color-text-muted);
  }

  .hub-error-retry {
    background: none;
    border: none;
    color: var(--color-primary);
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-medium);
    padding: var(--space-2) var(--space-4);
    border-radius: var(--radius-md);
    transition: background var(--transition-fast);
  }

  .hub-error-retry:hover {
    background: var(--color-surface);
  }

  .loading-skeleton {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    padding-top: var(--space-4);
  }

  :global(.shimmer-balance) {
    --shimmer-width: 100%;
    --shimmer-height: 180px;
    --shimmer-border-radius: 16px;
  }

  :global(.shimmer-tier) {
    --shimmer-width: 100%;
    --shimmer-height: 80px;
    --shimmer-border-radius: 12px;
  }

  :global(.shimmer-tx) {
    --shimmer-width: 100%;
    --shimmer-height: 60px;
    --shimmer-border-radius: 8px;
  }
</style>
