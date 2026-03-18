<script lang="ts">
  import type { ReferralAnalytics } from '$lib/client/modules/referrals';

  let { analytics }: { analytics: ReferralAnalytics } = $props();

  let formattedRate = $derived(
    analytics.conversion_rate > 0 ? `${analytics.conversion_rate.toFixed(1)}%` : '0%'
  );
</script>

<div class="analytics-card">
  <h3 class="analytics-title">Referral Analytics</h3>
  <div class="metrics-grid">
    <div class="metric">
      <span class="metric-value">{analytics.total_codes}</span>
      <span class="metric-label">Total Codes</span>
    </div>
    <div class="metric">
      <span class="metric-value">{analytics.total_referrals}</span>
      <span class="metric-label">Total Referrals</span>
    </div>
    <div class="metric">
      <span class="metric-value">{analytics.total_conversions}</span>
      <span class="metric-label">Conversions</span>
    </div>
    <div class="metric">
      <span class="metric-value">{formattedRate}</span>
      <span class="metric-label">Conversion Rate</span>
    </div>
    {#if analytics.total_suspicious > 0}
      <div class="metric metric-warning">
        <span class="metric-value">{analytics.total_suspicious}</span>
        <span class="metric-label">Suspicious</span>
      </div>
    {/if}
  </div>
</div>

<style>
  .analytics-card {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    padding: var(--space-6);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
  }

  .analytics-title {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .metrics-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(130px, 1fr));
    gap: var(--space-4);
  }

  .metric {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    padding: var(--space-3);
    background: var(--color-bg);
    border-radius: var(--radius-md);
    border: 1px solid var(--color-border);
  }

  .metric-warning {
    border-color: var(--color-warning);
  }

  .metric-warning .metric-value {
    color: var(--color-warning);
  }

  .metric-value {
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    line-height: var(--line-height-tight);
  }

  .metric-label {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    font-weight: var(--font-weight-medium);
  }
</style>
