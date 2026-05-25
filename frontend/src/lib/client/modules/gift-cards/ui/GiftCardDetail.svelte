<script lang="ts">
  import { Pill, Progress } from '@juspay/svelte-ui-components';

  import type { GiftCard } from '$lib/client/modules/gift-cards';
  import { formatCurrencyINR, formatDate, formatDateTime } from '$lib/client/modules/foundation';

  let { card }: { card: GiftCard } = $props();

  let codeRevealed = $state(false);
  let codeCopied = $state(false);
  let maskTimer: ReturnType<typeof setTimeout> | null = null;
  // svelte-ignore state_referenced_locally
  let prevCardId = $state(card.id);

  let _resetOnCardChange = $derived.by(() => {
    if (card.id !== prevCardId) {
      codeRevealed = false;
      codeCopied = false;
      if (maskTimer !== null) clearTimeout(maskTimer);
      prevCardId = card.id;
    }
    return null;
  });

  let status = $derived.by(() => {
    if (!card.is_active) return 'inactive';
    if (card.is_claimed) return 'claimed';
    if (card.expires_at && new Date(card.expires_at) < new Date()) return 'expired';
    return 'active';
  });

  let pillClass = $derived.by(() => {
    if (status === 'active') return 'pill-success';
    if (status === 'claimed') return 'pill-info';
    if (status === 'expired') return 'pill-warning';
    return 'pill-neutral';
  });

  let usedAmount = $derived(card.initial_amount - card.current_amount);
  let usagePercent = $derived(
    card.initial_amount > 0 ? Math.round((usedAmount / card.initial_amount) * 100) : 0
  );

  let displayCode = $derived(codeRevealed ? card.code : `****-****-****-${card.code.slice(-4)}`);

  function handleReveal() {
    codeRevealed = true;
    codeCopied = false;
    if (maskTimer !== null) clearTimeout(maskTimer);
    maskTimer = setTimeout(() => {
      codeRevealed = false;
      codeCopied = false;
    }, 5000);
  }

  async function handleCopy() {
    await navigator.clipboard.writeText(card.code);
    codeCopied = true;
    codeRevealed = false;
    if (maskTimer !== null) clearTimeout(maskTimer);
    maskTimer = setTimeout(() => {
      codeCopied = false;
    }, 5000);
  }
</script>

<div class="detail-card">
  <div class="detail-header">
    <div class="code-row">
      <code class="detail-code">{displayCode}</code>
      {#if !codeRevealed && !codeCopied}
        <button class="icon-btn" onclick={handleReveal} aria-label="Reveal code" title="Reveal code">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/>
            <circle cx="12" cy="12" r="3"/>
          </svg>
        </button>
      {:else if codeRevealed}
        <button class="icon-btn" onclick={handleCopy} aria-label="Copy code" title="Copy code">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
            <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
          </svg>
        </button>
      {:else if codeCopied}
        <button class="icon-btn copied" aria-label="Copied" title="Copied!">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="20 6 9 17 4 12"/>
          </svg>
        </button>
      {/if}
    </div>
    <Pill text={status} classes={pillClass} />
  </div>

  <div class="detail-grid">
    <div class="detail-item">
      <span class="detail-label">Initial Amount</span>
      <span class="detail-value">{formatCurrencyINR(card.initial_amount)}</span>
    </div>
    <div class="detail-item">
      <span class="detail-label">Current Balance</span>
      <span class="detail-value">{formatCurrencyINR(card.current_amount)}</span>
    </div>
    <div class="detail-item">
      <span class="detail-label">Used</span>
      <span class="detail-value">{formatCurrencyINR(usedAmount)} ({usagePercent}%)</span>
    </div>
    <div class="detail-item">
      <span class="detail-label">Claimed</span>
      <span class="detail-value">{card.is_claimed ? 'Yes' : 'No'}</span>
    </div>
  </div>

  <Progress value={usagePercent} classes="progress-usage" />

  <div class="detail-meta">
    <span class="meta-item">Created: {formatDateTime(card.created_at)}</span>
    {#if card.expires_at}
      <span class="meta-item">Expires: {formatDate(card.expires_at)}</span>
    {/if}
  </div>
</div>

<style>
  .detail-card {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .detail-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-3);
  }

  .code-row {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .detail-code {
    font-family: var(--font-mono);
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    padding: var(--space-1) var(--space-3);
    background: var(--color-surface-2);
    border-radius: var(--radius-md);
    letter-spacing: 0.05em;
  }

  .icon-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    padding: 0;
    background: none;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    color: var(--color-text-muted);
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .icon-btn:hover {
    color: var(--color-primary);
    border-color: var(--color-primary);
  }

  .icon-btn.copied {
    color: var(--color-success);
    border-color: var(--color-success);
    background: color-mix(in srgb, var(--color-success) 8%, transparent);
  }

  .detail-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
    gap: var(--space-4);
  }

  .detail-item {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .detail-label {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    font-weight: var(--font-weight-medium);
  }

  .detail-value {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  :global(.progress-usage) {
    --progress-track-height: 6px;
    --progress-bar-background: var(--color-primary);
    --progress-track-background: var(--color-surface-2);
    --progress-track-border-radius: var(--radius-full);
    --progress-bar-border-radius: var(--radius-full);
  }

  .detail-meta {
    display: flex;
    gap: var(--space-4);
    flex-wrap: wrap;
  }

  .meta-item {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }
</style>
