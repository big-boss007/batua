<script lang="ts">
  import { Table, Pill } from '@juspay/svelte-ui-components';
  import type { RecentEvent } from '$lib/client/modules/platform';
  import { formatDateTime } from '$lib/client/modules/foundation';

  let { events }: { events: RecentEvent[] } = $props();

  let headers = $derived(['Merchant', 'Event Type', 'Source', 'State', 'Created']);

  let tableData = $derived(
    events.map((e) => [
      e.merchant_name,
      e.event_type,
      e.event_source,
      e.state,
      formatDateTime(e.created_at)
    ])
  );

  function stateClass(state: string): string {
    if (state === 'processed' || state === 'completed') return 'pill-success';
    if (state === 'failed') return 'pill-error';
    if (state === 'received' || state === 'pending') return 'pill-warning';
    return 'pill-neutral';
  }
</script>

<Table
  tableHeaders={headers}
  tableData={tableData}
  sortable={false}
>
  {#snippet cell(value, rowIndex, colIndex)}
    {#if colIndex === 3}
      <Pill text={String(value)} classes={stateClass(String(value))} />
    {:else}
      {value}
    {/if}
  {/snippet}
  {#snippet empty()}
    <p class="empty-text">No recent events.</p>
  {/snippet}
</Table>

<style>
  .empty-text {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    padding: var(--space-4);
    text-align: center;
  }
</style>
