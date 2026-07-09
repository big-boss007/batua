<script lang="ts">
  import { Button, Input, Select, Loader, Shimmer, Modal } from '@juspay/svelte-ui-components';
  import { MODAL_CLOSE_ICON } from '$lib/client/modules/foundation';

  type TierOption = {
    id: string;
    name: string;
    rank: number;
    earn_rate_multiplier: number;
  };

  type CustomerInfo = {
    name: string;
    phone: string;
    walletBalance: number | null;
    currentTierName: string | null;
    currentTierRank: number;
    currentTierMultiplier: number;
    currentExpiry: string | null;
    joinedAt: string | null;
  };

  type AssignResult = {
    tierName: string;
    multiplier: number;
    expiresAt: string;
    previousTierName: string | null;
  };

  let {
    tiers,
    onLookup,
    onSubmit,
    onReset,
    loading = false,
    lookingUp = false,
    lookupError = null,
    customer = null,
    result = null
  }: {
    tiers: Array<TierOption>;
    onLookup: (phone: string) => void;
    onSubmit: (data: { tier_id: string }) => void;
    onReset: () => void;
    loading?: boolean;
    lookingUp?: boolean;
    lookupError?: string | null;
    customer?: CustomerInfo | null;
    result?: AssignResult | null;
  } = $props();

  let phone = $state('');
  let selectedTierId = $state('');
  let showConfirm = $state(false);

  let canLookup = $derived(phone.replace(/\D/g, '').length >= 10);

  let selectedTier = $derived(tiers.find((t) => t.id === selectedTierId) ?? null);

  let isUpgrade = $derived(
    customer !== null && selectedTier !== null && selectedTier.rank > customer.currentTierRank
  );
  let isDowngrade = $derived(
    customer !== null &&
      selectedTier !== null &&
      customer.currentTierRank > 0 &&
      selectedTier.rank < customer.currentTierRank
  );
  let isSameTier = $derived(
    customer !== null &&
      selectedTier !== null &&
      customer.currentTierRank > 0 &&
      selectedTier.rank === customer.currentTierRank
  );
  let isFirstAssignment = $derived(customer !== null && customer.currentTierRank === 0);

  let multiplierDiff = $derived(
    selectedTier !== null && customer !== null
      ? selectedTier.earn_rate_multiplier - customer.currentTierMultiplier
      : 0
  );

  let tierItems = $derived(
    tiers.map((t) => ({
      id: t.id,
      label: `${t.name} (Rank ${t.rank}, ${t.earn_rate_multiplier}x)${customer !== null && t.rank === customer.currentTierRank ? ' — current' : ''}`
    }))
  );

  let canAssign = $derived(customer !== null && selectedTier !== null && !isSameTier);

  let actionLabel = $derived(
    isFirstAssignment
      ? `Assign ${selectedTier?.name ?? ''} Membership`
      : isUpgrade
        ? `Upgrade to ${selectedTier?.name ?? ''}`
        : isDowngrade
          ? `Downgrade to ${selectedTier?.name ?? ''}`
          : 'Assign membership'
  );

  function handleLookup() {
    if (!canLookup) return;
    onLookup(phone.trim());
  }

  function handleSubmitClick() {
    if (!canAssign) return;
    showConfirm = true;
  }

  function handleConfirm() {
    showConfirm = false;
    onSubmit({ tier_id: selectedTierId });
  }

  function handleAssignAnother() {
    phone = '';
    selectedTierId = '';
    showConfirm = false;
    onReset();
  }

  function formatBalance(val: number): string {
    return new Intl.NumberFormat('en-IN', { style: 'currency', currency: 'INR' }).format(val);
  }
</script>

<div class="assign-form">
  <!-- SUCCESS STATE -->
  {#if result !== null && customer !== null}
    <div class="success-card">
      <div class="success-icon">✅</div>
      <div class="success-title">Membership assigned</div>
      <div class="success-sub">
        {customer.name} has been {result.previousTierName !== null ? 'upgraded' : 'assigned'} to {result.tierName}
      </div>

      <div class="success-summary">
        <div class="summary-row">
          <span class="summary-label">Customer</span>
          <span class="summary-value">{customer.name}</span>
        </div>
        <div class="summary-row">
          <span class="summary-label">Tier</span>
          <span class="summary-value">
            <span class="tier-badge">{result.tierName} {result.multiplier}x</span>
          </span>
        </div>
        {#if result.previousTierName !== null}
          <div class="summary-row">
            <span class="summary-label">Previous</span>
            <span class="summary-value old-value">{result.previousTierName}</span>
          </div>
        {/if}
        <div class="summary-row">
          <span class="summary-label">Expires</span>
          <span class="summary-value">{result.expiresAt}</span>
        </div>
        <div class="summary-row">
          <span class="summary-label">Status</span>
          <span class="summary-value"><span class="status-active">Active</span></span>
        </div>
      </div>

      <div class="success-actions">
        <Button text="Assign another" classes="btn-secondary" onclick={handleAssignAnother} />
      </div>
    </div>

    <!-- SAVING STATE -->
  {:else if loading}
    <div class="loading-state">
      {#if customer !== null}
        <div class="preview-card" style="opacity: 0.5;">
          <div class="preview-name">{customer.name}</div>
          <div class="preview-meta">
            <span class="preview-item">{customer.phone}</span>
          </div>
        </div>
      {/if}
      <div class="spinner-wrap">
        <Loader />
        <span class="spinner-text">Assigning {selectedTier?.name ?? ''} membership...</span>
      </div>
    </div>

    <!-- MAIN FORM -->
  {:else}
    <div class="form-title">Assign membership</div>
    <div class="form-sub">Look up a customer by phone to assign a loyalty tier</div>

    <!-- Phone lookup -->
    <div class="form-field">
      <span class="form-label">Customer phone</span>
      <div class="phone-row">
        <Input
          value={phone}
          placeholder="+91 9876543210"
          onInput={(val) => {
            phone = val;
          }}
        />
        {#if customer !== null}
          <Button text="Clear" classes="btn-ghost btn-sm" onclick={handleAssignAnother} />
        {:else}
          <Button
            text={lookingUp ? 'Looking up...' : 'Look Up'}
            classes="btn-secondary"
            disabled={!canLookup || lookingUp}
            onclick={handleLookup}
          />
        {/if}
      </div>
      {#if customer === null && !lookingUp && lookupError === null}
        <span class="form-hint">Enter a registered customer's phone number</span>
      {/if}
    </div>

    <!-- Lookup loading -->
    {#if lookingUp}
      <div class="preview-card">
        <Shimmer classes="shimmer-name" />
        <div style="display: flex; gap: 12px;">
          <Shimmer classes="shimmer-meta" />
          <Shimmer classes="shimmer-meta-sm" />
        </div>
      </div>
    {/if}

    <!-- Lookup error -->
    {#if lookupError !== null}
      <div class="error-box">
        <strong>Customer not found</strong> — {lookupError}
      </div>
    {/if}

    <!-- Customer found -->
    {#if customer !== null}
      <div class="preview-card">
        <div class="preview-name">{customer.name}</div>
        <div class="preview-meta">
          {#if customer.walletBalance !== null}
            <span class="preview-item">Wallet: {formatBalance(customer.walletBalance)}</span>
          {/if}
          <span class="preview-item">
            Current:
            {#if customer.currentTierName !== null}
              <span class="tier-badge">{customer.currentTierName}</span>
            {:else}
              <span class="no-tier-badge">No tier</span>
            {/if}
          </span>
          {#if customer.joinedAt !== null}
            <span class="preview-item">📅 Joined: {customer.joinedAt}</span>
          {/if}
        </div>
      </div>

      <!-- Tier + expiry selection -->
      <div class="form-grid">
        <div class="form-field">
          <span class="form-label">Assign tier</span>
          <Select
            items={tierItems}
            value={selectedTierId ? [selectedTierId] : []}
            onchange={(vals) => {
              selectedTierId = vals[0] ?? '';
            }}
            placeholder="Select tier..."
          />
        </div>
      </div>

      <!-- Tier comparison -->
      {#if selectedTier !== null && !isSameTier}
        {#if isFirstAssignment}
          <div class="info-box">
            <strong>First membership</strong> — This customer has no existing tier. They'll start
            earning at the {selectedTier.name}
            {selectedTier.earn_rate_multiplier}x multiplier immediately.
          </div>
        {:else if isUpgrade}
          <div class="compare-box upgrade">
            <div class="compare-title">⬆ Tier Upgrade</div>
            <div class="compare-row">
              <span class="tier-badge"
                >{customer.currentTierName} {customer.currentTierMultiplier}x</span
              >
              <span class="compare-arrow">→</span>
              <span class="tier-badge"
                >{selectedTier.name} {selectedTier.earn_rate_multiplier}x</span
              >
              <span class="compare-benefit">+{multiplierDiff.toFixed(1)}x earn rate boost</span>
            </div>
          </div>
        {:else if isDowngrade}
          <div class="compare-box downgrade">
            <div class="compare-title warn">⬇ Tier Downgrade</div>
            <div class="compare-row">
              <span class="tier-badge"
                >{customer.currentTierName} {customer.currentTierMultiplier}x</span
              >
              <span class="compare-arrow warn">→</span>
              <span class="tier-badge"
                >{selectedTier.name} {selectedTier.earn_rate_multiplier}x</span
              >
              <span class="compare-loss">{multiplierDiff.toFixed(1)}x earn rate</span>
            </div>
            {#if customer.currentExpiry !== null}
              <div class="compare-note">
                Current {customer.currentTierName} membership (expires {customer.currentExpiry})
                will be replaced.
              </div>
            {/if}
          </div>
        {/if}
      {/if}

      {#if isSameTier}
        <div class="info-box muted">
          Customer is already at {customer.currentTierName}. Select a different tier.
        </div>
      {/if}

      <!-- Action button -->
      <div class="form-actions">
        <Button text="Cancel" classes="btn-ghost" onclick={handleAssignAnother} />
        <Button
          text={actionLabel}
          classes={isDowngrade ? 'btn-danger' : 'btn-primary'}
          disabled={!canAssign}
          onclick={handleSubmitClick}
        />
      </div>
    {/if}

    <!-- Empty state -->
    {#if customer === null && !lookingUp && lookupError === null}
      <div class="empty-prompt">Look up a customer to get started</div>
    {/if}
  {/if}
</div>

<!-- Confirmation modal -->
{#if showConfirm && selectedTier !== null && customer !== null}
  <Modal
    size="medium"
    showOverlay={true}
    header={{
      text: isFirstAssignment
        ? 'Confirm assignment'
        : isUpgrade
          ? 'Confirm upgrade'
          : 'Confirm downgrade',
      rightImage: MODAL_CLOSE_ICON
    }}
    onclose={() => (showConfirm = false)}
    onoverlayClick={() => (showConfirm = false)}
    onheaderRightImageClick={() => (showConfirm = false)}
  >
    {#snippet content()}
      <div class="modal-body">
        <p class="modal-text">
          You are about to {isFirstAssignment ? 'assign' : isUpgrade ? 'upgrade' : 'downgrade'}
          <strong>{customer.name}</strong>
          {#if !isFirstAssignment}
            from <span class="tier-badge sm">{customer.currentTierName}</span>
          {/if}
          to <span class="tier-badge sm">{selectedTier.name}</span>.
        </p>
        <ul class="modal-details">
          <li>Earn rate multiplier: {selectedTier.earn_rate_multiplier}x</li>
          <li>Expires: 1 year from today</li>
          {#if !isFirstAssignment}
            <li>Current membership will be replaced</li>
          {/if}
        </ul>
      </div>
      <div class="modal-footer">
        <Button text="Cancel" classes="btn-ghost" onclick={() => (showConfirm = false)} />
        <Button
          text={isDowngrade ? 'Confirm downgrade' : 'Confirm'}
          classes={isDowngrade ? 'btn-danger' : 'btn-primary'}
          onclick={handleConfirm}
        />
      </div>
    {/snippet}
  </Modal>
{/if}

<style>
  .assign-form {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    max-width: 520px;
  }

  .form-title {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .form-sub {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    margin-top: calc(-1 * var(--space-2));
  }

  .form-field {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .form-label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-text-muted);
  }

  .form-hint {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .phone-row {
    display: flex;
    gap: var(--space-2);
    align-items: flex-start;
  }

  .form-grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: var(--space-3);
  }

  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: var(--space-3);
    padding-top: var(--space-2);
  }

  /* Customer preview */
  .preview-card {
    padding: var(--space-4);
    background: var(--color-surface);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-card);
  }

  .preview-name {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
    margin-bottom: var(--space-2);
  }

  .preview-meta {
    display: flex;
    gap: var(--space-4);
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    flex-wrap: wrap;
  }

  .preview-item {
    display: flex;
    align-items: center;
    gap: var(--space-1);
  }

  .tier-badge {
    display: inline-block;
    padding: 2px 8px;
    border-radius: 10px;
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-semibold);
    background: var(--color-surface-2);
    color: var(--color-text);
  }

  .tier-badge.sm {
    font-size: 10px;
    padding: 1px 6px;
  }

  .no-tier-badge {
    display: inline-block;
    padding: 2px 8px;
    border-radius: 10px;
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-medium);
    background: var(--color-surface-2);
    color: var(--color-text-muted);
  }

  /* Comparison boxes */
  .compare-box {
    padding: var(--space-3) var(--space-4);
    border-radius: var(--radius-md);
  }

  .compare-box.upgrade {
    background: var(--green-100);
    border: 1px solid var(--green-100);
  }

  .compare-box.downgrade {
    background: var(--yellow-100);
    border: 1px solid var(--yellow-100);
  }

  .compare-title {
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-semibold);
    color: var(--green-700);
    margin-bottom: var(--space-2);
  }

  .compare-title.warn {
    color: var(--yellow-700);
  }

  .compare-row {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    font-size: var(--font-size-sm);
    flex-wrap: wrap;
  }

  .compare-arrow {
    color: var(--green-500);
    font-weight: var(--font-weight-semibold);
  }

  .compare-arrow.warn {
    color: var(--yellow-500);
  }

  .compare-benefit {
    font-size: var(--font-size-xs);
    color: var(--green-500);
    font-weight: var(--font-weight-medium);
    margin-left: var(--space-2);
  }

  .compare-loss {
    font-size: var(--font-size-xs);
    color: var(--yellow-700);
    font-weight: var(--font-weight-medium);
    margin-left: var(--space-2);
  }

  .compare-note {
    font-size: var(--font-size-xs);
    color: var(--yellow-700);
    margin-top: var(--space-2);
  }

  /* Info / error boxes */
  .info-box {
    padding: var(--space-3) var(--space-4);
    background: var(--p-100);
    border: 1px solid var(--p-100);
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
    color: var(--p-700);
  }

  .info-box.muted {
    background: var(--color-surface);
    border-color: var(--color-border);
    color: var(--color-text-muted);
  }

  .error-box {
    padding: var(--space-3) var(--space-4);
    background: var(--red-100);
    border: 1px solid var(--red-100);
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
    color: var(--red-700);
  }

  /* Success */
  .success-card {
    text-align: center;
    padding: var(--space-8) var(--space-4);
  }

  .success-icon {
    font-size: 40px;
    margin-bottom: var(--space-3);
  }

  .success-title {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
    color: var(--green-700);
    margin-bottom: var(--space-1);
  }

  .success-sub {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    margin-bottom: var(--space-5);
  }

  .success-summary {
    display: inline-block;
    text-align: left;
    padding: var(--space-4);
    background: var(--color-surface);
    border-radius: var(--radius-lg);
    margin-bottom: var(--space-5);
    box-shadow: var(--shadow-card);
  }

  .summary-row {
    display: flex;
    justify-content: space-between;
    gap: var(--space-8);
    padding: var(--space-1) 0;
    font-size: var(--font-size-sm);
  }

  .summary-label {
    color: var(--color-text-muted);
  }

  .summary-value {
    font-weight: var(--font-weight-medium);
    color: var(--color-text);
  }

  .old-value {
    text-decoration: line-through;
    color: var(--color-text-muted);
  }

  .status-active {
    display: inline-block;
    padding: 2px 8px;
    border-radius: 10px;
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-semibold);
    background: var(--green-100);
    color: var(--green-700);
  }

  .success-actions {
    display: flex;
    gap: var(--space-3);
    justify-content: center;
  }

  /* Loading */
  .loading-state {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .spinner-wrap {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-6) 0;
  }

  .spinner-text {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  :global(.shimmer-name) {
    --shimmer-width: 140px;
    --shimmer-height: 14px;
    margin-bottom: 10px;
  }
  :global(.shimmer-meta) {
    --shimmer-width: 100px;
    --shimmer-height: 12px;
  }
  :global(.shimmer-meta-sm) {
    --shimmer-width: 80px;
    --shimmer-height: 12px;
  }

  /* Empty */
  .empty-prompt {
    padding: var(--space-8);
    text-align: center;
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
    border: 1px dashed var(--color-border);
    border-radius: var(--radius-md);
  }

  .modal-text {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    line-height: 1.6;
    margin-bottom: var(--space-3);
  }

  .modal-details {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    margin-left: var(--space-4);
    line-height: 1.8;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: var(--space-3);
    padding: var(--space-4) var(--space-5);
  }
</style>
