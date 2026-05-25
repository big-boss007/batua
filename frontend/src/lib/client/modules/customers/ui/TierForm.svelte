<script lang="ts">
  import { Input, Button } from '@juspay/svelte-ui-components';

  import type { LoyaltyTier } from '$lib/client/modules/customers';

  let {
    tier,
    onSave,
    onCancel
  }: {
    tier: LoyaltyTier | null;
    onSave: (data: {
      name: string;
      rank: number;
      threshold: number;
      earn_rate_multiplier: number;
      benefits: Record<string, unknown>;
    }) => void;
    onCancel?: () => void;
  } = $props();

  // svelte-ignore state_referenced_locally
  let name = $state(tier?.name ?? '');
  // svelte-ignore state_referenced_locally
  let rank = $state(String(tier?.rank ?? 1));
  // svelte-ignore state_referenced_locally
  let threshold = $state(String(tier?.threshold ?? 0));
  // svelte-ignore state_referenced_locally
  let earnRateMultiplier = $state(String(tier?.earn_rate_multiplier ?? 1));
  // svelte-ignore state_referenced_locally
  let benefitsJson = $state(tier ? JSON.stringify(tier.benefits, null, 2) : '{}');
  let submitting = $state(false);
  let benefitsError = $state<string | null>(null);

  let isValid = $derived(
    name.trim().length > 0 &&
      Number(rank) > 0 &&
      Number(threshold) >= 0 &&
      Number(earnRateMultiplier) > 0 &&
      benefitsError === null
  );
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

  function handleBenefitsInput(val: string) {
    benefitsJson = val;
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
      rank: Number(rank),
      threshold: Number(threshold),
      earn_rate_multiplier: Number(earnRateMultiplier),
      benefits
    });
    submitting = false;
  }
</script>

<form class="tier-form" onsubmit={handleSubmit}>
  <h3 class="form-title">{isEditing ? 'Edit' : 'Create'} Tier</h3>

  <div class="form-row">
    <Input
      value={name}
      label="Tier Name"
      placeholder="e.g. Gold"
      onInput={(val) => {
        name = val;
      }}
    />

    <div class="form-field-small">
      <Input
        value={rank}
        label="Rank"
        dataType="number"
        onInput={(val) => {
          rank = val;
        }}
      />
    </div>
  </div>

  <div class="form-row">
    <div class="form-field">
      <Input
        value={threshold}
        label="Threshold"
        dataType="number"
        onInput={(val) => {
          threshold = val;
        }}
      />
      <span class="form-hint">Minimum value to reach this tier</span>
    </div>

    <div class="form-field">
      <Input
        value={earnRateMultiplier}
        label="Earn Rate Multiplier"
        dataType="number"
        onInput={(val) => {
          earnRateMultiplier = val;
        }}
      />
      <span class="form-hint">Points multiplier for this tier</span>
    </div>
  </div>

  <div class="form-field">
    <Input
      value={benefitsJson}
      label="Benefits (JSON)"
      useTextArea
      onInput={(val) => {
        handleBenefitsInput(val);
      }}
      onErrorMessage={benefitsError}
      classes="input-mono"
    />
  </div>

  <div class="form-actions">
    {#if onCancel}
      <Button text="Cancel" classes="btn-ghost" onclick={onCancel} />
    {/if}
    <Button
      text={isEditing ? 'Update Tier' : 'Create Tier'}
      classes="btn-primary"
      disabled={!isValid || submitting}
      type="submit"
    />
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

  .form-hint {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: var(--space-3);
    padding-top: var(--space-2);
  }

  :global(.input-mono) {
    --input-font-family: var(--font-mono);
    --input-font-size: var(--font-size-sm);
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
