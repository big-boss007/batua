<script lang="ts">
  let {
    loyaltyRate,
    nonLoyaltyRate
  }: {
    loyaltyRate: number;
    nonLoyaltyRate: number;
  } = $props();

  let maxRate = $derived(Math.max(loyaltyRate, nonLoyaltyRate, 1));
  let loyaltyWidth = $derived((loyaltyRate / maxRate) * 100);
  let nonLoyaltyWidth = $derived((nonLoyaltyRate / maxRate) * 100);
  let reduction = $derived(
    nonLoyaltyRate > 0 ? Math.round(((nonLoyaltyRate - loyaltyRate) / nonLoyaltyRate) * 100) : 0
  );
</script>

<div class="rto-comparison">
  <h3 class="comparison-title">RTO Rate Comparison</h3>

  {#if reduction > 0}
    <p class="reduction-badge">{reduction}% lower RTO with loyalty</p>
  {/if}

  <div class="comparison-bars">
    <div class="bar-group">
      <div class="bar-header">
        <span class="bar-label">Loyalty Members</span>
        <span class="bar-value">{loyaltyRate.toFixed(1)}%</span>
      </div>
      <div class="bar-track">
        <div class="bar-fill bar-loyalty" style="width: {loyaltyWidth}%"></div>
      </div>
    </div>

    <div class="bar-group">
      <div class="bar-header">
        <span class="bar-label">Non-Members</span>
        <span class="bar-value">{nonLoyaltyRate.toFixed(1)}%</span>
      </div>
      <div class="bar-track">
        <div class="bar-fill bar-non-loyalty" style="width: {nonLoyaltyWidth}%"></div>
      </div>
    </div>
  </div>
</div>

<style>
  .rto-comparison {
    padding: var(--space-6);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-sm);
  }

  .comparison-title {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
    margin-bottom: var(--space-4);
  }

  .reduction-badge {
    display: inline-block;
    padding: var(--space-1) var(--space-3);
    background: color-mix(in srgb, var(--color-success) 12%, transparent);
    color: var(--color-success);
    border-radius: var(--radius-full);
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    margin-bottom: var(--space-6);
  }

  .comparison-bars {
    display: flex;
    flex-direction: column;
    gap: var(--space-5);
  }

  .bar-group {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .bar-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .bar-label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-text);
  }

  .bar-value {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
  }

  .bar-track {
    height: 12px;
    background: var(--color-surface-2);
    border-radius: var(--radius-full);
    overflow: hidden;
  }

  .bar-fill {
    height: 100%;
    border-radius: var(--radius-full);
    transition: width var(--transition-slow);
    min-width: 2px;
  }

  .bar-loyalty {
    background: var(--color-success);
  }

  .bar-non-loyalty {
    background: var(--color-error);
  }
</style>
