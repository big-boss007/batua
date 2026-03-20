<script lang="ts">
  import { Avatar, Pill } from '@juspay/svelte-ui-components';
  import type { CustomerDetail as CustomerDetailType } from '$lib/client/modules/customers';
  import { formatMovementType, formatBucketType } from '$lib/client/modules/customers';
  import { formatCurrencyINR, formatDate, formatDateTime } from '$lib/client/modules/foundation';
  import TierBadge from './TierBadge.svelte';
  import TierProgress from './TierProgress.svelte';

  let { detail }: { detail: CustomerDetailType } = $props();

  let customer = $derived(detail.customer);
  let wallet = $derived(detail.wallet);
  let tier = $derived(detail.tier);
  let entries = $derived(detail.recent_entries);
</script>

<div class="customer-detail">
  <section class="info-card">
    <div class="info-header">
      <Avatar name={customer.name ?? 'U'} alt={customer.name ?? 'User'} size="large" />
      <div class="info-identity">
        <h2 class="info-name">{customer.name ?? 'Unnamed Customer'}</h2>
        <span class="info-phone">{customer.phone}</span>
      </div>
      <Pill
        text={customer.is_verified ? 'Verified' : 'Unverified'}
        classes={customer.is_verified ? 'pill-success' : 'pill-warning'}
      />
    </div>

    <div class="info-fields">
      {#if customer.email}
        <div class="info-field">
          <span class="field-label">Email</span>
          <span class="field-value">{customer.email}</span>
        </div>
      {/if}
      <div class="info-field">
        <span class="field-label">Customer since</span>
        <span class="field-value">{formatDate(customer.created_at)}</span>
      </div>
    </div>
  </section>

  {#if wallet}
    <section class="wallet-card">
      <h3 class="card-title">Wallet</h3>
      <div class="wallet-balances">
        <div class="balance-item">
          <span class="balance-label">Displayed Balance</span>
          <span class="balance-value">{formatCurrencyINR(wallet.displayed_balance)}</span>
        </div>
        <div class="balance-item">
          <span class="balance-label">Spendable Balance</span>
          <span class="balance-value primary">{formatCurrencyINR(wallet.spendable_balance)}</span>
        </div>
      </div>
    </section>
  {/if}

  {#if tier}
    <section class="tier-card">
      <h3 class="card-title">Loyalty Tier</h3>
      <div class="tier-info">
        <TierBadge
          tierName={tier.tier_name}
          rank={tier.rank}
          multiplier={tier.earn_rate_multiplier}
        />
        {#if tier.progress_to_next}
          <div class="tier-progress-wrapper">
            <TierProgress progress={tier.progress_to_next} />
          </div>
        {/if}
      </div>
    </section>
  {/if}

  {#if entries.length > 0}
    <section class="transactions-card">
      <h3 class="card-title">Recent Transactions</h3>
      <div class="transactions-list">
        {#each entries as entry (entry.id)}
          <div class="transaction-row">
            <div class="transaction-info">
              <span class="transaction-type">{formatMovementType(entry.movement_type)}</span>
              <span class="transaction-bucket">{formatBucketType(entry.bucket_type)}</span>
            </div>
            <div class="transaction-right">
              <span
                class="transaction-amount"
                class:credit={entry.movement_type === 'credit'}
                class:debit={entry.movement_type === 'debit'}
              >
                {entry.movement_type === 'credit' ? '+' : '-'}{formatCurrencyINR(
                  Math.abs(entry.currency_equivalent)
                )}
              </span>
              <span class="transaction-date">{formatDateTime(entry.created_at)}</span>
            </div>
          </div>
        {/each}
      </div>
    </section>
  {/if}
</div>

<style>
  .customer-detail {
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
  }

  .info-card,
  .wallet-card,
  .tier-card,
  .transactions-card {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: var(--space-6);
  }

  .card-title {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    margin-bottom: var(--space-4);
    color: var(--color-text);
  }

  .info-header {
    display: flex;
    align-items: center;
    gap: var(--space-4);
    margin-bottom: var(--space-4);
  }

  .info-identity {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    flex: 1;
    min-width: 0;
  }

  .info-name {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .info-phone {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    font-family: var(--font-mono);
  }

  .info-fields {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: var(--space-4);
  }

  .info-field {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .field-label {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .field-value {
    font-size: var(--font-size-base);
    color: var(--color-text);
  }

  .wallet-balances {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
    gap: var(--space-4);
  }

  .balance-item {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .balance-label {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .balance-value {
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
  }

  .balance-value.primary {
    color: var(--color-primary);
  }

  .tier-info {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .tier-progress-wrapper {
    max-width: 400px;
  }

  .transactions-list {
    display: flex;
    flex-direction: column;
  }

  .transaction-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--space-3) 0;
    border-bottom: 1px solid var(--color-border);
  }

  .transaction-row:last-child {
    border-bottom: none;
  }

  .transaction-info {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .transaction-type {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-medium);
    color: var(--color-text);
  }

  .transaction-bucket {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .transaction-right {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: var(--space-1);
  }

  .transaction-amount {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-semibold);
    font-family: var(--font-mono);
  }

  .transaction-amount.credit {
    color: var(--color-success);
  }

  .transaction-amount.debit {
    color: var(--color-error);
  }

  .transaction-date {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }
</style>
