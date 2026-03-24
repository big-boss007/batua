<script lang="ts">
  import { Pill, Progress } from '@juspay/svelte-ui-components';

  import { formatCurrencyINR, formatDate } from '$lib/client/modules/foundation';
  import { claimGiftCardForCustomer } from '$lib/client/modules/storefront';
  import type { GiftCardInfo } from '../types';

  let {
    card,
    customerId = null,
    onClaimed = null
  }: {
    card: GiftCardInfo;
    customerId?: string | null;
    onClaimed?: (() => void) | null;
  } = $props();

  let claiming = $state(false);
  let claimSuccess = $state(false);
  let claimError = $state<string | null>(null);

  let remainingPercentage = $derived(
    card.initial_amount > 0
      ? ((card.current_amount / card.initial_amount) * 100)
      : 0
  );

  let isExpired = $derived(
    card.expires_at !== null && new Date(card.expires_at) < new Date()
  );

  let canClaim = $derived(
    !card.is_claimed && card.is_active && !isExpired && customerId !== null
  );

  let statusLabel = $derived.by(() => {
    if (!card.is_active || isExpired) return 'Expired';
    if (card.is_claimed) return 'Claimed';
    if (card.current_amount <= 0) return 'Used';
    return 'Active';
  });

  let statusClass = $derived.by(() => {
    if (!card.is_active || isExpired) return 'pill-error';
    if (card.is_claimed) return 'pill-info';
    if (card.current_amount <= 0) return 'pill-neutral';
    return 'pill-success';
  });

  async function handleClaim() {
    if (customerId === null || claiming) return;
    claiming = true;
    claimError = null;

    const result = await claimGiftCardForCustomer(card.code, customerId);
    if (result.tag === 'success') {
      claimSuccess = true;
    } else {
      claimError = result.message;
    }
    claiming = false;
  }
</script>

{#if claimSuccess}
  <div class="success-card">
    <div class="success-icon">&#10003;</div>
    <div class="success-title">Gift Card Claimed!</div>
    <div class="success-amount">+{formatCurrencyINR(card.current_amount)}</div>
    <div class="success-text">
      Added to your gift card wallet balance.<br />
      Use it at checkout on your next order.
    </div>
    {#if onClaimed !== null}
      <button class="done-btn" onclick={onClaimed}>Back to Rewards</button>
    {/if}
  </div>
{:else}
  <div class="gc-card">
    <div class="gc-header">
      <h3 class="gc-title">Gift Card</h3>
      <Pill text={statusLabel} classes={statusClass} />
    </div>

    <div class="gc-code-box">
      <span class="gc-code">{card.code}</span>
    </div>

    <div class="gc-amounts">
      <div class="gc-amount-item">
        <span class="gc-amount-label">Balance</span>
        <span class="gc-amount-value" class:balance={!isExpired}>
          {formatCurrencyINR(card.current_amount)}
        </span>
      </div>
      <div class="gc-amount-item">
        <span class="gc-amount-label">Initial</span>
        <span class="gc-amount-value">{formatCurrencyINR(card.initial_amount)}</span>
      </div>
    </div>

    <div class="gc-progress">
      <Progress value={remainingPercentage} classes={isExpired ? 'progress-expired' : 'progress-gc'} />
      <p class="progress-label">{Math.round(remainingPercentage)}% remaining</p>
    </div>

    {#if card.expires_at !== null}
      <p class="gc-expiry" class:expired={isExpired}>
        {isExpired ? 'Expired' : 'Expires'} {formatDate(card.expires_at)}
      </p>
    {/if}

    {#if canClaim}
      <div class="gc-actions">
        <button class="claim-btn" onclick={handleClaim} disabled={claiming}>
          {claiming ? 'Claiming...' : `Claim ${formatCurrencyINR(card.current_amount)} to Wallet`}
        </button>
        <div class="claim-hint">Amount will be added to your gift card balance</div>
      </div>
    {:else if card.is_claimed}
      <div class="claimed-info">Already in your wallet</div>
    {/if}

    {#if claimError !== null}
      <div class="claim-error">{claimError}</div>
    {/if}
  </div>
{/if}

<style>
  .gc-card {
    background: #1a1d27;
    border-radius: 12px;
    padding: 20px;
  }

  .gc-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 16px;
  }

  .gc-title {
    font-size: 16px;
    font-weight: 600;
    color: #ffffff;
  }

  .gc-code-box {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 10px 20px;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid #2a2d3a;
    border-radius: 10px;
    margin-bottom: 20px;
  }

  .gc-code {
    font-family: 'SF Mono', 'Fira Code', monospace;
    font-size: 18px;
    font-weight: 600;
    color: #ffffff;
    letter-spacing: 0.05em;
  }

  .gc-amounts {
    display: flex;
    justify-content: center;
    gap: 32px;
    margin-bottom: 16px;
  }

  .gc-amount-item {
    text-align: center;
  }

  .gc-amount-label {
    display: block;
    font-size: 11px;
    color: #9ca3af;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: 4px;
  }

  .gc-amount-value {
    font-size: 22px;
    font-weight: 700;
    color: #ffffff;
  }

  .gc-amount-value.balance {
    color: #4ade80;
  }

  .gc-progress {
    margin-bottom: 12px;
  }

  :global(.progress-gc) {
    --progress-track-height: 6px;
    --progress-bar-background: linear-gradient(90deg, #6366f1, #818cf8);
    --progress-track-background: #2a2d3a;
    --progress-track-border-radius: 3px;
    --progress-bar-border-radius: 3px;
  }

  :global(.progress-expired) {
    --progress-track-height: 6px;
    --progress-bar-background: #f87171;
    --progress-track-background: #2a2d3a;
    --progress-track-border-radius: 3px;
    --progress-bar-border-radius: 3px;
  }

  .progress-label {
    text-align: center;
    font-size: 11px;
    color: #9ca3af;
    margin-top: 6px;
  }

  .gc-expiry {
    font-size: 12px;
    color: #9ca3af;
    text-align: center;
    margin-bottom: 16px;
  }

  .gc-expiry.expired {
    color: #f87171;
  }

  .gc-actions {
    margin-top: 8px;
  }

  .claim-btn {
    width: 100%;
    padding: 14px;
    background: #4ade80;
    color: #000000;
    border: none;
    border-radius: 10px;
    font-size: 15px;
    font-weight: 700;
    cursor: pointer;
    transition: background 0.2s;
  }

  .claim-btn:hover:not(:disabled) {
    background: #22c55e;
  }

  .claim-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .claim-hint {
    text-align: center;
    font-size: 11px;
    color: #9ca3af;
    margin-top: 8px;
  }

  .claimed-info {
    text-align: center;
    font-size: 13px;
    color: #818cf8;
    font-weight: 500;
    padding: 8px 0;
  }

  .claim-error {
    text-align: center;
    font-size: 12px;
    color: #f87171;
    margin-top: 8px;
    padding: 8px;
    background: rgba(248, 113, 113, 0.1);
    border-radius: 8px;
  }

  /* Success state */
  .success-card {
    background: #1a1d27;
    border-radius: 12px;
    padding: 32px 20px;
    text-align: center;
  }

  .success-icon {
    width: 56px;
    height: 56px;
    border-radius: 50%;
    background: rgba(74, 222, 128, 0.12);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 28px;
    color: #4ade80;
    margin: 0 auto 16px;
  }

  .success-title {
    font-size: 20px;
    font-weight: 700;
    color: #ffffff;
    margin-bottom: 4px;
  }

  .success-amount {
    font-size: 32px;
    font-weight: 700;
    color: #4ade80;
    margin-bottom: 8px;
  }

  .success-text {
    font-size: 13px;
    color: #9ca3af;
    line-height: 1.5;
    margin-bottom: 24px;
  }

  .done-btn {
    width: 100%;
    padding: 12px;
    background: #6366f1;
    color: #ffffff;
    border: none;
    border-radius: 10px;
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.2s;
  }

  .done-btn:hover {
    background: #4f46e5;
  }
</style>
