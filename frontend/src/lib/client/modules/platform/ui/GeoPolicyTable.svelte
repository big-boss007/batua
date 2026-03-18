<script lang="ts">
  import { Table, Pill } from '@juspay/svelte-ui-components';
  import type { GeoPolicy } from '$lib/client/modules/platform';
  import { formatDate } from '$lib/client/modules/foundation';

  let { policies }: { policies: GeoPolicy[] } = $props();

  let headers = $derived(['Geo Code', 'Name', 'Status', 'Config Summary', 'Created']);

  function summarizeConfig(config: Record<string, unknown>): string {
    const parts: string[] = [];
    if (config['cod_enabled'] === true) parts.push('COD');
    if (config['whatsapp_default'] === true) parts.push('WhatsApp');
    if (config['upi_topup_enabled'] === true) parts.push('UPI');
    if (typeof config['default_currency'] === 'string') parts.push(config['default_currency']);
    return parts.length > 0 ? parts.join(', ') : '--';
  }

  let tableData = $derived(
    policies.map((p) => [
      p.geo_code,
      p.name,
      p.is_active ? 'Active' : 'Inactive',
      summarizeConfig(p.config),
      formatDate(p.created_at)
    ])
  );
</script>

<Table
  tableHeaders={headers}
  tableData={tableData}
  sortable={true}
  sortableColumns={[0, 1, 2]}
>
  {#snippet cell(value, rowIndex, colIndex)}
    {#if colIndex === 0}
      <code class="geo-code">{value}</code>
    {:else if colIndex === 2}
      <Pill
        text={String(value)}
        classes={value === 'Active' ? 'pill-success' : 'pill-error'}
      />
    {:else}
      {value}
    {/if}
  {/snippet}
  {#snippet empty()}
    <p class="empty-text">No geo policies configured.</p>
  {/snippet}
</Table>

<style>
  .geo-code {
    font-family: var(--font-mono);
    font-size: var(--font-size-sm);
    background: var(--color-surface-2);
    padding: 2px 6px;
    border-radius: var(--radius-sm);
  }

  .empty-text {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    padding: var(--space-4);
    text-align: center;
  }
</style>
