<script lang="ts">
  import { Table, Pagination, Shimmer, Modal } from '@juspay/svelte-ui-components';
  import type { Snippet } from 'svelte';
  import type {
    Customer,
    CustomerDetail as CustomerDetailType,
    MerchantCustomerRow
  } from '$lib/client/modules/customers';
  import { getCustomerDetail, fetchMerchantCustomers } from '$lib/client/modules/customers';
  import { currentMerchant, currentMerchantId } from '$lib/client/modules/admin';
  import type { Merchant } from '$lib/client/modules/admin';
  import { toastStore, formatDate, formatPhone, MODAL_CLOSE_ICON } from '$lib/client/modules/foundation';
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
      if (result.data.length === 0 && page > 1) {
        currentPage = page - 1;
        loadingList = false;
        return;
      }
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
      <Modal header={{ text: 'Customer Detail', rightImage: MODAL_CLOSE_ICON }} size="fit-content" classes="customer-detail-modal" onclose={handleCloseDetail} onoverlayClick={handleCloseDetail} onheaderRightImageClick={handleCloseDetail}>
        {#snippet content()}
          {#if loadingDetail}
            <div class="shimmer-detail">
              <Shimmer classes="shimmer-detail-header" />
              <Shimmer classes="shimmer-row" />
              <Shimmer classes="shimmer-row" />
              <Shimmer classes="shimmer-row" />
            </div>
          {:else if selectedDetail}
            <CustomerDetail
              detail={selectedDetail}
              merchantId={merchantId ?? ''}
              pointsIcon={merchant?.points_icon ?? '★'}
              pointsRate={merchant?.points_to_currency_rate ?? 1.0}
              onRefresh={async () => {
                if (merchantId && selectedDetail) {
                  const res = await getCustomerDetail(merchantId, selectedDetail.customer.id);
                  if (res.tag === 'success') selectedDetail = res.data;
                }
              }}
            />
          {/if}
        {/snippet}
      </Modal>
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

  :global(.customer-detail-modal) {
    --modal-fit-content-max-height: calc(100vh - 96px);
    --modal-content-background-color: var(--color-surface);
    --modal-border-radius: var(--radius-lg);
    --modal-header-background-color: var(--color-surface);
    --modal-header-border-bottom: 1px solid var(--color-border);
  }
  :global(.customer-detail-modal .modal-content) {
    width: 520px;
    max-width: 90vw;
  }
</style>
