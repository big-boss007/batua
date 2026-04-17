<script lang="ts">
  import { Avatar, Button, Input, Pill, Progress } from '@juspay/svelte-ui-components';
  import type { CustomerDetail as CustomerDetailType, BucketBalance, WalletActionType, WalletUnitType } from '$lib/client/modules/customers';
  import { formatMovementType, formatBucketType, updateCustomer } from '$lib/client/modules/customers';
  import WalletActionModal from './WalletActionModal.svelte';
  import {
    formatCurrencyINR,
    formatDate,
    formatDateTime,
    formatPhone,
    formatPoints,
    isPointsBucket,
    toastStore
  } from '$lib/client/modules/foundation';

  let {
    detail,
    merchantId = '',
    pointsIcon = '★',
    pointsRate = 1.0,
    onRefresh
  }: {
    detail: CustomerDetailType;
    merchantId?: string;
    pointsIcon?: string;
    pointsRate?: number;
    onRefresh?: () => void;
  } = $props();

  let showWalletAction = $state(false);
  let walletActionType: WalletActionType = $state('add');
  let walletActionUnit: WalletUnitType = $state('cash');
  let showPointsMenu = $state(false);
  let showCashMenu = $state(false);

  let showEditForm = $state(false);
  let editName = $state('');
  let editEmail = $state('');
  let saving = $state(false);

  function openEditForm() {
    editName = detail.customer.name ?? '';
    editEmail = detail.customer.email ?? '';
    showEditForm = true;
  }

  function cancelEdit() {
    showEditForm = false;
  }

  async function saveCustomer() {
    if (saving) return;
    saving = true;
    const result = await updateCustomer(merchantId, detail.customer.id, {
      name: editName.trim() || undefined,
      email: editEmail.trim() || undefined
    });
    if (result.tag === 'success') {
      toastStore.push({ message: 'Customer updated', level: 'success' });
      showEditForm = false;
      if (onRefresh) onRefresh();
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }
    saving = false;
  }

  function openWalletAction(actionType: WalletActionType, unitType: WalletUnitType) {
    walletActionType = actionType;
    walletActionUnit = unitType;
    showWalletAction = true;
    showPointsMenu = false;
    showCashMenu = false;
  }

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
      <div class="meta-right"><span class="meta-label">Customer since</span><div class="meta-value">{formatDate(customer.created_at)}</div></div>
    </div>
    {#if !showEditForm}
      <div class="edit-link-row">
        <button class="edit-link" onclick={openEditForm}>&#9998; Edit customer details</button>
      </div>
    {/if}
  </section>

  {#if showEditForm}
    <section class="card">
      <h3 class="card-title">&#9998; Edit Customer Details</h3>
      <div class="edit-form">
        <div class="edit-field">
          <span class="edit-label">Name</span>
          <Input value={editName} placeholder="Customer name" onInput={(val) => { editName = val; }} />
        </div>
        <div class="edit-field">
          <span class="edit-label">Email</span>
          <Input value={editEmail} placeholder="email@example.com" onInput={(val) => { editEmail = val; }} />
        </div>
        <div class="edit-field">
          <span class="edit-label">Phone</span>
          <div class="edit-readonly">{formatPhone(customer.phone)}</div>
          <span class="edit-hint">Phone cannot be changed</span>
        </div>
        <div class="edit-actions">
          <Button text="Cancel" classes="btn-ghost" onclick={cancelEdit} />
          <Button text={saving ? 'Saving...' : 'Save Changes'} classes="btn-primary" disabled={saving} onclick={saveCustomer} />
        </div>
      </div>
    </section>
  {/if}

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
            <div class="wd-value-line">
              <span class="wd-row-worth">worth {formatCurrencyINR(wallet.points_balance * pointsRate)}</span>
              <span class="wd-row-value" style="color: var(--color-primary);">{formatPoints(wallet.points_balance, pointsIcon)}</span>
            </div>
            {#if pointsSubBuckets.length > 0}
              {@const pointsTotal = pointsSubBuckets.reduce((s, b) => s + b.spendable, 0)}
              <div class="wd-bar">
                {#each pointsSubBuckets as b, i}
                  <div class="wd-bar-seg" style="flex: {Math.round(b.spendable)}; background: var(--color-points-{i});"></div>
                {/each}
              </div>
              <div class="wd-legend">
                {#each pointsSubBuckets as b, i}
                  <span class="wd-legend-item"><span class="wd-legend-dot" style="background: var(--color-points-{i});"></span>{bucketLabel(b.bucket_type)} <span class="wd-legend-val">{Math.round(b.spendable)}</span></span>
                {/each}
              </div>
            {/if}
          </div>
          <div class="wd-row-actions">
            <div class="wd-menu-wrap">
              <button class="wd-more-btn" onclick={() => { showPointsMenu = !showPointsMenu; showCashMenu = false; }}>&#8943;</button>
              {#if showPointsMenu}
                <div class="wd-dropdown">
                  <button class="wd-dropdown-item wd-dropdown-green" onclick={() => openWalletAction('add', 'points')}>Add</button>
                  {#if wallet.points_balance > 0}
                    <button class="wd-dropdown-item wd-dropdown-red" onclick={() => openWalletAction('remove', 'points')}>Remove</button>
                    <button class="wd-dropdown-item wd-dropdown-amber" onclick={() => openWalletAction('expire', 'points')}>Expire</button>
                  {/if}
                </div>
              {/if}
            </div>
          </div>
        </div>
        <div class="wd-row">
          <div class="wd-row-left">
            <span class="wd-dot" style="background: var(--color-success);"></span>
            <span class="wd-row-label">Cash</span>
          </div>
          <div class="wd-row-right">
            <div class="wd-value-line">
              <span class="wd-row-value" style="color: var(--color-success);">{formatCurrencyINR(wallet.cash_balance)}</span>
            </div>
            {#if pointsCashBuckets.length > 0}
              <div class="wd-bar">
                {#each pointsCashBuckets as b, i}
                  <div class="wd-bar-seg" style="flex: {Math.round(b.spendable)}; background: var(--color-cash-{i});"></div>
                {/each}
              </div>
              <div class="wd-legend">
                {#each pointsCashBuckets as b, i}
                  <span class="wd-legend-item"><span class="wd-legend-dot" style="background: var(--color-cash-{i});"></span>{bucketLabel(b.bucket_type)} <span class="wd-legend-val">{formatCurrencyINR(b.spendable)}</span></span>
                {/each}
              </div>
            {:else}
              <span class="wd-no-balance">No cash balance</span>
            {/if}
          </div>
          <div class="wd-row-actions">
            <div class="wd-menu-wrap">
              <button class="wd-more-btn" onclick={() => { showCashMenu = !showCashMenu; showPointsMenu = false; }}>&#8943;</button>
              {#if showCashMenu}
                <div class="wd-dropdown">
                  <button class="wd-dropdown-item wd-dropdown-green" onclick={() => openWalletAction('add', 'cash')}>Add</button>
                  {#if wallet.cash_balance > 0}
                    <button class="wd-dropdown-item wd-dropdown-red" onclick={() => openWalletAction('remove', 'cash')}>Remove</button>
                    <button class="wd-dropdown-item wd-dropdown-amber" onclick={() => openWalletAction('expire', 'cash')}>Expire</button>
                  {/if}
                </div>
              {/if}
            </div>
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
          <Progress value={tier.progress_to_next.percentage} classes="progress-bar-purple" />
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
        <Progress value={Math.min(100, (membership.days_remaining / 365) * 100)} classes={membershipExpiring ? 'progress-bar-amber' : 'progress-bar-green'} />
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

{#if showWalletAction}
  <WalletActionModal
    {detail}
    {merchantId}
    initialAction={walletActionType}
    initialUnit={walletActionUnit}
    {pointsRate}
    {pointsIcon}
    onClose={() => { showWalletAction = false; }}
    onSuccess={() => { if (onRefresh) onRefresh(); }}
  />
{/if}

<style>
  .customer-detail { display: flex; flex-direction: column; gap: var(--space-4); width: 100%; }

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
  .meta-value { font-size: var(--font-size-sm); font-weight: var(--font-weight-medium); color: var(--color-text); margin-top: 2px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .meta-right { text-align: right; }
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
    align-items: flex-start;
    justify-content: space-between;
    padding: var(--space-3) var(--space-3);
    background: var(--color-surface-2);
    border-radius: var(--radius-md);
  }
  .wd-row-left { display: flex; align-items: center; gap: var(--space-2); }
  .wd-dot { width: 10px; height: 10px; border-radius: 50%; flex-shrink: 0; margin-top: 5px; }
  .wd-row-label { font-size: var(--font-size-sm); font-weight: var(--font-weight-semibold); color: var(--color-text); }
  .wd-row-right { display: flex; flex-direction: column; align-items: flex-end; flex: 1; min-width: 0; }
  .wd-value-line { display: flex; align-items: baseline; gap: var(--space-2); justify-content: flex-end; }
  .wd-row-value { font-size: var(--font-size-base); font-weight: var(--font-weight-bold); }
  .wd-row-worth { font-size: 11px; color: var(--color-text-muted); }
  .wd-no-balance { font-size: 11px; color: var(--color-text-muted); }
  .wd-bar { display: flex; height: 6px; border-radius: 3px; overflow: hidden; gap: 1px; margin-top: 8px; width: 100%; }
  .wd-bar-seg { height: 100%; border-radius: 3px; min-width: 4px; }
  .wd-legend { display: flex; flex-wrap: wrap; gap: 8px; margin-top: 6px; justify-content: flex-end; }
  .wd-legend-item { display: flex; align-items: center; gap: 3px; font-size: 10px; color: var(--color-text-muted); white-space: nowrap; }
  .wd-legend-dot { width: 6px; height: 6px; border-radius: 50%; flex-shrink: 0; }
  .wd-legend-val { font-weight: var(--font-weight-semibold); color: var(--color-text); }

  .wd-row { --color-points-0: #818cf8; --color-points-1: #a78bfa; --color-points-2: #c4b5fd; --color-points-3: #ddd6fe; --color-points-4: #ede9fe; }
  .wd-row { --color-cash-0: #34d399; --color-cash-1: #6ee7b7; --color-cash-2: #a7f3d0; --color-cash-3: #d1fae5; --color-cash-4: #ecfdf5; }

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
  :global(.progress-bar-purple) { --progress-fill-color: var(--color-primary); --progress-height: 6px; --progress-border-radius: 3px; --progress-background-color: var(--color-border); }
  :global(.progress-bar-green) { --progress-fill-color: var(--color-success); --progress-height: 6px; --progress-border-radius: 3px; --progress-background-color: var(--color-border); }
  :global(.progress-bar-amber) { --progress-fill-color: #f59e0b; --progress-height: 6px; --progress-border-radius: 3px; --progress-background-color: var(--color-border); }
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

  /* Wallet row actions */
  .wd-row-actions { display: flex; align-items: center; gap: 6px; margin-left: 8px; flex-shrink: 0; }
  .wd-menu-wrap { position: relative; }
  .wd-more-btn {
    width: 24px; height: 24px; border-radius: 5px; border: 1px solid var(--color-border);
    background: transparent; cursor: pointer; font-size: 14px; color: var(--color-text-muted);
    display: flex; align-items: center; justify-content: center; font-family: inherit;
  }
  .wd-more-btn:hover { background: var(--color-surface-2); }
  .wd-dropdown {
    position: absolute; right: 0; top: 28px; z-index: 10;
    background: var(--color-surface); border: 1px solid var(--color-border);
    border-radius: var(--radius-md); box-shadow: 0 4px 16px rgba(0,0,0,0.1);
    padding: 4px; min-width: 120px;
  }
  .wd-dropdown-item {
    display: block; width: 100%; padding: 6px 12px; border-radius: 5px;
    font-size: var(--font-size-sm); cursor: pointer; border: none;
    background: transparent; text-align: left; font-family: inherit;
  }
  .wd-dropdown-item:hover { background: var(--color-surface-2); }
  .wd-dropdown-green { color: var(--color-success); }
  .wd-dropdown-red { color: var(--color-error); }
  .wd-dropdown-amber { color: #92400e; }

  /* Edit form */
  .edit-link-row { margin-top: var(--space-3); padding-top: var(--space-3); border-top: 1px solid var(--color-border); }
  .edit-link {
    font-size: var(--font-size-xs); color: var(--color-primary); cursor: pointer; font-weight: var(--font-weight-medium);
    background: none; border: none; padding: 0; font-family: inherit; display: inline-flex; align-items: center; gap: 4px;
  }
  .edit-link:hover { text-decoration: underline; }
  .edit-form { display: flex; flex-direction: column; gap: var(--space-3); }
  .edit-field { display: flex; flex-direction: column; gap: var(--space-1); }
  .edit-label { font-size: 11px; font-weight: var(--font-weight-semibold); color: var(--color-text-muted); text-transform: uppercase; letter-spacing: 0.04em; }
  .edit-readonly {
    font-size: var(--font-size-sm); padding: 8px 12px; border: 1px solid var(--color-border);
    border-radius: var(--radius-md); background: var(--color-surface-2); color: var(--color-text-muted);
    font-family: var(--font-mono);
  }
  .edit-hint { font-size: 11px; color: var(--color-text-muted); }
  .edit-actions { display: flex; justify-content: flex-end; gap: var(--space-2); margin-top: var(--space-1); }
</style>
