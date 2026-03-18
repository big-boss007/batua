<script lang="ts">
  import { Table } from '@juspay/svelte-ui-components';

  import type { CampaignPerformance } from '$lib/client/modules/analytics';
  import { formatCurrencyINR } from '$lib/client/modules/foundation';

  let { campaigns }: { campaigns: Array<CampaignPerformance> } = $props();

  let tableData = $derived(
    campaigns.map((c) => [
      c.name,
      c.total_entries.toLocaleString('en-IN'),
      formatCurrencyINR(c.total_value),
      c.unique_customers.toLocaleString('en-IN'),
      formatCurrencyINR(c.average_reward)
    ])
  );
</script>

<div class="campaign-table-wrapper">
  <Table
    tableTitle="Campaign Performance"
    tableHeaders={['Campaign', 'Entries', 'Total Value', 'Customers', 'Avg. Reward']}
    tableData={tableData}
    sortable={false}
  >
    {#snippet empty()}
      <p class="empty-state">No campaign data available for this period.</p>
    {/snippet}
  </Table>
</div>

<style>
  .campaign-table-wrapper {
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
