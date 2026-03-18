<script lang="ts">
  import type { FestiveTemplate, Rule, CreateCampaignFromTemplateRequest } from '../types';

  let { template, rules, onSave, onCancel }: {
    template: FestiveTemplate;
    rules: Array<Rule>;
    onSave: (req: CreateCampaignFromTemplateRequest) => void;
    onCancel: () => void;
  } = $props();

  let campaignName = $state(`${template.display_name} Campaign`);
  let baseRuleId = $state(rules.length > 0 ? rules[0].id : '');
  let multiplier = $state(template.default_multiplier);

  let startsAt = $state(todayISO());
  let endsAt = $state(addDaysISO(todayISO(), template.default_duration_days));

  let activeRules = $derived(rules.filter((r) => r.is_active));
  let nameError = $derived(campaignName.trim().length === 0 ? 'Campaign name is required' : null);
  let ruleError = $derived(baseRuleId.length === 0 ? 'Select a base rule' : null);
  let dateError = $derived(
    new Date(endsAt) <= new Date(startsAt) ? 'End date must be after start date' : null
  );
  let isValid = $derived(nameError === null && ruleError === null && dateError === null && multiplier > 0);

  function todayISO(): string {
    return new Date().toISOString().split('T')[0];
  }

  function addDaysISO(dateStr: string, days: number): string {
    const date = new Date(dateStr);
    date.setDate(date.getDate() + days);
    return date.toISOString().split('T')[0];
  }

  function handleSubmit() {
    if (!isValid) return;

    onSave({
      merchant_id: '',
      template_name: template.name,
      base_rule_id: baseRuleId,
      name: campaignName.trim(),
      multiplier,
      starts_at: new Date(startsAt).toISOString(),
      ends_at: new Date(endsAt).toISOString()
    });
  }
</script>

<form class="campaign-form" onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
  <h2 class="form-title">Create Campaign</h2>
  <p class="form-subtitle">Based on: <strong>{template.display_name}</strong></p>

  <section class="form-section">
    <div class="field-group">
      <label class="field-label" for="campaign-name">Campaign Name</label>
      <input
        id="campaign-name"
        type="text"
        class="field-input"
        class:field-error={nameError !== null && campaignName.length > 0}
        value={campaignName}
        oninput={(e) => { campaignName = e.currentTarget.value; }}
      />
      {#if nameError !== null && campaignName.length > 0}
        <span class="error-text">{nameError}</span>
      {/if}
    </div>

    <div class="field-group">
      <label class="field-label" for="base-rule">Base Rule</label>
      {#if activeRules.length === 0}
        <p class="no-rules-text">No active rules available. Create a rule first.</p>
      {:else}
        <select
          id="base-rule"
          class="field-input"
          value={baseRuleId}
          onchange={(e) => { baseRuleId = e.currentTarget.value; }}
        >
          {#each activeRules as r}
            <option value={r.id}>{r.name} ({r.rule_type})</option>
          {/each}
        </select>
      {/if}
    </div>

    <div class="field-group">
      <label class="field-label" for="multiplier">Reward Multiplier</label>
      <div class="multiplier-input-row">
        <input
          id="multiplier"
          type="number"
          class="field-input"
          value={multiplier}
          oninput={(e) => { multiplier = Number(e.currentTarget.value); }}
          min="1"
          max="20"
          step="0.5"
        />
        <span class="multiplier-suffix">x rewards</span>
      </div>
      <span class="field-hint">Default for {template.display_name}: {template.default_multiplier}x</span>
    </div>

    <div class="field-row">
      <div class="field-group">
        <label class="field-label" for="starts-at">Start Date</label>
        <input
          id="starts-at"
          type="date"
          class="field-input"
          value={startsAt}
          oninput={(e) => { startsAt = e.currentTarget.value; }}
        />
      </div>
      <div class="field-group">
        <label class="field-label" for="ends-at">End Date</label>
        <input
          id="ends-at"
          type="date"
          class="field-input"
          class:field-error={dateError !== null}
          value={endsAt}
          oninput={(e) => { endsAt = e.currentTarget.value; }}
        />
        {#if dateError !== null}
          <span class="error-text">{dateError}</span>
        {/if}
      </div>
    </div>
  </section>

  <div class="form-actions">
    <button type="button" class="btn-cancel" onclick={onCancel}>Cancel</button>
    <button type="submit" class="btn-save" disabled={!isValid}>Create Campaign</button>
  </div>
</form>

<style>
  .campaign-form {
    display: flex;
    flex-direction: column;
    gap: var(--space-5);
    padding: var(--space-6);
    max-width: 560px;
  }

  .form-title {
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
  }

  .form-subtitle {
    font-size: var(--font-size-base);
    color: var(--color-text-muted);
    margin-top: calc(-1 * var(--space-3));
  }

  .form-subtitle strong {
    color: var(--color-primary);
    font-weight: var(--font-weight-semibold);
  }

  .form-section {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    padding: var(--space-5);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
  }

  .field-group {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    flex: 1;
  }

  .field-label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-text-muted);
  }

  .field-input {
    padding: var(--space-2) var(--space-3);
    font-size: var(--font-size-base);
    color: var(--color-text);
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    transition: border-color var(--transition-fast);
  }

  .field-input:focus {
    outline: none;
    border-color: var(--color-primary);
  }

  .field-error {
    border-color: var(--color-error);
  }

  .error-text {
    font-size: var(--font-size-xs);
    color: var(--color-error);
  }

  .field-hint {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .field-row {
    display: flex;
    gap: var(--space-4);
  }

  .multiplier-input-row {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .multiplier-input-row .field-input {
    max-width: 120px;
  }

  .multiplier-suffix {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    font-weight: var(--font-weight-medium);
  }

  .no-rules-text {
    font-size: var(--font-size-sm);
    color: var(--color-warning);
    padding: var(--space-2) 0;
  }

  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: var(--space-3);
    padding-top: var(--space-4);
    border-top: 1px solid var(--color-border);
  }

  .btn-cancel {
    padding: var(--space-2) var(--space-5);
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-medium);
    color: var(--color-text-muted);
    background: none;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    transition: all var(--transition-fast);
  }

  .btn-cancel:hover {
    color: var(--color-text);
    border-color: var(--color-text-muted);
  }

  .btn-save {
    padding: var(--space-2) var(--space-5);
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-medium);
    color: white;
    background: var(--color-primary);
    border: none;
    border-radius: var(--radius-md);
    transition: all var(--transition-fast);
  }

  .btn-save:hover:not(:disabled) {
    background: var(--color-primary-hover);
  }

  .btn-save:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
