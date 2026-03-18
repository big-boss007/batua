<script lang="ts">
  import { Table, Pill } from '@juspay/svelte-ui-components';

  import type { GiftCard } from '$lib/client/modules/gift-cards';
  import { formatCurrencyINR, formatDate } from '$lib/client/modules/foundation';

  let { cards }: { cards: Array<GiftCard> } = $props();

  function getStatus(card: GiftCard): string {
    if (!card.is_active) return 'inactive';
    if (card.is_claimed) return 'claimed';
    if (card.expires_at && new Date(card.expires_at) < new Date()) return 'expired';
    return 'active';
  }

  function statusClass(status: string): string {
    if (status === 'active') return 'pill-success';
    if (status === 'claimed') return 'pill-info';
    if (status === 'expired') return 'pill-warning';
    return 'pill-neutral';
  }

  let tableData = $derived(
    cards.map((c) => [
      c.code,
      formatCurrencyINR(c.initial_amount),
      formatCurrencyINR(c.current_amount),
      getStatus(c),
      formatDate(c.created_at)
    ])
  );
</script>

<Table
  tableHeaders={['Code', 'Initial', 'Current', 'Status', 'Created']}
  tableData={tableData}
  sortable={false}
>
  {#snippet cell(value, rowIndex, colIndex)}
    {#if colIndex === 3}
      <Pill text={String(value)} classes={statusClass(String(value))} />
    {:else}
      {value}
    {/if}
  {/snippet}
  {#snippet empty()}
    <p class="empty-state">No gift cards found</p>
  {/snippet}
</Table>

<style>
  .empty-state {
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
  }
</style>
