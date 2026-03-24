<script lang="ts">
  import { Button } from '@juspay/svelte-ui-components';

  import type { GiftCard } from '$lib/client/modules/gift-cards';
  import { formatCurrencyINR, formatDate } from '$lib/client/modules/foundation';

  let {
    amount,
    expiresAt,
    merchantName,
    onConfirm,
    onCancel
  }: {
    amount: number;
    expiresAt: string | null;
    merchantName: string;
    onConfirm: () => Promise<GiftCard | null>;
    onCancel: () => void;
  } = $props();

  let issuedCard = $state<GiftCard | null>(null);
  let issuing = $state(false);
  let codeRevealed = $state(false);
  let codeCopied = $state(false);
  let maskTimer: ReturnType<typeof setTimeout> | null = null;

  let amountFormatted = $derived(formatCurrencyINR(amount).replace('₹', ''));

  let displayCode = $derived.by(() => {
    if (issuedCard === null) return '****-****-****-****';
    if (codeRevealed) return issuedCard.code;
    return `****-****-****-${issuedCard.code.slice(-4)}`;
  });

  async function handleConfirm() {
    issuing = true;
    const card = await onConfirm();
    if (card !== null) {
      issuedCard = card;
    }
    issuing = false;
  }

  function handleReveal() {
    if (issuedCard === null) return;
    codeRevealed = true;
    codeCopied = false;
    if (maskTimer !== null) clearTimeout(maskTimer);
    maskTimer = setTimeout(() => {
      codeRevealed = false;
      codeCopied = false;
    }, 5000);
  }

  async function handleCopy() {
    if (issuedCard === null) return;
    await navigator.clipboard.writeText(issuedCard.code);
    codeCopied = true;
    codeRevealed = false;
    if (maskTimer !== null) clearTimeout(maskTimer);
    maskTimer = setTimeout(() => {
      codeCopied = false;
    }, 5000);
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="overlay" onclick={issuedCard === null ? onCancel : undefined}>
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal" onclick={(e) => e.stopPropagation()}>
    {#if issuedCard === null}
      <h2 class="modal-title">Confirm Gift Card</h2>
      <p class="modal-subtitle">Review before issuing</p>
    {:else}
      <h2 class="modal-title modal-title-success">Gift Card Issued</h2>
    {/if}

    <div class="card">
      <div class="card-shine"></div>
      <div class="card-content">
        <div class="card-top">
          <span class="card-brand">{merchantName}</span>
          <span class="card-tag">GIFT CARD</span>
        </div>
        <div class="card-chip">
          <svg width="36" height="28" viewBox="0 0 36 28" fill="none">
            <rect x="0.5" y="0.5" width="35" height="27" rx="3.5" stroke="#c9a84c" stroke-width="1"/>
            <line x1="0" y1="10" x2="36" y2="10" stroke="#c9a84c" stroke-width="0.5"/>
            <line x1="0" y1="18" x2="36" y2="18" stroke="#c9a84c" stroke-width="0.5"/>
            <line x1="12" y1="0" x2="12" y2="28" stroke="#c9a84c" stroke-width="0.5"/>
            <line x1="24" y1="0" x2="24" y2="28" stroke="#c9a84c" stroke-width="0.5"/>
          </svg>
        </div>
        <div class="card-code-row">
          <span class="card-code">{displayCode}</span>
          {#if issuedCard !== null}
            {#if !codeRevealed && !codeCopied}
              <button class="card-icon-btn" onclick={handleReveal} aria-label="Reveal code">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/>
                  <circle cx="12" cy="12" r="3"/>
                </svg>
              </button>
            {:else if codeRevealed}
              <button class="card-icon-btn" onclick={handleCopy} aria-label="Copy code">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
                  <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
                </svg>
              </button>
            {:else if codeCopied}
              <span class="card-copied">Copied</span>
            {/if}
          {/if}
        </div>
        <div class="card-bottom">
          <div class="card-amount">
            <span class="card-currency">₹</span>
            <span class="card-value">{amountFormatted}</span>
          </div>
          <div class="card-expiry">
            {#if expiresAt}
              <span class="card-expiry-label">VALID THRU</span>
              <span class="card-expiry-date">{formatDate(expiresAt)}</span>
            {:else}
              <span class="card-expiry-label">NO EXPIRY</span>
            {/if}
          </div>
        </div>
      </div>
    </div>

    <div class="actions">
      {#if issuedCard === null}
        <Button text="Cancel" classes="btn-cancel" onclick={onCancel} />
        <Button
          text={issuing ? 'Issuing...' : 'Confirm & Issue'}
          classes="btn-confirm"
          disabled={issuing}
          onclick={handleConfirm}
        />
      {:else}
        <Button text="Done" classes="btn-confirm" onclick={onCancel} />
      {/if}
    </div>
  </div>
</div>

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal {
    background: var(--color-bg);
    border-radius: var(--radius-lg);
    padding: var(--space-6);
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-5);
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
    max-width: 500px;
    width: 100%;
  }

  .modal-title {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
  }

  .modal-title-success {
    color: var(--color-success);
  }

  .modal-subtitle {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    margin-top: calc(-1 * var(--space-3));
  }

  .card {
    width: 420px;
    height: 260px;
    border-radius: 16px;
    position: relative;
    overflow: hidden;
    background: linear-gradient(135deg, #1a1a2e 0%, #16213e 40%, #0f3460 100%);
    box-shadow:
      0 10px 40px rgba(0, 0, 0, 0.15),
      0 2px 10px rgba(0, 0, 0, 0.08);
  }

  .card-shine {
    position: absolute;
    top: -60%;
    right: -20%;
    width: 300px;
    height: 300px;
    background: radial-gradient(circle, rgba(201, 168, 76, 0.12) 0%, transparent 70%);
    pointer-events: none;
  }

  .card-content {
    position: relative;
    height: 100%;
    padding: 28px 32px;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
  }

  .card-top {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
  }

  .card-brand {
    font-size: 18px;
    font-weight: 700;
    color: #c9a84c;
    letter-spacing: 0.05em;
  }

  .card-tag {
    font-size: 10px;
    font-weight: 600;
    color: rgba(201, 168, 76, 0.6);
    letter-spacing: 0.15em;
    border: 1px solid rgba(201, 168, 76, 0.25);
    padding: 3px 10px;
    border-radius: 4px;
  }

  .card-chip {
    margin: 4px 0;
  }

  .card-code-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .card-code {
    font-family: var(--font-mono, 'Courier New', monospace);
    font-size: 20px;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.85);
    letter-spacing: 0.12em;
  }

  .card-icon-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    padding: 0;
    background: rgba(201, 168, 76, 0.15);
    border: 1px solid rgba(201, 168, 76, 0.3);
    border-radius: 4px;
    color: #c9a84c;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .card-icon-btn:hover {
    background: rgba(201, 168, 76, 0.25);
    border-color: #c9a84c;
  }

  .card-copied {
    font-size: 11px;
    color: #4ade80;
    font-weight: 600;
    letter-spacing: 0.05em;
  }

  .card-bottom {
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
  }

  .card-amount {
    display: flex;
    align-items: baseline;
    gap: 4px;
  }

  .card-currency {
    font-size: 20px;
    font-weight: 400;
    color: #c9a84c;
  }

  .card-value {
    font-size: 28px;
    font-weight: 700;
    color: #fff;
  }

  .card-expiry {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 2px;
  }

  .card-expiry-label {
    font-size: 8px;
    font-weight: 600;
    color: rgba(201, 168, 76, 0.5);
    letter-spacing: 0.15em;
  }

  .card-expiry-date {
    font-size: 12px;
    color: rgba(255, 255, 255, 0.6);
    font-weight: 500;
  }

  .actions {
    display: flex;
    gap: var(--space-3);
    width: 100%;
    justify-content: center;
  }

  :global(.btn-cancel) {
    --button-color: transparent;
    --button-text-color: var(--color-text-muted);
    --button-padding: 10px 24px;
    --button-border-radius: 8px;
    --button-border: 1px solid var(--color-border);
    --button-font-size: var(--font-size-sm);
    --button-font-weight: 500;
    --button-hover-color: var(--color-surface-2);
    --button-hover-text-color: var(--color-text);
  }

  :global(.btn-confirm) {
    --button-color: var(--color-primary);
    --button-text-color: #fff;
    --button-padding: 10px 32px;
    --button-border-radius: 8px;
    --button-font-size: var(--font-size-sm);
    --button-font-weight: 600;
    --button-hover-color: var(--color-primary-hover);
  }
</style>
