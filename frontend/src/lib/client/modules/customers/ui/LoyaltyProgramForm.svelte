<script lang="ts">
  import type { LoyaltyProgram } from '$lib/client/modules/customers';

  let {
    program,
    onSave
  }: {
    program: LoyaltyProgram | null;
    onSave: (data: { name: string; evaluation_criteria: string }) => void;
  } = $props();

  let name = $state(program?.name ?? '');
  let evaluationCriteria = $state(program?.evaluation_criteria ?? 'spend');
  let submitting = $state(false);

  let isValid = $derived(name.trim().length > 0);
  let isEditing = $derived(program !== null);

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

  <div class="form-field">
    <label class="form-label" for="program-name">Program Name</label>
    <input
      id="program-name"
      type="text"
      class="form-input"
      placeholder="e.g. Rewards Club"
      bind:value={name}
    />
  </div>

  <div class="form-field">
    <label class="form-label" for="program-criteria">Evaluation Criteria</label>
    <select id="program-criteria" class="form-select" bind:value={evaluationCriteria}>
      <option value="spend">Total Spend</option>
      <option value="orders">Order Count</option>
      <option value="points">Points Earned</option>
    </select>
    <span class="form-hint">Determines how customers progress through tiers</span>
  </div>

  <div class="form-actions">
    <button type="submit" class="btn-primary" disabled={!isValid || submitting}>
      {isEditing ? 'Update Program' : 'Create Program'}
    </button>
  </div>
</form>

<style>
  .program-form {
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

  .form-field {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .form-label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-text);
  }

  .form-input,
  .form-select {
    padding: var(--space-3);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background: var(--color-bg);
    color: var(--color-text);
    font-size: var(--font-size-base);
    outline: none;
    transition: border-color var(--transition-fast);
  }

  .form-input:focus,
  .form-select:focus {
    border-color: var(--color-primary);
  }

  .form-input::placeholder {
    color: var(--color-text-muted);
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
</style>
