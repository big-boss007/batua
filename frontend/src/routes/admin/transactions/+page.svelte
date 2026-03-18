<script lang="ts">
  import { goto } from '$app/navigation';
  import { Table, Pill, Select, Pagination, Shimmer } from '@juspay/svelte-ui-components';
  import type { MerchantTransactionRow } from '$lib/client/modules/transactions';
  import { fetchMerchantTransactions, formatBucketType, formatMovementType, formatState } from '$lib/client/modules/transactions';
  import { currentMerchantId } from '$lib/client/modules/admin';
  import { toastStore, formatCurrencyINR, formatDateTime } from '$lib/client/modules/foundation';

  const BUCKET_TYPES = ['earned_credit', 'cashback', 'gift_card', 'promotional', 'referral'];
  const MOVEMENT_TYPES = ['in', 'out', 'held', 'across'];

  let bucketItems = $derived(BUCKET_TYPES.map((bt) => ({ id: bt, label: formatBucketType(bt) })));
  let movementItems = $derived(MOVEMENT_TYPES.map((mt) => ({ id: mt, label: formatMovementType(mt).label })));

  let transactions = $state<Array<MerchantTransactionRow>>([]);
  let loading = $state(false);
  let merchantId = $state<string | null>(null);
  let searchQuery = $state('');
  let selectedBucket = $state<string | null>(null);
  let selectedMovement = $state<string | null>(null);
  let currentPage = $state(1);
  let pageSize = 25;
  let debounceTimer: ReturnType<typeof setTimeout> | null = $state(null);

  const TABLE_HEADERS = ['Customer', 'Bucket', 'Movement', 'Amount', 'State', 'Date'];

  const MOVEMENT_PILL_CLASS: Record<string, string> = {
    in: 'pill-success',
    out: 'pill-error',
    held: 'pill-warning',
    across: 'pill-info'
  };

  const STATE_PILL_CLASS: Record<string, string> = {
    completed: 'pill-success',
    approved: 'pill-success',
    pending: 'pill-warning',
    processing: 'pill-info',
    failed: 'pill-error',
    cancelled: 'pill-error',
    rejected: 'pill-error',
    reversed: 'pill-info'
  };

  let tableData = $derived(
    transactions.map((tx) => {
      const customerLabel = tx.customer_name !== null
        ? `${tx.customer_name} (${tx.customer_phone})`
        : tx.customer_phone;
      return [
        customerLabel,
        formatBucketType(tx.bucket_type),
        tx.movement_type,
        formatCurrencyINR(tx.currency_equivalent),
        tx.state,
        formatDateTime(tx.created_at)
      ];
    })
  );

  let totalPages = $derived(
    transactions.length < pageSize ? currentPage : currentPage + 1
  );

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
      loadTransactions(id, null, null, null, 1);
    } else if (id === null) {
      merchantId = null;
      transactions = [];
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

  function handleRowClick(rowIndex: number) {
    const tx = transactions[rowIndex];
    if (tx === undefined) return;
    const phone = tx.customer_phone;
    goto(`/admin/customers?search=${encodeURIComponent(phone)}`);
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
          <span class="filter-label">Bucket Type</span>
          <Select
            placeholder="All buckets"
            items={bucketItems}
            value={selectedBucketValue}
            onchange={handleBucketChange}
          />
        </div>

        <div class="filter-group">
          <span class="filter-label">Movement Type</span>
          <Select
            placeholder="All movements"
            items={movementItems}
            value={selectedMovementValue}
            onchange={handleMovementChange}
          />
        </div>

        {#if hasActiveFilters}
          <button class="clear-btn" onclick={handleClearFilters}>Clear Filters</button>
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
        tableData={tableData}
        sortable={false}
        onRowClick={handleRowClick}
        --table-row-hover-background="var(--color-surface-2)"
        --table-content-font-size="var(--font-size-sm)"
      >
        {#snippet cell(value, _rowIndex, colIndex)}
          {#if colIndex === 2}
            <Pill
              text={formatMovementType(String(value)).label}
              classes={MOVEMENT_PILL_CLASS[String(value).toLowerCase()] ?? ''}
            />
          {:else if colIndex === 4}
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
          <Pagination
            totalPages={totalPages}
            currentPage={currentPage}
            onchange={handlePageChange}
          />
        </div>
      {/if}
    {/if}
  {/if}
</div>

<style>
  .page {
    max-width: 1200px;
    margin: 0 auto;
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
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
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    background: var(--color-surface);
    color: var(--color-text);
    font-size: var(--font-size-sm);
    transition: border-color var(--transition-fast);
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
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
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

  .clear-btn {
    padding: var(--space-2) var(--space-4);
    background: none;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
    cursor: pointer;
    transition:
      color var(--transition-fast),
      border-color var(--transition-fast);
  }

  .clear-btn:hover {
    color: var(--color-error);
    border-color: var(--color-error);
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
</style>
