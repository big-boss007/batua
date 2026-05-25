<script lang="ts">
  import { Toggle, Button } from '@juspay/svelte-ui-components';

  import type { WalletPolicy } from '$lib/client/modules/settings';

  import { isPointsBucket } from '$lib/client/modules/foundation';

  const BUCKET_LABELS: Record<string, string> = {
    EarnedCredit: 'Reward Points',
    CodPending: 'COD Pending Points',
    GiftCard: 'Gift Card',
    CustomerFunded: 'Customer Funded',
    ReferralReward: 'Referral Points',
    GoodwillCredit: 'Courtesy Points',
    MembershipBenefit: 'Membership Benefit',
    RefundCredit: 'Store Credit'
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

  let pointsPolicies = $derived(policies.filter((p) => isPointsBucket(p.bucket_type)));
  let cashPolicies = $derived(policies.filter((p) => !isPointsBucket(p.bucket_type)));

  function toggleExpand(id: string) {
    expandedId = expandedId === id ? null : id;
  }
</script>

<div class="policies-list">
  {#if policies.length === 0}
    <p class="empty-state">No wallet policies configured.</p>
  {:else}
    {#if pointsPolicies.length > 0}
      <div class="category-section">
        <div class="category-header">
          <span class="category-label">Points Buckets</span>
          <span class="category-badge badge-points">Points</span>
          <span class="category-line"></span>
        </div>
        {#each pointsPolicies as policy (policy.id)}
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
                <Button text="Edit Policy" classes="btn-secondary" onclick={() => onEdit(policy)} />
              </div>
            {/if}
          </div>
        {/each}
      </div>
    {/if}

    {#if cashPolicies.length > 0}
      <div class="category-section">
        <div class="category-header">
          <span class="category-label">Cash Buckets</span>
          <span class="category-badge badge-cash">Cash</span>
          <span class="category-line"></span>
        </div>
        {#each cashPolicies as policy (policy.id)}
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
                <Button text="Edit Policy" classes="btn-secondary" onclick={() => onEdit(policy)} />
              </div>
            {/if}
          </div>
        {/each}
      </div>
    {/if}
  {/if}
</div>

<style>
  .policies-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
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


  .category-section {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .category-header {
    display: flex;
    align-items: center;
    gap: var(--space-3);
  }

  .category-label {
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-bold);
    letter-spacing: 1px;
    text-transform: uppercase;
    color: var(--color-text-muted);
    white-space: nowrap;
  }

  .category-badge {
    font-size: 11px;
    font-weight: var(--font-weight-semibold);
    padding: 2px 10px;
    border-radius: 100px;
    white-space: nowrap;
  }

  .badge-points {
    background: color-mix(in srgb, var(--color-primary) 12%, transparent);
    color: var(--color-primary);
  }

  .badge-cash {
    background: color-mix(in srgb, var(--color-success) 12%, transparent);
    color: var(--color-success);
  }

  .category-line {
    flex: 1;
    height: 1px;
    background: var(--color-border);
  }
</style>
