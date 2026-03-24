<script lang="ts">
  import { formatCurrencyINR } from '$lib/client/modules/foundation';
  import type { BucketBalance } from '../types';

  let { buckets }: { buckets: Array<BucketBalance> } = $props();

  let stats = $derived.by(() => {
    const earned = buckets.find((b) => b.bucket_type === 'earned_credit');
    const codPending = buckets.find((b) => b.bucket_type === 'cod_pending');
    const totalHeld = buckets.reduce((sum, b) => sum + Math.max(0, b.displayed - b.spendable), 0);

    return [
      { value: earned?.spendable ?? 0, label: 'Earned', highlight: false },
      { value: codPending?.spendable ?? 0, label: 'COD Pending', highlight: false },
      { value: totalHeld, label: 'On Hold', highlight: totalHeld > 0 },
      { value: 0, label: 'Expiring Soon', highlight: false }
    ];
  });
</script>

<div class="bucket-section">
  <div class="section-title">Bucket Breakdown</div>
  <div class="bucket-grid">
    {#each stats as stat}
      <div class="bucket-item">
        <div class="bucket-label">{stat.label}</div>
        <div class="bucket-value" class:highlight={stat.highlight}>
          {formatCurrencyINR(stat.value)}
        </div>
      </div>
    {/each}
  </div>
</div>

<style>
  .bucket-section {
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

  .bucket-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 10px;
  }

  .bucket-item {
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid #2a2d3a;
    border-radius: 10px;
    padding: 14px 16px;
  }

  .bucket-label {
    font-size: 11px;
    color: #9ca3af;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: 6px;
  }

  .bucket-value {
    font-size: 17px;
    font-weight: 600;
    color: #ffffff;
  }

  .bucket-value.highlight {
    color: #fde047;
  }
</style>
