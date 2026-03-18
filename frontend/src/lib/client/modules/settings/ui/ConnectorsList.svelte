<script lang="ts">
  import { Table, Pill } from '@juspay/svelte-ui-components';

  import type { Connector } from '$lib/client/modules/settings';

  let { connectors }: { connectors: Array<Connector> } = $props();

  let tableData = $derived(
    connectors.map((c) => [c.capability, c.vendor, String(c.priority), c.is_active ? 'Active' : 'Inactive'])
  );
</script>

<div class="connectors-wrapper">
  <Table
    tableTitle="Connectors"
    tableHeaders={['Capability', 'Vendor', 'Priority', 'Status']}
    tableData={tableData}
    sortable={false}
  >
    {#snippet cell(value, rowIndex, colIndex)}
      {#if colIndex === 3}
        <Pill text={String(value)} classes={value === 'Active' ? 'pill-success' : 'pill-neutral'} />
      {:else}
        {value}
      {/if}
    {/snippet}
    {#snippet empty()}
      <p class="empty-state">No connectors configured.</p>
    {/snippet}
  </Table>
</div>

<style>
  .connectors-wrapper {
    padding: var(--space-6);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-sm);
  }

  .empty-state {
    padding: var(--space-8) 0;
    text-align: center;
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
  }
</style>
