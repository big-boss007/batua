<script lang="ts">
  import { Shimmer } from '@juspay/svelte-ui-components';
  import { normalizePhoneE164 } from '$lib/client/modules/foundation';
  import {
    customerPhone,
    lookupCustomer,
    fetchCustomerReferralCode,
    fetchReferralProgram
  } from '$lib/client/modules/storefront';
  import type {
    StorefrontMerchant,
    ReferralCodeInfo,
    ReferralProgramInfo
  } from '$lib/client/modules/storefront';
  import { PhoneInput, ReferralCard } from '$lib/client/modules/storefront/ui';

  let { data }: { data: { merchant: StorefrontMerchant | null } } = $props();

  let merchant = $derived(data.merchant);
  let phone: string | null = $state(null);
  let loading = $state(false);
  let error: string | null = $state(null);
  let referralCode: ReferralCodeInfo | null = $state(null);
  let referralProgram: ReferralProgramInfo | null = $state(null);

  customerPhone.subscribe((stored) => {
    if (stored !== null && phone === null) {
      phone = stored;
      if (merchant !== null) {
        loadReferralData(stored);
      }
    }
  });

  async function handlePhoneSubmit(rawPhone: string) {
    phone = rawPhone;
    const normalized = normalizePhoneE164(rawPhone);
    customerPhone.set(normalized);
    await loadReferralData(normalized);
  }

  async function loadReferralData(phoneNumber: string) {
    if (merchant === null) return;

    loading = true;
    error = null;
    referralCode = null;
    referralProgram = null;

    const customerResult = await lookupCustomer(phoneNumber);
    if (customerResult.tag === 'error' || customerResult.data.length === 0) {
      error = 'No account found for this phone number.';
      loading = false;
      return;
    }

    const customer = customerResult.data[0];
    const [codeResult, programResult] = await Promise.all([
      fetchCustomerReferralCode(merchant.id, customer.id),
      fetchReferralProgram(merchant.id)
    ]);

    if (codeResult.tag === 'success') {
      referralCode = codeResult.data;
    }
    if (programResult.tag === 'success') {
      referralProgram = programResult.data;
    }

    if (referralCode === null) {
      error = "You don't have a referral code yet. Make your first purchase to get one!";
    }

    loading = false;
  }

  function handleChangePhone() {
    phone = null;
    referralCode = null;
    referralProgram = null;
    error = null;
    customerPhone.clear();
  }
</script>

<div class="my-referrals">
  {#if phone === null}
    <div class="referral-hero">
      <h2 class="referral-title">My Referrals</h2>
      <p class="referral-subtitle">Enter your phone number to view your referral code and stats</p>
    </div>
    <PhoneInput onSubmit={handlePhoneSubmit} />
  {:else if loading}
    <div class="loading-skeleton">
      <div class="skeleton-card">
        <Shimmer />
      </div>
    </div>
  {:else if error !== null && referralCode === null}
    <div class="referral-empty">
      <div class="empty-icon">&#x1F91D;</div>
      <p class="empty-text">{error}</p>
      <button class="empty-retry" onclick={handleChangePhone}> Try a different number </button>
    </div>
  {:else if referralCode !== null && merchant !== null}
    <div class="referral-phone-bar">
      <span class="phone-label">Your referral program</span>
      <button class="phone-change" onclick={handleChangePhone}>Change number</button>
    </div>
    <ReferralCard
      code={referralCode}
      referralReward={referralProgram !== null ? referralProgram.referrer_reward_amount : 0}
      merchantName={merchant.name}
    />
  {/if}
</div>

<style>
  .my-referrals {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .referral-hero {
    text-align: center;
    padding: var(--space-6) 0 var(--space-2);
  }

  .referral-title {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    margin-bottom: var(--space-2);
  }

  .referral-subtitle {
    font-size: var(--font-size-base);
    color: var(--color-text-muted);
  }

  .referral-phone-bar {
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

  .referral-empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    padding: var(--space-10) var(--space-4);
    gap: var(--space-3);
  }

  .empty-icon {
    font-size: 48px;
    line-height: 1;
    margin-bottom: var(--space-2);
  }

  .empty-text {
    font-size: var(--font-size-base);
    color: var(--color-text-muted);
    line-height: var(--line-height-base);
    max-width: 280px;
  }

  .empty-retry {
    background: none;
    border: none;
    color: var(--color-primary);
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-medium);
    padding: var(--space-2) var(--space-4);
    border-radius: var(--radius-md);
    transition: background var(--transition-fast);
  }

  .empty-retry:hover {
    background: var(--color-surface);
  }

  .loading-skeleton {
    padding-top: var(--space-4);
  }

  .skeleton-card {
    --shimmer-height: 280px;
    --shimmer-border-radius: var(--radius-lg);
    --shimmer-background: var(--color-surface-2);
  }
</style>
