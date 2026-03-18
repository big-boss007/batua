<script lang="ts">
  import { Pill, Progress } from '@juspay/svelte-ui-components';
  import { formatCurrencyINR, formatDate } from '$lib/client/modules/foundation';
  import type { GiftCardInfo } from '../types';

  let { card }: { card: GiftCardInfo } = $props();

  let usedPercentage = $derived(
    card.initial_amount > 0
      ? ((card.initial_amount - card.current_amount) / card.initial_amount) * 100
      : 0
  );

  let remainingPercentage = $derived(100 - usedPercentage);

  let statusLabel = $derived.by(() => {
    if (!card.is_active) return 'Expired';
    if (card.is_claimed) return 'Claimed';
    if (card.current_amount <= 0) return 'Used';
    return 'Active';
  });

  let statusClass = $derived.by(() => {
    if (!card.is_active) return 'pill-error';
    if (card.is_claimed) return 'pill-info';
    if (card.current_amount <= 0) return 'pill-neutral';
    return 'pill-success';
  });

  let isExpired = $derived(
    card.expires_at !== null && new Date(card.expires_at) < new Date()
  );
</script>

<div class="gift-card">
  <div class="gift-card-header">
    <h3 class="gift-card-title">Gift Card</h3>
    <Pill text={statusLabel} classes={statusClass} />
  </div>

  <div class="gift-card-code-box">
    <span class="gift-card-code">{card.code}</span>
  </div>

  <div class="gift-card-amounts">
    <div class="amount-row">
      <span class="amount-label">Initial</span>
      <span class="amount-value">{formatCurrencyINR(card.initial_amount)}</span>
    </div>
    <div class="amount-row">
      <span class="amount-label">Remaining</span>
      <span class="amount-value amount-highlight">
        {formatCurrencyINR(card.current_amount)}
      </span>
    </div>
  </div>

  <div class="gift-card-progress">
    <Progress value={remainingPercentage} classes="gift-progress-bar" />
    <p class="progress-label">{Math.round(remainingPercentage)}% remaining</p>
  </div>

  {#if card.expires_at !== null}
    <p class="gift-card-expiry" class:expired={isExpired}>
      {isExpired ? 'Expired' : 'Expires'} {formatDate(card.expires_at)}
    </p>
  {/if}
</div>

<style>
  .gift-card {
    background: var(--color-surface);
    border-radius: var(--radius-lg);
    padding: var(--space-5);
    box-shadow: var(--shadow-sm);
  }

  .gift-card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: var(--space-4);
  }

  .gift-card-title {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .gift-card-code-box {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--space-3) var(--space-4);
    background: var(--color-surface-2);
    border-radius: var(--radius-md);
    margin-bottom: var(--space-4);
  }

  .gift-card-code {
    font-family: var(--font-mono);
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
    letter-spacing: 0.04em;
  }

  .gift-card-amounts {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    margin-bottom: var(--space-4);
  }

  .amount-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .amount-label {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .amount-value {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-medium);
    color: var(--color-text);
  }

  .amount-highlight {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-bold);
    color: var(--color-primary);
  }

  .gift-card-progress {
    margin-bottom: var(--space-3);
  }

  .gift-card-progress :global(.gift-progress-bar) {
    --progress-track-height: 8px;
  }

  .progress-label {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    margin-top: var(--space-1);
  }

  .gift-card-expiry {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .gift-card-expiry.expired {
    color: var(--color-error);
  }
</style>
