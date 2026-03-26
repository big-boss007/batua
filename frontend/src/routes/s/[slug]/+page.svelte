<script lang="ts">
  import { page } from '$app/stores';
  import { Shimmer } from '@juspay/svelte-ui-components';
  import { normalizePhoneE164, formatCurrencyINR } from '$lib/client/modules/foundation';
  import {
    customerPhone,
    merchantContext,
    lookupCustomer,
    lookupWallet,
    fetchBalance,
    fetchEntries,
    fetchCustomerTier,
    fetchCustomerReferralCode,
    fetchReferralProgram,
    computeRunningBalances
  } from '$lib/client/modules/storefront';
  import type {
    StorefrontMerchant,
    CustomerBalance,
    CustomerTierInfo,
    TransactionEntry,
    ReferralCodeInfo,
    ReferralProgramInfo
  } from '$lib/client/modules/storefront';
  import { getMembershipStatus } from '$lib/client/modules/memberships';
  import type { MembershipStatus } from '$lib/client/modules/memberships';
  import {
    PhoneInput,
    BalanceCard,
    TierCard,
    TransactionList,
    ReferralCard,
    ProfileBar
  } from '$lib/client/modules/storefront/ui';

  let { data }: { data: { merchant: StorefrontMerchant | null } } = $props();

  let merchant = $derived(data.merchant);
  let basePath = $derived($page.url.pathname.replace(/\/$/, ''));
  let phone: string | null = $state(null);
  let loading = $state(false);
  let error: string | null = $state(null);

  let customerName: string | null = $state(null);
  let balance: CustomerBalance | null = $state(null);
  let tier: CustomerTierInfo | null = $state(null);
  let entries: Array<TransactionEntry> = $state([]);
  let referralCode: ReferralCodeInfo | null = $state(null);
  let referralProgram: ReferralProgramInfo | null = $state(null);
  let membershipStatus: MembershipStatus | null = $state(null);

  let runningBalances = $derived.by(() => {
    const b = balance;
    if (b === null || entries.length === 0) return null;
    return computeRunningBalances(entries, b.spendable_balance);
  });

  let lifetimeSaved = $derived(
    entries
      .filter((e) => e.movement_type === 'In' || e.movement_type === 'Released')
      .reduce((sum, e) => sum + Math.abs(e.currency_equivalent), 0)
  );

  let effectiveTier = $derived.by(() => {
    const loyaltyMult = tier?.earn_rate_multiplier ?? 1.0;
    const ms = membershipStatus;
    const membershipMult = ms !== null && ms.is_active ? ms.earn_rate_multiplier : 1.0;

    if (ms !== null && ms.is_active && membershipMult >= loyaltyMult && ms.tier_name !== null) {
      return {
        name: ms.tier_name,
        isMember: true,
        daysRemaining: ms.days_remaining
      };
    }

    return {
      name: tier?.tier_name ?? null,
      isMember: false,
      daysRemaining: null as number | null
    };
  });

  let effectiveMultiplier = $derived.by(() => {
    const loyaltyMult = tier?.earn_rate_multiplier ?? 1.0;
    const ms = membershipStatus;
    const membershipMult = ms !== null && ms.is_active ? ms.earn_rate_multiplier : 1.0;
    return Math.max(loyaltyMult, membershipMult);
  });

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
    customerName = null;
    balance = null;
    tier = null;
    entries = [];
    referralCode = null;
    referralProgram = null;
    membershipStatus = null;

    const customerResult = await lookupCustomer(phoneNumber);
    if (customerResult.tag === 'error' || customerResult.data.length === 0) {
      error = 'No rewards found for this phone number.';
      loading = false;
      return;
    }

    const customer = customerResult.data[0];
    customerName = customer.name;
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

    const [balanceResult, entriesResult, tierResult, refCodeResult, refProgramResult, membershipResult] =
      await Promise.all([
        fetchBalance(walletId),
        fetchEntries(walletId, 10),
        fetchCustomerTier(merchant.id, customer.id),
        fetchCustomerReferralCode(merchant.id, customer.id),
        fetchReferralProgram(merchant.id),
        getMembershipStatus(merchant.id, customer.id)
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
    if (membershipResult.tag === 'success') {
      membershipStatus = membershipResult.data;
    }

    loading = false;
  }

  function handleChangePhone() {
    phone = null;
    customerName = null;
    balance = null;
    tier = null;
    entries = [];
    referralCode = null;
    referralProgram = null;
    membershipStatus = null;
    error = null;
    customerPhone.clear();
  }
</script>

<div class="loyalty-hub">
  {#if phone === null}
    <div class="hub-hero">
      <h2 class="hub-title">Your Rewards</h2>
      <p class="hub-subtitle">Check your balance, tier status, and recent activity</p>
    </div>
    <PhoneInput onSubmit={handlePhoneSubmit} />

    <div class="hub-quick-links">
      <a href="{basePath}/gift-cards/check" class="quick-link">
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
      <button class="hub-error-retry" onclick={handleChangePhone}> Try a different number </button>
    </div>
  {:else}
    <section class="section">
      {#if customerName !== null}
        <ProfileBar
          name={customerName}
          tierName={effectiveTier.name}
          isMember={effectiveTier.isMember}
          membershipDaysRemaining={effectiveTier.daysRemaining}
          lifetimeSaved={lifetimeSaved > 0 ? lifetimeSaved : null}
          onSwitch={handleChangePhone}
        />
      {:else}
        <div class="hub-switch-bar">
          <button class="hub-switch-btn" onclick={handleChangePhone}>Switch account</button>
        </div>
      {/if}
    </section>

    <div class="multiplier-strip">
      <span class="multiplier-icon">{effectiveMultiplier > 1 ? '⚡' : '✦'}</span>
      <span class="multiplier-text">
        {#if effectiveMultiplier > 1}
          You're earning <span class="multiplier-value">{effectiveMultiplier}x</span> {merchant?.points_name ?? 'rewards'} on every order
        {:else}
          Earning <span class="multiplier-value">1x</span> {merchant?.points_name ?? 'rewards'} on every order
        {/if}
      </span>
    </div>

    {#if balance !== null && merchant !== null}
      <div class="divider"></div>
      <section class="section">
        <BalanceCard {balance} {merchant} />
      </section>
      {#if lifetimeSaved > 0}
        <div class="lifetime-strip">
          <span class="lifetime-label">Lifetime Saved</span>
          <span class="lifetime-value">{formatCurrencyINR(lifetimeSaved)}</span>
        </div>
      {/if}
    {/if}

    {#if tier !== null}
      <div class="divider"></div>
      <section class="section">
        <TierCard
          {tier}
          isMember={effectiveTier.isMember}
          effectiveTierName={effectiveTier.isMember ? effectiveTier.name : null}
          membershipDaysRemaining={effectiveTier.daysRemaining}
          pointsIcon={merchant?.points_icon ?? 'pts'}
        />
      </section>
    {/if}

    {#if entries.length > 0}
      <div class="divider"></div>
      <section class="section">
        {#if merchant !== null}
          <TransactionList {entries} {runningBalances} {merchant} />
        {/if}
      </section>
    {/if}

    {#if referralCode !== null && referralProgram !== null && merchant !== null}
      <div class="divider"></div>
      <section class="section">
        <ReferralCard
          code={referralCode}
          referralReward={referralProgram.referrer_reward_amount}
          merchantName={merchant.name}
          pointsIcon={merchant.points_icon}
        />
      </section>
    {/if}

    <div class="divider"></div>
    <div class="bottom-actions">
      <a href="{basePath}/gift-cards/check" class="bottom-action">Gift Card</a>
      <a href="{basePath}/refer" class="bottom-action bottom-action-primary">Refer & Earn</a>
    </div>
  {/if}
</div>

<style>
  .loyalty-hub {
    display: flex;
    flex-direction: column;
  }

  .section {
    padding: 20px 24px;
  }

  .multiplier-strip {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 12px 24px;
    background: linear-gradient(90deg, rgba(99, 102, 241, 0.12), rgba(129, 140, 248, 0.06));
    border-top: 1px solid #2a2d3a;
    border-bottom: 1px solid #2a2d3a;
  }

  .multiplier-icon {
    font-size: 14px;
    line-height: 1;
  }

  .multiplier-text {
    font-size: 13px;
    font-weight: 600;
    color: #818cf8;
  }

  .multiplier-value {
    font-weight: 700;
    color: #4ade80;
  }

  .divider {
    height: 1px;
    background: #2a2d3a;
    margin: 0;
  }

  .lifetime-strip {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 24px;
    background: rgba(99, 102, 241, 0.06);
    border-top: 1px solid #2a2d3a;
    border-bottom: 1px solid #2a2d3a;
  }

  .lifetime-label {
    font-size: 13px;
    color: #9ca3af;
  }

  .lifetime-value {
    font-size: 16px;
    font-weight: 700;
    color: #818cf8;
  }

  .hub-hero {
    text-align: center;
    padding: 28px 24px 12px;
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

  .hub-switch-bar {
    display: flex;
    justify-content: flex-end;
  }

  .hub-switch-btn {
    background: none;
    border: none;
    color: var(--color-primary);
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-medium);
    padding: var(--space-1) var(--space-2);
    border-radius: var(--radius-sm);
    cursor: pointer;
    transition: background var(--transition-fast);
  }

  .hub-switch-btn:hover {
    background: var(--color-surface-2);
  }

  .hub-quick-links {
    display: flex;
    gap: var(--space-3);
    flex-wrap: wrap;
    padding: 12px 24px 24px;
  }

  .quick-link {
    flex: 1;
    min-width: 120px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-4);
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    text-decoration: none;
    color: var(--color-text);
    transition: background var(--transition-fast);
  }

  .quick-link:hover {
    background: rgba(255, 255, 255, 0.08);
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

  .bottom-actions {
    display: flex;
    gap: var(--space-2);
    padding: 20px 24px;
  }

  .bottom-action {
    flex: 1;
    padding: var(--space-4);
    text-align: center;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-medium);
    color: var(--color-text-muted);
    text-decoration: none;
    transition:
      background var(--transition-fast),
      border-color var(--transition-fast);
  }

  .bottom-action:hover {
    background: var(--color-surface);
    color: var(--color-text);
  }

  .bottom-action-primary {
    background: var(--color-primary);
    border-color: var(--color-primary);
    color: #ffffff;
  }

  .bottom-action-primary:hover {
    background: var(--color-primary-hover);
    color: #ffffff;
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
    cursor: pointer;
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
