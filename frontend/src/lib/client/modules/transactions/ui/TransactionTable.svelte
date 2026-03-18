<script lang="ts">
  import type { LedgerEntry } from '$lib/client/modules/transactions';
  import { formatBucketType, formatMovementType, formatState } from '$lib/client/modules/transactions';
  import { formatCurrencyINR, formatDateTime } from '$lib/client/modules/foundation';
  import { Table, Pill, Pagination } from '@juspay/svelte-ui-components';

  let {
    entries,
    total = 0,
    page = 1,
    limit = 25,
    onPageChange
  }: {
    entries: Array<LedgerEntry>;
    total?: number;
    page?: number;
    limit?: number;
    onPageChange: (page: number) => void;
  } = $props();

  type SortField = 'created_at' | 'bucket_type' | 'movement_type' | 'earning_unit' | 'state';
  type SortDirection = 'asc' | 'desc';

  let sortField = $state<SortField>('created_at');
  let sortDirection = $state<SortDirection>('desc');

  let sortedEntries = $derived(sortRows(entries, sortField, sortDirection));
  let totalPages = $derived(total > 0 ? Math.ceil(total / limit) : 1);

  function sortRows(
    rows: Array<LedgerEntry>,
    field: SortField,
    direction: SortDirection
  ): Array<LedgerEntry> {
    const sorted = [...rows];
    const dir = direction === 'asc' ? 1 : -1;

    sorted.sort((a, b) => {
      const av = a[field];
      const bv = b[field];

      if (typeof av === 'number' && typeof bv === 'number') {
        return (av - bv) * dir;
      }

      return String(av).localeCompare(String(bv)) * dir;
    });

    return sorted;
  }

  function sortIndicator(field: SortField): string {
    if (sortField !== field) return '';
    return sortDirection === 'asc' ? ' \u2191' : ' \u2193';
  }

  function movementPillClass(type: string): string {
    const map: Record<string, string> = {
      In: 'pill-success',
      Out: 'pill-error',
      Held: 'pill-warning',
      Across: 'pill-info'
    };
    return map[type] ?? 'pill-neutral';
  }

  function statePillClass(state: string): string {
    const map: Record<string, string> = {
      completed: 'pill-success',
      approved: 'pill-success',
      pending: 'pill-warning',
      processing: 'pill-info',
      reversed: 'pill-info',
      failed: 'pill-error',
      rejected: 'pill-error',
      cancelled: 'pill-neutral'
    };
    return map[state.toLowerCase()] ?? 'pill-neutral';
  }

  let tableHeaders = $derived.by(() => {
    const fields: Array<{ label: string; sortKey: SortField | null }> = [
      { label: 'Date', sortKey: 'created_at' },
      { label: 'Bucket', sortKey: 'bucket_type' },
      { label: 'Movement', sortKey: 'movement_type' },
      { label: 'Amount', sortKey: 'earning_unit' },
      { label: 'Currency Equiv.', sortKey: null },
      { label: 'State', sortKey: 'state' },
      { label: 'Actor', sortKey: null }
    ];
    return fields.map((f) =>
      f.sortKey ? `${f.label}${sortIndicator(f.sortKey)}` : f.label
    );
  });

  let tableData = $derived(
    sortedEntries.map((entry) => [
      formatDateTime(entry.created_at),
      formatBucketType(entry.bucket_type),
      formatMovementType(entry.movement_type).label,
      entry.earning_unit,
      formatCurrencyINR(entry.currency_equivalent),
      formatState(entry.state).label,
      formatBucketType(entry.actor_type)
    ])
  );

</script>

<div class="table-container">
  <Table {tableHeaders} {tableData}>
    {#snippet cell(value, rowIndex, colIndex)}
      {#if colIndex === 2}
        <Pill text={String(value)} classes={movementPillClass(String(value))} />
      {:else if colIndex === 5}
        <Pill
          text={String(value)}
          classes={statePillClass(sortedEntries[rowIndex].state)}
        />
      {:else if colIndex === 3}
        <span class="cell-amount" style="color: {formatMovementType(sortedEntries[rowIndex].movement_type).color}">
          {value}
        </span>
      {:else if colIndex === 0}
        <span class="cell-date">{value}</span>
      {:else if colIndex === 4}
        <span class="cell-amount">{value}</span>
      {:else if colIndex === 6}
        <span class="cell-actor">{value}</span>
      {:else}
        {value}
      {/if}
    {/snippet}
  </Table>

  {#if sortedEntries.length === 0}
    <div class="empty-row">No transactions found</div>
  {/if}

  {#if total > 0}
    <div class="pagination-wrapper">
      <span class="page-info">
        Page {page} of {totalPages} ({total} entries)
      </span>
      <Pagination {totalPages} currentPage={page} onchange={(p) => onPageChange(p)} />
    </div>
  {/if}
</div>

<style>
  .table-container {
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    overflow: hidden;
  }

  .cell-date {
    white-space: nowrap;
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .cell-amount {
    font-family: var(--font-mono);
    font-weight: var(--font-weight-medium);
    white-space: nowrap;
  }

  .cell-actor {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .empty-row {
    text-align: center;
    color: var(--color-text-muted);
    padding: var(--space-10) var(--space-4);
    font-size: var(--font-size-base);
  }

  .pagination-wrapper {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--space-3) var(--space-4);
    border-top: 1px solid var(--color-border);
    background: var(--color-surface);
  }

  .page-info {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }
</style>
