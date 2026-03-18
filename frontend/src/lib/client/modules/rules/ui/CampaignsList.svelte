<script lang="ts">
  import type { Campaign } from '../types';

  let { campaigns }: {
    campaigns: Array<Campaign>;
  } = $props();

  function getStatus(campaign: Campaign): 'active' | 'upcoming' | 'expired' {
    const now = new Date();
    const start = new Date(campaign.starts_at);
    const end = new Date(campaign.ends_at);

    if (now < start) return 'upcoming';
    if (now > end || !campaign.is_active) return 'expired';
    return 'active';
  }

  function formatDate(iso: string): string {
    return new Date(iso).toLocaleDateString('en-IN', {
      day: 'numeric',
      month: 'short',
      year: 'numeric'
    });
  }

  function formatDateRange(startsAt: string, endsAt: string): string {
    return `${formatDate(startsAt)} - ${formatDate(endsAt)}`;
  }
</script>

<div class="campaigns-list">
  {#if campaigns.length === 0}
    <div class="empty-state">
      <p class="empty-title">No campaigns yet</p>
      <p class="empty-description">Create a campaign from a festive template below.</p>
    </div>
  {:else}
    <div class="campaigns-grid">
      {#each campaigns as campaign (campaign.id)}
        {@const status = getStatus(campaign)}
        <div class="campaign-card">
          <div class="campaign-header">
            <h3 class="campaign-name">{campaign.name}</h3>
            <span class="status-badge" class:active={status === 'active'} class:upcoming={status === 'upcoming'} class:expired={status === 'expired'}>
              {status}
            </span>
          </div>
          <div class="campaign-details">
            <div class="detail-row">
              <span class="detail-label">Type</span>
              <span class="detail-value">{campaign.campaign_type}</span>
            </div>
            <div class="detail-row">
              <span class="detail-label">Dates</span>
              <span class="detail-value">{formatDateRange(campaign.starts_at, campaign.ends_at)}</span>
            </div>
            {#if campaign.multiplier !== null}
              <div class="detail-row">
                <span class="detail-label">Multiplier</span>
                <span class="detail-value multiplier">{campaign.multiplier}x</span>
              </div>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .campaigns-list {
    width: 100%;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: var(--space-12) var(--space-8);
    text-align: center;
  }

  .empty-title {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
    margin-bottom: var(--space-2);
  }

  .empty-description {
    font-size: var(--font-size-base);
    color: var(--color-text-muted);
  }

  .campaigns-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
    gap: var(--space-4);
  }

  .campaign-card {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    padding: var(--space-5);
    transition: box-shadow var(--transition-fast);
  }

  .campaign-card:hover {
    box-shadow: var(--shadow-md);
  }

  .campaign-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: var(--space-4);
  }

  .campaign-name {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .status-badge {
    display: inline-block;
    padding: var(--space-1) var(--space-2);
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-semibold);
    border-radius: var(--radius-full);
    text-transform: uppercase;
    letter-spacing: 0.03em;
  }

  .status-badge.active {
    color: var(--color-success);
    background: color-mix(in srgb, var(--color-success) 12%, transparent);
  }

  .status-badge.upcoming {
    color: var(--color-info);
    background: color-mix(in srgb, var(--color-info) 12%, transparent);
  }

  .status-badge.expired {
    color: var(--color-text-muted);
    background: var(--color-surface-2);
  }

  .campaign-details {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .detail-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .detail-label {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .detail-value {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-text);
  }

  .multiplier {
    color: var(--color-primary);
    font-weight: var(--font-weight-bold);
  }
</style>
