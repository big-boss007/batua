<script lang="ts">
  import { SvelteSet } from 'svelte/reactivity';
  import { Modal, Button, Avatar, Toggle } from '@juspay/svelte-ui-components';
  import Icon from '$lib/components/Icon.svelte';
  import {
    MODAL_CLOSE_ICON,
    formatCurrencyINR,
    formatPoints,
    isPointsBucket
  } from '$lib/client/modules/foundation';
  import { addCredit, removeBalance, expireBalance } from '$lib/client/modules/customers';
  import type {
    CustomerDetail,
    BucketBalance,
    WalletActionType,
    WalletUnitType,
    WalletActionStep,
    WalletActionResult
  } from '$lib/client/modules/customers';

  let {
    detail,
    merchantId,
    initialAction = 'add',
    initialUnit = 'cash',
    pointsRate = 0.1,
    pointsIcon = '★',
    onClose,
    onSuccess
  }: {
    detail: CustomerDetail;
    merchantId: string;
    initialAction?: WalletActionType;
    initialUnit?: WalletUnitType;
    pointsRate?: number;
    pointsIcon?: string;
    onClose: () => void;
    onSuccess: () => void;
  } = $props();

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

  const ADD_CASH_BUCKETS = ['RefundCredit'];
  const ADD_POINTS_BUCKETS = ['GoodwillCredit'];

  const REASON_PILLS: Record<WalletActionType, Array<string>> = {
    add: ['Refund', 'Goodwill', 'Apology', 'Compensation', 'Promotional', 'Other'],
    remove: ['Incorrect', 'Fraud/abuse', 'Chargeback', 'Dispute', 'Correction', 'Other'],
    expire: ['Campaign ended', 'Policy change', 'Fraud/abuse', 'Account cleanup', 'Other']
  };

  // svelte-ignore state_referenced_locally
  let action: WalletActionType = $state(initialAction);
  // svelte-ignore state_referenced_locally
  let unit: WalletUnitType = $state(initialUnit);
  let step: WalletActionStep = $state('form');

  // svelte-ignore state_referenced_locally
  let selectedBucket: string = $state(
    initialAction === 'add'
      ? initialUnit === 'cash'
        ? ADD_CASH_BUCKETS[0]
        : ADD_POINTS_BUCKETS[0]
      : ''
  );
  let selectedExpireBuckets = $state(new SvelteSet<string>());
  let amountStr: string = $state('');
  let expiryDaysStr: string = $state('');
  let reasonCategory: string = $state('');
  let reasonText: string = $state('');
  let reference: string = $state('');
  let notifyCustomer: boolean = $state(true);
  let confirmInput: string = $state('');
  let errorMessage: string | null = $state(null);
  let result: WalletActionResult | null = $state(null);

  let customer = $derived(detail.customer);
  let wallet = $derived(detail.wallet);

  let currentBalance = $derived(
    wallet !== null ? (unit === 'cash' ? wallet.cash_balance : wallet.points_balance) : 0
  );

  let availableBuckets = $derived.by((): Array<BucketBalance> => {
    if (wallet === null) return [];
    if (action === 'add') {
      const allowed = unit === 'cash' ? ADD_CASH_BUCKETS : ADD_POINTS_BUCKETS;
      return allowed.map((bt) => {
        const existing = wallet.buckets.find((b) => b.bucket_type === bt);
        return { bucket_type: bt, spendable: existing?.spendable ?? 0 };
      });
    }
    return wallet.buckets.filter((b) => {
      const isPoints = isPointsBucket(b.bucket_type);
      return (unit === 'points' ? isPoints : !isPoints) && b.spendable > 0;
    });
  });

  let selectedBucketBalance = $derived(
    wallet?.buckets.find((b) => b.bucket_type === selectedBucket)?.spendable ?? 0
  );

  let parsedAmount = $derived(parseFloat(amountStr) || 0);

  let expireTotalAmount = $derived(
    availableBuckets
      .filter((b) => selectedExpireBuckets.has(b.bucket_type))
      .reduce((sum, b) => sum + b.spendable, 0)
  );

  let isAmountValid = $derived.by(() => {
    if (action === 'expire') return selectedExpireBuckets.size > 0;
    if (parsedAmount <= 0) return false;
    if (action === 'remove' && parsedAmount > selectedBucketBalance) return false;
    return true;
  });

  let isFormValid = $derived(
    isAmountValid &&
      reasonCategory.length > 0 &&
      reasonText.length > 0 &&
      (action !== 'expire' || selectedExpireBuckets.size > 0) &&
      (action === 'expire' || selectedBucket.length > 0)
  );

  let previewNewBalance = $derived.by(() => {
    if (action === 'add') return currentBalance + parsedAmount;
    if (action === 'remove') return currentBalance - parsedAmount;
    return currentBalance - expireTotalAmount;
  });

  let isConfirmValid = $derived.by(() => {
    if (action === 'add') return true;
    if (action === 'remove') return confirmInput === String(parsedAmount);
    return confirmInput.toUpperCase() === 'EXPIRE';
  });

  let actionColor = $derived(action === 'add' ? 'green' : action === 'remove' ? 'red' : 'amber');

  let actionBtnClass = $derived(
    action === 'add' ? 'btn-success' : action === 'remove' ? 'btn-danger' : 'btn-warning'
  );

  let actionButtonLabel = $derived.by(() => {
    if (action === 'add') {
      if (unit === 'cash') return `Add ${formatCurrencyINR(parsedAmount)}`;
      return `Add ${Math.round(parsedAmount)} Points`;
    }
    if (action === 'remove') {
      if (unit === 'cash') return `Remove ${formatCurrencyINR(parsedAmount)}`;
      return `Remove ${Math.round(parsedAmount)} Points`;
    }
    if (unit === 'cash') return `Expire ${formatCurrencyINR(expireTotalAmount)}`;
    return `Expire ${Math.round(expireTotalAmount)} Points`;
  });

  const BUCKET_TO_SNAKE: Record<string, string> = {
    EarnedCredit: 'earned_credit',
    CodPending: 'cod_pending',
    GiftCard: 'gift_card',
    CustomerFunded: 'customer_funded',
    ReferralReward: 'referral_reward',
    GoodwillCredit: 'goodwill_credit',
    MembershipBenefit: 'membership_benefit',
    RefundCredit: 'refund_credit'
  };

  function toSnakeBucket(bt: string): string {
    return BUCKET_TO_SNAKE[bt] ?? bt;
  }

  function bucketLabel(bt: string): string {
    return BUCKET_LABELS[bt] ?? bt;
  }

  function formatBalance(amount: number): string {
    if (unit === 'cash') return formatCurrencyINR(amount);
    return formatPoints(amount, pointsIcon);
  }

  function switchAction(newAction: WalletActionType) {
    action = newAction;
    step = 'form';
    selectedBucket =
      newAction === 'add' ? (unit === 'cash' ? ADD_CASH_BUCKETS[0] : ADD_POINTS_BUCKETS[0]) : '';
    selectedExpireBuckets = new SvelteSet();
    amountStr = '';
    expiryDaysStr = '';
    reasonCategory = '';
    reasonText = '';
    reference = '';
    confirmInput = '';
    errorMessage = null;
    result = null;
  }

  function toggleExpireBucket(bt: string) {
    if (selectedExpireBuckets.has(bt)) selectedExpireBuckets.delete(bt);
    else selectedExpireBuckets.add(bt);
  }

  function handleSubmit() {
    if (!isFormValid) return;
    step = 'confirm';
    confirmInput = '';
  }

  async function handleConfirm() {
    if (!isConfirmValid) return;
    step = 'loading';
    errorMessage = null;

    try {
      if (action === 'add') {
        const res = await addCredit(merchantId, {
          merchant_id: merchantId,
          customer_id: customer.id,
          bucket_type: toSnakeBucket(selectedBucket),
          amount: parsedAmount,
          expiry_days: expiryDaysStr ? parseInt(expiryDaysStr) : null,
          reason_category: reasonCategory,
          reason_text: reasonText,
          reference: reference || null
        });
        if (res.tag === 'success') {
          result = res.data;
          step = 'success';
        } else {
          errorMessage = res.message ?? 'Failed to add';
          step = 'error';
        }
      } else if (action === 'remove') {
        const res = await removeBalance(merchantId, {
          merchant_id: merchantId,
          customer_id: customer.id,
          bucket_type: toSnakeBucket(selectedBucket),
          amount: parsedAmount,
          reason_category: reasonCategory,
          reason_text: reasonText,
          reference: reference || null
        });
        if (res.tag === 'success') {
          result = res.data;
          step = 'success';
        } else {
          errorMessage = res.message ?? 'Failed to remove';
          step = 'error';
        }
      } else {
        const res = await expireBalance(merchantId, {
          merchant_id: merchantId,
          customer_id: customer.id,
          bucket_types: [...selectedExpireBuckets].map(toSnakeBucket),
          reason_category: reasonCategory,
          reason_text: reasonText,
          reference: reference || null
        });
        if (res.tag === 'success') {
          result = res.data;
          step = 'success';
        } else {
          errorMessage = res.message ?? 'Failed to expire';
          step = 'error';
        }
      }
    } catch {
      errorMessage = 'Network error. Please try again.';
      step = 'error';
    }
  }

  function handleDone() {
    onSuccess();
    onClose();
  }

  function initials(name: string | null): string {
    if (name === null || name.length === 0) return 'U';
    return name
      .split(' ')
      .map((w) => w[0])
      .join('')
      .toUpperCase()
      .slice(0, 2);
  }
</script>

<Modal
  size="fit-content"
  header={{ text: 'Wallet action', rightImage: MODAL_CLOSE_ICON }}
  onclose={onClose}
  onoverlayClick={onClose}
  onheaderRightImageClick={onClose}
  classes="wallet-action-modal"
>
  {#snippet content()}
    <div class="wa-body">
      {#if step === 'form' || step === 'error'}
        <!-- Action Tabs -->
        <div class="wa-tabs">
          <button
            class="wa-tab"
            class:wa-tab-add={action === 'add'}
            onclick={() => switchAction('add')}>Add</button
          >
          <button
            class="wa-tab"
            class:wa-tab-remove={action === 'remove'}
            onclick={() => switchAction('remove')}>Remove</button
          >
          <button
            class="wa-tab"
            class:wa-tab-expire={action === 'expire'}
            onclick={() => switchAction('expire')}>Expire</button
          >
        </div>

        <!-- Error Banner -->
        {#if step === 'error' && errorMessage}
          <div class="wa-error-banner">
            <span class="wa-error-icon">&#9888;</span>
            <div class="wa-error-text">{errorMessage}</div>
          </div>
        {/if}

        <!-- Customer Banner -->
        <div class="wa-customer">
          <Avatar name={initials(customer.name)} alt={customer.name ?? 'User'} size="small" />
          <div class="wa-customer-info">
            <div class="wa-customer-name">{customer.name ?? 'Unnamed'}</div>
            <div class="wa-customer-phone">{customer.phone}</div>
          </div>
          <div class="wa-customer-balance">
            <div
              class="wa-balance-val"
              class:points={unit === 'points'}
              class:cash={unit === 'cash'}
            >
              {formatBalance(currentBalance)}
            </div>
            <div class="wa-balance-label">current {unit} balance</div>
          </div>
        </div>

        <!-- Expire: irreversible warning -->
        {#if action === 'expire'}
          <div class="wa-warning-banner">
            <span>&#9888;&#65039;</span>
            <div>
              <strong>Irreversible action.</strong> All balance in selected bucket(s) will be permanently
              expired.
            </div>
          </div>
        {/if}

        <!-- Bucket Selector -->
        <div class="wa-field">
          <span class="wa-label">
            {action === 'expire'
              ? 'Select bucket(s) to expire'
              : action === 'remove'
                ? 'Remove from bucket'
                : 'Bucket'}
            <span class="wa-req">*</span>
          </span>

          {#if action === 'expire'}
            <div class="wa-bucket-list">
              {#each availableBuckets as bucket (bucket.bucket_type)}
                <button
                  class="wa-expire-card"
                  class:selected={selectedExpireBuckets.has(bucket.bucket_type)}
                  onclick={() => toggleExpireBucket(bucket.bucket_type)}
                >
                  <span
                    class="wa-check"
                    class:checked={selectedExpireBuckets.has(bucket.bucket_type)}
                  >
                    {#if selectedExpireBuckets.has(bucket.bucket_type)}&#10003;{/if}
                  </span>
                  <span class="wa-bucket-name">{bucketLabel(bucket.bucket_type)}</span>
                  <span class="wa-bucket-bal">{formatBalance(bucket.spendable)}</span>
                </button>
              {/each}
              {#if availableBuckets.length === 0}
                <div class="wa-no-data">No buckets with balance to expire</div>
              {/if}
            </div>
          {:else}
            <div class="wa-bucket-list">
              {#each availableBuckets as bucket (bucket.bucket_type)}
                <button
                  class="wa-radio-card"
                  class:selected={selectedBucket === bucket.bucket_type}
                  onclick={() => {
                    selectedBucket = bucket.bucket_type;
                  }}
                >
                  <span class="wa-radio" class:checked={selectedBucket === bucket.bucket_type}
                  ></span>
                  <span class="wa-bucket-name">{bucketLabel(bucket.bucket_type)}</span>
                  {#if action === 'remove'}
                    <span class="wa-bucket-bal">{formatBalance(bucket.spendable)}</span>
                  {/if}
                </button>
              {/each}
              {#if availableBuckets.length === 0}
                <div class="wa-no-data">No buckets available</div>
              {/if}
            </div>
          {/if}
        </div>

        <!-- Amount + Expiry (not for expire action) -->
        {#if action !== 'expire'}
          <div class="wa-row">
            <div class="wa-field">
              <span class="wa-label">
                {unit === 'cash' ? 'Amount (₹)' : 'Points'} <span class="wa-req">*</span>
              </span>
              <div class="wa-amount-wrap">
                {#if unit === 'cash'}
                  <span class="wa-amount-prefix">₹</span>
                {/if}
                <input
                  class="wa-input wa-amount-input"
                  class:cash-input={unit === 'cash'}
                  type="number"
                  step="0.01"
                  min="0"
                  placeholder="0"
                  bind:value={amountStr}
                />
                {#if unit === 'points'}
                  <span class="wa-amount-suffix">points</span>
                {/if}
              </div>
              {#if action === 'remove' && selectedBucketBalance > 0}
                <div class="wa-hint">Max: {formatBalance(selectedBucketBalance)}</div>
              {/if}
              {#if unit === 'points' && parsedAmount > 0}
                <div class="wa-hint">Worth {formatCurrencyINR(parsedAmount * pointsRate)}</div>
              {/if}
              {#if action === 'remove' && parsedAmount > selectedBucketBalance && selectedBucketBalance > 0}
                <div class="wa-field-error">
                  Exceeds {bucketLabel(selectedBucket)} balance ({formatBalance(
                    selectedBucketBalance
                  )})
                </div>
              {/if}
            </div>
            {#if action === 'add'}
              <div class="wa-field">
                <label class="wa-label" for="wa-expiry-days">Expiry (days)</label>
                <input
                  id="wa-expiry-days"
                  class="wa-input"
                  type="number"
                  min="1"
                  placeholder="Policy default"
                  bind:value={expiryDaysStr}
                />
                <div class="wa-hint">Leave blank for wallet policy default</div>
              </div>
            {/if}
          </div>
        {/if}

        <!-- Reason -->
        <div class="wa-field">
          <span class="wa-label">Reason <span class="wa-req">*</span></span>
          <div class="wa-pills">
            {#each REASON_PILLS[action] as pill (pill)}
              <button
                class="wa-pill"
                class:active={reasonCategory === pill}
                class:pill-green={action === 'add' && reasonCategory === pill}
                class:pill-red={action === 'remove' && reasonCategory === pill}
                class:pill-amber={action === 'expire' && reasonCategory === pill}
                onclick={() => {
                  reasonCategory = reasonCategory === pill ? '' : pill;
                }}>{pill}</button
              >
            {/each}
          </div>
          <textarea class="wa-textarea" placeholder="Describe the reason..." bind:value={reasonText}
          ></textarea>
        </div>

        <!-- Reference -->
        <div class="wa-field">
          <label class="wa-label" for="wa-internal-ref">Internal reference</label>
          <input
            id="wa-internal-ref"
            class="wa-input"
            placeholder="e.g. Shopify #1234, support ticket #567"
            bind:value={reference}
          />
        </div>

        <!-- Notify (add only) -->
        {#if action === 'add'}
          <div class="wa-notify-row">
            <div>
              <div class="wa-notify-text">Notify customer</div>
              <div class="wa-hint">Send email notification</div>
            </div>
            <Toggle
              checked={notifyCustomer}
              text=""
              onclick={(val) => {
                notifyCustomer = val;
              }}
            />
          </div>
        {/if}

        <!-- Preview -->
        {#if (action !== 'expire' && parsedAmount > 0 && selectedBucket) || (action === 'expire' && selectedExpireBuckets.size > 0)}
          <div
            class="wa-preview"
            class:preview-green={action === 'add'}
            class:preview-red={action === 'remove'}
            class:preview-amber={action === 'expire'}
          >
            <div class="wa-preview-title">
              {action === 'add' ? 'Add' : action === 'remove' ? 'Remove' : 'Expire'} Summary
            </div>
            {#if action === 'expire'}
              {#each availableBuckets.filter( (b) => selectedExpireBuckets.has(b.bucket_type) ) as bucket (bucket.bucket_type)}
                <div class="wa-preview-row">
                  <span>{bucketLabel(bucket.bucket_type)}</span><span class="wa-preview-val"
                    >{formatBalance(bucket.spendable)}</span
                  >
                </div>
              {/each}
            {:else}
              <div class="wa-preview-row">
                <span>Amount</span><span class="wa-preview-val"
                  >{action === 'remove' ? '-' : '+'}{formatBalance(parsedAmount)}</span
                >
              </div>
              <div class="wa-preview-row">
                <span>Bucket</span><span class="wa-preview-val">{bucketLabel(selectedBucket)}</span>
              </div>
            {/if}
            <div class="wa-preview-divider"></div>
            <div class="wa-preview-row wa-preview-total">
              <span>New {unit} balance</span><span class="wa-preview-val"
                >{formatBalance(Math.max(0, previewNewBalance))}</span
              >
            </div>
          </div>
        {/if}
      {:else if step === 'confirm' || step === 'loading'}
        <!-- Confirmation -->
        <div class="wa-confirm">
          <div class="wa-confirm-icon">
            <Icon
              name={action === 'add' ? 'wallet' : action === 'remove' ? 'alert' : 'clock'}
              size={20}
            />
          </div>
          <div
            class="wa-confirm-title"
            class:text-green={action === 'add'}
            class:text-red={action === 'remove'}
            class:text-amber={action === 'expire'}
          >
            {#if action === 'add'}Add {formatBalance(parsedAmount)} to {bucketLabel(
                selectedBucket
              )}?
            {:else if action === 'remove'}Remove {formatBalance(parsedAmount)} from {bucketLabel(
                selectedBucket
              )}?
            {:else}Expire {formatBalance(expireTotalAmount)}?
            {/if}
          </div>
          <div class="wa-confirm-subtitle">
            {#if action === 'add'}This will immediately add funds to the customer's wallet.
            {:else if action === 'remove'}This will remove funds. You would need to add again to
              reverse it.
            {:else}This action is irreversible. The balance cannot be restored.
            {/if}
          </div>

          <div class="wa-confirm-summary">
            <div class="wa-confirm-row">
              <span class="lbl">Customer</span><span class="val"
                >{customer.name ?? customer.phone}</span
              >
            </div>
            {#if action === 'expire'}
              <div class="wa-confirm-row">
                <span class="lbl">Buckets</span><span class="val"
                  >{[...selectedExpireBuckets].map(bucketLabel).join(', ')}</span
                >
              </div>
              <div class="wa-confirm-row">
                <span class="lbl">Amount</span><span class="val"
                  >{formatBalance(expireTotalAmount)}</span
                >
              </div>
            {:else}
              <div class="wa-confirm-row">
                <span class="lbl">Amount</span><span class="val"
                  >{action === 'remove' ? '-' : '+'}{formatBalance(parsedAmount)}</span
                >
              </div>
              <div class="wa-confirm-row">
                <span class="lbl">Bucket</span><span class="val">{bucketLabel(selectedBucket)}</span
                >
              </div>
            {/if}
            <div class="wa-confirm-row">
              <span class="lbl">Reason</span><span class="val">{reasonCategory}</span>
            </div>
            <div class="wa-confirm-row wa-confirm-highlight">
              <span class="lbl">New balance</span><span class="val"
                >{formatBalance(currentBalance)} → {formatBalance(
                  Math.max(0, previewNewBalance)
                )}</span
              >
            </div>
          </div>

          <!-- Type-to-confirm for remove/expire -->
          {#if action !== 'add'}
            <div class="wa-type-confirm">
              <p>
                Type <code>{action === 'remove' ? String(parsedAmount) : 'EXPIRE'}</code> to confirm
              </p>
              <input
                class="wa-input wa-type-input"
                placeholder={action === 'remove' ? 'Enter amount...' : 'Type EXPIRE...'}
                bind:value={confirmInput}
                disabled={step === 'loading'}
              />
            </div>
          {/if}

          <div class="wa-confirm-actions">
            <Button
              text="Go back"
              classes="btn-ghost"
              onclick={() => {
                step = 'form';
                confirmInput = '';
              }}
              disabled={step === 'loading'}
            />
            <Button
              text={step === 'loading'
                ? 'Processing...'
                : action === 'add'
                  ? 'Confirm & Add'
                  : 'Confirm'}
              classes={actionBtnClass}
              onclick={handleConfirm}
              disabled={step === 'loading' || !isConfirmValid}
              showLoader={step === 'loading'}
              loaderType="Circular"
            />
          </div>
        </div>
      {:else if step === 'success'}
        <!-- Success -->
        <div class="wa-result">
          <div
            class="wa-result-icon"
            class:icon-green={action === 'add'}
            class:icon-red={action === 'remove'}
            class:icon-amber={action === 'expire'}
          >
            ✓
          </div>
          <div class="wa-result-title">
            {#if action === 'add'}{formatBalance(parsedAmount)} added
            {:else if action === 'remove'}{formatBalance(parsedAmount)} removed
            {:else}{formatBalance(result?.amount_affected ?? expireTotalAmount)} expired
            {/if}
          </div>
          <div class="wa-result-subtitle">
            {#if action === 'add'}Added to {customer.name ?? customer.phone}'s {bucketLabel(
                selectedBucket
              )}
            {:else if action === 'remove'}Removed from {customer.name ?? customer.phone}'s {bucketLabel(
                selectedBucket
              )}
            {:else}{result?.entries_affected ?? 0} entries expired for {customer.name ??
                customer.phone}
            {/if}
          </div>

          <div class="wa-result-detail">
            {#if result?.ledger_entry_id}
              <div class="wa-result-row">
                <span class="lbl">Transaction ID</span><span class="val mono"
                  >{result.ledger_entry_id.slice(0, 12)}</span
                >
              </div>
            {/if}
            <div class="wa-result-row">
              <span class="lbl">Amount</span><span class="val"
                >{action === 'remove' ? '-' : action === 'expire' ? '-' : '+'}{formatBalance(
                  result?.amount_affected ?? parsedAmount
                )}</span
              >
            </div>
            {#if result?.new_balance !== undefined && result.new_balance > 0}
              <div class="wa-result-row">
                <span class="lbl">New balance</span><span class="val"
                  >{formatCurrencyINR(result.new_balance)}</span
                >
              </div>
            {/if}
          </div>

          <Button text="Done" classes={actionBtnClass} onclick={handleDone} />
        </div>
      {/if}

      <!-- Footer (form step only) -->
      {#if step === 'form' || step === 'error'}
        <div class="wa-footer">
          <Button text="Cancel" classes="btn-ghost" onclick={onClose} />
          <Button
            text={isFormValid
              ? actionButtonLabel
              : action === 'add'
                ? 'Add'
                : action === 'remove'
                  ? 'Remove'
                  : 'Expire'}
            classes={actionBtnClass}
            onclick={handleSubmit}
            disabled={!isFormValid}
          />
        </div>
      {/if}
    </div>
  {/snippet}
</Modal>

<style>
  .wa-body {
    padding: var(--space-5);
  }

  /* Tabs */
  .wa-tabs {
    display: flex;
    gap: 4px;
    padding: 4px;
    background: var(--color-surface-2);
    border-radius: var(--radius-md);
    margin-bottom: var(--space-4);
  }
  .wa-tab {
    flex: 1;
    padding: 7px 12px;
    border-radius: var(--radius-sm);
    border: none;
    font-size: var(--font-size-sm);
    font-weight: 600;
    cursor: pointer;
    background: transparent;
    color: var(--color-text-muted);
    transition: all 0.15s;
    text-align: center;
  }
  .wa-tab-add {
    background: var(--green-100);
    color: var(--green-700);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.06);
  }
  .wa-tab-remove {
    background: var(--red-100);
    color: var(--red-700);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.06);
  }
  .wa-tab-expire {
    background: var(--yellow-100);
    color: var(--yellow-700);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.06);
  }

  /* Customer Banner */
  .wa-customer {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: 10px 14px;
    background: var(--color-surface-2);
    border-radius: var(--radius-md);
    margin-bottom: var(--space-4);
  }
  .wa-customer-info {
    flex: 1;
    min-width: 0;
  }
  .wa-customer-name {
    font-size: var(--font-size-base);
    font-weight: 700;
  }
  .wa-customer-phone {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    font-family: var(--font-mono);
  }
  .wa-customer-balance {
    text-align: right;
  }
  .wa-balance-val {
    font-size: var(--font-size-base);
    font-weight: 800;
  }
  .wa-balance-val.points {
    color: var(--color-primary);
  }
  .wa-balance-val.cash {
    color: var(--color-success);
  }
  .wa-balance-label {
    font-size: 10px;
    color: var(--color-text-muted);
  }

  /* Error & Warning Banners */
  .wa-error-banner {
    display: flex;
    align-items: flex-start;
    gap: 8px;
    padding: 10px 14px;
    border-radius: var(--radius-md);
    background: var(--red-100);
    border: 1px solid var(--red-100);
    margin-bottom: var(--space-4);
  }
  .wa-error-icon {
    color: var(--color-error);
    font-size: 15px;
    flex-shrink: 0;
  }
  .wa-error-text {
    font-size: var(--font-size-sm);
    color: var(--red-700);
  }

  .wa-warning-banner {
    display: flex;
    align-items: flex-start;
    gap: 8px;
    padding: 10px 14px;
    border-radius: var(--radius-md);
    background: var(--yellow-100);
    border: 1px solid var(--yellow-100);
    margin-bottom: var(--space-4);
    font-size: var(--font-size-sm);
    color: var(--yellow-700);
  }

  /* Form Fields */
  .wa-field {
    margin-bottom: var(--space-4);
  }
  .wa-label {
    display: block;
    font-size: var(--font-size-xs);
    font-weight: 700;
    color: var(--color-text);
    margin-bottom: 5px;
  }
  .wa-req {
    color: var(--color-error);
  }
  .wa-hint {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    margin-top: 3px;
  }
  .wa-field-error {
    font-size: var(--font-size-xs);
    color: var(--color-error);
    margin-top: 3px;
  }
  .wa-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-3);
  }
  .wa-no-data {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    font-style: italic;
    padding: var(--space-2);
  }

  .wa-input {
    width: 100%;
    padding: 8px 12px;
    font-size: var(--font-size-base);
    border-radius: var(--radius-md);
    background: var(--color-surface);
    color: var(--color-text);
    outline: none;
    font-family: inherit;
    border: 1px solid var(--color-border);
  }
  .wa-input:focus {
    border-color: var(--color-primary);
    box-shadow: 0 0 0 2px var(--p-200);
  }
  .wa-textarea {
    width: 100%;
    padding: 8px 12px;
    font-size: var(--font-size-sm);
    border-radius: var(--radius-md);
    background: var(--color-surface);
    color: var(--color-text);
    outline: none;
    font-family: inherit;
    resize: vertical;
    min-height: 56px;
    border: 1px solid var(--color-border);
  }
  .wa-textarea:focus {
    border-color: var(--color-primary);
    box-shadow: 0 0 0 2px var(--p-200);
  }

  /* Amount Input */
  .wa-amount-wrap {
    position: relative;
  }
  .wa-amount-prefix {
    position: absolute;
    left: 12px;
    top: 50%;
    transform: translateY(-50%);
    font-size: var(--font-size-base);
    font-weight: 700;
    color: var(--color-text-muted);
  }
  .wa-amount-suffix {
    position: absolute;
    right: 12px;
    top: 50%;
    transform: translateY(-50%);
    font-size: var(--font-size-xs);
    font-weight: 600;
    color: var(--color-text-muted);
  }
  .wa-amount-input {
    font-weight: 700;
    font-family: var(--font-mono);
  }
  .wa-amount-input.cash-input {
    padding-left: 28px;
  }

  /* Bucket Cards */
  .wa-bucket-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .wa-radio-card,
  .wa-expire-card {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 9px 14px;
    border-radius: var(--radius-lg);
    cursor: pointer;
    background: var(--color-surface);
    transition: all 0.15s;
    width: 100%;
    text-align: left;
    font-family: inherit;
    box-shadow: var(--shadow-card);
  }
  .wa-radio-card:hover,
  .wa-expire-card:hover {
    border-color: var(--color-primary);
  }
  .wa-radio-card.selected {
    border-color: var(--color-primary);
    background: color-mix(in srgb, var(--purple-500) 8%, #fff);
  }
  .wa-expire-card.selected {
    border-color: var(--color-warning);
    background: var(--yellow-100);
  }
  .wa-bucket-name {
    font-size: var(--font-size-sm);
    font-weight: 600;
    flex: 1;
    color: var(--color-text);
  }
  .wa-bucket-bal {
    font-size: var(--font-size-sm);
    font-weight: 700;
    font-family: var(--font-mono);
    color: var(--color-text-muted);
  }

  .wa-radio {
    width: 16px;
    height: 16px;
    border-radius: 50%;
    border: 2px solid var(--color-border);
    flex-shrink: 0;
    position: relative;
    transition: all 0.15s;
  }
  .wa-radio.checked {
    border-color: var(--color-primary);
    background: var(--color-primary);
  }
  .wa-radio.checked::after {
    content: '';
    position: absolute;
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #fff;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
  }

  .wa-check {
    width: 18px;
    height: 18px;
    border-radius: 4px;
    border: 2px solid var(--color-border);
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    font-size: var(--font-size-xs);
    color: transparent;
    transition: all 0.15s;
  }
  .wa-check.checked {
    background: var(--color-warning);
    border-color: var(--color-warning);
    color: #fff;
  }

  /* Reason Pills */
  .wa-pills {
    display: flex;
    flex-wrap: wrap;
    gap: 5px;
    margin-bottom: 6px;
  }
  .wa-pill {
    padding: 3px 11px;
    border-radius: var(--radius-md);
    background: var(--color-surface);
    font-size: var(--font-size-xs);
    cursor: pointer;
    color: var(--color-text-muted);
    transition: all 0.15s;
    font-weight: 500;
    font-family: inherit;
    border: 1px solid var(--color-border);
  }
  .wa-pill:hover {
    border-color: var(--color-primary);
    color: var(--color-primary);
  }
  .wa-pill.active {
    border-color: var(--color-primary);
    color: var(--color-primary);
    background: color-mix(in srgb, var(--purple-500) 8%, #fff);
  }
  .wa-pill.pill-green {
    background: var(--green-100);
    color: var(--green-700);
    border-color: var(--green-100);
  }
  .wa-pill.pill-red {
    background: var(--red-100);
    color: var(--red-700);
    border-color: var(--red-100);
  }
  .wa-pill.pill-amber {
    background: var(--yellow-100);
    color: var(--yellow-700);
    border-color: var(--yellow-100);
  }

  /* Notify Row */
  .wa-notify-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 0;
    margin-bottom: var(--space-3);
  }
  .wa-notify-text {
    font-size: var(--font-size-sm);
  }

  /* Preview */
  .wa-preview {
    border-radius: var(--radius-md);
    padding: 12px 16px;
    margin-top: 4px;
  }
  .wa-preview.preview-green {
    background: var(--green-100);
    border: 1px solid var(--green-100);
    color: var(--green-700);
  }
  .wa-preview.preview-red {
    background: var(--red-100);
    border: 1px solid var(--red-100);
    color: var(--red-700);
  }
  .wa-preview.preview-amber {
    background: var(--yellow-100);
    border: 1px solid var(--yellow-100);
    color: var(--yellow-700);
  }
  .wa-preview-title {
    font-size: var(--font-size-xs);
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    margin-bottom: 6px;
  }
  .wa-preview-row {
    display: flex;
    justify-content: space-between;
    font-size: var(--font-size-sm);
    padding: 2px 0;
  }
  .wa-preview-val {
    font-weight: 700;
  }
  .wa-preview-divider {
    border-top: 1px dashed currentColor;
    margin: 5px 0;
    opacity: 0.3;
  }
  .wa-preview-total .wa-preview-val {
    font-size: 15px;
    font-weight: 800;
  }

  /* Footer */
  .wa-footer {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 8px;
    padding-top: var(--space-4);
    margin-top: var(--space-2);
    border-top: 1px solid var(--color-border);
  }

  /* Confirm */
  .wa-confirm {
    padding: var(--space-6) var(--space-5);
    text-align: center;
  }
  .wa-confirm-icon {
    font-size: 40px;
    margin-bottom: 10px;
  }
  .wa-confirm-title {
    font-size: var(--font-size-lg);
    font-weight: 800;
    margin-bottom: 4px;
  }
  .wa-confirm-title.text-green {
    color: var(--green-700);
  }
  .wa-confirm-title.text-red {
    color: var(--red-700);
  }
  .wa-confirm-title.text-amber {
    color: var(--yellow-700);
  }
  .wa-confirm-subtitle {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    margin-bottom: 18px;
  }
  .wa-confirm-summary {
    background: var(--color-surface-2);
    border-radius: var(--radius-md);
    padding: 14px 18px;
    text-align: left;
    margin-bottom: 18px;
  }
  .wa-confirm-row {
    display: flex;
    justify-content: space-between;
    font-size: var(--font-size-sm);
    padding: 3px 0;
  }
  .wa-confirm-row .lbl {
    color: var(--color-text-muted);
  }
  .wa-confirm-row .val {
    font-weight: 700;
  }
  .wa-confirm-highlight {
    border-top: 1px solid var(--color-border);
    padding-top: 8px;
    margin-top: 4px;
  }
  .wa-confirm-actions {
    display: flex;
    gap: 8px;
    justify-content: center;
  }

  .wa-type-confirm {
    background: var(--color-surface-2);
    border-radius: var(--radius-md);
    padding: 14px 18px;
    margin-bottom: 16px;
    text-align: center;
  }
  .wa-type-confirm p {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    margin-bottom: 8px;
  }
  .wa-type-confirm code {
    background: var(--red-100);
    color: var(--color-error);
    padding: 2px 8px;
    border-radius: 4px;
    font-family: var(--font-mono);
    font-size: var(--font-size-sm);
    font-weight: 700;
  }
  .wa-type-input {
    max-width: 200px;
    margin: 0 auto;
    text-align: center;
    font-family: var(--font-mono);
  }

  /* Result */
  .wa-result {
    padding: var(--space-8) var(--space-6);
    text-align: center;
  }
  .wa-result-icon {
    width: 52px;
    height: 52px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 26px;
    margin: 0 auto 14px;
  }
  .wa-result-icon.icon-green {
    background: var(--green-100);
    color: var(--color-success);
  }
  .wa-result-icon.icon-red {
    background: var(--red-100);
    color: var(--color-error);
  }
  .wa-result-icon.icon-amber {
    background: var(--yellow-100);
    color: var(--color-warning);
  }
  .wa-result-title {
    font-size: var(--font-size-lg);
    font-weight: 800;
    margin-bottom: 4px;
  }
  .wa-result-subtitle {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    margin-bottom: 14px;
  }
  .wa-result-detail {
    background: var(--color-surface-2);
    border-radius: var(--radius-md);
    padding: 14px 18px;
    text-align: left;
    margin-bottom: 18px;
  }
  .wa-result-row {
    display: flex;
    justify-content: space-between;
    font-size: var(--font-size-sm);
    padding: 2px 0;
  }
  .wa-result-row .lbl {
    color: var(--color-text-muted);
  }
  .wa-result-row .val {
    font-weight: 700;
  }
  .wa-result-row .mono {
    font-family: var(--font-mono);
    font-size: var(--font-size-xs);
  }

  :global(.wallet-action-modal) {
    --modal-content-background-color: var(--color-surface);
    --modal-border-radius: var(--radius-lg);
    --modal-header-background-color: var(--color-surface);
    --modal-header-border-bottom: 1px solid var(--color-border);
    --modal-header-padding: 14px 20px;
    --modal-fit-content-max-height: calc(100vh - 96px);
    --modal-content-overflow: auto;
    --modal-large-width: 520px;
    --modal-medium-width: 520px;
  }
</style>
