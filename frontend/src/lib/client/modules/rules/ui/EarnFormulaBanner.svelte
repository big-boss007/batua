<script lang="ts">
  let {
    multiplier,
    campaignName,
    stackingMode = 'multiplicative',
    maxCap = 10
  }: {
    multiplier: number;
    campaignName: string;
    stackingMode?: string;
    maxCap?: number;
  } = $props();

  let baseEarn = 100;
  let tierMultiplier = 1.25;
  let effective = $derived(Math.round(baseEarn * multiplier * tierMultiplier));

  function formatStackingMode(mode: string): string {
    if (mode === 'best_of') return 'Best-of';
    return mode.charAt(0).toUpperCase() + mode.slice(1);
  }
</script>

<div class="earn-banner">
  <div class="earn-banner-title">Active Campaign Earning Formula</div>
  <div class="earn-formula">
    <div class="formula-block">
      <span class="formula-label">Base Earn</span>
      <span class="formula-val">{baseEarn}</span>
      <span class="formula-src">from rule</span>
    </div>
    <span class="formula-op">&times;</span>
    <div class="formula-block">
      <span class="formula-label">Campaign</span>
      <span class="formula-val blue">{multiplier}x</span>
      <span class="formula-src">{campaignName}</span>
    </div>
    <span class="formula-op">&times;</span>
    <div class="formula-block">
      <span class="formula-label">Tier</span>
      <span class="formula-val purple">{tierMultiplier}x</span>
      <span class="formula-src">Silver member</span>
    </div>
    <span class="formula-op">=</span>
    <div class="formula-result">
      <span class="formula-label">Effective</span>
      <span class="formula-val">{effective}</span>
    </div>
  </div>
  <div class="earn-example">
    A Silver member earning {baseEarn} base points will receive <strong>{effective} points</strong> during this campaign.
    Stacking: <strong>{formatStackingMode(stackingMode)}</strong> &middot; Max cap: <strong>{maxCap}x</strong>
  </div>
</div>

<style>
  .earn-banner {
    background: linear-gradient(135deg, #f0fdf4 0%, #eff6ff 100%);
    border: 1px solid #bbf7d0;
    border-radius: var(--radius-lg);
    padding: var(--space-4) var(--space-5);
  }

  .earn-banner-title {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: #065f46;
    margin-bottom: var(--space-3);
    display: flex;
    align-items: center;
    gap: var(--space-2);
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
    padding: var(--space-2) var(--space-3);
    background: white;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    min-width: 70px;
  }

  .formula-label {
    font-size: 9px;
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.03em;
  }

  .formula-val {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-bold);
  }

  .formula-val.blue { color: #3b82f6; }
  .formula-val.purple { color: #6366f1; }

  .formula-src {
    font-size: 9px;
    color: var(--color-text-muted);
  }

  .formula-op {
    font-size: 18px;
    font-weight: 300;
    color: var(--color-text-muted);
  }

  .formula-result {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2px;
    padding: var(--space-2) var(--space-3);
    background: #065f46;
    border-radius: var(--radius-md);
    min-width: 70px;
  }

  .formula-result .formula-label { color: rgba(255, 255, 255, 0.7); }
  .formula-result .formula-val { color: white; }

  .earn-example {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    margin-top: var(--space-3);
    line-height: 1.5;
  }

  .earn-example strong { color: var(--color-text); }
</style>
