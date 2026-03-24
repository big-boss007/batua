<script lang="ts">
  import { Button } from '@juspay/svelte-ui-components';

  import type {
    LoyaltyProgram,
    LoyaltyTier,
    TierDistribution
  } from '$lib/client/modules/customers';
  import {
    createTier,
    updateTier,
    deleteTier,
    evaluateTier,
    fetchLoyaltyProgram,
    fetchTiers,
    fetchTierDistribution,
    sortTiersByRank
  } from '$lib/client/modules/customers';
  import { currentMerchantId } from '$lib/client/modules/admin';
  import { toastStore } from '$lib/client/modules/foundation';
  import {
    TierForm,
    TierBadge,
    TierDistributionChart,
    TierWizard
  } from '$lib/client/modules/customers/ui';

  let program = $state<LoyaltyProgram | null>(null);
  let tiers = $state<Array<LoyaltyTier>>([]);
  let distribution = $state<Array<TierDistribution>>([]);
  let loading = $state(true);
  let showTierForm = $state(false);
  let editingTierId = $state<string | null>(null);
  let deletingTierId = $state<string | null>(null);
  let showWizard = $state(false);
  let wizardMode = $state<'fresh' | 'reconfigure'>('fresh');
  let evaluating = $state(false);
  let evaluateResult = $state<string | null>(null);
  let merchantId = $state<string | null>(null);
  let distributionEl = $state<HTMLElement | null>(null);

  let sortedTiers = $derived(sortTiersByRank(tiers));
  let shouldShowWizard = $derived(showWizard || (program === null && !loading));

  currentMerchantId.subscribe((id) => {
    const prevId = merchantId;
    merchantId = id;
    if (id !== null && id !== prevId) {
      if (typeof window !== 'undefined') {
        loadData(id);
      }
    }
  });

  async function loadData(mId: string) {
    loading = true;
    showWizard = false;
    const programResult = await fetchLoyaltyProgram(mId);
    program = programResult.tag === 'success' ? programResult.data : null;

    if (program !== null) {
      const [tiersResult, distributionResult] = await Promise.all([
        fetchTiers(program.id),
        fetchTierDistribution(mId)
      ]);
      tiers = tiersResult.tag === 'success' ? tiersResult.data : [];
      distribution = distributionResult.tag === 'success' ? distributionResult.data : [];
    } else {
      tiers = [];
      distribution = [];
    }
    loading = false;
  }

  function openReconfigure() {
    wizardMode = 'reconfigure';
    showWizard = true;
  }

  function handleWizardComplete() {
    showWizard = false;
    if (merchantId !== null) {
      loadData(merchantId);
    }
  }

  function handleWizardCancel() {
    showWizard = false;
  }

  async function handleSaveTier(formData: {
    name: string;
    rank: number;
    threshold: number;
    earn_rate_multiplier: number;
    benefits: Record<string, unknown>;
  }) {
    if (merchantId === null || program === null) return;
    const result = await createTier(program.id, formData);

    if (result.tag === 'success') {
      tiers = [...tiers, result.data];
      showTierForm = false;
      toastStore.push({ message: `Tier "${result.data.name}" created`, level: 'success' });
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }
  }

  async function handleUpdateTier(
    tierId: string,
    formData: {
      name: string;
      rank: number;
      threshold: number;
      earn_rate_multiplier: number;
      benefits: Record<string, unknown>;
    }
  ) {
    const result = await updateTier(tierId, formData);

    if (result.tag === 'success') {
      tiers = tiers.map((t) => (t.id === tierId ? result.data : t));
      editingTierId = null;
      toastStore.push({ message: `Tier "${result.data.name}" updated`, level: 'success' });
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }
  }

  async function handleDeleteTier(tierId: string) {
    const result = await deleteTier(tierId);

    if (result.tag === 'success') {
      tiers = tiers.filter((t) => t.id !== tierId);
      deletingTierId = null;
      toastStore.push({ message: 'Tier deleted', level: 'success' });
    } else {
      toastStore.push({ message: result.message, level: 'error' });
      deletingTierId = null;
    }
  }

  async function handleEvaluate() {
    if (merchantId === null || program === null) return;
    evaluating = true;
    evaluateResult = null;

    const result = await evaluateTier(merchantId);

    if (result.tag === 'success') {
      evaluateResult = `Evaluated ${result.data.evaluated} customers`;

      const [tiersResult, distResult] = await Promise.all([
        fetchTiers(program.id),
        fetchTierDistribution(merchantId)
      ]);

      if (tiersResult.tag === 'success') {
        tiers = tiersResult.data;
      }
      if (distResult.tag === 'success') {
        distribution = distResult.data;
      }

      distributionEl?.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
    } else {
      evaluateResult = result.message;
    }

    evaluating = false;
  }
</script>

<svelte:head>
  <title>Program & Tiers - Batua</title>
</svelte:head>

<div class="loyalty-page">
  {#if shouldShowWizard && merchantId !== null}
    <header class="page-header">
      <div class="page-header-left">
        <h1 class="page-title">Program & Tiers</h1>
        <p class="page-subtitle">
          {program === null ? 'Set up your loyalty program in a few easy steps' : 'Reconfigure your loyalty program'}
        </p>
      </div>
    </header>

    <TierWizard
      mode={program === null ? 'fresh' : wizardMode}
      existingProgram={program}
      existingTiers={tiers}
      {merchantId}
      onComplete={handleWizardComplete}
      onCancel={handleWizardCancel}
    />
  {:else if program !== null}
    <header class="page-header">
      <div class="page-header-left">
        <h1 class="page-title">Program & Tiers</h1>
        <p class="page-subtitle">Manage your loyalty program, tiers, and customer distribution</p>
      </div>
      <div class="page-actions">
        {#if evaluateResult !== null}
          <span class="evaluate-result">{evaluateResult}</span>
        {/if}
        <Button
          text="Reconfigure"
          classes="btn-ghost"
          onclick={openReconfigure}
        />
        <Button
          text={evaluating ? 'Evaluating...' : 'Evaluate Tiers'}
          classes="btn-secondary"
          disabled={evaluating}
          onclick={handleEvaluate}
        />
      </div>
    </header>

    <div class="loyalty-layout">
      <section class="program-summary">
        <div class="summary-grid">
          <div class="summary-item">
            <span class="summary-label">Program Name</span>
            <span class="summary-value">{program.name}</span>
          </div>
          <div class="summary-item">
            <span class="summary-label">Evaluation Criteria</span>
            <span class="summary-value">
              {program.evaluation_criteria === 'spend'
                ? 'Total Spend'
                : program.evaluation_criteria === 'order_count'
                  ? 'Order Count'
                  : program.evaluation_criteria === 'points'
                    ? 'Points Earned'
                    : program.evaluation_criteria}
            </span>
          </div>
          <div class="summary-item">
            <span class="summary-label">Evaluation Period</span>
            <span class="summary-value">
              {program.evaluation_period_days === null
                ? 'Lifetime'
                : `Last ${program.evaluation_period_days} days`}
            </span>
          </div>
        </div>
      </section>

      <section class="tiers-section">
        <div class="section-header">
          <h2 class="section-title">Tiers</h2>
          <Button
            text={showTierForm ? 'Cancel' : 'Add Tier'}
            classes={showTierForm ? 'btn-secondary' : 'btn-primary'}
            onclick={() => (showTierForm = !showTierForm)}
          />
        </div>

        {#if showTierForm}
          <TierForm tier={null} onSave={handleSaveTier} onCancel={() => (showTierForm = false)} />
        {/if}

        {#if sortedTiers.length > 0}
          <div class="tiers-list">
            {#each sortedTiers as t (t.id)}
              {#if editingTierId === t.id}
                <div class="tier-edit-row">
                  <TierForm
                    tier={t}
                    onSave={(formData) => handleUpdateTier(t.id, formData)}
                    onCancel={() => (editingTierId = null)}
                  />
                </div>
              {:else}
                <div class="tier-row">
                  <div class="tier-row-left">
                    <TierBadge
                      tierName={t.name}
                      rank={t.rank}
                      multiplier={t.earn_rate_multiplier}
                    />
                  </div>
                  <div class="tier-row-right">
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
                    <div class="tier-row-actions">
                      {#if deletingTierId === t.id}
                        <span class="delete-confirm">
                          <span class="delete-confirm-text">Delete?</span>
                          <Button
                            text="Yes"
                            classes="btn-danger btn-sm"
                            onclick={() => handleDeleteTier(t.id)}
                          />
                          <Button
                            text="No"
                            classes="btn-ghost btn-sm"
                            onclick={() => (deletingTierId = null)}
                          />
                        </span>
                      {:else}
                        <button
                          class="icon-btn"
                          title="Edit tier"
                          onclick={() => (editingTierId = t.id)}
                        >
                          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z"/><path d="m15 5 4 4"/></svg>
                        </button>
                        <button
                          class="icon-btn icon-btn-danger"
                          title="Delete tier"
                          onclick={() => (deletingTierId = t.id)}
                        >
                          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/><line x1="10" x2="10" y1="11" y2="17"/><line x1="14" x2="14" y1="11" y2="17"/></svg>
                        </button>
                      {/if}
                    </div>
                  </div>
                </div>
              {/if}
            {/each}
          </div>
        {:else}
          <p class="empty-tiers">No tiers configured yet. Add your first tier above.</p>
        {/if}
      </section>

      <section class="distribution-section" bind:this={distributionEl}>
        <TierDistributionChart {distribution} />
      </section>
    </div>
  {/if}
</div>

<style>
  .loyalty-page {
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
    max-width: 1000px;
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
    display: flex;
    align-items: center;
    gap: var(--space-3);
    flex-shrink: 0;
  }

  .evaluate-result {
    font-size: var(--font-size-sm);
    color: var(--color-success);
    font-weight: var(--font-weight-medium);
  }

  .loyalty-layout {
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
  }

  .program-summary {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: var(--space-5) var(--space-6);
  }

  .summary-grid {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    gap: var(--space-4);
  }

  .summary-item {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .summary-label {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .summary-value {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-medium);
    color: var(--color-text);
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

  .tier-row:last-child,
  .tier-edit-row:last-child {
    border-bottom: none;
  }

  .tier-edit-row {
    border-bottom: 1px solid var(--color-border);
  }

  .tier-edit-row :global(.tier-form) {
    border: none;
    border-radius: 0;
  }

  .tier-row-left {
    display: flex;
    align-items: center;
  }

  .tier-row-right {
    display: flex;
    align-items: center;
    gap: var(--space-6);
  }

  .tier-row-meta {
    display: flex;
    gap: var(--space-6);
  }

  .tier-row-actions {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .icon-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border: none;
    border-radius: var(--radius-md);
    background: transparent;
    color: var(--color-text-muted);
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .icon-btn:hover {
    background: var(--color-surface-hover);
    color: var(--color-text);
  }

  .icon-btn-danger:hover {
    background: var(--color-error-bg, hsl(0 70% 95%));
    color: var(--color-error);
  }

  .delete-confirm {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .delete-confirm-text {
    font-size: var(--font-size-sm);
    color: var(--color-error);
    font-weight: var(--font-weight-medium);
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

  @media (max-width: 600px) {
    .page-header {
      flex-direction: column;
    }

    .summary-grid {
      grid-template-columns: 1fr;
    }

    .tier-row {
      flex-direction: column;
      align-items: flex-start;
      gap: var(--space-3);
    }

    .tier-row-right {
      width: 100%;
      justify-content: space-between;
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
