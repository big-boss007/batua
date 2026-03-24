<script lang="ts">
  import { Toggle } from '@juspay/svelte-ui-components';

  import type { WalletPolicy } from '$lib/client/modules/settings';

  const BUCKET_LABELS: Record<string, string> = {
    EarnedCredit: 'Earned Credit',
    CodPending: 'COD Pending',
    GiftCard: 'Gift Card',
    CustomerFunded: 'Customer Funded',
    ReferralReward: 'Referral Reward',
    GoodwillCredit: 'Goodwill Credit',
    MembershipBenefit: 'Membership Benefit',
    RefundCredit: 'Refund Credit'
  };

  function bucketLabel(bt: string): string {
    return BUCKET_LABELS[bt] ?? bt;
  }

  let {
    policies,
    onEdit
  }: {
    policies: Array<WalletPolicy>;
    onEdit: (policy: WalletPolicy) => void;
  } = $props();

  let expandedId = $state<string | null>(null);

  function toggleExpand(id: string) {
    expandedId = expandedId === id ? null : id;
  }
</script>

<div class="policies-list">
  {#if policies.length === 0}
    <p class="empty-state">No wallet policies configured.</p>
  {:else}
    {#each policies as policy (policy.id)}
      <div class="policy-item" class:expanded={expandedId === policy.id}>
        <button class="policy-header" onclick={() => toggleExpand(policy.id)}>
          <span class="policy-bucket">{bucketLabel(policy.bucket_type)}</span>
          <span class="policy-summary">
            {#if policy.default_expiry_days !== null}
              {policy.default_expiry_days}d expiry
            {:else}
              No expiry
            {/if}
          </span>
          <span class="policy-stackable">
            <Toggle checked={policy.stackable_with_discounts} text="Stackable" />
          </span>
          <span class="expand-icon">{expandedId === policy.id ? '-' : '+'}</span>
        </button>

        {#if expandedId === policy.id}
          <div class="policy-details">
            <div class="detail-grid">
              <div class="detail-item">
                <span class="detail-label">Min Redemption</span>
                <span class="detail-value">
                  {policy.min_redemption !== null ? policy.min_redemption : 'None'}
                </span>
              </div>
              <div class="detail-item">
                <span class="detail-label">Max Per Order (%)</span>
                <span class="detail-value">
                  {policy.max_per_order_pct !== null ? `${policy.max_per_order_pct}%` : 'None'}
                </span>
              </div>
              <div class="detail-item">
                <span class="detail-label">Max Per Order (cap)</span>
                <span class="detail-value">
                  {policy.max_per_order_fixed !== null ? policy.max_per_order_fixed : 'None'}
                </span>
              </div>
            </div>
            <button class="edit-button" onclick={() => onEdit(policy)}>Edit Policy</button>
          </div>
        {/if}
      </div>
    {/each}
  {/if}
</div>

<style>
  .policies-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .list-title {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
    margin-bottom: var(--space-2);
  }

  .empty-state {
    padding: var(--space-8);
    text-align: center;
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
  }

  .policy-item {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    overflow: hidden;
    transition: box-shadow var(--transition-fast);
  }

  .policy-item.expanded {
    box-shadow: var(--shadow-md);
  }

  .policy-header {
    display: flex;
    align-items: center;
    width: 100%;
    padding: var(--space-4) var(--space-5);
    background: none;
    border: none;
    text-align: left;
    gap: var(--space-3);
  }

  .policy-header:hover {
    background: var(--color-surface-2);
  }

  .policy-bucket {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
    text-transform: capitalize;
    flex-shrink: 0;
  }

  .policy-summary {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    flex: 1;
  }

  .policy-stackable {
    flex-shrink: 0;
  }

  .expand-icon {
    font-size: var(--font-size-md);
    color: var(--color-text-muted);
    flex-shrink: 0;
    width: 20px;
    text-align: center;
  }

  .policy-details {
    padding: var(--space-4) var(--space-5) var(--space-5);
    border-top: 1px solid var(--color-border);
  }

  .detail-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: var(--space-4);
    margin-bottom: var(--space-4);
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
    font-size: var(--font-size-sm);
    color: var(--color-text);
    font-weight: var(--font-weight-semibold);
  }

  .edit-button {
    padding: var(--space-2) var(--space-4);
    background: var(--color-primary);
    color: #ffffff;
    border: none;
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    transition: background var(--transition-fast);
  }

  .edit-button:hover {
    background: var(--color-primary-hover);
  }
</style>
