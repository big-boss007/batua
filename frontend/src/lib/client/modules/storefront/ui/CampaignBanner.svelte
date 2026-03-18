<script lang="ts">
  import { formatDate } from '$lib/client/modules/foundation';

  let { campaign }: {
    campaign: { name: string; multiplier: number; ends_at: string }
  } = $props();
</script>

<div class="campaign-banner">
  <div class="campaign-icon">
    <span class="campaign-emoji">&#x1F525;</span>
  </div>
  <div class="campaign-content">
    <div class="campaign-top">
      <span class="campaign-name">{campaign.name}</span>
      <span class="campaign-multiplier">{campaign.multiplier}x</span>
    </div>
    <p class="campaign-date">Ends {formatDate(campaign.ends_at)}</p>
  </div>
</div>

<style>
  .campaign-banner {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-4);
    background: linear-gradient(135deg, #fefce8 0%, #fef3c7 100%);
    border: 1px solid #fde68a;
    border-radius: var(--radius-lg);
  }

  :global([data-theme='dark']) .campaign-banner {
    background: linear-gradient(135deg, #422006 0%, #451a03 100%);
    border-color: #854d0e;
  }

  .campaign-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 40px;
    height: 40px;
    flex-shrink: 0;
  }

  .campaign-emoji {
    font-size: 24px;
    line-height: 1;
  }

  .campaign-content {
    flex: 1;
    min-width: 0;
  }

  .campaign-top {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    margin-bottom: var(--space-1);
  }

  .campaign-name {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-semibold);
    color: #854d0e;
  }

  :global([data-theme='dark']) .campaign-name {
    color: #fde047;
  }

  .campaign-multiplier {
    display: inline-flex;
    align-items: center;
    padding: 2px 8px;
    background: #f59e0b;
    color: #ffffff;
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-bold);
    border-radius: var(--radius-full);
  }

  .campaign-date {
    font-size: var(--font-size-xs);
    color: #a16207;
  }

  :global([data-theme='dark']) .campaign-date {
    color: #fbbf24;
  }
</style>
