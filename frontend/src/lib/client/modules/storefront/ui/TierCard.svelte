<script lang="ts">
  import { Pill, Progress } from '@juspay/svelte-ui-components';
  import type { CustomerTierInfo } from '../types';

  let { tier }: { tier: CustomerTierInfo | null } = $props();

  let tierColorClass = $derived.by(() => {
    if (tier === null) return 'pill-neutral';
    if (tier.rank >= 3) return 'pill-warning';
    if (tier.rank === 2) return 'pill-info';
    return 'pill-neutral';
  });

  let earnLabel = $derived(
    tier !== null && tier.earn_rate_multiplier > 1
      ? `${tier.earn_rate_multiplier}x earn rate`
      : null
  );
</script>

{#if tier !== null}
  <div class="tier-card">
    <div class="tier-header">
      <div class="tier-badge">
        <Pill text={tier.tier_name} classes={tierColorClass} />
        {#if earnLabel !== null}
          <span class="earn-rate">{earnLabel}</span>
        {/if}
      </div>
    </div>

    {#if tier.progress_to_next !== null}
      <div class="tier-progress">
        <div class="tier-progress-label">
          <span class="tier-progress-text">
            Progress to {tier.progress_to_next.next_tier_name}
          </span>
          <span class="tier-progress-value">
            {Math.round(tier.progress_to_next.percentage)}%
          </span>
        </div>
        <Progress value={tier.progress_to_next.percentage} classes="tier-progress-bar" />
        <p class="tier-progress-detail">
          {tier.progress_to_next.current_value.toLocaleString('en-IN')}
          / {tier.progress_to_next.threshold.toLocaleString('en-IN')} points
        </p>
      </div>
    {:else}
      <p class="tier-max">You've reached the highest tier</p>
    {/if}
  </div>
{/if}

<style>
  .tier-card {
    background: var(--color-surface);
    border-radius: var(--radius-lg);
    padding: var(--space-5);
    box-shadow: var(--shadow-sm);
  }

  .tier-header {
    margin-bottom: var(--space-4);
  }

  .tier-badge {
    display: flex;
    align-items: center;
    gap: var(--space-3);
  }

  .earn-rate {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    font-weight: var(--font-weight-medium);
  }

  .tier-progress {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .tier-progress-label {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .tier-progress-text {
    font-size: var(--font-size-sm);
    color: var(--color-text);
    font-weight: var(--font-weight-medium);
  }

  .tier-progress-value {
    font-size: var(--font-size-sm);
    color: var(--color-primary);
    font-weight: var(--font-weight-semibold);
  }

  .tier-progress :global(.tier-progress-bar) {
    --progress-track-height: 8px;
  }

  .tier-progress-detail {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .tier-max {
    font-size: var(--font-size-sm);
    color: var(--color-success);
    font-weight: var(--font-weight-medium);
  }
</style>
