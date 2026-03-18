<script lang="ts">
  import { Table, Pagination } from '@juspay/svelte-ui-components';
  import type { Customer, CustomerDetail as CustomerDetailType, MerchantCustomerRow } from '$lib/client/modules/customers';
  import { getCustomerDetail, fetchMerchantCustomers } from '$lib/client/modules/customers';
  import { currentMerchantId } from '$lib/client/modules/admin';
  import { toastStore, formatDate } from '$lib/client/modules/foundation';
  import { CustomerDetail } from '$lib/client/modules/customers/ui';

  let selectedDetail = $state<CustomerDetailType | null>(null);
  let loadingDetail = $state(false);
  let merchantId = $state<string | null>(null);

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
      c.customer_phone,
      c.customer_email ?? '-',
      formatDate(c.created_at)
    ])
  );

  let totalPages = $derived(
    customers.length < pageSize ? currentPage : currentPage + 1
  );

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
      <div class="list-panel">
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
          <div class="loading-state">
            <span class="loading-spinner"></span>
            <p>Loading customers...</p>
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
            {#snippet empty()}
              <p class="table-empty">No customers found</p>
            {/snippet}
          </Table>

          {#if customers.length > 0}
            <div class="pagination-wrapper">
              <Pagination
                totalPages={totalPages}
                currentPage={currentPage}
                onchange={handlePageChange}
              />
            </div>
          {/if}
        {/if}
      </div>

      {#if loadingDetail || selectedDetail !== null}
        <aside class="detail-panel">
          {#if loadingDetail}
            <div class="loading-state">
              <span class="loading-spinner"></span>
              <p>Loading customer details...</p>
            </div>
          {:else if selectedDetail}
            <div class="detail-header">
              <button class="close-btn" onclick={handleCloseDetail} aria-label="Close detail">
                &times;
              </button>
            </div>
            <CustomerDetail detail={selectedDetail} />
          {/if}
        </aside>
      {/if}
    </div>
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
    display: grid;
    grid-template-columns: 1fr;
    gap: var(--space-6);
    align-items: start;
  }

  .customers-layout:has(.detail-panel) {
    grid-template-columns: 1fr 420px;
  }

  .list-panel {
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

  .detail-panel {
    position: sticky;
    top: 72px;
    max-height: calc(100vh - 100px);
    overflow-y: auto;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: var(--space-4);
  }

  .detail-header {
    display: flex;
    justify-content: flex-end;
    margin-bottom: var(--space-2);
  }

  .close-btn {
    background: none;
    border: none;
    font-size: var(--font-size-xl);
    color: var(--color-text-muted);
    cursor: pointer;
    padding: var(--space-1) var(--space-2);
    border-radius: var(--radius-sm);
    line-height: 1;
    transition: color var(--transition-fast);
  }

  .close-btn:hover {
    color: var(--color-text);
  }

  .pagination-wrapper {
    display: flex;
    justify-content: center;
    padding: var(--space-4) 0;
  }

  .loading-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: var(--space-4);
    padding: var(--space-16);
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
  }

  .loading-spinner {
    width: 24px;
    height: 24px;
    border: 2px solid var(--color-border);
    border-top-color: var(--color-primary);
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
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

  @media (max-width: 960px) {
    .customers-layout,
    .customers-layout:has(.detail-panel) {
      grid-template-columns: 1fr;
    }

    .detail-panel {
      position: static;
      max-height: none;
    }
  }
</style>
