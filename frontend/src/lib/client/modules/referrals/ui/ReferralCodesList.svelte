<script lang="ts">
  import type { ReferralCode } from '$lib/client/modules/referrals';

  let { codes }: { codes: Array<ReferralCode> } = $props();
</script>

{#if codes.length === 0}
  <div class="empty-state">
    <p class="empty-text">No referral codes found</p>
  </div>
{:else}
  <div class="table-wrapper">
    <table class="table">
      <thead>
        <tr>
          <th>Code</th>
          <th>Customer</th>
          <th>Type</th>
          <th>Referrals</th>
          <th>Conversions</th>
          <th>Status</th>
        </tr>
      </thead>
      <tbody>
        {#each codes as code (code.id)}
          <tr>
            <td><code class="code-cell">{code.code}</code></td>
            <td class="truncate">{code.customer_id}</td>
            <td>
              <div class="type-badges">
                {#if code.is_vanity}
                  <span class="badge badge-info">vanity</span>
                {/if}
                {#if code.is_creator}
                  <span class="badge badge-primary">creator</span>
                {/if}
                {#if !code.is_vanity && !code.is_creator}
                  <span class="badge badge-muted">auto</span>
                {/if}
              </div>
            </td>
            <td>{code.total_referrals}</td>
            <td>{code.total_conversions}</td>
            <td>
              <span class="badge {code.is_active ? 'badge-success' : 'badge-muted'}">
                {code.is_active ? 'active' : 'inactive'}
              </span>
            </td>
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

  .truncate {
    max-width: 140px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .code-cell {
    font-family: var(--font-mono);
    font-size: var(--font-size-xs);
    padding: var(--space-1) var(--space-2);
    background: var(--color-surface-2);
    border-radius: var(--radius-sm);
  }

  .type-badges {
    display: flex;
    gap: var(--space-1);
  }

  .badge {
    display: inline-block;
    padding: var(--space-1) var(--space-2);
    border-radius: var(--radius-full);
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-medium);
  }

  .badge-success {
    color: var(--color-success);
    background: color-mix(in srgb, var(--color-success) 12%, transparent);
  }

  .badge-info {
    color: var(--color-info);
    background: color-mix(in srgb, var(--color-info) 12%, transparent);
  }

  .badge-primary {
    color: var(--color-primary);
    background: color-mix(in srgb, var(--color-primary) 12%, transparent);
  }

  .badge-muted {
    color: var(--color-text-muted);
    background: var(--color-surface-2);
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
