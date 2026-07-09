<script lang="ts">
  import type { TransactionFilters } from '$lib/client/modules/transactions';
  import { formatBucketType, formatMovementType } from '$lib/client/modules/transactions';
  import { Select, Button } from '@juspay/svelte-ui-components';

  let {
    filters,
    onChange
  }: {
    filters: TransactionFilters;
    onChange: (updated: TransactionFilters) => void;
  } = $props();

  const BUCKET_TYPES = [
    'earned_credit',
    'cod_pending',
    'gift_card',
    'customer_funded',
    'referral_reward',
    'goodwill_credit',
    'membership_benefit',
    'refund_credit'
  ];
  const MOVEMENT_TYPES = ['in', 'out', 'held', 'across'];

  let bucketItems = $derived(BUCKET_TYPES.map((bt) => ({ id: bt, label: formatBucketType(bt) })));
  let movementItems = $derived(
    MOVEMENT_TYPES.map((mt) => ({ id: mt, label: formatMovementType(mt).label }))
  );

  let selectedBucket = $derived(filters.bucket_type ? [filters.bucket_type] : []);
  let selectedMovement = $derived(filters.movement_type ? [filters.movement_type] : []);

  function handleBucketChange(value: string[]) {
    const raw = value.length > 0 ? value[0] : null;
    onChange({ ...filters, bucket_type: raw, page: 1 });
  }

  function handleMovementChange(value: string[]) {
    const raw = value.length > 0 ? value[0] : null;
    onChange({ ...filters, movement_type: raw, page: 1 });
  }

  function handleClear() {
    onChange({ bucket_type: null, movement_type: null, page: 1, limit: filters.limit });
  }

  let hasActiveFilters = $derived(filters.bucket_type !== null || filters.movement_type !== null);
</script>

<div class="filter-bar">
  <div class="filter-group">
    <span class="filter-label">Bucket type</span>
    <Select
      placeholder="All buckets"
      items={bucketItems}
      value={selectedBucket}
      onchange={handleBucketChange}
    />
  </div>

  <div class="filter-group">
    <span class="filter-label">Movement type</span>
    <Select
      placeholder="All movements"
      items={movementItems}
      value={selectedMovement}
      onchange={handleMovementChange}
    />
  </div>

  {#if hasActiveFilters}
    <Button text="Clear filters" classes="btn-ghost" onclick={handleClear} />
  {/if}
</div>

<style>
  .filter-bar {
    display: flex;
    align-items: flex-end;
    gap: var(--space-4);
    flex-wrap: wrap;
    padding: var(--space-4);
    background: var(--color-surface);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-card);
  }

  .filter-group {
    min-width: 160px;
  }

  .filter-label {
    display: block;
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    margin-bottom: var(--space-1);
    font-weight: var(--font-weight-medium);
  }
</style>
