<script lang="ts">
  import type { Rule, RewardRuleConfig, Condition, RewardAction } from '../types';

  let { rule = null, onSave, onCancel }: {
    rule?: Rule | null;
    onSave: (name: string, ruleType: string, config: RewardRuleConfig) => void;
    onCancel: () => void;
  } = $props();

  let isEdit = $derived(rule !== null);

  let name = $state(rule?.name ?? '');
  let ruleType = $state(rule?.rule_type ?? 'earn');
  let eventType = $state(rule?.config.event_type ?? 'order_completed');
  let conditions = $state<Array<Condition>>(
    rule?.config.conditions
      ? rule.config.conditions.map((c) => ({ ...c }))
      : [{ field: '', operator: 'eq', value: '' }]
  );
  let bucketType = $state(rule?.config.action.bucket_type ?? 'loyalty_points');
  let calculation = $state(rule?.config.action.calculation ?? 'percentage');
  let actionValue = $state(rule?.config.action.value ?? 0);
  let maxAmount = $state<number | null>(rule?.config.action.max_amount ?? null);
  let conversionRate = $state<number | null>(rule?.config.action.conversion_rate ?? null);
  let expiryDays = $state<number | null>(rule?.config.action.expiry_days ?? null);

  let nameError = $derived(name.trim().length === 0 ? 'Name is required' : null);
  let hasConditionError = $derived(
    conditions.some((c) => c.field.trim().length === 0)
  );
  let isValid = $derived(nameError === null && !hasConditionError && actionValue > 0);

  const EVENT_TYPES = [
    'order_completed',
    'signup',
    'referral',
    'review_submitted',
    'birthday',
    'custom'
  ];

  const RULE_TYPES = ['earn', 'burn', 'bonus'];
  const BUCKET_TYPES = ['loyalty_points', 'cashback', 'gift_card_credit'];
  const CALCULATION_TYPES = ['percentage', 'fixed', 'per_unit'];
  const OPERATORS = ['eq', 'neq', 'gt', 'gte', 'lt', 'lte', 'contains', 'in'];

  function addCondition() {
    conditions = [...conditions, { field: '', operator: 'eq', value: '' }];
  }

  function removeCondition(index: number) {
    conditions = conditions.filter((_, i) => i !== index);
  }

  function updateConditionField(index: number, field: string) {
    conditions = conditions.map((c, i) => (i === index ? { ...c, field } : c));
  }

  function updateConditionOperator(index: number, operator: string) {
    conditions = conditions.map((c, i) => (i === index ? { ...c, operator } : c));
  }

  function updateConditionValue(index: number, value: string) {
    conditions = conditions.map((c, i) => (i === index ? { ...c, value } : c));
  }

  function handleSubmit() {
    if (!isValid) return;

    const action: RewardAction = {
      bucket_type: bucketType,
      calculation,
      value: actionValue,
      max_amount: maxAmount,
      conversion_rate: conversionRate,
      expiry_days: expiryDays
    };

    const config: RewardRuleConfig = {
      event_type: eventType,
      conditions: conditions.filter((c) => c.field.trim().length > 0),
      action
    };

    onSave(name.trim(), ruleType, config);
  }
</script>

<form class="rule-form" onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
  <h2 class="form-title">{isEdit ? 'Edit Rule' : 'Create Rule'}</h2>

  <section class="form-section">
    <h3 class="section-title">Basic Info</h3>
    <div class="field-group">
      <label class="field-label" for="rule-name">Name</label>
      <input
        id="rule-name"
        type="text"
        class="field-input"
        class:field-error={nameError !== null && name.length > 0}
        value={name}
        oninput={(e) => { name = e.currentTarget.value; }}
        placeholder="e.g. 10% Cashback on Orders"
      />
      {#if nameError !== null && name.length > 0}
        <span class="error-text">{nameError}</span>
      {/if}
    </div>

    <div class="field-row">
      <div class="field-group">
        <label class="field-label" for="rule-type">Rule Type</label>
        <select
          id="rule-type"
          class="field-input"
          value={ruleType}
          onchange={(e) => { ruleType = e.currentTarget.value; }}
          disabled={isEdit}
        >
          {#each RULE_TYPES as rt}
            <option value={rt}>{rt.charAt(0).toUpperCase() + rt.slice(1)}</option>
          {/each}
        </select>
      </div>

      <div class="field-group">
        <label class="field-label" for="event-type">Event Type</label>
        <select
          id="event-type"
          class="field-input"
          value={eventType}
          onchange={(e) => { eventType = e.currentTarget.value; }}
        >
          {#each EVENT_TYPES as et}
            <option value={et}>{et.replaceAll('_', ' ')}</option>
          {/each}
        </select>
      </div>
    </div>
  </section>

  <section class="form-section">
    <div class="section-header">
      <h3 class="section-title">Conditions</h3>
      <button type="button" class="add-btn" onclick={addCondition}>+ Add Condition</button>
    </div>

    {#each conditions as condition, index (index)}
      <div class="condition-row">
        <div class="field-group condition-field">
          <label class="field-label" for="cond-field-{index}">Field</label>
          <input
            id="cond-field-{index}"
            type="text"
            class="field-input"
            value={condition.field}
            oninput={(e) => updateConditionField(index, e.currentTarget.value)}
            placeholder="e.g. order_total"
          />
        </div>
        <div class="field-group condition-operator">
          <label class="field-label" for="cond-op-{index}">Operator</label>
          <select
            id="cond-op-{index}"
            class="field-input"
            value={condition.operator}
            onchange={(e) => updateConditionOperator(index, e.currentTarget.value)}
          >
            {#each OPERATORS as op}
              <option value={op}>{op}</option>
            {/each}
          </select>
        </div>
        <div class="field-group condition-value">
          <label class="field-label" for="cond-val-{index}">Value</label>
          <input
            id="cond-val-{index}"
            type="text"
            class="field-input"
            value={String(condition.value)}
            oninput={(e) => updateConditionValue(index, e.currentTarget.value)}
            placeholder="e.g. 500"
          />
        </div>
        <button
          type="button"
          class="remove-btn"
          onclick={() => removeCondition(index)}
          disabled={conditions.length <= 1}
        >
          Remove
        </button>
      </div>
    {/each}
  </section>

  <section class="form-section">
    <h3 class="section-title">Reward Action</h3>

    <div class="field-row">
      <div class="field-group">
        <label class="field-label" for="bucket-type">Bucket Type</label>
        <select
          id="bucket-type"
          class="field-input"
          value={bucketType}
          onchange={(e) => { bucketType = e.currentTarget.value; }}
        >
          {#each BUCKET_TYPES as bt}
            <option value={bt}>{bt.replaceAll('_', ' ')}</option>
          {/each}
        </select>
      </div>
      <div class="field-group">
        <label class="field-label" for="calculation">Calculation</label>
        <select
          id="calculation"
          class="field-input"
          value={calculation}
          onchange={(e) => { calculation = e.currentTarget.value; }}
        >
          {#each CALCULATION_TYPES as ct}
            <option value={ct}>{ct.replaceAll('_', ' ')}</option>
          {/each}
        </select>
      </div>
    </div>

    <div class="field-row">
      <div class="field-group">
        <label class="field-label" for="action-value">
          Value {calculation === 'percentage' ? '(%)' : ''}
        </label>
        <input
          id="action-value"
          type="number"
          class="field-input"
          value={actionValue}
          oninput={(e) => { actionValue = Number(e.currentTarget.value); }}
          min="0"
          step={calculation === 'percentage' ? '0.1' : '1'}
        />
      </div>
      <div class="field-group">
        <label class="field-label" for="max-amount">Max Amount</label>
        <input
          id="max-amount"
          type="number"
          class="field-input"
          value={maxAmount ?? ''}
          oninput={(e) => {
            const val = e.currentTarget.value;
            maxAmount = val === '' ? null : Number(val);
          }}
          min="0"
          placeholder="No limit"
        />
      </div>
    </div>

    <div class="field-row">
      <div class="field-group">
        <label class="field-label" for="conversion-rate">Conversion Rate</label>
        <input
          id="conversion-rate"
          type="number"
          class="field-input"
          value={conversionRate ?? ''}
          oninput={(e) => {
            const val = e.currentTarget.value;
            conversionRate = val === '' ? null : Number(val);
          }}
          min="0"
          step="0.01"
          placeholder="Optional"
        />
      </div>
      <div class="field-group">
        <label class="field-label" for="expiry-days">Expiry (days)</label>
        <input
          id="expiry-days"
          type="number"
          class="field-input"
          value={expiryDays ?? ''}
          oninput={(e) => {
            const val = e.currentTarget.value;
            expiryDays = val === '' ? null : Number(val);
          }}
          min="0"
          step="1"
          placeholder="No expiry"
        />
      </div>
    </div>
  </section>

  <div class="form-actions">
    <button type="button" class="btn-cancel" onclick={onCancel}>Cancel</button>
    <button type="submit" class="btn-save" disabled={!isValid}>
      {isEdit ? 'Update Rule' : 'Create Rule'}
    </button>
  </div>
</form>

<style>
  .rule-form {
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
    padding: var(--space-6);
    max-width: 720px;
  }

  .form-title {
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
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

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .section-title {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
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

  .field-input:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .field-error {
    border-color: var(--color-error);
  }

  .error-text {
    font-size: var(--font-size-xs);
    color: var(--color-error);
  }

  .field-row {
    display: flex;
    gap: var(--space-4);
  }

  .condition-row {
    display: flex;
    gap: var(--space-3);
    align-items: flex-end;
  }

  .condition-field {
    flex: 2;
  }

  .condition-operator {
    flex: 1;
  }

  .condition-value {
    flex: 2;
  }

  .add-btn {
    padding: var(--space-1) var(--space-3);
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-primary);
    background: none;
    border: 1px dashed var(--color-primary);
    border-radius: var(--radius-sm);
    transition: all var(--transition-fast);
  }

  .add-btn:hover {
    background: color-mix(in srgb, var(--color-primary) 8%, transparent);
  }

  .remove-btn {
    padding: var(--space-2) var(--space-3);
    font-size: var(--font-size-sm);
    color: var(--color-error);
    background: none;
    border: 1px solid var(--color-error);
    border-radius: var(--radius-sm);
    transition: all var(--transition-fast);
    white-space: nowrap;
  }

  .remove-btn:hover:not(:disabled) {
    background: color-mix(in srgb, var(--color-error) 8%, transparent);
  }

  .remove-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
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
