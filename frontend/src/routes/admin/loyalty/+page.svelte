<script lang="ts">
  import type { PageData } from './$types';
  import type {
    LoyaltyProgram,
    LoyaltyTier,
    TierDistribution
  } from '$lib/client/modules/customers';
  import {
    createProgram,
    createTier,
    evaluateTier,
    fetchTiers,
    fetchTierDistribution,
    sortTiersByRank
  } from '$lib/client/modules/customers';
  import { toastStore } from '$lib/client/modules/foundation';
  import {
    LoyaltyProgramForm,
    TierForm,
    TierBadge,
    TierDistributionChart
  } from '$lib/client/modules/customers/ui';

  let { data }: { data: PageData } = $props();

  let program = $state<LoyaltyProgram | null>(data.program);
  let tiers = $state<Array<LoyaltyTier>>(data.tiers);
  let distribution = $state<Array<TierDistribution>>(data.distribution);
  let showTierForm = $state(false);
  let evaluating = $state(false);

  let sortedTiers = $derived(sortTiersByRank(tiers));

  async function handleSaveProgram(formData: { name: string; evaluation_criteria: string }) {
    const result = await createProgram('default', formData);

    if (result.tag === 'success') {
      program = result.data;
      toastStore.push({ message: 'Loyalty program created', level: 'success' });
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }
  }

  async function handleSaveTier(formData: {
    name: string;
    rank: number;
    threshold: number;
    earn_rate_multiplier: number;
    benefits: Record<string, unknown>;
  }) {
    const result = await createTier('default', formData);

    if (result.tag === 'success') {
      tiers = [...tiers, result.data];
      showTierForm = false;
      toastStore.push({ message: `Tier "${result.data.name}" created`, level: 'success' });
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }
  }

  async function handleEvaluate() {
    evaluating = true;

    const result = await evaluateTier('default');

    if (result.tag === 'success') {
      toastStore.push({
        message: `Evaluated ${result.data.evaluated} customers`,
        level: 'success'
      });

      const [tiersResult, distResult] = await Promise.all([
        fetchTiers('default'),
        fetchTierDistribution('default')
      ]);

      if (tiersResult.tag === 'success') {
        tiers = tiersResult.data;
      }
      if (distResult.tag === 'success') {
        distribution = distResult.data;
      }
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }

    evaluating = false;
  }
</script>

<svelte:head>
  <title>Loyalty Program - Batua</title>
</svelte:head>

<div class="loyalty-page">
  <header class="page-header">
    <div class="page-header-left">
      <h1 class="page-title">Loyalty Program</h1>
      <p class="page-subtitle">Manage your loyalty program, tiers, and customer distribution</p>
    </div>
    {#if program}
      <div class="page-actions">
        <button class="btn-secondary" onclick={handleEvaluate} disabled={evaluating}>
          {evaluating ? 'Evaluating...' : 'Evaluate Tiers'}
        </button>
      </div>
    {/if}
  </header>

  <div class="loyalty-layout">
    <section class="program-section">
      <LoyaltyProgramForm {program} onSave={handleSaveProgram} />
    </section>

    {#if program}
      <section class="tiers-section">
        <div class="section-header">
          <h2 class="section-title">Tiers</h2>
          <button class="btn-primary-sm" onclick={() => (showTierForm = !showTierForm)}>
            {showTierForm ? 'Cancel' : 'Add Tier'}
          </button>
        </div>

        {#if showTierForm}
          <TierForm tier={null} onSave={handleSaveTier} />
        {/if}

        {#if sortedTiers.length > 0}
          <div class="tiers-list">
            {#each sortedTiers as t (t.id)}
              <div class="tier-row">
                <div class="tier-row-left">
                  <TierBadge tierName={t.name} rank={t.rank} multiplier={t.earn_rate_multiplier} />
                </div>
                <div class="tier-row-meta">
                  <span class="tier-meta-item">
                    <span class="meta-label">Threshold</span>
                    <span class="meta-value">{t.threshold.toLocaleString('en-IN')}</span>
                  </span>
                  <span class="tier-meta-item">
                    <span class="meta-label">Rank</span>
                    <span class="meta-value">{t.rank}</span>
                  </span>
                </div>
              </div>
            {/each}
          </div>
        {:else}
          <p class="empty-tiers">No tiers configured yet. Add your first tier above.</p>
        {/if}
      </section>

      <section class="distribution-section">
        <TierDistributionChart {distribution} />
      </section>
    {/if}
  </div>
</div>

<style>
  .loyalty-page {
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
    padding: var(--space-8);
    max-width: 1000px;
    margin: 0 auto;
    width: 100%;
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: var(--space-4);
  }

  .page-header-left {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .page-title {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
  }

  .page-subtitle {
    font-size: var(--font-size-base);
    color: var(--color-text-muted);
  }

  .page-actions {
    flex-shrink: 0;
  }

  .loyalty-layout {
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
  }

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: var(--space-4);
  }

  .section-title {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .tiers-section {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .tiers-list {
    display: flex;
    flex-direction: column;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    overflow: hidden;
  }

  .tier-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--space-4) var(--space-6);
    border-bottom: 1px solid var(--color-border);
  }

  .tier-row:last-child {
    border-bottom: none;
  }

  .tier-row-left {
    display: flex;
    align-items: center;
  }

  .tier-row-meta {
    display: flex;
    gap: var(--space-6);
  }

  .tier-meta-item {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: var(--space-1);
  }

  .meta-label {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .meta-value {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
    font-family: var(--font-mono);
  }

  .empty-tiers {
    padding: var(--space-8);
    text-align: center;
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
    background: var(--color-surface);
    border: 1px dashed var(--color-border);
    border-radius: var(--radius-lg);
  }

  .btn-secondary {
    padding: var(--space-2) var(--space-5);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background: var(--color-surface);
    color: var(--color-text);
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    transition:
      background var(--transition-fast),
      border-color var(--transition-fast);
  }

  .btn-secondary:hover:not(:disabled) {
    background: var(--color-surface-2);
    border-color: var(--color-text-muted);
  }

  .btn-secondary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-primary-sm {
    padding: var(--space-2) var(--space-4);
    border: none;
    border-radius: var(--radius-md);
    background: var(--color-primary);
    color: #ffffff;
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    transition: background var(--transition-fast);
  }

  .btn-primary-sm:hover {
    background: var(--color-primary-hover);
  }

  @media (max-width: 600px) {
    .loyalty-page {
      padding: var(--space-4);
    }

    .page-header {
      flex-direction: column;
    }

    .tier-row {
      flex-direction: column;
      align-items: flex-start;
      gap: var(--space-3);
    }

    .tier-row-meta {
      width: 100%;
      justify-content: space-between;
    }

    .tier-meta-item {
      align-items: flex-start;
    }
  }
</style>
