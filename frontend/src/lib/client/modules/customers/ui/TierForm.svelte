<script lang="ts">
  import type { LoyaltyTier } from '$lib/client/modules/customers';

  let {
    tier,
    onSave
  }: {
    tier: LoyaltyTier | null;
    onSave: (data: {
      name: string;
      rank: number;
      threshold: number;
      earn_rate_multiplier: number;
      benefits: Record<string, unknown>;
    }) => void;
  } = $props();

  let name = $state(tier?.name ?? '');
  let rank = $state(tier?.rank ?? 1);
  let threshold = $state(tier?.threshold ?? 0);
  let earnRateMultiplier = $state(tier?.earn_rate_multiplier ?? 1);
  let benefitsJson = $state(tier ? JSON.stringify(tier.benefits, null, 2) : '{}');
  let submitting = $state(false);
  let benefitsError = $state<string | null>(null);

  let isValid = $derived(name.trim().length > 0 && rank > 0 && threshold >= 0 && earnRateMultiplier > 0 && benefitsError === null);
  let isEditing = $derived(tier !== null);

  function validateBenefits(json: string): Record<string, unknown> | null {
    try {
      const parsed = JSON.parse(json);
      if (typeof parsed === 'object' && parsed !== null && !Array.isArray(parsed)) {
        return parsed as Record<string, unknown>;
      }
      return null;
    } catch {
      return null;
    }
  }

  function handleBenefitsInput(event: Event) {
    const target = event.target as HTMLTextAreaElement;
    benefitsJson = target.value;
    const parsed = validateBenefits(benefitsJson);
    benefitsError = parsed === null ? 'Invalid JSON object' : null;
  }

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    if (!isValid || submitting) return;

    const benefits = validateBenefits(benefitsJson);
    if (benefits === null) return;

    submitting = true;
    onSave({
      name: name.trim(),
      rank,
      threshold,
      earn_rate_multiplier: earnRateMultiplier,
      benefits
    });
    submitting = false;
  }
</script>

<form class="tier-form" onsubmit={handleSubmit}>
  <h3 class="form-title">{isEditing ? 'Edit' : 'Create'} Tier</h3>

  <div class="form-row">
    <div class="form-field">
      <label class="form-label" for="tier-name">Tier Name</label>
      <input
        id="tier-name"
        type="text"
        class="form-input"
        placeholder="e.g. Gold"
        bind:value={name}
      />
    </div>

    <div class="form-field form-field-small">
      <label class="form-label" for="tier-rank">Rank</label>
      <input
        id="tier-rank"
        type="number"
        class="form-input"
        min="1"
        bind:value={rank}
      />
    </div>
  </div>

  <div class="form-row">
    <div class="form-field">
      <label class="form-label" for="tier-threshold">Threshold</label>
      <input
        id="tier-threshold"
        type="number"
        class="form-input"
        min="0"
        bind:value={threshold}
      />
      <span class="form-hint">Minimum value to reach this tier</span>
    </div>

    <div class="form-field">
      <label class="form-label" for="tier-multiplier">Earn Rate Multiplier</label>
      <input
        id="tier-multiplier"
        type="number"
        class="form-input"
        min="0.1"
        step="0.1"
        bind:value={earnRateMultiplier}
      />
      <span class="form-hint">Points multiplier for this tier</span>
    </div>
  </div>

  <div class="form-field">
    <label class="form-label" for="tier-benefits">Benefits (JSON)</label>
    <textarea
      id="tier-benefits"
      class="form-textarea"
      rows="4"
      value={benefitsJson}
      oninput={handleBenefitsInput}
    ></textarea>
    {#if benefitsError}
      <span class="form-error">{benefitsError}</span>
    {/if}
  </div>

  <div class="form-actions">
    <button type="submit" class="btn-primary" disabled={!isValid || submitting}>
      {isEditing ? 'Update Tier' : 'Create Tier'}
    </button>
  </div>
</form>

<style>
  .tier-form {
    display: flex;
    flex-direction: column;
    gap: var(--space-5);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: var(--space-6);
  }

  .form-title {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-4);
  }

  .form-field {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .form-field-small {
    max-width: 120px;
  }

  .form-label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-text);
  }

  .form-input,
  .form-textarea {
    padding: var(--space-3);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background: var(--color-bg);
    color: var(--color-text);
    font-size: var(--font-size-base);
    outline: none;
    transition: border-color var(--transition-fast);
  }

  .form-textarea {
    resize: vertical;
    font-family: var(--font-mono);
    font-size: var(--font-size-sm);
  }

  .form-input:focus,
  .form-textarea:focus {
    border-color: var(--color-primary);
  }

  .form-input::placeholder {
    color: var(--color-text-muted);
  }

  .form-hint {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .form-error {
    font-size: var(--font-size-xs);
    color: var(--color-error);
  }

  .form-actions {
    display: flex;
    justify-content: flex-end;
    padding-top: var(--space-2);
  }

  .btn-primary {
    padding: var(--space-2) var(--space-5);
    border: none;
    border-radius: var(--radius-md);
    background: var(--color-primary);
    color: #ffffff;
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    transition: background var(--transition-fast);
  }

  .btn-primary:hover:not(:disabled) {
    background: var(--color-primary-hover);
  }

  .btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  @media (max-width: 600px) {
    .form-row {
      grid-template-columns: 1fr;
    }

    .form-field-small {
      max-width: none;
    }
  }
</style>
