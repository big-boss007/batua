<script lang="ts">
  import { formatCurrencyINR, formatPoints, isPointsBucket } from '$lib/client/modules/foundation';
  import type { CustomerBalance, StorefrontMerchant } from '../types';

  let {
    balance,
    merchant
  }: {
    balance: CustomerBalance;
    merchant: StorefrontMerchant;
  } = $props();

  let pointsSpendable = $derived(balance.points_balance);
  let pointsDisplayed = $derived(
    balance.buckets
      .filter((b) => isPointsBucket(b.bucket_type))
      .reduce((sum, b) => sum + b.displayed, 0)
  );
  let pendingPoints = $derived(
    pointsDisplayed > pointsSpendable ? pointsDisplayed - pointsSpendable : 0
  );
  let pointsCurrency = $derived(pointsSpendable * merchant.points_to_currency_rate);
  let cashBalance = $derived(balance.cash_balance);
  let totalRedeemable = $derived(pointsCurrency + cashBalance);

  let pointsBuckets = $derived(
    balance.buckets.filter((b) => isPointsBucket(b.bucket_type) && b.spendable > 0)
  );

  let hasMultipleSources = $derived(pointsBuckets.length > 1);
  let allPending = $derived(pointsSpendable === 0 && pointsDisplayed > 0);
  let hasExpiring = $derived(balance.expiring_soon !== null);

  let starsCardClass = $derived.by(() => {
    if (hasExpiring && balance.expiring_soon !== null && balance.expiring_soon.days <= 3)
      return 'stars-card stars-card-expiry';
    if (allPending) return 'stars-card stars-card-pending';
    if (pointsDisplayed === 0) return 'stars-card stars-card-empty';
    return 'stars-card';
  });

  let icon = $derived(merchant.points_icon);
  let name = $derived(merchant.points_name);
</script>

{#if totalRedeemable > 0 || pointsDisplayed > 0}
  <div class="total-hero">
    <div class="total-eyebrow">Total Redeemable</div>
    <div class="total-num">{formatCurrencyINR(totalRedeemable)}</div>
  </div>
{/if}

<div class={starsCardClass}>
  <div class="stars-top">
    <div>
      <div class="stars-label">{allPending ? name + ' Pending' : name + ' Earned'}</div>
      <div class="stars-num">
        {#if pointsDisplayed === 0}
          <span class="stars-zero">0 {icon}</span>
        {:else}
          {formatPoints(allPending ? pointsDisplayed : pointsSpendable, icon)}
        {/if}
      </div>
    </div>
    <div class="stars-right">
      {#if pointsDisplayed > 0}
        <div class="stars-inr">
          {allPending ? '' : '≈ '}{formatCurrencyINR(
            allPending
              ? pointsDisplayed * merchant.points_to_currency_rate
              : pointsCurrency
          )}
        </div>
        {#if pendingPoints > 0 && !allPending}
          <div class="stars-pending">+{formatPoints(pendingPoints, icon)} pending</div>
        {/if}
        {#if allPending}
          <div class="stars-pending">all pending</div>
        {/if}
      {:else}
        <div class="stars-inr stars-inr-zero">{formatCurrencyINR(0)}</div>
      {/if}
    </div>
  </div>

  {#if hasMultipleSources}
    <div class="stars-breakdown">
      {#each pointsBuckets as bucket}
        {@const shortLabels: Record<string, string> = {
          earned_credit: 'Earned', EarnedCredit: 'Earned',
          referral_reward: 'Referral', ReferralReward: 'Referral',
          cod_pending: 'COD', CodPending: 'COD',
          goodwill_credit: 'Goodwill', GoodwillCredit: 'Goodwill',
          membership_benefit: 'Membership', MembershipBenefit: 'Membership'
        }}
        {@const label = shortLabels[bucket.bucket_type] ?? bucket.bucket_type}
        {@const isPending = bucket.bucket_type === 'cod_pending' || bucket.bucket_type === 'CodPending'}
        <span
          >{label} <span class="breakdown-val" class:breakdown-pending={isPending}
            >{bucket.spendable.toLocaleString('en-IN')}</span
          ></span>
      {/each}
    </div>
  {/if}

  {#if pointsDisplayed === 0}
    <div class="stars-cta">Place your first order to start earning {name}</div>
  {:else if allPending}
    <div class="stars-cta">{name} will unlock once your order is delivered</div>
  {/if}

  {#if hasExpiring && balance.expiring_soon !== null}
    <div class="stars-expiry">
      <div class="expiry-dot"></div>
      {#if balance.expiring_soon.days <= 1}
        <strong>{formatPoints(balance.expiring_soon.amount, icon)} expiring tomorrow!</strong>
      {:else}
        {formatPoints(balance.expiring_soon.amount, icon)} expiring in {balance.expiring_soon.days} days
      {/if}
    </div>
  {/if}
</div>

{#if cashBalance > 0}
  <div class="cash-card">
    <div class="cash-top">
      <div>
        <div class="cash-label">Cash Balance</div>
        <div class="cash-num">{formatCurrencyINR(cashBalance)}</div>
      </div>
      <div class="cash-detail">
        {#each balance.buckets.filter((b) => !isPointsBucket(b.bucket_type) && b.spendable > 0) as bucket}
          <div>
            {bucket.bucket_type === 'gift_card' || bucket.bucket_type === 'GiftCard'
              ? 'Gift Card'
              : bucket.bucket_type === 'refund_credit' || bucket.bucket_type === 'RefundCredit'
                ? 'Refund'
                : 'Funded'} {formatCurrencyINR(bucket.spendable)}
          </div>
        {/each}
      </div>
    </div>
  </div>
{/if}

<style>
  .total-hero {
    text-align: center;
    padding: 22px 24px 14px;
  }

  .total-eyebrow {
    font-size: 11px;
    color: #9ca3af;
    text-transform: uppercase;
    letter-spacing: 0.8px;
    margin-bottom: 5px;
  }

  .total-num {
    font-size: 36px;
    font-weight: 700;
    color: #ffffff;
    line-height: 1;
  }

  .stars-card {
    margin: 0 24px;
    border-radius: 14px;
    padding: 16px 16px;
    background: rgba(74, 222, 128, 0.04);
    border: 1px solid rgba(74, 222, 128, 0.12);
  }

  .stars-card-pending {
    background: rgba(253, 224, 71, 0.03);
    border-color: rgba(253, 224, 71, 0.15);
  }

  .stars-card-expiry {
    border-color: rgba(248, 113, 113, 0.2);
  }

  .stars-card-empty {
    background: rgba(255, 255, 255, 0.02);
    border-color: rgba(74, 222, 128, 0.06);
  }

  .stars-top {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .stars-label {
    font-size: 9px;
    color: #6ee7b7;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    font-weight: 600;
  }

  .stars-card-pending .stars-label {
    color: #fde047;
  }

  .stars-card-empty .stars-label {
    color: #6b7280;
  }

  .stars-num {
    font-size: 26px;
    font-weight: 700;
    color: #4ade80;
    margin-top: 4px;
  }

  .stars-card-pending .stars-num {
    color: #fde047;
  }

  .stars-zero {
    color: #6b7280;
  }

  .stars-right {
    text-align: right;
  }

  .stars-inr {
    font-size: 14px;
    font-weight: 600;
    color: #ffffff;
  }

  .stars-inr-zero {
    color: #6b7280;
  }

  .stars-pending {
    font-size: 10px;
    color: #9ca3af;
    margin-top: 2px;
  }

  .stars-card-pending .stars-pending {
    color: #fde047;
  }

  .stars-breakdown {
    display: flex;
    gap: 14px;
    margin-top: 10px;
    padding-top: 10px;
    border-top: 1px solid rgba(74, 222, 128, 0.1);
    font-size: 10px;
    color: #9ca3af;
  }

  .breakdown-val {
    color: #ffffff;
    font-weight: 500;
  }

  .breakdown-pending {
    color: #fde047;
  }

  .stars-cta {
    margin-top: 10px;
    padding-top: 10px;
    border-top: 1px solid rgba(74, 222, 128, 0.06);
    font-size: 10px;
    color: #6b7280;
  }

  .stars-expiry {
    display: flex;
    align-items: center;
    gap: 5px;
    margin-top: 8px;
    font-size: 10px;
    color: #f87171;
  }

  .expiry-dot {
    width: 5px;
    height: 5px;
    border-radius: 50%;
    background: #f87171;
    animation: pulse 2s infinite;
  }

  @keyframes pulse {
    0%,
    100% {
      opacity: 1;
    }
    50% {
      opacity: 0.4;
    }
  }

  .cash-card {
    margin: 10px 24px 0;
    border-radius: 14px;
    padding: 14px 16px;
    background: rgba(196, 181, 253, 0.04);
    border: 1px solid rgba(196, 181, 253, 0.12);
  }

  .cash-top {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .cash-label {
    font-size: 9px;
    color: #c4b5fd;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    font-weight: 600;
  }

  .cash-num {
    font-size: 24px;
    font-weight: 700;
    color: #c4b5fd;
    margin-top: 3px;
  }

  .cash-detail {
    font-size: 10px;
    color: #9ca3af;
    text-align: right;
  }
</style>
