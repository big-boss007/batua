<script lang="ts">
  import { Pill } from '@juspay/svelte-ui-components';

  import type { ReferralConversion } from '$lib/client/modules/referrals';
  import { formatDateTime } from '$lib/client/modules/foundation';

  let { conversions }: { conversions: Array<ReferralConversion> } = $props();
</script>

{#if conversions.length === 0}
  <div class="empty-state">
    <p class="empty-text">No conversions yet</p>
  </div>
{:else}
  <div class="table-wrapper">
    <table class="table">
      <thead>
        <tr>
          <th>Referrer</th>
          <th>Referee</th>
          <th>Order</th>
          <th>Fraud signals</th>
          <th>Date</th>
        </tr>
      </thead>
      <tbody>
        {#each conversions as conversion (conversion.id)}
          <tr class:suspicious-row={conversion.is_suspicious}>
            <td class="truncate">{conversion.referrer_id}</td>
            <td class="truncate">{conversion.referee_id}</td>
            <td>{conversion.order_id ?? '--'}</td>
            <td>
              {#if conversion.fraud_signals.length > 0}
                <div class="signal-list">
                  {#each conversion.fraud_signals as signal}
                    <Pill text={signal} classes="pill-warning" />
                  {/each}
                </div>
              {:else}
                <Pill text="clean" classes="pill-success" />
              {/if}
            </td>
            <td class="nowrap">{formatDateTime(conversion.created_at)}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
{/if}

<style>
  .table-wrapper {
    overflow-x: auto;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
  }

  .table {
    width: 100%;
    border-collapse: collapse;
    font-size: var(--font-size-sm);
  }

  .table th {
    text-align: left;
    padding: var(--space-3) var(--space-4);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-muted);
    background: var(--color-surface);
    border-bottom: 1px solid var(--color-border);
    white-space: nowrap;
  }

  .table td {
    padding: var(--space-3) var(--space-4);
    border-bottom: 1px solid var(--color-border);
    vertical-align: middle;
  }

  .table tr:last-child td {
    border-bottom: none;
  }

  .suspicious-row {
    background: color-mix(in srgb, var(--color-warning) 5%, transparent);
  }

  .truncate {
    max-width: 140px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .nowrap {
    white-space: nowrap;
  }

  .signal-list {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-1);
  }

  .empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--space-12);
    border: 1px dashed var(--color-border);
    border-radius: var(--radius-lg);
  }

  .empty-text {
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
  }
</style>
