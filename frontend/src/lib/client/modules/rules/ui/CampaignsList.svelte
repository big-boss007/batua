<script lang="ts">
  import type { Campaign, Rule } from '../types';
  import { Progress } from '@juspay/svelte-ui-components';

  let {
    campaigns,
    rules = [],
    onSelect
  }: {
    campaigns: Array<Campaign>;
    rules?: Array<Rule>;
    onSelect?: (campaign: Campaign) => void;
  } = $props();

  function getStatus(campaign: Campaign): 'active' | 'upcoming' | 'ended' {
    const now = new Date();
    const start = new Date(campaign.starts_at);
    const end = new Date(campaign.ends_at);
    if (now < start) return 'upcoming';
    if (now > end || !campaign.is_active) return 'ended';
    return 'active';
  }

  function formatDateRange(startsAt: string, endsAt: string): string {
    const fmt = (iso: string) =>
      new Date(iso).toLocaleDateString('en-IN', { day: 'numeric', month: 'short' });
    const fmtYear = (iso: string) =>
      new Date(iso).toLocaleDateString('en-IN', { day: 'numeric', month: 'short', year: 'numeric' });
    return `${fmt(startsAt)} – ${fmtYear(endsAt)}`;
  }

  function getProgressPercent(campaign: Campaign): number {
    const now = Date.now();
    const start = new Date(campaign.starts_at).getTime();
    const end = new Date(campaign.ends_at).getTime();
    if (now <= start) return 0;
    if (now >= end) return 100;
    return Math.round(((now - start) / (end - start)) * 100);
  }

  function getDaysLeft(campaign: Campaign): number {
    const diff = new Date(campaign.ends_at).getTime() - Date.now();
    return Math.max(0, Math.ceil(diff / 86400000));
  }

  function getDaysUntilStart(campaign: Campaign): number {
    const diff = new Date(campaign.starts_at).getTime() - Date.now();
    return Math.max(0, Math.ceil(diff / 86400000));
  }

  function getRuleName(campaign: Campaign): string | null {
    if (campaign.base_rule_id === null) return null;
    const rule = rules.find((r) => r.id === campaign.base_rule_id);
    return rule ? rule.name : null;
  }
</script>

<div class="campaigns-list">
  {#if campaigns.length === 0}
    <div class="empty-state">
      <div class="empty-icon">📢</div>
      <div class="empty-text">No campaigns yet. Create a campaign to boost reward earning during special periods.</div>
    </div>
  {:else}
    <div class="campaigns-grid">
      {#each campaigns as campaign (campaign.id)}
        {@const status = getStatus(campaign)}
        {@const ruleName = getRuleName(campaign)}
        <div
          class="campaign-card"
          class:active-card={status === 'active'}
          class:upcoming-card={status === 'upcoming'}
          class:ended-card={status === 'ended'}
          onclick={() => onSelect?.(campaign)}
          onkeydown={(e) => { if (e.key === 'Enter') onSelect?.(campaign); }}
          role="button"
          tabindex="0"
        >
          <div class="campaign-header">
            <span class="campaign-name">{campaign.name}</span>
            <span class="status-badge" class:status-active={status === 'active'} class:status-upcoming={status === 'upcoming'} class:status-ended={status === 'ended'}>
              {status}
            </span>
          </div>

          <div class="campaign-meta">
            <div class="campaign-row">
              <span class="campaign-label">Dates</span>
              <span class="campaign-val">{formatDateRange(campaign.starts_at, campaign.ends_at)}</span>
            </div>
            {#if campaign.multiplier !== null}
              <div class="campaign-row">
                <span class="campaign-label">Multiplier</span>
                <span class="campaign-val mult">{campaign.multiplier}x</span>
              </div>
            {/if}
          </div>

          {#if ruleName !== null}
            <div class="campaign-rule">
              <span class="campaign-rule-label">Rule:</span>
              <span class="campaign-rule-name">{ruleName}</span>
            </div>
          {/if}

          {#if status === 'active'}
            {@const pct = getProgressPercent(campaign)}
            {@const left = getDaysLeft(campaign)}
            <div class="campaign-progress">
              <div class="campaign-progress-header">
                <span>Progress</span>
                <span>{pct}% &middot; {left} days left</span>
              </div>
              <Progress value={pct} max={100} classes="campaign-progress-bar" />
            </div>
          {/if}

          {#if status === 'upcoming'}
            <div class="starts-in">Starts in {getDaysUntilStart(campaign)} days</div>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .campaigns-list { width: 100%; }

  .empty-state {
    padding: var(--space-8);
    text-align: center;
    border: 1px dashed var(--color-border);
    border-radius: var(--radius-lg);
  }

  .empty-icon { font-size: 32px; margin-bottom: var(--space-2); }

  .empty-text {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .campaigns-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-4);
  }

  .campaign-card {
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: var(--space-4);
    background: white;
    cursor: pointer;
    transition: all 0.15s;
  }

  .campaign-card:hover {
    border-color: #c7d2fe;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  }

  .active-card { border-color: #86efac; background: #fafffe; }
  .upcoming-card { border-color: #c7d2fe; }
  .ended-card { opacity: 0.6; }

  .campaign-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: var(--space-3);
  }

  .campaign-name {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
  }

  .status-badge {
    padding: 3px 10px;
    border-radius: 12px;
    font-size: 10px;
    font-weight: var(--font-weight-semibold);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .status-active { background: #d1fae5; color: #065f46; }
  .status-upcoming { background: #dbeafe; color: #1d4ed8; }
  .status-ended { background: #f3f4f6; color: #6b7280; }

  .campaign-meta {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .campaign-row {
    display: flex;
    justify-content: space-between;
    font-size: var(--font-size-sm);
  }

  .campaign-label { color: var(--color-text-muted); }

  .campaign-val {
    color: var(--color-text);
    font-weight: var(--font-weight-medium);
  }

  .campaign-val.mult {
    color: #6366f1;
    font-weight: var(--font-weight-bold);
    font-family: var(--font-mono);
  }

  .campaign-rule {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    margin-top: var(--space-2);
    padding-top: var(--space-2);
    border-top: 1px solid var(--color-surface-2);
  }

  .campaign-rule-label {
    font-size: 10px;
    color: var(--color-text-muted);
  }

  .campaign-rule-name {
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-medium);
    color: var(--color-text);
    padding: 2px 8px;
    background: var(--color-surface);
    border-radius: var(--radius-sm);
  }

  .campaign-progress {
    margin-top: var(--space-2);
  }

  .campaign-progress-header {
    display: flex;
    justify-content: space-between;
    font-size: 10px;
    color: var(--color-text-muted);
    margin-bottom: 3px;
  }

  .starts-in {
    margin-top: var(--space-2);
    font-size: var(--font-size-xs);
    color: #1d4ed8;
  }
</style>
