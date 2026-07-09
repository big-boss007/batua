<script lang="ts">
  import {
    Table,
    Pill,
    Select,
    Pagination,
    Shimmer,
    Modal,
    Button
  } from '@juspay/svelte-ui-components';
  import type { Snippet } from 'svelte';
  import type { MerchantTransactionRow, LedgerEntryDetail } from '$lib/client/modules/transactions';
  import {
    fetchMerchantTransactions,
    fetchEntryDetail,
    formatBucketType,
    formatMovementType,
    formatState
  } from '$lib/client/modules/transactions';
  import { currentMerchant, currentMerchantId } from '$lib/client/modules/admin';
  import type { Merchant } from '$lib/client/modules/admin';
  import {
    toastStore,
    formatCurrencyINR,
    formatDateTime,
    formatPhone,
    formatPoints,
    isPointsBucket,
    MODAL_CLOSE_ICON
  } from '$lib/client/modules/foundation';

  const BUCKET_TYPES = [
    'earned_credit',
    'cod_pending',
    'gift_card',
    'referral_reward',
    'goodwill_credit',
    'membership_benefit',
    'refund_credit'
  ];
  const MOVEMENT_TYPES = ['in', 'out', 'held', 'across'];

  let bucketItems = $derived(BUCKET_TYPES.map((bt) => ({ id: bt, label: formatBucketType(bt) })));
  let movementItems = $derived(
    MOVEMENT_TYPES.map((mt) => ({ id: mt, label: formatMovementType(mt).label }))
  );

  let transactions = $state<Array<MerchantTransactionRow>>([]);
  let loading = $state(false);
  let merchantId = $state<string | null>(null);
  let searchQuery = $state('');
  let selectedBucket = $state<string | null>(null);
  let selectedMovement = $state<string | null>(null);
  let currentPage = $state(1);
  let pageSize = 25;
  let debounceTimer: ReturnType<typeof setTimeout> | null = $state(null);

  let selectedEntry = $state<LedgerEntryDetail | null>(null);
  let loadingDetail = $state(false);
  let merchant = $state<Merchant | null>(null);

  currentMerchant.subscribe((m) => {
    merchant = m;
  });

  let pIcon = $derived(merchant?.points_icon ?? 'pts');

  const TABLE_HEADERS = [
    'Customer',
    'Bucket',
    'Movement',
    'Points / Cash',
    '₹ Value',
    'State',
    'Date'
  ];

  const MOVEMENT_PILL_CLASS: Record<string, string> = {
    in: 'pill-success',
    out: 'pill-error',
    held: 'pill-warning',
    across: 'pill-info'
  };

  const STATE_PILL_CLASS: Record<string, string> = {
    completed: 'pill-success',
    approved: 'pill-success',
    active: 'pill-success',
    pending: 'pill-warning',
    processing: 'pill-info',
    failed: 'pill-error',
    cancelled: 'pill-error',
    rejected: 'pill-error',
    reversed: 'pill-info',
    expired: 'pill-warning',
    redeemed: 'pill-info'
  };

  let tableData = $derived(
    transactions.map((tx) => {
      const customerLabel =
        tx.customer_name !== null
          ? `${tx.customer_name} (${formatPhone(tx.customer_phone)})`
          : formatPhone(tx.customer_phone);
      const pts = isPointsBucket(tx.bucket_type);
      const prefix =
        tx.movement_type === 'in' || tx.movement_type === 'held'
          ? '+'
          : tx.movement_type === 'out'
            ? '-'
            : '';
      const nativeCell = pts
        ? `${prefix}${formatPoints(Math.abs(tx.earning_unit), pIcon)}`
        : `${prefix}${formatCurrencyINR(Math.abs(tx.currency_equivalent))}`;
      return [
        customerLabel,
        formatBucketType(tx.bucket_type),
        tx.movement_type,
        nativeCell,
        formatCurrencyINR(tx.currency_equivalent),
        tx.state,
        formatDateTime(tx.created_at)
      ];
    })
  );

  let totalPages = $derived(transactions.length < pageSize ? currentPage : currentPage + 1);

  let selectedBucketValue = $derived(selectedBucket !== null ? [selectedBucket] : []);
  let selectedMovementValue = $derived(selectedMovement !== null ? [selectedMovement] : []);

  let hasActiveFilters = $derived(
    selectedBucket !== null || selectedMovement !== null || searchQuery.trim().length > 0
  );

  currentMerchantId.subscribe((id) => {
    if (id !== null && id !== merchantId) {
      merchantId = id;
      currentPage = 1;
      searchQuery = '';
      selectedBucket = null;
      selectedMovement = null;
      selectedEntry = null;
      loadTransactions(id, null, null, null, 1);
    } else if (id === null) {
      merchantId = null;
      transactions = [];
      selectedEntry = null;
    }
  });

  async function loadTransactions(
    mid: string,
    search: string | null,
    bucket: string | null,
    movement: string | null,
    page: number
  ) {
    loading = true;
    const result = await fetchMerchantTransactions(mid, search, bucket, movement, page, pageSize);
    if (result.tag === 'success') {
      if (result.data.length === 0 && page > 1) {
        currentPage = page - 1;
        loading = false;
        return;
      }
      transactions = result.data;
    } else {
      transactions = [];
      toastStore.push({ message: result.message, level: 'error' });
    }
    loading = false;
  }

  function triggerLoad() {
    if (merchantId === null) return;
    const search = searchQuery.trim().length > 0 ? searchQuery.trim() : null;
    loadTransactions(merchantId, search, selectedBucket, selectedMovement, currentPage);
  }

  function handleSearchInput(e: Event) {
    const target = e.target as HTMLInputElement;
    searchQuery = target.value;

    if (debounceTimer !== null) {
      clearTimeout(debounceTimer);
    }

    debounceTimer = setTimeout(() => {
      currentPage = 1;
      triggerLoad();
    }, 400);
  }

  function handleBucketChange(value: string[]) {
    selectedBucket = value.length > 0 ? value[0] : null;
    currentPage = 1;
    triggerLoad();
  }

  function handleMovementChange(value: string[]) {
    selectedMovement = value.length > 0 ? value[0] : null;
    currentPage = 1;
    triggerLoad();
  }

  function handleClearFilters() {
    searchQuery = '';
    selectedBucket = null;
    selectedMovement = null;
    currentPage = 1;
    triggerLoad();
  }

  function handlePageChange(page: number) {
    currentPage = page;
    triggerLoad();
  }

  async function handleRowClick(rowIndex: number) {
    const tx = transactions[rowIndex];
    if (tx === undefined) return;

    loadingDetail = true;
    selectedEntry = null;

    const result = await fetchEntryDetail(tx.entry_id);
    if (result.tag === 'success') {
      selectedEntry = result.data;
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }

    loadingDetail = false;
  }

  function handleCloseDetail() {
    selectedEntry = null;
  }

  function movementPillClass(movementType: string): string {
    return MOVEMENT_PILL_CLASS[movementType.toLowerCase()] ?? '';
  }

  function statePillClass(state: string): string {
    return STATE_PILL_CLASS[state.toLowerCase()] ?? '';
  }
</script>

<svelte:head>
  <title>Transactions - Batua</title>
</svelte:head>

<div class="page">
  <header class="page-header">
    <h1 class="page-title">Transactions</h1>
    <p class="page-subtitle">View merchant-wide transaction feed across all customers</p>
  </header>

  {#if merchantId === null}
    <div class="empty-state">
      <p class="empty-text">Select a merchant to view transactions</p>
    </div>
  {:else}
    <div class="transactions-layout">
      <div>
        <div class="filters-section">
          <div class="search-bar">
            <input
              class="search-input"
              type="text"
              placeholder="Search by phone number..."
              value={searchQuery}
              oninput={handleSearchInput}
            />
          </div>

          <div class="filter-row">
            <div class="filter-group">
              <span class="filter-label">Bucket type</span>
              <Select
                placeholder="All buckets"
                items={bucketItems}
                value={selectedBucketValue}
                onchange={handleBucketChange}
              />
            </div>

            <div class="filter-group">
              <span class="filter-label">Movement type</span>
              <Select
                placeholder="All movements"
                items={movementItems}
                value={selectedMovementValue}
                onchange={handleMovementChange}
              />
            </div>

            {#if hasActiveFilters}
              <Button text="Clear filters" classes="btn-ghost" onclick={handleClearFilters} />
            {/if}
          </div>
        </div>

        {#if loading}
          <div class="shimmer-rows">
            <Shimmer classes="shimmer-row" />
            <Shimmer classes="shimmer-row" />
            <Shimmer classes="shimmer-row" />
            <Shimmer classes="shimmer-row" />
            <Shimmer classes="shimmer-row" />
          </div>
        {:else}
          <Table
            tableHeaders={TABLE_HEADERS}
            {tableData}
            sortable={false}
            onRowClick={handleRowClick}
            --table-row-hover-background="var(--color-surface-2)"
            --table-content-font-size="var(--font-size-sm)"
          >
            {#snippet cell(value, rowIndex, colIndex)}
              {#if colIndex === 2}
                <Pill
                  text={formatMovementType(String(value)).label}
                  classes={MOVEMENT_PILL_CLASS[String(value).toLowerCase()] ?? ''}
                />
              {:else if colIndex === 3}
                <span
                  class="cell-native"
                  style="color: {isPointsBucket(transactions[rowIndex]?.bucket_type ?? '')
                    ? transactions[rowIndex]?.movement_type === 'out'
                      ? 'var(--color-error)'
                      : 'var(--color-success)'
                    : 'var(--color-primary)'}; font-weight: 500; font-family: var(--font-mono);"
                >
                  {value}
                </span>
              {:else if colIndex === 4}
                <span style="font-family: var(--font-mono); color: var(--color-text-muted);"
                  >{value}</span
                >
              {:else if colIndex === 5}
                <Pill
                  text={formatState(String(value)).label}
                  classes={STATE_PILL_CLASS[String(value).toLowerCase()] ?? ''}
                />
              {:else}
                {value}
              {/if}
            {/snippet}
            {#snippet empty()}
              <p class="table-empty">No transactions found</p>
            {/snippet}
          </Table>

          {#if transactions.length > 0}
            <div class="pagination-wrapper">
              <Pagination {totalPages} {currentPage} onchange={handlePageChange} />
            </div>
          {/if}
        {/if}
      </div>

      {#if loadingDetail || selectedEntry !== null}
        <Modal
          header={{ text: 'Transaction detail', rightImage: MODAL_CLOSE_ICON }}
          size="fit-content"
          onclose={handleCloseDetail}
          onoverlayClick={handleCloseDetail}
          onheaderRightImageClick={handleCloseDetail}
        >
          {#snippet content()}
            {#if loadingDetail}
              <div class="shimmer-detail">
                <Shimmer classes="shimmer-detail-header" />
                <Shimmer classes="shimmer-row" />
                <Shimmer classes="shimmer-row" />
                <Shimmer classes="shimmer-row" />
                <Shimmer classes="shimmer-row" />
              </div>
            {:else if selectedEntry}
              <div class="detail-body">
                <div class="detail-amount">
                  <Pill
                    text={formatMovementType(selectedEntry.movement_type).label}
                    classes={movementPillClass(selectedEntry.movement_type)}
                  />
                  <span class="amount">{formatCurrencyINR(selectedEntry.currency_equivalent)}</span>
                </div>

                {#if selectedEntry.customer_name !== null}
                  <div class="detail-row">
                    <span class="label">Customer</span>
                    <span class="value">{selectedEntry.customer_name}</span>
                  </div>
                {/if}
                {#if selectedEntry.customer_phone !== null}
                  <div class="detail-row">
                    <span class="label">Phone</span>
                    <span class="value mono">{formatPhone(selectedEntry.customer_phone)}</span>
                  </div>
                {/if}
                {#if selectedEntry.customer_email !== null}
                  <div class="detail-row">
                    <span class="label">Email</span>
                    <span class="value">{selectedEntry.customer_email}</span>
                  </div>
                {/if}

                <div class="detail-row">
                  <span class="label">Bucket</span>
                  <span class="value">
                    <Pill
                      text={formatBucketType(selectedEntry.bucket_type)}
                      classes="pill-neutral"
                    />
                  </span>
                </div>

                <div class="detail-row">
                  <span class="label">State</span>
                  <span class="value">
                    <Pill
                      text={formatState(selectedEntry.state).label}
                      classes={statePillClass(selectedEntry.state)}
                    />
                  </span>
                </div>

                <h4 class="section-heading">Cause</h4>

                {#if selectedEntry.event_type !== null}
                  <div class="detail-row">
                    <span class="label">Event</span>
                    <span class="value">{selectedEntry.event_type}</span>
                  </div>
                {/if}

                {#if selectedEntry.rule_name !== null}
                  <div class="detail-row">
                    <span class="label">Rule</span>
                    <span class="value">{selectedEntry.rule_name}</span>
                  </div>
                {/if}

                {#if selectedEntry.campaign_name !== null}
                  <div class="detail-row">
                    <span class="label">Campaign</span>
                    <span class="value">{selectedEntry.campaign_name}</span>
                  </div>
                {/if}

                <div class="detail-row">
                  <span class="label">Actor</span>
                  <span class="value"
                    >{selectedEntry.actor_type}{selectedEntry.actor_id !== null
                      ? ` (${selectedEntry.actor_id})`
                      : ''}</span
                  >
                </div>

                {#if selectedEntry.payment_reference !== null}
                  <div class="detail-row">
                    <span class="label">Payment ref</span>
                    <span class="value mono">{selectedEntry.payment_reference}</span>
                  </div>
                {/if}

                <h4 class="section-heading">Amounts</h4>

                <div class="detail-row">
                  <span class="label">Earning unit</span>
                  <span class="value">{selectedEntry.earning_unit}</span>
                </div>

                <div class="detail-row">
                  <span class="label">Currency equivalent</span>
                  <span class="value">{formatCurrencyINR(selectedEntry.currency_equivalent)}</span>
                </div>

                <div class="detail-row">
                  <span class="label">Conversion rate</span>
                  <span class="value">{selectedEntry.conversion_rate}</span>
                </div>

                {#if selectedEntry.transfer_id !== null}
                  <h4 class="section-heading">Transfer</h4>

                  <div class="detail-row">
                    <span class="label">Transfer ID</span>
                    <span class="value mono">{selectedEntry.transfer_id}</span>
                  </div>

                  {#if selectedEntry.linked_entry_id !== null}
                    <div class="detail-row">
                      <span class="label">Linked entry</span>
                      <span class="value mono">{selectedEntry.linked_entry_id}</span>
                    </div>
                  {/if}
                {/if}

                {#if Object.keys(selectedEntry.constraints ?? {}).length > 0}
                  <h4 class="section-heading">Constraints</h4>
                  {#each Object.entries(selectedEntry.constraints) as [key, value]}
                    <div class="detail-row">
                      <span class="label">{key.replace(/_/g, ' ')}</span>
                      <span class="value"
                        >{typeof value === 'object' ? JSON.stringify(value) : String(value)}</span
                      >
                    </div>
                  {/each}
                {/if}

                {#if selectedEntry.expires_at !== null}
                  <div class="detail-row">
                    <span class="label">Expires</span>
                    <span class="value">{formatDateTime(selectedEntry.expires_at)}</span>
                  </div>
                {/if}

                <h4 class="section-heading">Metadata</h4>

                <div class="detail-row">
                  <span class="label">Entry ID</span>
                  <span class="value mono small">{selectedEntry.id}</span>
                </div>

                <div class="detail-row">
                  <span class="label">Idempotency key</span>
                  <span class="value mono small">{selectedEntry.idempotency_key}</span>
                </div>

                <div class="detail-row">
                  <span class="label">Created</span>
                  <span class="value">{formatDateTime(selectedEntry.created_at)}</span>
                </div>
              </div>
            {/if}
          {/snippet}
        </Modal>
      {/if}
    </div>
  {/if}
</div>

<style>
  .page {
    max-width: 1400px;
    margin: 0 auto;
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
    width: 100%;
  }

  .page-header {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .page-title {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
  }

  .page-subtitle {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .transactions-layout {
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
  }

  .filters-section {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .search-bar {
    display: flex;
    gap: var(--space-3);
  }

  .search-input {
    flex: 1;
    padding: var(--space-2) var(--space-3);
    border-radius: var(--radius-sm);
    background: var(--color-surface);
    color: var(--color-text);
    font-size: var(--font-size-sm);
    transition: border-color var(--transition-fast);
    border: 1px solid var(--color-border);
  }

  .search-input::placeholder {
    color: var(--color-text-muted);
    opacity: 0.6;
  }

  .search-input:focus {
    outline: none;
    border-color: var(--color-primary);
    box-shadow: 0 0 0 2px rgba(124, 106, 255, 0.15);
  }

  .filter-row {
    display: flex;
    align-items: flex-end;
    gap: var(--space-4);
    flex-wrap: wrap;
    padding: var(--space-4);
    background: var(--color-surface);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-card);
  }

  .filter-group {
    min-width: 160px;
  }

  .filter-label {
    display: block;
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    margin-bottom: var(--space-1);
    font-weight: var(--font-weight-medium);
  }

  .pagination-wrapper {
    display: flex;
    justify-content: center;
    padding: var(--space-4) 0;
  }

  .shimmer-rows {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
    padding: var(--space-4) 0;
  }

  :global(.shimmer-row) {
    --shimmer-width: 100%;
    --shimmer-height: 48px;
    --shimmer-border-radius: 4px;
  }

  .empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--space-16);
    background: var(--color-surface);
    border: 1px dashed var(--color-border);
    border-radius: var(--radius-lg);
  }

  .empty-text {
    color: var(--color-text-muted);
    font-size: var(--font-size-base);
  }

  .table-empty {
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
    text-align: center;
    padding: var(--space-8);
  }

  .detail-body {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .detail-amount {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-3) 0;
    border-bottom: 1px solid var(--color-border);
    margin-bottom: var(--space-2);
  }

  .amount {
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
  }

  .detail-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--space-1) 0;
    gap: var(--space-3);
  }

  .label {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    flex-shrink: 0;
  }

  .value {
    font-size: var(--font-size-sm);
    color: var(--color-text);
    text-align: right;
    word-break: break-all;
  }

  .value.mono {
    font-family: var(--font-mono, monospace);
  }

  .value.small {
    font-size: var(--font-size-xs);
  }

  .section-heading {
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-top: var(--space-4);
    margin-bottom: var(--space-1);
    padding-bottom: var(--space-1);
    border-bottom: 1px solid var(--color-border);
  }

  .shimmer-detail {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
    padding: var(--space-4) 0;
  }

  :global(.shimmer-detail-header) {
    --shimmer-width: 60%;
    --shimmer-height: 24px;
    --shimmer-border-radius: 4px;
  }
</style>
