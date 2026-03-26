<script lang="ts">
  import { Avatar, Pill } from '@juspay/svelte-ui-components';
  import type { CustomerDetail as CustomerDetailType, BucketBalance } from '$lib/client/modules/customers';
  import { formatMovementType, formatBucketType } from '$lib/client/modules/customers';
  import {
    formatCurrencyINR,
    formatDate,
    formatDateTime,
    formatPhone,
    formatPoints,
    isPointsBucket
  } from '$lib/client/modules/foundation';

  let { detail, pointsIcon = '★', pointsRate = 1.0 }: { detail: CustomerDetailType; pointsIcon?: string; pointsRate?: number } = $props();

  let customer = $derived(detail.customer);
  let wallet = $derived(detail.wallet);
  let tier = $derived(detail.tier);
  let membership = $derived(detail.membership);
  let referral = $derived(detail.referral);
  let entries = $derived(detail.recent_entries);

  let totalRedeemable = $derived(
    wallet !== null ? wallet.points_balance * pointsRate + wallet.cash_balance : 0
  );

  let pointsCashBuckets = $derived.by(() => {
    if (wallet === null) return [];
    const POINTS_TYPES = new Set(['EarnedCredit', 'ReferralReward', 'GoodwillCredit', 'MembershipBenefit']);
    return wallet.buckets.filter((b) => !POINTS_TYPES.has(b.bucket_type) && b.spendable > 0);
  });

  let pointsSubBuckets = $derived.by(() => {
    if (wallet === null) return [];
    const POINTS_TYPES = new Set(['EarnedCredit', 'ReferralReward', 'GoodwillCredit', 'MembershipBenefit']);
    return wallet.buckets.filter((b) => POINTS_TYPES.has(b.bucket_type) && b.spendable > 0);
  });

  function bucketLabel(bt: string): string {
    const map: Record<string, string> = {
      EarnedCredit: 'Earned',
      ReferralReward: 'Referral',
      GoodwillCredit: 'Goodwill',
      MembershipBenefit: 'Membership',
      GiftCard: 'Gift Card',
      RefundCredit: 'Refund',
      CustomerFunded: 'Funded',
      CodPending: 'COD'
    };
    return map[bt] ?? bt;
  }

  let membershipExpiring = $derived(
    membership !== null && membership.days_remaining > 0 && membership.days_remaining <= 30
  );

  let earnedMult = $derived(tier?.earn_rate_multiplier ?? 1);
  let membershipMult = $derived(membership?.earn_rate_multiplier ?? 0);
  let membershipHigher = $derived(membership !== null && membershipMult > earnedMult);
  let earnedHigher = $derived(membership !== null && earnedMult > membershipMult);
  let sameTier = $derived(membership !== null && earnedMult === membershipMult && earnedMult > 1);
</script>

<div class="customer-detail">
  <!-- Customer Info -->
  <section class="card">
    <div class="info-header">
      <Avatar name={customer.name ?? 'U'} alt={customer.name ?? 'User'} size="large" />
      <div class="info-identity">
        <h2 class="info-name">{customer.name ?? 'Unnamed Customer'}</h2>
        <span class="info-phone">{formatPhone(customer.phone)}</span>
      </div>
      <Pill
        text={customer.is_verified ? 'Verified' : 'Unverified'}
        classes={customer.is_verified ? 'pill-success' : 'pill-warning'}
      />
    </div>
    <div class="meta-grid meta-2">
      {#if customer.email}
        <div><span class="meta-label">Email</span><div class="meta-value">{customer.email}</div></div>
      {/if}
      <div><span class="meta-label">Customer since</span><div class="meta-value">{formatDate(customer.created_at)}</div></div>
    </div>
  </section>

  <!-- Wallet -->
  {#if wallet}
    <section class="card">
      <h3 class="card-title">💰 Wallet</h3>
      <div class="wd-total">
        <span class="wd-total-value">{formatCurrencyINR(totalRedeemable)}</span>
        <span class="wd-total-label">total redeemable</span>
      </div>
      <div class="wd-rows">
        <div class="wd-row">
          <div class="wd-row-left">
            <span class="wd-dot" style="background: var(--color-primary);"></span>
            <span class="wd-row-label">Points</span>
          </div>
          <div class="wd-row-right">
            <span class="wd-row-value" style="color: var(--color-primary);">{formatPoints(wallet.points_balance, pointsIcon)}</span>
            <span class="wd-row-equiv">worth {formatCurrencyINR(wallet.points_balance * pointsRate)}{#if pointsSubBuckets.length > 0}
              &nbsp;&middot; {pointsSubBuckets.map((b) => `${bucketLabel(b.bucket_type)} ${Math.round(b.spendable)}`).join(' + ')}
            {/if}</span>
          </div>
        </div>
        <div class="wd-row">
          <div class="wd-row-left">
            <span class="wd-dot" style="background: var(--color-success);"></span>
            <span class="wd-row-label">Cash</span>
          </div>
          <div class="wd-row-right">
            <span class="wd-row-value" style="color: var(--color-success);">{formatCurrencyINR(wallet.cash_balance)}</span>
            <span class="wd-row-equiv">{#if pointsCashBuckets.length > 0}{pointsCashBuckets.map((b) => `${bucketLabel(b.bucket_type)} ${formatCurrencyINR(b.spendable)}`).join(' + ')}{:else}No cash balance{/if}</span>
          </div>
        </div>
      </div>
    </section>
  {/if}

  <!-- Loyalty Tier -->
  <section class="card">
    <h3 class="card-title">🏆 Loyalty Tier</h3>
    {#if tier}
      <div class="tier-row">
        <span class="tier-badge">{tier.tier_name} <span class="tier-mult">{tier.earn_rate_multiplier}x</span></span>
        <span class="tier-source">Earned via evaluation</span>
      </div>
      {#if tier.progress_to_next}
        <div class="progress">
          <div class="progress-header">
            <span class="progress-label">Progress to {tier.progress_to_next.next_tier_name}</span>
            <span class="progress-value">{Math.round(tier.progress_to_next.percentage)}%</span>
          </div>
          <div class="progress-track"><div class="progress-fill fill-purple" style="width: {tier.progress_to_next.percentage}%;"></div></div>
          <div class="progress-footer">
            <span>₹{tier.progress_to_next.current_value.toLocaleString('en-IN')}</span>
            <span>₹{tier.progress_to_next.threshold.toLocaleString('en-IN')}</span>
          </div>
        </div>
      {/if}
      {#if membershipHigher}
        <div class="callout callout-upgrade">
          <span class="callout-icon">⬆</span>
          <span>Membership active — currently earning at <strong>{membershipMult}x</strong> ({membership?.tier_name}) instead of {earnedMult}x ({tier.tier_name})</span>
        </div>
      {:else if earnedHigher}
        <div class="callout callout-muted">
          <span class="callout-icon">🏆</span>
          <span>Earned tier is higher — earning at <strong>{earnedMult}x</strong> (membership {membership?.tier_name} {membershipMult}x is superseded)</span>
        </div>
      {:else if sameTier}
        <div class="callout callout-muted">
          <span class="callout-icon">↔</span>
          <span>Membership matches earned tier — earning at <strong>{earnedMult}x</strong></span>
        </div>
      {/if}
    {:else}
      <span class="no-data">No tier yet — qualifying value below threshold</span>
      {#if membershipHigher}
        <div class="callout callout-upgrade">
          <span class="callout-icon">⬆</span>
          <span>Membership active — currently earning at <strong>{membershipMult}x</strong> ({membership?.tier_name})</span>
        </div>
      {/if}
    {/if}
  </section>

  <!-- Membership -->
  <section class="card" class:card-warn={membershipExpiring}>
    <h3 class="card-title">
      💎 Membership
      {#if membershipExpiring}
        <span class="card-title-warn">⚠ Expiring in {membership?.days_remaining} days</span>
      {/if}
    </h3>
    {#if membership}
      <div class="tier-row">
        <span class="tier-badge">{membership.tier_name} <span class="tier-mult">{membership.earn_rate_multiplier}x</span></span>
        {#if membershipExpiring}
          <Pill text="Expiring Soon" classes="pill-warning" />
        {:else}
          <Pill text="Active" classes="pill-success" />
        {/if}
        <span class="tier-source" class:tier-source-warn={membershipExpiring}>
          Expires {formatDate(membership.expires_at)}
        </span>
      </div>
      <div class="meta-grid meta-2">
        <div>
          <span class="meta-label">Renewed</span>
          <div class="meta-value">{membership.renewed_count} times</div>
        </div>
        {#if (membershipHigher || membershipExpiring) && tier}
          <div>
            <span class="meta-label">Falls back to</span>
            <div class="meta-value"><span class="tier-badge tier-badge-sm">{tier.tier_name} <span class="tier-mult">{tier.earn_rate_multiplier}x</span></span></div>
          </div>
        {/if}
      </div>
      <div class="progress">
        <div class="progress-header">
          <span class="progress-label">Time remaining</span>
          <span class="progress-value" class:progress-value-warn={membershipExpiring}>{membership.days_remaining} days</span>
        </div>
        <div class="progress-track">
          <div
            class="progress-fill"
            class:fill-green={!membershipExpiring}
            class:fill-amber={membershipExpiring}
            style="width: {Math.min(100, (membership.days_remaining / 365) * 100)}%;"
          ></div>
        </div>
      </div>
    {:else}
      <span class="no-data">No active membership</span>
    {/if}
  </section>

  <!-- Referrals -->
  <section class="card">
    <h3 class="card-title">🔗 Referrals</h3>
    {#if referral && (referral.code !== null || referral.referrals_made > 0)}
      <div class="meta-grid meta-3">
        {#if referral.code !== null}
          <div><span class="meta-label">Referral Code</span><div class="meta-value meta-mono">{referral.code}</div></div>
        {/if}
        <div><span class="meta-label">Referrals Made</span><div class="meta-value meta-highlight">{referral.referrals_made}</div></div>
        <div><span class="meta-label">Rewards Earned</span><div class="meta-value">{formatCurrencyINR(referral.rewards_earned)}</div></div>
      </div>
    {:else}
      <span class="no-data">No referral activity</span>
    {/if}
  </section>

  <!-- Recent Transactions -->
  <section class="card">
    <h3 class="card-title">📋 Recent Transactions</h3>
    {#if entries.length > 0}
      <div class="tx-list">
        {#each entries as entry (entry.id)}
          <div class="tx-row">
            <div class="tx-info">
              <span class="tx-type">{formatMovementType(entry.movement_type)}</span>
              <span class="tx-bucket">{formatBucketType(entry.bucket_type)}</span>
            </div>
            <div class="tx-right">
              <span
                class="tx-amount"
                class:credit={entry.movement_type === 'in' || entry.movement_type === 'held'}
                class:debit={entry.movement_type === 'out'}
              >
                {entry.movement_type === 'out' ? '-' : '+'}{isPointsBucket(entry.bucket_type)
                  ? formatPoints(Math.abs(entry.earning_unit ?? entry.currency_equivalent), '★')
                  : formatCurrencyINR(Math.abs(entry.currency_equivalent))}
              </span>
              <span class="tx-date">{formatDateTime(entry.created_at)}</span>
            </div>
          </div>
        {/each}
      </div>
    {:else}
      <span class="no-data">No transactions yet</span>
    {/if}
  </section>
</div>

<style>
  .customer-detail { display: flex; flex-direction: column; gap: var(--space-4); }

  .card {
    background: var(--color-surface); border: 1px solid var(--color-border);
    border-radius: var(--radius-lg); padding: var(--space-5);
  }
  .card.card-warn { border-color: #fde68a; background: #fffdf5; }

  .card-title {
    font-size: var(--font-size-sm); font-weight: var(--font-weight-semibold);
    margin-bottom: var(--space-3); color: var(--color-text);
    display: flex; align-items: center; gap: var(--space-2);
  }
  .card-title-warn { font-size: var(--font-size-xs); color: #f59e0b; font-weight: var(--font-weight-medium); margin-left: auto; }

  /* Info header */
  .info-header { display: flex; align-items: center; gap: var(--space-4); margin-bottom: var(--space-4); }
  .info-identity { display: flex; flex-direction: column; gap: var(--space-1); flex: 1; min-width: 0; }
  .info-name { font-size: var(--font-size-lg); font-weight: var(--font-weight-semibold); color: var(--color-text); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .info-phone { font-size: var(--font-size-sm); color: var(--color-text-muted); font-family: var(--font-mono); }

  /* Shared meta grid */
  .meta-grid { display: grid; gap: var(--space-3); }
  .meta-2 { grid-template-columns: 1fr 1fr; }
  .meta-3 { grid-template-columns: 1fr 1fr 1fr; }
  .meta-label { font-size: 10px; color: var(--color-text-muted); text-transform: uppercase; letter-spacing: 0.04em; }
  .meta-value { font-size: var(--font-size-sm); font-weight: var(--font-weight-medium); color: var(--color-text); margin-top: 2px; }
  .meta-mono { font-family: var(--font-mono); }
  .meta-highlight { color: var(--color-primary); }

  /* Wallet — Option D */
  .wd-total {
    display: flex;
    align-items: baseline;
    gap: var(--space-2);
    padding-bottom: var(--space-3);
    margin-bottom: var(--space-3);
    border-bottom: 1px solid var(--color-border);
  }
  .wd-total-value { font-size: var(--font-size-2xl); font-weight: 800; color: var(--color-text); }
  .wd-total-label { font-size: var(--font-size-sm); color: var(--color-text-muted); font-weight: var(--font-weight-medium); }
  .wd-rows { display: flex; flex-direction: column; gap: var(--space-2); }
  .wd-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-2) var(--space-3);
    background: var(--color-surface-2);
    border-radius: var(--radius-md);
  }
  .wd-row-left { display: flex; align-items: center; gap: var(--space-2); }
  .wd-dot { width: 10px; height: 10px; border-radius: 50%; flex-shrink: 0; }
  .wd-row-label { font-size: var(--font-size-sm); font-weight: var(--font-weight-semibold); color: var(--color-text); }
  .wd-row-right { display: flex; flex-direction: column; align-items: flex-end; }
  .wd-row-value { font-size: var(--font-size-base); font-weight: var(--font-weight-bold); }
  .wd-row-equiv { font-size: 11px; color: var(--color-text-muted); }

  /* Shared tier row: badge + source/status */
  .tier-row { display: flex; align-items: center; gap: var(--space-2); flex-wrap: wrap; }
  .tier-badge {
    display: inline-flex; align-items: center; gap: 3px;
    padding: 4px 12px; border-radius: 20px; font-size: var(--font-size-xs); font-weight: var(--font-weight-semibold);
    background: var(--color-surface-2); color: var(--color-text);
  }
  .tier-badge-sm { padding: 2px 8px; font-size: 10px; }
  .tier-mult { font-weight: var(--font-weight-normal); opacity: 0.6; }
  .tier-source { font-size: var(--font-size-xs); color: var(--color-text-muted); margin-left: auto; }
  .tier-source-warn { color: #f59e0b; font-weight: var(--font-weight-medium); }

  /* Shared progress bar */
  .progress { margin-top: var(--space-3); }
  .progress-header { display: flex; justify-content: space-between; align-items: baseline; margin-bottom: 5px; }
  .progress-label { font-size: var(--font-size-xs); color: var(--color-text-muted); }
  .progress-value { font-size: var(--font-size-xs); font-weight: var(--font-weight-medium); color: var(--color-text); }
  .progress-value-warn { color: #f59e0b; }
  .progress-track { height: 6px; border-radius: 3px; background: var(--color-border); overflow: hidden; }
  .progress-fill { height: 100%; border-radius: 3px; }
  .fill-purple { background: var(--color-primary); }
  .fill-green { background: var(--color-success); }
  .fill-amber { background: #f59e0b; }
  .progress-footer { display: flex; justify-content: space-between; margin-top: 3px; font-size: 10px; color: var(--color-text-muted); }

  /* Effective tier callout */
  .callout { display: flex; align-items: center; gap: var(--space-3); padding: var(--space-3) var(--space-4); border-radius: var(--radius-md); margin-top: var(--space-3); font-size: var(--font-size-xs); line-height: 1.4; }
  .callout-upgrade { background: #f0fdf4; border: 1px solid #bbf7d0; color: #166534; }
  .callout-muted { background: var(--color-surface); border: 1px solid var(--color-border); color: var(--color-text-muted); }
  .callout-icon { font-size: 14px; flex-shrink: 0; }

  .no-data { font-size: var(--font-size-xs); color: var(--color-text-muted); font-style: italic; }

  /* Transactions */
  .tx-list { display: flex; flex-direction: column; }
  .tx-row { display: flex; justify-content: space-between; align-items: center; padding: var(--space-2) 0; border-bottom: 1px solid var(--color-border); }
  .tx-row:last-child { border-bottom: none; }
  .tx-info { display: flex; flex-direction: column; gap: 1px; }
  .tx-type { font-size: var(--font-size-sm); font-weight: var(--font-weight-medium); color: var(--color-text); }
  .tx-bucket { font-size: var(--font-size-xs); color: var(--color-text-muted); }
  .tx-right { display: flex; flex-direction: column; align-items: flex-end; gap: 1px; }
  .tx-amount { font-size: var(--font-size-sm); font-weight: var(--font-weight-semibold); font-family: var(--font-mono); }
  .tx-amount.credit { color: var(--color-success); }
  .tx-amount.debit { color: var(--color-error); }
  .tx-date { font-size: 10px; color: var(--color-text-muted); }
</style>
