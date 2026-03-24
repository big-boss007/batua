<script lang="ts">
  import { formatPoints } from '$lib/client/modules/foundation';
  import { toastStore } from '$lib/client/modules/foundation';
  import type { ReferralCodeInfo } from '../types';
  import ShareButtons from './ShareButtons.svelte';

  let {
    code,
    referralReward,
    merchantName,
    pointsIcon = 'pts'
  }: {
    code: ReferralCodeInfo;
    referralReward: number;
    merchantName: string;
    pointsIcon?: string;
  } = $props();

  let shareUrl = $derived(
    typeof window !== 'undefined'
      ? `${window.location.origin}/s/${encodeURIComponent(merchantName.toLowerCase().replace(/\s+/g, '-'))}/refer/${code.code}`
      : ''
  );
  let shareText = $derived(
    `Hey! Use my referral code ${code.code} at ${merchantName} and get ${formatPoints(referralReward, pointsIcon)} in rewards!`
  );

  function handleCopy() {
    toastStore.push({ message: 'Link copied!', level: 'success' });
  }
</script>

<div class="referral-card">
  <h3 class="referral-title">Refer & Earn</h3>
  <p class="referral-subtitle">
    Share your code and earn {formatPoints(referralReward, pointsIcon)} for each referral
  </p>

  <div class="referral-code-box">
    <span class="referral-code">{code.code}</span>
  </div>

  <div class="referral-stats">
    <div class="stat">
      <span class="stat-value">{code.total_referrals}</span>
      <span class="stat-label">Referrals</span>
    </div>
    <div class="stat-divider"></div>
    <div class="stat">
      <span class="stat-value">{code.total_conversions}</span>
      <span class="stat-label">Conversions</span>
    </div>
  </div>

  <ShareButtons text={shareText} url={shareUrl} onCopy={handleCopy} />
</div>

<style>
  .referral-card {
    background: var(--color-surface);
    border-radius: var(--radius-lg);
    padding: var(--space-5);
    box-shadow: var(--shadow-sm);
  }

  .referral-title {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
    margin-bottom: var(--space-1);
  }

  .referral-subtitle {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    margin-bottom: var(--space-4);
  }

  .referral-code-box {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--space-4);
    background: var(--color-surface-2);
    border: 2px dashed var(--color-border);
    border-radius: var(--radius-md);
    margin-bottom: var(--space-4);
  }

  .referral-code {
    font-family: var(--font-mono);
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-primary);
    letter-spacing: 0.05em;
  }

  .referral-stats {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-6);
    margin-bottom: var(--space-5);
  }

  .stat {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-1);
  }

  .stat-value {
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
  }

  .stat-label {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .stat-divider {
    width: 1px;
    height: 32px;
    background: var(--color-border);
  }
</style>
