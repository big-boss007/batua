<script lang="ts">
  import { Button } from '@juspay/svelte-ui-components';
  import { formatCurrencyINR } from '$lib/client/modules/foundation';
  import type {
    StorefrontMerchant,
    ReferralCodeInfo,
    ReferralProgramInfo
  } from '$lib/client/modules/storefront';

  let {
    data
  }: {
    data: {
      merchant: StorefrontMerchant | null;
      referralCode: ReferralCodeInfo | null;
      program: ReferralProgramInfo | null;
      code: string;
    };
  } = $props();

  let merchant = $derived(data.merchant);
  let referralCode = $derived(data.referralCode);
  let program = $derived(data.program);

  let rewardAmount = $derived(program !== null ? program.referee_reward_amount : 0);

  let shopUrl = $derived(
    merchant !== null && merchant.domain !== null ? `https://${merchant.domain}` : '#'
  );

  function handleShop() {
    if (shopUrl !== '#') {
      window.open(shopUrl, '_blank', 'noopener');
    }
  }
</script>

<div class="referral-landing">
  {#if referralCode !== null && merchant !== null}
    <div class="referral-hero">
      <div class="referral-gift-icon">&#x1F381;</div>
      <h2 class="referral-headline">You've been invited!</h2>
      {#if rewardAmount > 0}
        <p class="referral-reward">
          Get <strong class="reward-amount">{formatCurrencyINR(rewardAmount)}</strong>
          in rewards on your first order
        </p>
      {/if}
    </div>

    <div class="referral-code-display">
      <span class="referral-code-label">Your referral code</span>
      <span class="referral-code-value">{referralCode.code}</span>
    </div>

    <div class="referral-how">
      <h3 class="how-title">How it works</h3>
      <div class="how-steps">
        <div class="how-step">
          <span class="step-number">1</span>
          <span class="step-text">Shop at {merchant.name}</span>
        </div>
        <div class="how-step">
          <span class="step-number">2</span>
          <span class="step-text">
            Use code <strong>{referralCode.code}</strong> at checkout
          </span>
        </div>
        <div class="how-step">
          <span class="step-number">3</span>
          <span class="step-text">
            Get {formatCurrencyINR(rewardAmount)} credited to your wallet
          </span>
        </div>
      </div>
    </div>

    <div class="referral-cta">
      <Button text="Shop Now" onclick={handleShop} classes="btn-primary shop-now-btn" />
    </div>
  {:else}
    <div class="referral-not-found">
      <div class="not-found-icon">?</div>
      <h2 class="not-found-title">Referral code not found</h2>
      <p class="not-found-text">This referral link may have expired or is invalid.</p>
    </div>
  {/if}
</div>

<style>
  .referral-landing {
    display: flex;
    flex-direction: column;
    gap: var(--space-5);
  }

  .referral-hero {
    text-align: center;
    padding: var(--space-8) 0 var(--space-2);
  }

  .referral-gift-icon {
    font-size: 48px;
    line-height: 1;
    margin-bottom: var(--space-4);
  }

  .referral-headline {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    margin-bottom: var(--space-3);
  }

  .referral-reward {
    font-size: var(--font-size-md);
    color: var(--color-text-muted);
    line-height: var(--line-height-base);
  }

  .reward-amount {
    color: var(--color-success);
    font-weight: var(--font-weight-bold);
    font-size: var(--font-size-lg);
  }

  .referral-code-display {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-5);
    background: var(--color-surface);
    border: 2px dashed var(--color-primary);
    border-radius: var(--radius-lg);
  }

  .referral-code-label {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .referral-code-value {
    font-family: var(--font-mono);
    font-size: var(--font-size-3xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-primary);
    letter-spacing: 0.08em;
  }

  .referral-how {
    background: var(--color-surface);
    border-radius: var(--radius-lg);
    padding: var(--space-5);
    box-shadow: var(--shadow-sm);
  }

  .how-title {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
    margin-bottom: var(--space-4);
  }

  .how-steps {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .how-step {
    display: flex;
    align-items: center;
    gap: var(--space-3);
  }

  .step-number {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border-radius: var(--radius-full);
    background: var(--color-primary);
    color: #ffffff;
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-bold);
    flex-shrink: 0;
  }

  .step-text {
    font-size: var(--font-size-base);
    color: var(--color-text);
    line-height: var(--line-height-base);
  }

  .referral-cta {
    padding: var(--space-2) 0;
    --button-width: 100%;
    --button-height: 52px;
    --button-font-size: var(--font-size-lg);
    --button-font-weight: var(--font-weight-bold);
    --button-border-radius: var(--radius-lg);
  }

  .referral-not-found {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    padding: var(--space-16) var(--space-4);
    gap: var(--space-3);
  }

  .not-found-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 64px;
    height: 64px;
    border-radius: var(--radius-full);
    background: var(--color-surface-2);
    color: var(--color-text-muted);
    font-size: var(--font-size-3xl);
    font-weight: var(--font-weight-bold);
  }

  .not-found-title {
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .not-found-text {
    font-size: var(--font-size-base);
    color: var(--color-text-muted);
    max-width: 280px;
  }
</style>
