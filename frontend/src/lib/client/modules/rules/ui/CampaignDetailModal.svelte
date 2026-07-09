<script lang="ts">
  import type { Campaign, Rule, CampaignStackingConfig } from '../types';
  import { Button, Modal, Progress } from '@juspay/svelte-ui-components';
  import { MODAL_CLOSE_ICON } from '$lib/client/modules/foundation';

  let {
    campaign,
    rules,
    stackingConfig,
    onDeactivate,
    onClose
  }: {
    campaign: Campaign;
    rules: Array<Rule>;
    stackingConfig: CampaignStackingConfig;
    onDeactivate: (campaignId: string) => void;
    onClose: () => void;
  } = $props();

  let linkedRule = $derived(rules.find((r) => r.id === campaign.base_rule_id) ?? null);

  let status = $derived.by(() => {
    const now = new Date();
    const start = new Date(campaign.starts_at);
    const end = new Date(campaign.ends_at);
    if (now < start) return 'upcoming' as const;
    if (now > end || !campaign.is_active) return 'ended' as const;
    return 'active' as const;
  });

  let progressPercent = $derived.by(() => {
    const now = Date.now();
    const start = new Date(campaign.starts_at).getTime();
    const end = new Date(campaign.ends_at).getTime();
    if (now <= start) return 0;
    if (now >= end) return 100;
    return Math.round(((now - start) / (end - start)) * 100);
  });

  let daysRemaining = $derived.by(() => {
    const now = Date.now();
    const end = new Date(campaign.ends_at).getTime();
    const diff = end - now;
    return Math.max(0, Math.ceil(diff / 86400000));
  });

  function formatDateRange(startsAt: string, endsAt: string): string {
    const fmt = (iso: string) =>
      new Date(iso).toLocaleDateString('en-IN', {
        day: 'numeric',
        month: 'short',
        year: 'numeric'
      });
    return `${fmt(startsAt)} – ${fmt(endsAt)}`;
  }

  function formatStackingMode(mode: string): string {
    if (mode === 'best_of') return 'Best-of';
    return mode.charAt(0).toUpperCase() + mode.slice(1);
  }

  let mult = $derived(campaign.multiplier ?? 1);
</script>

<Modal
  size="medium"
  header={{ text: 'Campaign details', rightImage: MODAL_CLOSE_ICON }}
  onclose={onClose}
  onoverlayClick={onClose}
  onheaderRightImageClick={onClose}
>
  {#snippet content()}
    <div class="modal-body">
      <div class="campaign-title-row">
        <span class="campaign-name">{campaign.name}</span>
        <span
          class="status-badge"
          class:status-active={status === 'active'}
          class:status-upcoming={status === 'upcoming'}
          class:status-ended={status === 'ended'}
        >
          {status}
        </span>
      </div>

      <div class="info-grid">
        <div>
          <span class="label">Multiplier</span>
          <div class="value multiplier-val">{mult}x</div>
        </div>
        <div>
          <span class="label">Duration</span>
          <div class="value">{formatDateRange(campaign.starts_at, campaign.ends_at)}</div>
        </div>
      </div>

      {#if linkedRule !== null}
        <div class="linked-rule-section">
          <span class="label">Linked reward rule</span>
          <div class="rule-info">
            <span class="rule-name">{linkedRule.name}</span>
            <span class="rule-type-pill">{linkedRule.rule_type}</span>
            <span
              class="rule-status-pill"
              class:rule-active={linkedRule.is_active}
              class:rule-inactive={!linkedRule.is_active}
            >
              {linkedRule.is_active ? 'Active' : 'Inactive'}
            </span>
          </div>
          <div class="rule-event">
            Event: {linkedRule.config.event_type} &middot; {linkedRule.config.action.calculation ===
            'percentage'
              ? `${linkedRule.config.action.value}%`
              : `${linkedRule.config.action.value} point${linkedRule.config.action.value === 1 ? '' : 's'}`}
            per transaction
          </div>
        </div>
      {/if}

      {#if status === 'active'}
        <div class="progress-section">
          <div class="progress-header">
            <span>Campaign progress</span>
            <span>{progressPercent}% elapsed &middot; {daysRemaining} days remaining</span>
          </div>
          <Progress value={progressPercent} max={100} classes="campaign-progress-bar" />
        </div>
      {/if}

      <div class="formula-section">
        <div class="formula-title">Earn Formula During This Campaign</div>
        <div class="earn-formula">
          <div class="formula-block">
            <span class="formula-label">Base</span><span class="formula-val">100</span>
          </div>
          <span class="formula-op">&times;</span>
          <div class="formula-block">
            <span class="formula-label">Campaign</span><span class="formula-val blue">{mult}x</span>
          </div>
          <span class="formula-op">&times;</span>
          <div class="formula-block">
            <span class="formula-label">Tier</span><span class="formula-val purple">1x–2x</span>
          </div>
          <span class="formula-op">=</span>
          <div class="formula-result">
            <span class="formula-label">Effective</span><span class="formula-val"
              >{mult}x–{mult * 2}x</span
            >
          </div>
        </div>
      </div>

      <div class="performance-section">
        <span class="label">Campaign performance</span>
        <div class="perf-grid">
          <div class="perf-item">
            <span class="perf-label">Orders</span><span class="perf-value">—</span>
          </div>
          <div class="perf-item">
            <span class="perf-label">Customers</span><span class="perf-value">—</span>
          </div>
          <div class="perf-item">
            <span class="perf-label">Extra points</span><span class="perf-value green">—</span>
          </div>
          <div class="perf-item">
            <span class="perf-label">Extra cost</span><span class="perf-value green">—</span>
          </div>
        </div>
      </div>

      {#if status === 'active'}
        <Button
          text="Deactivate campaign"
          onclick={() => onDeactivate(campaign.id)}
          classes="btn-danger deactivate-btn"
        />
      {/if}
    </div>
  {/snippet}
</Modal>

<style>
  .modal-body {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    width: 100%;
  }

  .campaign-title-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .campaign-name {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
  }

  .status-badge {
    padding: 3px 10px;
    border-radius: var(--radius-lg);
    font-size: 10px;
    font-weight: var(--font-weight-semibold);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .status-active {
    background: var(--green-100);
    color: var(--green-700);
  }
  .status-upcoming {
    background: var(--p-100);
    color: var(--p-700);
  }
  .status-ended {
    background: var(--color-surface-2);
    color: var(--color-text-muted);
  }

  .label {
    font-size: 10px;
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .value {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-text);
    margin-top: 2px;
  }

  .multiplier-val {
    color: var(--purple-500);
    font-family: var(--font-mono);
    font-weight: var(--font-weight-bold);
  }

  .info-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-3);
    padding: var(--space-3);
    background: var(--color-surface);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-card);
  }

  .linked-rule-section {
    padding: var(--space-3);
    background: var(--color-surface);
    border-radius: var(--radius-lg);
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    box-shadow: var(--shadow-card);
  }

  .rule-info {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .rule-name {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
  }

  .rule-type-pill {
    font-size: 10px;
    padding: 2px 8px;
    background: var(--p-100);
    color: var(--p-700);
    border-radius: var(--radius-sm);
    font-weight: var(--font-weight-medium);
  }

  .rule-status-pill {
    font-size: 10px;
    padding: 2px 8px;
    border-radius: var(--radius-sm);
    font-weight: var(--font-weight-medium);
  }

  .rule-active {
    background: var(--green-100);
    color: var(--green-700);
  }
  .rule-inactive {
    background: var(--red-100);
    color: var(--red-700);
  }

  .rule-event {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .progress-section {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .progress-header {
    display: flex;
    justify-content: space-between;
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .formula-section {
    background: linear-gradient(135deg, var(--green-100) 0%, var(--p-100) 100%);
    border: 1px solid var(--green-100);
    border-radius: var(--radius-md);
    padding: var(--space-3) var(--space-4);
  }

  .formula-title {
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-semibold);
    color: var(--green-700);
    margin-bottom: var(--space-2);
  }

  .earn-formula {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    flex-wrap: wrap;
  }

  .formula-block {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2px;
    padding: 4px 10px;
    background: white;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    min-width: 50px;
  }

  .formula-label {
    font-size: 8px;
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.03em;
  }

  .formula-val {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-bold);
  }

  .formula-val.blue {
    color: var(--p-600);
  }
  .formula-val.purple {
    color: var(--purple-500);
  }

  .formula-op {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-normal);
    color: var(--color-text-muted);
  }

  .formula-result {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2px;
    padding: 4px 10px;
    background: var(--green-700);
    border-radius: var(--radius-md);
    min-width: 50px;
  }

  .formula-result .formula-label {
    color: rgba(255, 255, 255, 0.7);
  }
  .formula-result .formula-val {
    color: white;
  }

  .performance-section {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .perf-grid {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr 1fr;
    gap: var(--space-3);
    padding: var(--space-3);
    background: var(--color-surface);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-card);
  }

  .perf-item {
    text-align: center;
  }

  .perf-label {
    font-size: 9px;
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.04em;
    display: block;
  }

  .perf-value {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    margin-top: 2px;
    display: block;
  }

  .perf-value.green {
    color: var(--green-500);
  }
</style>
