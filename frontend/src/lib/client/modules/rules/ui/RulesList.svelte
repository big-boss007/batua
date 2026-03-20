<script lang="ts">
  import { Table, Pill, Toggle, Button } from '@juspay/svelte-ui-components';

  import type { Rule } from '../types';

  let {
    rules,
    onEdit,
    onToggle
  }: {
    rules: Array<Rule>;
    onEdit: (rule: Rule) => void;
    onToggle: (rule: Rule) => void;
  } = $props();

  function formatDate(iso: string): string {
    return new Date(iso).toLocaleDateString('en-IN', {
      day: 'numeric',
      month: 'short',
      year: 'numeric'
    });
  }

  let tableData = $derived(
    rules.map((r) => [
      r.name,
      r.rule_type,
      r.config.event_type,
      `v${r.version}`,
      r.is_active ? 'Active' : 'Inactive',
      formatDate(r.created_at),
      ''
    ])
  );
</script>

<Table
  tableHeaders={['Name', 'Type', 'Event', 'Version', 'Status', 'Created', 'Actions']}
  {tableData}
  sortableColumns={[0, 1, 3, 5]}
  classes="rules-table"
  onRowClick={(idx) => onEdit(rules[idx])}
>
  {#snippet cell(value, rowIndex, colIndex)}
    {#if colIndex === 1}
      <Pill text={String(value)} classes="pill-info" />
    {:else if colIndex === 4}
      <Toggle
        checked={rules[rowIndex].is_active}
        text={String(value)}
        onclick={(checked) => onToggle(rules[rowIndex])}
      />
    {:else if colIndex === 6}
      <Button
        text="Edit"
        classes="btn-ghost"
        onclick={(e) => {
          e.stopPropagation();
          onEdit(rules[rowIndex]);
        }}
      />
    {:else}
      {value}
    {/if}
  {/snippet}
  {#snippet empty()}
    <p class="empty-text">No rules configured yet. Create your first reward rule to get started.</p>
  {/snippet}
</Table>

<style>
  .empty-text {
    font-size: var(--font-size-base);
    color: var(--color-text-muted);
    text-align: center;
  }
</style>
