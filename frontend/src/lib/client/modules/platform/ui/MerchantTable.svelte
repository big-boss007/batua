<script lang="ts">
  import { goto } from '$app/navigation';
  import { Table, Pill, Input } from '@juspay/svelte-ui-components';
  import type { PlatformMerchant } from '$lib/client/modules/platform';
  import { formatDate } from '$lib/client/modules/foundation';

  let { merchants }: { merchants: PlatformMerchant[] } = $props();

  let searchQuery = $state('');

  let filtered = $derived(
    searchQuery.length === 0
      ? merchants
      : merchants.filter((m) => {
          const q = searchQuery.toLowerCase();
          return (
            m.name.toLowerCase().includes(q) ||
            (m.slug ?? '').toLowerCase().includes(q) ||
            (m.domain ?? '').toLowerCase().includes(q)
          );
        })
  );

  let headers = $derived(['Name', 'Slug', 'Domain', 'Plan Tier', 'Status', 'Created']);

  let tableData = $derived(
    filtered.map((m) => [
      m.name,
      m.slug ?? '--',
      m.domain ?? '--',
      m.plan_tier,
      m.is_active ? 'Active' : 'Inactive',
      formatDate(m.created_at)
    ])
  );

  function planClass(plan: string): string {
    if (plan === 'enterprise') return 'pill-info';
    if (plan === 'scale') return 'pill-success';
    if (plan === 'grow') return 'pill-warning';
    return 'pill-neutral';
  }

  function statusClass(status: string): string {
    return status === 'Active' ? 'pill-success' : 'pill-error';
  }

  function handleRowClick(rowIndex: number) {
    const merchant = filtered[rowIndex];
    if (merchant) {
      goto(`/platform/merchants/${merchant.id}`);
    }
  }

  function handleSearch(value: string) {
    searchQuery = value;
  }
</script>

<div class="merchant-table-wrapper">
  <div class="search-bar">
    <Input
      value={searchQuery}
      placeholder="Search by name, slug, or domain..."
      onInput={handleSearch}
      classes="merchant-search-input"
    />
  </div>

  <Table
    tableHeaders={headers}
    tableData={tableData}
    sortable={true}
    sortableColumns={[0, 3, 4, 5]}
    onRowClick={handleRowClick}
  >
    {#snippet cell(value, rowIndex, colIndex)}
      {#if colIndex === 3}
        <Pill text={String(value)} classes={planClass(String(value))} />
      {:else if colIndex === 4}
        <Pill text={String(value)} classes={statusClass(String(value))} />
      {:else}
        {value}
      {/if}
    {/snippet}
    {#snippet empty()}
      <p class="empty-text">No merchants found.</p>
    {/snippet}
  </Table>
</div>

<style>
  .merchant-table-wrapper {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .search-bar {
    max-width: 400px;
    --input-margin: 0;
    --input-width: 100%;
  }

  .empty-text {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    padding: var(--space-4);
    text-align: center;
  }
</style>
