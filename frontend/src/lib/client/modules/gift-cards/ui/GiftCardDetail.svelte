<script lang="ts">
  import type { GiftCard } from '$lib/client/modules/gift-cards';
  import { formatCurrencyINR, formatDate, formatDateTime } from '$lib/client/modules/foundation';

  let { card }: { card: GiftCard } = $props();

  let status = $derived.by(() => {
    if (!card.is_active) return 'inactive';
    if (card.is_claimed) return 'claimed';
    if (card.expires_at && new Date(card.expires_at) < new Date()) return 'expired';
    return 'active';
  });

  let statusClass = $derived.by(() => {
    if (status === 'active') return 'badge-success';
    if (status === 'claimed') return 'badge-info';
    if (status === 'expired') return 'badge-warning';
    return 'badge-muted';
  });

  let usedAmount = $derived(card.initial_amount - card.current_amount);
  let usagePercent = $derived(
    card.initial_amount > 0 ? Math.round((usedAmount / card.initial_amount) * 100) : 0
  );
</script>

<div class="detail-card">
  <div class="detail-header">
    <code class="detail-code">{card.code}</code>
    <span class="badge {statusClass}">{status}</span>
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

  <div class="progress-track">
    <div class="progress-fill" style="width: {usagePercent}%"></div>
  </div>

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
    padding: var(--space-6);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
  }

  .detail-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-3);
  }

  .detail-code {
    font-family: var(--font-mono);
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    padding: var(--space-1) var(--space-3);
    background: var(--color-surface-2);
    border-radius: var(--radius-md);
  }

  .badge {
    display: inline-block;
    padding: var(--space-1) var(--space-3);
    border-radius: var(--radius-full);
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-medium);
    text-transform: capitalize;
  }

  .badge-success {
    color: var(--color-success);
    background: color-mix(in srgb, var(--color-success) 12%, transparent);
  }

  .badge-info {
    color: var(--color-info);
    background: color-mix(in srgb, var(--color-info) 12%, transparent);
  }

  .badge-warning {
    color: var(--color-warning);
    background: color-mix(in srgb, var(--color-warning) 12%, transparent);
  }

  .badge-muted {
    color: var(--color-text-muted);
    background: var(--color-surface-2);
  }

  .detail-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
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

  .progress-track {
    height: 6px;
    background: var(--color-surface-2);
    border-radius: var(--radius-full);
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: var(--color-primary);
    border-radius: var(--radius-full);
    transition: width var(--transition-base);
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
