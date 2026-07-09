<script lang="ts">
  import { Input, Select, Button } from '@juspay/svelte-ui-components';

  import type { LoyaltyProgram } from '$lib/client/modules/customers';

  let {
    program,
    onSave
  }: {
    program: LoyaltyProgram | null;
    onSave: (data: { name: string; evaluation_criteria: string }) => void;
  } = $props();

  let nameOverride = $state<string | null>(null);
  let criteriaOverride = $state<string | null>(null);
  let submitting = $state(false);

  let name = $derived(nameOverride ?? program?.name ?? '');
  let evaluationCriteria = $derived(criteriaOverride ?? program?.evaluation_criteria ?? 'spend');
  let isValid = $derived(name.trim().length > 0);
  let isEditing = $derived(program !== null);

  const criteriaItems = [
    { id: 'spend', label: 'Total spend' },
    { id: 'orders', label: 'Order count' },
    { id: 'points', label: 'Points earned' }
  ];

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    if (!isValid || submitting) return;

    submitting = true;
    onSave({
      name: name.trim(),
      evaluation_criteria: evaluationCriteria
    });
    submitting = false;
  }
</script>

<form class="program-form" onsubmit={handleSubmit}>
  <h3 class="form-title">{isEditing ? 'Edit' : 'Create'} Loyalty Program</h3>

  <Input
    value={name}
    label="Tier program name"
    placeholder="e.g. Rewards Club"
    onInput={(val) => {
      nameOverride = val;
    }}
  />

  <div class="form-field">
    <span class="field-label">Evaluation criteria</span>
    <Select
      items={criteriaItems}
      value={[evaluationCriteria]}
      onchange={(val) => {
        criteriaOverride = val[0] ?? 'spend';
      }}
    />
    <span class="form-hint">Determines how customers progress through tiers</span>
  </div>

  <div class="form-actions">
    <Button
      text={isEditing ? 'Update program' : 'Create program'}
      classes="btn-primary"
      disabled={!isValid || submitting}
      type="submit"
    />
  </div>
</form>

<style>
  .program-form {
    display: flex;
    flex-direction: column;
    gap: var(--space-5);
    background: var(--color-surface);
    border-radius: var(--radius-lg);
    padding: var(--space-6);
    box-shadow: var(--shadow-card);
  }

  .form-title {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .form-field {
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

  .form-actions {
    display: flex;
    justify-content: flex-end;
    padding-top: var(--space-2);
  }
</style>
