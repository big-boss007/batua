<script lang="ts">
  import { Button, Input, Select } from '@juspay/svelte-ui-components';

  import type { LoyaltyProgram, LoyaltyTier } from '$lib/client/modules/customers';
  import {
    createProgram,
    updateProgram,
    createTier,
    updateTier,
    deleteTier,
    sortTiersByRank
  } from '$lib/client/modules/customers';
  import { toastStore } from '$lib/client/modules/foundation';

  type WizardTier = {
    id: string | null;
    name: string;
    rank: number;
    threshold: number;
    earn_rate_multiplier: number;
    benefits: Record<string, unknown>;
    _status: 'existing' | 'modified' | 'new' | 'removed';
    _original: LoyaltyTier | null;
  };

  let {
    mode,
    existingProgram,
    existingTiers,
    merchantId,
    onComplete,
    onCancel
  }: {
    mode: 'fresh' | 'reconfigure';
    existingProgram: LoyaltyProgram | null;
    existingTiers: Array<LoyaltyTier>;
    merchantId: string;
    onComplete: () => void;
    onCancel: () => void;
  } = $props();

  let step = $state(1);
  let saving = $state(false);

  let programName = $state(existingProgram?.name ?? '');
  let evaluationCriteria = $state(existingProgram?.evaluation_criteria ?? 'spend');
  let evaluationPeriodDays = $state<string>(
    existingProgram?.evaluation_period_days !== null &&
      existingProgram?.evaluation_period_days !== undefined
      ? String(existingProgram.evaluation_period_days)
      : ''
  );

  let wizardTiers = $state<Array<WizardTier>>(
    existingTiers.map((t) => ({
      id: t.id,
      name: t.name,
      rank: t.rank,
      threshold: t.threshold,
      earn_rate_multiplier: t.earn_rate_multiplier,
      benefits: t.benefits,
      _status: 'existing' as const,
      _original: { ...t }
    }))
  );

  let customName = $state('');
  let customRank = $state('');
  let customThreshold = $state('');
  let customMultiplier = $state('1');
  let editingIndex = $state<number | null>(null);

  const criteriaItems = [
    { id: 'spend', label: 'Total Spend' },
    { id: 'order_count', label: 'Order Count' },
    { id: 'points', label: 'Points Earned' }
  ];

  const periodItems = [
    { id: '', label: 'Lifetime (no reset)' },
    { id: '30', label: 'Last 30 days' },
    { id: '90', label: 'Last 90 days' },
    { id: '180', label: 'Last 180 days' },
    { id: '365', label: 'Last 365 days' }
  ];

  const presets: Array<{ name: string; icon: string; threshold: number; multiplier: number }> = [
    { name: 'Bronze', icon: '\u{1F944}', threshold: 0, multiplier: 1 },
    { name: 'Silver', icon: '\u{1FA99}', threshold: 500, multiplier: 1.5 },
    { name: 'Gold', icon: '\u{1F947}', threshold: 2000, multiplier: 2 },
    { name: 'Platinum', icon: '\u{1F48E}', threshold: 5000, multiplier: 3 }
  ];

  let activeTiers = $derived(wizardTiers.filter((t) => t._status !== 'removed'));
  let sortedActiveTiers = $derived(sortTiersByRank(activeTiers as Array<LoyaltyTier>) as unknown as Array<WizardTier>);
  let canProceedStep1 = $derived(programName.trim().length > 0);
  let canProceedStep2 = $derived(activeTiers.length >= 2);

  let nextRank = $derived(() => {
    const ranks = activeTiers.map((t) => t.rank);
    return ranks.length > 0 ? Math.max(...ranks) + 1 : 1;
  });

  function isPresetUsed(presetName: string): boolean {
    return activeTiers.some((t) => t.name.toLowerCase() === presetName.toLowerCase());
  }

  function addPreset(preset: { name: string; threshold: number; multiplier: number }) {
    if (isPresetUsed(preset.name)) return;
    wizardTiers = [
      ...wizardTiers,
      {
        id: null,
        name: preset.name,
        rank: nextRank(),
        threshold: preset.threshold,
        earn_rate_multiplier: preset.multiplier,
        benefits: {},
        _status: 'new',
        _original: null
      }
    ];
  }

  function parseNum(val: string, fallback: number): number {
    const n = Number(val);
    return Number.isFinite(n) ? n : fallback;
  }

  function addCustomTier() {
    const name = customName.trim();
    if (name.length === 0) return;
    wizardTiers = [
      ...wizardTiers,
      {
        id: null,
        name,
        rank: customRank.trim() ? parseNum(customRank, nextRank()) : nextRank(),
        threshold: parseNum(customThreshold, 0),
        earn_rate_multiplier: parseNum(customMultiplier, 1),
        benefits: {},
        _status: 'new',
        _original: null
      }
    ];
    customName = '';
    customRank = '';
    customThreshold = '';
    customMultiplier = '1';
  }

  function findWizardIndex(tier: WizardTier): number {
    return wizardTiers.findIndex(
      (wt) => (wt.id !== null && wt.id === tier.id) || wt === tier
    );
  }

  function removeTier(tier: WizardTier) {
    const index = findWizardIndex(tier);
    if (index === -1) return;
    if (wizardTiers[index]._status === 'new') {
      wizardTiers = wizardTiers.filter((_, i) => i !== index);
    } else {
      wizardTiers = wizardTiers.map((t, i) => (i === index ? { ...t, _status: 'removed' as const } : t));
    }
  }

  function startEditTier(tier: WizardTier) {
    const index = findWizardIndex(tier);
    if (index === -1) return;
    editingIndex = index;
    customName = tier.name;
    customRank = String(tier.rank);
    customThreshold = String(tier.threshold);
    customMultiplier = String(tier.earn_rate_multiplier);
  }

  function saveEditTier() {
    if (editingIndex === null) return;
    const t = wizardTiers[editingIndex];
    const updated: WizardTier = {
      ...t,
      name: customName.trim() || t.name,
      rank: parseNum(customRank, t.rank),
      threshold: parseNum(customThreshold, t.threshold),
      earn_rate_multiplier: parseNum(customMultiplier, t.earn_rate_multiplier),
      _status: t._status === 'existing' ? 'modified' : t._status
    };
    if (
      t._status === 'modified' &&
      t._original !== null &&
      updated.name === t._original.name &&
      updated.rank === t._original.rank &&
      updated.threshold === t._original.threshold &&
      updated.earn_rate_multiplier === t._original.earn_rate_multiplier
    ) {
      updated._status = 'existing';
    }
    wizardTiers = wizardTiers.map((wt, i) => (i === editingIndex ? updated : wt));
    editingIndex = null;
    customName = '';
    customRank = '';
    customThreshold = '';
    customMultiplier = '1';
  }

  function cancelEdit() {
    editingIndex = null;
    customName = '';
    customRank = '';
    customThreshold = '';
    customMultiplier = '1';
  }

  function getCriteriaLabel(val: string): string {
    return criteriaItems.find((c) => c.id === val)?.label ?? val;
  }

  function getPeriodLabel(val: string): string {
    return periodItems.find((p) => p.id === val)?.label ?? 'Lifetime';
  }

  function getChanges(): Array<{ type: 'add' | 'edit' | 'remove'; description: string }> {
    const changes: Array<{ type: 'add' | 'edit' | 'remove'; description: string }> = [];
    for (const t of wizardTiers) {
      if (t._status === 'new') {
        changes.push({
          type: 'add',
          description: `${t.name} tier added (rank ${t.rank}, threshold ${t.threshold.toLocaleString('en-IN')}, ${t.earn_rate_multiplier}x)`
        });
      } else if (t._status === 'modified' && t._original !== null) {
        const diffs: Array<string> = [];
        if (t.name !== t._original.name) diffs.push(`name ${t._original.name} → ${t.name}`);
        if (t.threshold !== t._original.threshold)
          diffs.push(`threshold ${t._original.threshold} → ${t.threshold}`);
        if (t.rank !== t._original.rank) diffs.push(`rank ${t._original.rank} → ${t.rank}`);
        if (t.earn_rate_multiplier !== t._original.earn_rate_multiplier)
          diffs.push(`multiplier ${t._original.earn_rate_multiplier}x → ${t.earn_rate_multiplier}x`);
        if (diffs.length > 0) {
          changes.push({ type: 'edit', description: `${t._original.name}: ${diffs.join(', ')}` });
        }
      } else if (t._status === 'removed') {
        changes.push({ type: 'remove', description: `${t.name} tier removed` });
      }
    }
    return changes;
  }

  async function handleSave() {
    saving = true;
    try {
      if (mode === 'fresh') {
        const progResult = await createProgram(merchantId, {
          name: programName.trim(),
          evaluation_criteria: evaluationCriteria
        });
        if (progResult.tag !== 'success') {
          toastStore.push({ message: progResult.message, level: 'error' });
          saving = false;
          return;
        }
        const programId = progResult.data.id;

        for (const t of sortedActiveTiers) {
          const tierResult = await createTier(programId, {
            name: t.name,
            rank: t.rank,
            threshold: t.threshold,
            earn_rate_multiplier: t.earn_rate_multiplier,
            benefits: t.benefits
          });
          if (tierResult.tag !== 'success') {
            toastStore.push({ message: tierResult.message, level: 'error' });
            saving = false;
            return;
          }
        }

        toastStore.push({ message: 'Loyalty tiers activated!', level: 'success' });
      } else {
        if (existingProgram !== null) {
          const progResult = await updateProgram(existingProgram.id, {
            name: programName.trim(),
            evaluation_criteria: evaluationCriteria,
            evaluation_period_days: evaluationPeriodDays ? Number(evaluationPeriodDays) : null,
            is_active: true
          });
          if (progResult.tag !== 'success') {
            toastStore.push({ message: progResult.message, level: 'error' });
            saving = false;
            return;
          }
        }

        for (const t of wizardTiers) {
          if (t._status === 'new') {
            const tierResult = await createTier(existingProgram?.id ?? '', {
              name: t.name,
              rank: t.rank,
              threshold: t.threshold,
              earn_rate_multiplier: t.earn_rate_multiplier,
              benefits: t.benefits
            });
            if (tierResult.tag !== 'success') {
              toastStore.push({ message: tierResult.message, level: 'error' });
            }
          } else if (t._status === 'modified' && t.id !== null) {
            const tierResult = await updateTier(t.id, {
              name: t.name,
              rank: t.rank,
              threshold: t.threshold,
              earn_rate_multiplier: t.earn_rate_multiplier
            });
            if (tierResult.tag !== 'success') {
              toastStore.push({ message: tierResult.message, level: 'error' });
            }
          } else if (t._status === 'removed' && t.id !== null) {
            const delResult = await deleteTier(t.id);
            if (delResult.tag !== 'success') {
              toastStore.push({ message: delResult.message, level: 'error' });
            }
          }
        }

        toastStore.push({ message: 'Loyalty tiers updated!', level: 'success' });
      }

      onComplete();
    } catch {
      toastStore.push({ message: 'Something went wrong', level: 'error' });
    }
    saving = false;
  }

  let changes = $derived(getChanges());
</script>

<div class="wizard">
  {#if mode === 'reconfigure'}
    <div class="info-banner amber">
      <strong>Reconfigure mode:</strong> Your current settings are pre-filled. No changes are saved until
      you confirm on the final step.
    </div>
  {/if}

  <!-- STEP 1: Program -->
  {#if step === 1}
    <div class="card">
      <div class="card-header">
        <div class="card-header-left">
          <h2 class="card-title">{mode === 'fresh' ? 'Set Up Your Tiers' : 'Update Tier Settings'}</h2>
          <p class="card-sub-inline">Configure how customers qualify for tiers</p>
        </div>
        <div class="step-pills">
          <span class="step-pill active">1 Settings</span>
          <span class="step-pill">2 Tiers</span>
          <span class="step-pill">3 Review</span>
        </div>
      </div>

      <div class="card-body">
        <div class="form-group">
          <Input
            value={programName}
            label="Tier Program Name"
            placeholder="e.g. Rewards Club"
            onInput={(val) => {
              programName = val;
            }}
          />
        </div>

        <div class="form-grid-2col">
          <div class="form-group">
            <span class="field-label">Evaluation Criteria</span>
            <Select
              items={criteriaItems}
              value={[evaluationCriteria]}
              onchange={(val) => {
                evaluationCriteria = val[0] ?? 'spend';
              }}
            />
            <span class="form-hint">How customers progress through tiers</span>
          </div>

          <div class="form-group">
            <span class="field-label">Evaluation Period</span>
            <Select
              items={periodItems}
              value={[evaluationPeriodDays]}
              onchange={(val) => {
                evaluationPeriodDays = val[0] ?? '';
              }}
            />
            <span class="form-hint">How far back to evaluate progress</span>
          </div>
        </div>
      </div>

      <div class="card-footer">
        {#if mode === 'reconfigure'}
          <Button text="← Cancel" classes="btn-ghost" onclick={onCancel} />
        {:else}
          <span></span>
        {/if}
        <Button
          text="Next: {mode === 'fresh' ? 'Add' : 'Edit'} Tiers →"
          classes="btn-primary"
          disabled={!canProceedStep1}
          onclick={() => (step = 2)}
        />
      </div>
    </div>
  {/if}

  <!-- STEP 2: Tiers -->
  {#if step === 2}
    <div class="card">
      <div class="card-header">
        <div class="card-header-left">
          <h2 class="card-title">{mode === 'fresh' ? 'Add Tiers' : 'Edit Tiers'}</h2>
          <p class="card-sub-inline">
            {mode === 'fresh'
              ? 'Quick-add from presets or create custom tiers. Need at least 2.'
              : 'Edit, remove, or add new tiers.'}
          </p>
        </div>
        <div class="step-pills">
          <span class="step-pill done">1 Settings</span>
          <span class="step-pill active">2 Tiers</span>
          <span class="step-pill">3 Review</span>
        </div>
      </div>
      <div class="card-body">

      <span class="field-label" style="margin-bottom: 8px; display: block;">Quick-add a preset</span>
      <div class="presets-grid">
        {#each presets as preset}
          {@const used = isPresetUsed(preset.name)}
          <button
            class="preset-card"
            class:added={used}
            disabled={used}
            onclick={() => addPreset(preset)}
          >
            <span class="preset-icon">{preset.icon}</span>
            <span class="preset-name">{preset.name}</span>
            <span class="preset-detail">
              {#if used}
                <s>Threshold: {preset.threshold.toLocaleString('en-IN')} &middot; {preset.multiplier}x</s>
              {:else}
                Threshold: {preset.threshold.toLocaleString('en-IN')} &middot; {preset.multiplier}x
              {/if}
            </span>
            {#if used}
              <span class="preset-added-label">{mode === 'reconfigure' ? 'Exists' : 'Added'}</span>
            {/if}
          </button>
        {/each}
      </div>

      <div class="or-divider"><span>or add custom</span></div>

      {#if editingIndex !== null}
        <div class="inline-form">
          <div class="if-name">
            <span class="field-label">Tier Name</span>
            <Input value={customName} placeholder="e.g. Diamond" onInput={(val) => { customName = val; }} />
          </div>
          <div class="if-group">
            <div>
              <span class="field-label">Rank</span>
              <Input value={customRank} dataType="number" onInput={(val) => { customRank = val; }} />
            </div>
            <div>
              <span class="field-label">Threshold</span>
              <Input value={customThreshold} dataType="number" onInput={(val) => { customThreshold = val; }} />
            </div>
          </div>
          <div class="if-mult">
            <span class="field-label">Multiplier</span>
            <Input value={customMultiplier} dataType="number" onInput={(val) => { customMultiplier = val; }} />
          </div>
          <div class="if-btn">
            <Button text="Save" classes="btn-primary btn-sm" onclick={saveEditTier} />
            <Button text="Cancel" classes="btn-ghost btn-sm" onclick={cancelEdit} />
          </div>
        </div>
      {:else}
        <div class="inline-form">
          <div class="if-name">
            <span class="field-label">Tier Name</span>
            <Input value={customName} placeholder="e.g. Diamond" onInput={(val) => { customName = val; }} />
          </div>
          <div class="if-group">
            <div>
              <span class="field-label">Rank</span>
              <Input value={customRank} dataType="number" placeholder="{String(nextRank())}" onInput={(val) => { customRank = val; }} />
            </div>
            <div>
              <span class="field-label">Threshold</span>
              <Input value={customThreshold} dataType="number" placeholder="0" onInput={(val) => { customThreshold = val; }} />
            </div>
          </div>
          <div class="if-mult">
            <span class="field-label">Multiplier</span>
            <Input value={customMultiplier} dataType="number" onInput={(val) => { customMultiplier = val; }} />
          </div>
          <div class="if-btn">
            <Button text="+ Add" classes="btn-primary btn-sm" onclick={addCustomTier} disabled={customName.trim().length === 0} />
          </div>
        </div>
      {/if}

      {#if activeTiers.length > 0}
        <span class="field-label tier-list-label">Your tiers ({activeTiers.length} {activeTiers.length === 1 ? 'tier' : 'tiers'})</span>
        <div class="wt-list">
          {#each sortedActiveTiers as t (t.id ?? t.name + t.rank)}
            <div
              class="wt-item"
              class:wt-modified={t._status === 'modified'}
              class:wt-new={t._status === 'new'}
            >
              <div class="wt-left">
                <span class="wt-rank" class:wt-rank-new={t._status === 'new'}>{t.rank}</span>
                <span class="wt-name">{t.name}</span>
                {#if t._status === 'modified'}
                  <span class="change-badge modified">Modified</span>
                {:else if t._status === 'new'}
                  <span class="change-badge new">New</span>
                {/if}
              </div>
              <div class="wt-right">
                <div class="wt-stat">
                  <span class="wt-stat-label">Threshold</span>
                  <span class="wt-stat-val">
                    {#if t._status === 'modified' && t._original !== null && t.threshold !== t._original.threshold}
                      <span class="old-val">{t._original.threshold.toLocaleString('en-IN')}</span>
                      {t.threshold.toLocaleString('en-IN')}
                    {:else}
                      {t.threshold.toLocaleString('en-IN')}
                    {/if}
                  </span>
                </div>
                <div class="wt-stat">
                  <span class="wt-stat-label">Multiplier</span>
                  <span class="wt-stat-val">
                    {#if t._status === 'modified' && t._original !== null && t.earn_rate_multiplier !== t._original.earn_rate_multiplier}
                      <span class="old-val">{t._original.earn_rate_multiplier}x</span>
                      {t.earn_rate_multiplier}x
                    {:else}
                      {t.earn_rate_multiplier}x
                    {/if}
                  </span>
                </div>
                <div class="wt-actions">
                  <button class="icon-btn" title="Edit" onclick={() => startEditTier(t)}>
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z"/><path d="m15 5 4 4"/></svg>
                  </button>
                  <button class="icon-btn icon-btn-danger" title="Remove" onclick={() => removeTier(t)}>
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/></svg>
                  </button>
                </div>
              </div>
            </div>
          {/each}
        </div>
      {/if}

      </div>
      <div class="card-footer">
        <Button text="← Back" classes="btn-ghost" onclick={() => (step = 1)} />
        <Button
          text="Next: Review →"
          classes="btn-primary"
          disabled={!canProceedStep2}
          onclick={() => (step = 3)}
        />
      </div>
    </div>
  {/if}

  <!-- STEP 3: Review -->
  {#if step === 3}
    <div class="card">
      <div class="card-header">
        <div class="card-header-left">
          <h2 class="card-title">{mode === 'fresh' ? 'Review & Activate' : 'Review Changes'}</h2>
          <p class="card-sub-inline">
            {mode === 'fresh'
              ? 'Review your tier configuration before activating'
              : "Here's what will change when you save"}
          </p>
        </div>
        <div class="step-pills">
          <span class="step-pill done">1 Settings</span>
          <span class="step-pill done">2 Tiers</span>
          <span class="step-pill active">3 Review</span>
        </div>
      </div>
      <div class="card-body">

      {#if mode === 'reconfigure' && changes.length > 0}
        <div class="review-section">
          <h3 class="review-heading">Changes Summary</h3>
          <div class="diff-box">
            {#each changes as change}
              <div class="diff-row">
                <span class="diff-icon" class:diff-add={change.type === 'add'} class:diff-edit={change.type === 'edit'} class:diff-del={change.type === 'remove'}>
                  {#if change.type === 'add'}+{:else if change.type === 'edit'}&cir;{:else}&times;{/if}
                </span>
                <span>{change.description}</span>
              </div>
            {/each}
          </div>
        </div>
      {/if}

      <div class="review-section">
        <h3 class="review-heading">Settings</h3>
        <div class="review-grid">
          <div>
            <span class="review-label">Tier Program Name</span>
            <span class="review-value">{programName}</span>
          </div>
          <div>
            <span class="review-label">Evaluation Criteria</span>
            <span class="review-value">{getCriteriaLabel(evaluationCriteria)}</span>
          </div>
          <div>
            <span class="review-label">Evaluation Period</span>
            <span class="review-value">{getPeriodLabel(evaluationPeriodDays)}</span>
          </div>
          <div>
            <span class="review-label">Status</span>
            <span class="review-value review-active">Active</span>
          </div>
        </div>
      </div>

      <div class="review-section">
        <h3 class="review-heading">{mode === 'fresh' ? 'Tiers' : 'Final Tier Structure'} ({activeTiers.length})</h3>
        <div class="review-tier-stack">
          {#each sortedActiveTiers as t}
            <div
              class="review-tier"
              class:review-tier-modified={t._status === 'modified'}
              class:review-tier-new={t._status === 'new'}
            >
              <span class="review-badge">{t.name}</span>
              <div class="review-tier-info">
                <span class="review-tier-detail">
                  Threshold: <strong>
                    {#if t._status === 'modified' && t._original !== null && t.threshold !== t._original.threshold}
                      <span class="old-val">{t._original.threshold.toLocaleString('en-IN')}</span>
                    {/if}
                    {t.threshold.toLocaleString('en-IN')}
                  </strong>
                </span>
                <span class="review-tier-detail">
                  Multiplier: <strong>{t.earn_rate_multiplier}x</strong>
                </span>
              </div>
              {#if t._status === 'modified'}
                <span class="change-badge modified">Modified</span>
              {:else if t._status === 'new'}
                <span class="change-badge new">New</span>
              {/if}
            </div>
          {/each}
        </div>
      </div>

      {#if mode === 'fresh'}
        <div class="info-banner green">
          <strong>What happens next:</strong> Your tiers will be created and activated.
          Customers will be evaluated against these tiers when you run "Evaluate Tiers".
        </div>
      {:else}
        <div class="info-banner amber">
          <strong>Note:</strong> Existing customers will keep their current tier until the next evaluation.
          Run "Evaluate Tiers" after saving to reassign customers based on the new thresholds.
        </div>
      {/if}

      </div>
      <div class="card-footer">
        <Button text="← Back to Tiers" classes="btn-ghost" onclick={() => (step = 2)} />
        <Button
          text={saving ? 'Saving...' : mode === 'fresh' ? 'Activate Tiers' : 'Save Changes'}
          classes={mode === 'fresh' ? 'btn-success' : 'btn-primary'}
          disabled={saving}
          onclick={handleSave}
        />
      </div>
    </div>
  {/if}
</div>

<style>
  .wizard {
    display: flex;
    flex-direction: column;
    gap: var(--space-5);
    max-width: 720px;
    width: 100%;
  }

  .card {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    overflow: hidden;
  }

  .card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-5) var(--space-6);
    border-bottom: 1px solid var(--color-border);
  }

  .card-header-left {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .card-title {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
  }

  .card-sub-inline {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .step-pills {
    display: flex;
    gap: var(--space-2);
  }

  .step-pill {
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-medium);
    padding: var(--space-1) var(--space-3);
    border-radius: var(--radius-full, 9999px);
    color: var(--color-text-muted);
    background: var(--color-surface-2);
  }

  .step-pill.active {
    color: var(--color-primary);
    font-weight: var(--font-weight-semibold);
  }

  .step-pill.done {
    color: var(--color-success);
  }

  .card-body {
    padding: var(--space-6);
  }

  .card-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--space-4) var(--space-6);
    border-top: 1px solid var(--color-border);
  }

  .form-grid-2col {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-4);
  }

  .form-group {
    margin-bottom: var(--space-5);
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .field-label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-text);
  }

  .form-hint {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .btn-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  /* Info banners */
  .info-banner {
    padding: var(--space-3) var(--space-4);
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
    line-height: 1.5;
  }

  .info-banner.green {
    background: #f0fdf4;
    border: 1px solid #bbf7d0;
    color: #166534;
  }

  .info-banner.amber {
    background: #fffbeb;
    border: 1px solid #fde68a;
    color: #92400e;
  }

  /* Presets */
  .presets-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: var(--space-3);
    margin-bottom: var(--space-5);
  }

  .preset-card {
    border: 2px dashed var(--color-border);
    border-radius: var(--radius-lg);
    padding: var(--space-4);
    cursor: pointer;
    text-align: center;
    transition: all 0.15s;
    background: transparent;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-1);
  }

  .preset-card:hover:not(.added) {
    border-color: var(--color-primary, #6366f1);
    background: var(--color-primary-bg, #f5f3ff);
  }

  .preset-card.added {
    opacity: 0.5;
    border-style: solid;
    cursor: default;
  }

  .preset-icon {
    font-size: 24px;
  }

  .preset-name {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .preset-detail {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .preset-added-label {
    font-size: 10px;
    color: var(--color-success);
    font-weight: var(--font-weight-medium);
  }

  /* Or divider */
  .or-divider {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    margin: var(--space-4) 0;
  }

  .or-divider::before,
  .or-divider::after {
    content: '';
    flex: 1;
    height: 1px;
    background: var(--color-border);
  }

  .or-divider span {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    font-weight: var(--font-weight-medium);
  }

  /* Inline form */
  .inline-form {
    display: flex;
    gap: var(--space-3);
    align-items: flex-end;
    padding: var(--space-4);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
  }

  .inline-form > * {
    min-width: 0;
  }

  .if-name {
    flex: 3;
  }

  .if-group {
    flex: 2;
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-3);
    min-width: 0;
  }

  .if-group > * {
    min-width: 0;
  }

  .if-mult {
    flex: 1;
    min-width: 0;
    overflow: hidden;
  }

  .if-group :global(input),
  .if-mult :global(input),
  .if-name :global(input) {
    min-width: 0 !important;
    width: 100% !important;
  }

  .if-btn {
    display: flex;
    gap: var(--space-2);
    align-items: flex-end;
    padding-bottom: 1px;
    flex-shrink: 0;
  }

  .inline-form-action {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .inline-form-row {
    display: flex;
    gap: var(--space-2);
    align-items: center;
  }

  /* Tier list */
  .tier-list-label {
    display: block;
    margin: var(--space-5) 0 var(--space-3);
  }

  .wt-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .wt-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-3) var(--space-4);
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
  }

  .wt-item.wt-modified {
    border-color: #fbbf24;
    background: #fffbeb;
  }

  .wt-item.wt-new {
    border-color: #86efac;
    background: #f0fdf4;
  }

  .wt-left {
    display: flex;
    align-items: center;
    gap: var(--space-3);
  }

  .wt-rank {
    width: 26px;
    height: 26px;
    border-radius: 50%;
    background: var(--color-primary, #6366f1);
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-semibold);
  }

  .wt-rank.wt-rank-new {
    background: var(--color-success, #22c55e);
  }

  .wt-name {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .wt-right {
    display: flex;
    align-items: center;
    gap: var(--space-4);
  }

  .wt-stat {
    text-align: right;
  }

  .wt-stat-label {
    font-size: 10px;
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .wt-stat-val {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    font-family: var(--font-mono);
    color: var(--color-text);
  }

  .wt-actions {
    display: flex;
    gap: var(--space-1);
  }

  .icon-btn {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    border-radius: var(--radius-sm);
    background: transparent;
    color: var(--color-text-muted);
    cursor: pointer;
    transition: all 0.15s;
  }

  .icon-btn:hover {
    background: var(--color-surface-hover);
    color: var(--color-text);
  }

  .icon-btn-danger:hover {
    background: var(--color-error-bg, hsl(0 70% 95%));
    color: var(--color-error);
  }

  .change-badge {
    font-size: 10px;
    font-weight: var(--font-weight-semibold);
    text-transform: uppercase;
    padding: 2px 6px;
    border-radius: 4px;
    letter-spacing: 0.04em;
  }

  .change-badge.modified {
    background: #fef3c7;
    color: #92400e;
  }

  .change-badge.new {
    background: #d1fae5;
    color: #065f46;
  }

  .old-val {
    text-decoration: line-through;
    color: var(--color-text-muted);
    margin-right: var(--space-1);
  }

  /* Review */
  .review-section {
    margin-bottom: var(--space-5);
  }

  .review-heading {
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-bottom: var(--space-3);
  }

  .review-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-3);
    padding: var(--space-4);
    background: var(--color-bg);
    border-radius: var(--radius-md);
  }

  .review-label {
    display: block;
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .review-value {
    display: block;
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-medium);
    margin-top: 1px;
    color: var(--color-text);
  }

  .review-active {
    color: var(--color-success);
  }

  .review-tier-stack {
    display: flex;
    flex-direction: column;
  }

  .review-tier {
    display: flex;
    align-items: center;
    gap: var(--space-4);
    padding: var(--space-3) var(--space-4);
    background: var(--color-bg);
    border: 1px solid var(--color-border);
  }

  .review-tier:first-child {
    border-radius: var(--radius-md) var(--radius-md) 0 0;
  }

  .review-tier:last-child {
    border-radius: 0 0 var(--radius-md) var(--radius-md);
  }

  .review-tier:only-child {
    border-radius: var(--radius-md);
  }

  .review-tier + .review-tier {
    border-top: none;
  }

  .review-tier.review-tier-modified {
    background: #fffbeb;
  }

  .review-tier.review-tier-new {
    background: #f0fdf4;
  }

  .review-badge {
    padding: 3px 10px;
    border-radius: 16px;
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-semibold);
    background: var(--color-surface-hover);
    color: var(--color-text);
    white-space: nowrap;
  }

  .review-tier-info {
    flex: 1;
    display: flex;
    justify-content: space-between;
  }

  .review-tier-detail {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .review-tier-detail strong {
    color: var(--color-text);
  }

  /* Diff box */
  .diff-box {
    background: var(--color-bg);
    border-radius: var(--radius-md);
    padding: var(--space-3) var(--space-4);
  }

  .diff-row {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-2) 0;
    font-size: var(--font-size-sm);
    border-bottom: 1px solid var(--color-border);
  }

  .diff-row:last-child {
    border-bottom: none;
  }

  .diff-icon {
    width: 20px;
    text-align: center;
    font-weight: var(--font-weight-bold);
    font-size: var(--font-size-base);
  }

  .diff-add {
    color: var(--color-success);
  }

  .diff-edit {
    color: #f59e0b;
  }

  .diff-del {
    color: var(--color-error);
  }

  @media (max-width: 600px) {
    .presets-grid {
      grid-template-columns: 1fr;
    }

    .inline-form {
      grid-template-columns: 1fr;
    }

    .wt-item {
      flex-direction: column;
      align-items: flex-start;
      gap: var(--space-3);
    }

    .wt-right {
      width: 100%;
      justify-content: space-between;
    }

    .review-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
