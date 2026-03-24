<script lang="ts">
  import type { CustomerTierInfo } from '../types';

  let {
    tier,
    isMember = false,
    effectiveTierName = null,
    membershipDaysRemaining = null,
    pointsIcon = 'pts'
  }: {
    tier: CustomerTierInfo | null;
    isMember?: boolean;
    effectiveTierName?: string | null;
    membershipDaysRemaining?: number | null;
    pointsIcon?: string;
  } = $props();

  let icon = $derived(pointsIcon);

  let displayTierName = $derived(
    isMember && effectiveTierName !== null ? effectiveTierName : tier?.tier_name ?? null
  );

  let sectionTitle = $derived(isMember ? 'Your Tier' : 'Loyalty Tier');

  let percentage = $derived.by(() => {
    if (tier === null || tier.progress_to_next === null) return 0;
    return Math.round(tier.progress_to_next.percentage);
  });

  let earnLabel = $derived.by(() => {
    if (tier === null || tier.progress_to_next === null) return '';
    return isMember
      ? `Earning toward ${tier.progress_to_next.next_tier_name}`
      : `${(tier.progress_to_next.threshold - tier.progress_to_next.current_value).toLocaleString('en-IN')} ${icon} to next tier`;
  });

  let earnHint = $derived.by(() => {
    if (tier === null || tier.progress_to_next === null) return '';
    if (!isMember) return '';
    const remaining = tier.progress_to_next.threshold - tier.progress_to_next.current_value;
    return `${remaining.toLocaleString('en-IN')} ${icon} to go`;
  });
</script>

{#if tier !== null}
  <div class="tier-section">
    <div class="section-title">{sectionTitle}</div>

    <!-- Active tier badge row -->
    {#if isMember}
      <div class="active-tier-row">
        <div class="tier-badge">
          <span class="tier-icon">&#9733;</span>
          {displayTierName}
        </div>
        <div class="member-right">
          <div class="member-tag">Member</div>
          {#if membershipDaysRemaining !== null}
            <div class="days-left">{membershipDaysRemaining}d left</div>
          {/if}
        </div>
      </div>
    {:else}
      {#if tier.progress_to_next !== null}
        <div class="tier-header">
          <div class="tier-badge">
            <span class="tier-icon">&#9733;</span>
            {displayTierName}
          </div>
          <div class="tier-hint">{earnLabel}</div>
        </div>
      {:else}
        <div class="tier-badge">
          <span class="tier-icon">&#9733;</span>
          {displayTierName}
        </div>
      {/if}
    {/if}

    <!-- Earn progress -->
    {#if tier.progress_to_next !== null}
      {#if isMember}
        <div class="earn-progress">
          <div class="earn-header">
            <span class="earn-label">{earnLabel}</span>
            <span class="earn-hint-text">{earnHint}</span>
          </div>
          <div class="progress-track">
            <div class="progress-fill" style="width: {percentage}%"></div>
          </div>
          <div class="progress-labels">
            <span>{tier.progress_to_next.current_value.toLocaleString('en-IN')} {icon}</span>
            <span class="progress-total">{tier.progress_to_next.threshold.toLocaleString('en-IN')}</span>
          </div>
        </div>
      {:else}
        <div class="progress-track">
          <div class="progress-fill" style="width: {percentage}%"></div>
        </div>
        <div class="progress-labels">
          <span>{tier.progress_to_next.current_value.toLocaleString('en-IN')} {icon}</span>
          <span class="progress-total">{tier.progress_to_next.threshold.toLocaleString('en-IN')}</span>
        </div>
      {/if}
    {:else if !isMember}
      <div class="max-tier-msg">You've reached the highest tier</div>
    {/if}
  </div>
{/if}

<style>
  .tier-section {
    padding: 0;
  }

  .section-title {
    font-size: 12px;
    color: #9ca3af;
    text-transform: uppercase;
    letter-spacing: 0.8px;
    margin-bottom: 14px;
    font-weight: 500;
  }

  .active-tier-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 16px;
  }

  .tier-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 14px;
  }

  .tier-badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid #2a2d3a;
    border-radius: 20px;
    padding: 5px 14px 5px 10px;
    font-size: 13px;
    font-weight: 600;
    color: #ffffff;
  }

  .tier-icon {
    font-size: 14px;
  }

  .tier-hint {
    font-size: 12px;
    color: #9ca3af;
  }

  .member-right {
    text-align: right;
  }

  .member-tag {
    font-size: 11px;
    font-weight: 600;
    color: #fde047;
    background: rgba(253, 224, 71, 0.1);
    border: 1px solid rgba(253, 224, 71, 0.2);
    padding: 3px 10px;
    border-radius: 12px;
    display: inline-block;
  }

  .days-left {
    font-size: 11px;
    color: #9ca3af;
    margin-top: 4px;
  }

  .earn-progress {
    border-top: 1px solid #2a2d3a;
    padding-top: 14px;
  }

  .earn-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 10px;
  }

  .earn-label {
    font-size: 12px;
    color: #9ca3af;
  }

  .earn-hint-text {
    font-size: 11px;
    color: #9ca3af;
  }

  .progress-track {
    width: 100%;
    height: 6px;
    background: #2a2d3a;
    border-radius: 3px;
    overflow: hidden;
    margin-bottom: 6px;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #6366f1, #818cf8);
    border-radius: 3px;
  }

  .progress-labels {
    display: flex;
    justify-content: space-between;
    font-size: 11px;
    color: #9ca3af;
  }

  .progress-total {
    color: #ffffff;
    font-weight: 500;
  }

  .max-tier-msg {
    font-size: 12px;
    color: #4ade80;
    margin-top: 8px;
  }
</style>
