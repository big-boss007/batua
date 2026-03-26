<script lang="ts">
  import { Table, Pagination, Shimmer } from '@juspay/svelte-ui-components';
  import type {
    Customer,
    CustomerDetail as CustomerDetailType,
    MerchantCustomerRow
  } from '$lib/client/modules/customers';
  import { getCustomerDetail, fetchMerchantCustomers } from '$lib/client/modules/customers';
  import { currentMerchant, currentMerchantId } from '$lib/client/modules/admin';
  import type { Merchant } from '$lib/client/modules/admin';
  import { toastStore, formatDate, formatPhone } from '$lib/client/modules/foundation';
  import { CustomerDetail } from '$lib/client/modules/customers/ui';

  let selectedDetail = $state<CustomerDetailType | null>(null);
  let loadingDetail = $state(false);
  let merchantId = $state<string | null>(null);
  let merchant = $state<Merchant | null>(null);

  currentMerchant.subscribe((m) => {
    merchant = m;
  });

  let customers = $state<Array<MerchantCustomerRow>>([]);
  let loadingList = $state(false);
  let searchQuery = $state('');
  let currentPage = $state(1);
  let pageSize = 25;
  let debounceTimer: ReturnType<typeof setTimeout> | null = $state(null);

  const TABLE_HEADERS = ['Name', 'Phone', 'Email', 'Joined'];

  let tableData = $derived(
    customers.map((c) => [
      c.customer_name ?? 'Unnamed',
      formatPhone(c.customer_phone),
      c.customer_email ?? '-',
      formatDate(c.created_at)
    ])
  );

  let totalPages = $derived(customers.length < pageSize ? currentPage : currentPage + 1);

  currentMerchantId.subscribe((id) => {
    if (id !== null && id !== merchantId) {
      merchantId = id;
      selectedDetail = null;
      currentPage = 1;
      searchQuery = '';
      loadCustomers(id, null, 1);
    } else if (id === null) {
      merchantId = null;
      customers = [];
      selectedDetail = null;
    }
  });

  async function loadCustomers(mid: string, search: string | null, page: number) {
    loadingList = true;
    const result = await fetchMerchantCustomers(mid, search, page, pageSize);
    if (result.tag === 'success') {
      customers = result.data;
    } else {
      customers = [];
      toastStore.push({ message: result.message, level: 'error' });
    }
    loadingList = false;
  }

  function handleSearchInput(e: Event) {
    const target = e.target as HTMLInputElement;
    searchQuery = target.value;

    if (debounceTimer !== null) {
      clearTimeout(debounceTimer);
    }

    debounceTimer = setTimeout(() => {
      if (merchantId !== null) {
        currentPage = 1;
        loadCustomers(merchantId, searchQuery.trim().length > 0 ? searchQuery.trim() : null, 1);
      }
    }, 400);
  }

  function handlePageChange(page: number) {
    currentPage = page;
    if (merchantId !== null) {
      loadCustomers(merchantId, searchQuery.trim().length > 0 ? searchQuery.trim() : null, page);
    }
  }

  async function handleRowClick(rowIndex: number) {
    if (merchantId === null) return;
    const row = customers[rowIndex];
    if (row === undefined) return;

    loadingDetail = true;
    selectedDetail = null;

    const result = await getCustomerDetail(merchantId, row.customer_id);

    if (result.tag === 'success') {
      selectedDetail = result.data;
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }

    loadingDetail = false;
  }

  function handleCloseDetail() {
    selectedDetail = null;
  }
</script>

<svelte:head>
  <title>Customers - Batua</title>
</svelte:head>

<div class="customers-page">
  <header class="page-header">
    <h1 class="page-title">Customers</h1>
    <p class="page-subtitle">Browse and manage customers for this merchant</p>
  </header>

  {#if merchantId === null}
    <div class="empty-state">
      <p class="empty-text">Select a merchant to view customers</p>
    </div>
  {:else}
    <div class="customers-layout">
      <div class="search-bar">
        <input
          class="search-input"
          type="text"
          placeholder="Search by name, phone, or email..."
          value={searchQuery}
          oninput={handleSearchInput}
        />
      </div>

      {#if loadingList}
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
          {#snippet empty()}
            <p class="table-empty">No customers found</p>
          {/snippet}
        </Table>

        {#if customers.length > 0}
          <div class="pagination-wrapper">
            <Pagination {totalPages} {currentPage} onchange={handlePageChange} />
          </div>
        {/if}
      {/if}
    </div>

    {#if loadingDetail || selectedDetail !== null}
      <div class="modal-overlay" onclick={handleCloseDetail} onkeydown={(e) => { if (e.key === 'Escape') handleCloseDetail(); }} role="button" tabindex="-1">
        <div class="modal-card" onclick={(e) => e.stopPropagation()} role="dialog">
          <div class="modal-header">
            <h3 class="modal-title">Customer Detail</h3>
            <button class="modal-close" onclick={handleCloseDetail}>&times;</button>
          </div>
          <div class="modal-body">
            {#if loadingDetail}
              <div class="shimmer-detail">
                <Shimmer classes="shimmer-detail-header" />
                <Shimmer classes="shimmer-row" />
                <Shimmer classes="shimmer-row" />
                <Shimmer classes="shimmer-row" />
              </div>
            {:else if selectedDetail}
              <CustomerDetail detail={selectedDetail} pointsIcon={merchant?.points_icon ?? '★'} pointsRate={merchant?.points_to_currency_rate ?? 1.0} />
            {/if}
          </div>
        </div>
      </div>
    {/if}
  {/if}
</div>

<style>
  .customers-page {
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
    max-width: 1400px;
    margin: 0 auto;
    width: 100%;
  }

  .page-header {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .page-title {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
  }

  .page-subtitle {
    font-size: var(--font-size-base);
    color: var(--color-text-muted);
  }

  .customers-layout {
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

  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: var(--z-modal, 400);
  }

  .modal-card {
    background: var(--color-bg);
    border-radius: var(--radius-lg);
    width: 600px;
    max-width: 90vw;
    max-height: 85vh;
    display: flex;
    flex-direction: column;
    box-shadow: var(--shadow-lg);
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--space-4) var(--space-6);
    border-bottom: 1px solid var(--color-border);
    flex-shrink: 0;
  }

  .modal-title {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .modal-close {
    background: none;
    border: none;
    font-size: var(--font-size-xl);
    color: var(--color-text-muted);
    cursor: pointer;
    padding: var(--space-1) var(--space-2);
    border-radius: var(--radius-sm);
    line-height: 1;
  }

  .modal-close:hover {
    color: var(--color-text);
    background: var(--color-surface-2);
  }

  .modal-body {
    padding: var(--space-6);
    overflow-y: auto;
    flex: 1;
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

  .shimmer-detail {
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

  :global(.shimmer-detail-header) {
    --shimmer-width: 60%;
    --shimmer-height: 24px;
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
