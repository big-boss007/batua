<script lang="ts">
  import type { Rule, RewardRuleConfig, Condition, RewardAction } from '../types';
  import { currentMerchant } from '$lib/client/modules/admin';
  import type { Merchant } from '$lib/client/modules/admin';
  import { Button, Input, Select } from '@juspay/svelte-ui-components';

  let {
    rule = null,
    onSave,
    onCancel
  }: {
    rule?: Rule | null;
    onSave: (name: string, ruleType: string, config: RewardRuleConfig) => void;
    onCancel: () => void;
  } = $props();

  let merchant = $state<Merchant | null>(null);
  currentMerchant.subscribe((m) => { merchant = m; });

  let pIcon = $derived(merchant?.points_icon ?? 'pts');
  let pRate = $derived(merchant?.points_to_currency_rate ?? 1.0);

  let isEdit = $derived(rule !== null);

  let name = $state(rule?.name ?? '');
  let eventType = $state(rule?.config.event_type ?? 'order.completed');
  let conditions = $state<Array<Condition>>(
    rule?.config.conditions
      ? rule.config.conditions.map((c) => ({ ...c }))
      : []
  );
  let calculation = $state(rule?.config.action.calculation ?? 'percentage');
  let actionValue = $state(rule?.config.action.value ?? 0);
  let maxAmount = $state<number | null>(rule?.config.action.max_amount ?? null);
  let expiryDays = $state<number | null>(rule?.config.action.expiry_days ?? null);

  let nameError = $derived(name.trim().length === 0 ? 'Name is required' : null);
  let isValid = $derived(nameError === null && actionValue > 0);

  const EVENT_TYPES: Array<{ value: string; label: string }> = [
    { value: 'order.completed', label: 'Order Completed' },
    { value: 'review.submitted', label: 'Review Submitted' },
    { value: 'signup', label: 'Signup' },
    { value: 'referral', label: 'Referral' },
    { value: 'birthday', label: 'Birthday' }
  ];

  const CONDITION_FIELDS: Array<{ value: string; label: string }> = [
    { value: 'order_amount', label: 'Order Amount' },
    { value: 'payment_method', label: 'Payment Method' },
    { value: 'is_cod', label: 'Is COD' },
    { value: 'is_first_order', label: 'First Order' },
    { value: 'collections', label: 'Collection' },
    { value: 'customer_tags', label: 'Customer Tag' }
  ];

  const OPERATORS: Array<{ value: string; label: string }> = [
    { value: 'eq', label: '=' },
    { value: 'neq', label: '≠' },
    { value: 'gt', label: '>' },
    { value: 'gte', label: '≥' },
    { value: 'lt', label: '<' },
    { value: 'lte', label: '≤' },
    { value: 'in', label: 'in' },
    { value: 'not_in', label: 'not in' }
  ];

  let previewText = $derived.by(() => {
    if (calculation === 'percentage') {
      const pts = Math.round(1000 * actionValue / 100);
      const worth = Math.round(pts * pRate);
      let text = `On a ₹1,000 order → customer earns ${pts.toLocaleString('en-IN')} ${pIcon}`;
      let sub = `worth ₹${worth}`;
      if (maxAmount !== null && maxAmount > 0) {
        const maxWorth = Math.round(maxAmount * pRate);
        sub += ` · capped at ${maxAmount.toLocaleString('en-IN')} ${pIcon} (₹${maxWorth})`;
      }
      if (expiryDays !== null && expiryDays > 0) {
        sub += ` · expires in ${expiryDays} days`;
      }
      return { text, sub };
    }
    const worth = Math.round(actionValue * pRate);
    let sub = `worth ₹${worth}`;
    if (expiryDays !== null && expiryDays > 0) {
      sub += ` · expires in ${expiryDays} days`;
    }
    return {
      text: `Customer earns ${actionValue.toLocaleString('en-IN')} ${pIcon}`,
      sub
    };
  });

  function addCondition() {
    conditions = [...conditions, { field: 'order_amount', operator: 'gte', value: '' }];
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
      bucket_type: 'earned_credit',
      calculation,
      value: actionValue,
      max_amount: calculation === 'percentage' ? maxAmount : null,
      expiry_days: expiryDays
    };

    const config: RewardRuleConfig = {
      event_type: eventType,
      conditions: conditions.filter((c) => c.field.trim().length > 0),
      action
    };

    onSave(name.trim(), 'reward', config);
  }
</script>

<form
  class="rule-form"
  onsubmit={(e) => {
    e.preventDefault();
    handleSubmit();
  }}
>
  <h2 class="form-title">{isEdit ? 'Edit Rule' : 'Create Earn Rule'}</h2>

  <section class="form-section">
    <div class="field-group">
      <Input
        value={name}
        label="Rule Name"
        placeholder="e.g. 10% Points Back on Orders"
        onInput={(val) => { name = val; }}
        classes="field-input{nameError !== null && name.length > 0 ? ' field-error' : ''}"
      />
    </div>

    <div class="field-group">
      <label class="field-label">Trigger Event</label>
      <Select
        items={EVENT_TYPES.map((et) => ({ id: et.value, label: et.label }))}
        value={[eventType]}
        onchange={(vals) => { eventType = vals[0] ?? ''; }}
        classes="field-input"
      />
      <span class="field-hint">When should this rule fire?</span>
    </div>
  </section>

  <section class="form-section">
    <div class="section-header">
      <h3 class="section-title">Conditions</h3>
      <Button text="+ Add Condition" classes="btn-ghost" onclick={addCondition} />
    </div>

    {#if conditions.length === 0}
      <span class="field-hint">No conditions — rule applies to all events of this type</span>
    {/if}

    {#each conditions as condition, index (index)}
      <div class="condition-row">
        <div class="field-group condition-field">
          <Select
            items={[{ id: '', label: 'Select field...' }, ...CONDITION_FIELDS.map((cf) => ({ id: cf.value, label: cf.label }))]}
            value={[condition.field]}
            onchange={(vals) => updateConditionField(index, vals[0] ?? '')}
            classes="field-input"
          />
        </div>
        <div class="field-group condition-operator">
          <Select
            items={OPERATORS.map((op) => ({ id: op.value, label: op.label }))}
            value={[condition.operator]}
            onchange={(vals) => updateConditionOperator(index, vals[0] ?? '')}
            classes="field-input"
          />
        </div>
        <div class="field-group condition-value">
          <Input
            value={String(condition.value)}
            onInput={(val) => updateConditionValue(index, val)}
            placeholder="value"
            classes="field-input"
          />
        </div>
        <button type="button" class="remove-btn" onclick={() => removeCondition(index)}>×</button>
      </div>
    {/each}
  </section>

  <section class="form-section">
    <h3 class="section-title">Reward</h3>

    <div class="field-row">
      <div class="field-group">
        <label class="field-label">Type</label>
        <Select
          items={[{ id: 'percentage', label: 'Percentage of order' }, { id: 'fixed', label: 'Fixed amount' }]}
          value={[calculation]}
          onchange={(vals) => { calculation = vals[0] ?? 'percentage'; }}
          classes="field-input"
        />
      </div>
      <div class="field-group">
        <label class="field-label" for="action-value">
          {calculation === 'percentage' ? 'Earn Rate' : 'Reward'}
        </label>
        <div class="inline-field">
          <Input
            value={String(actionValue)}
            onInput={(val) => { actionValue = Number(val) || 0; }}
            classes="field-input inline-input"
          />
          <span class="inline-suffix">{calculation === 'percentage' ? '%' : pIcon}</span>
        </div>
      </div>
    </div>

    <div class="field-row">
      <div class="field-group" class:field-disabled={calculation !== 'percentage'}>
        <label class="field-label" for="max-amount">Max per order</label>
        <div class="inline-field">
          <Input
            value={calculation === 'percentage' ? (maxAmount !== null ? String(maxAmount) : '') : ''}
            onInput={(val) => { maxAmount = val === '' ? null : Number(val); }}
            placeholder={calculation === 'percentage' ? 'No cap' : 'N/A'}
            disable={calculation !== 'percentage'}
            classes="field-input inline-input"
          />
          <span class="inline-suffix">{pIcon}</span>
        </div>
        <span class="field-hint">{calculation === 'percentage' ? 'Leave empty for no cap' : 'Only applies to percentage'}</span>
      </div>
      <div class="field-group">
        <label class="field-label" for="expiry-days">Points expire after</label>
        <div class="inline-field">
          <Input
            value={expiryDays !== null ? String(expiryDays) : ''}
            onInput={(val) => { expiryDays = val === '' ? null : Number(val); }}
            placeholder="Never"
            classes="field-input inline-input"
          />
          <span class="inline-suffix">days</span>
        </div>
        <span class="field-hint">Leave empty for no expiry</span>
      </div>
    </div>

    {#if actionValue > 0}
      <div class="preview-box">
        <div class="preview-text">{previewText.text}</div>
        <div class="preview-sub">{previewText.sub}</div>
      </div>
    {/if}
  </section>

  <div class="form-actions">
    <Button text="Cancel" onclick={onCancel} classes="btn-secondary" />
    <Button text={isEdit ? 'Update Rule' : 'Create Rule'} onclick={handleSubmit} disabled={!isValid} classes="btn-primary" />
  </div>
</form>

<style>
  .rule-form {
    display: flex;
    flex-direction: column;
    gap: var(--space-5);
    padding: var(--space-5);
  }

  .form-title {
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
  }

  .form-section {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
    padding: var(--space-4);
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

  .field-hint {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .field-disabled {
    opacity: 0.45;
    pointer-events: none;
  }

  .field-input {
    padding: var(--space-2) var(--space-3);
    font-size: var(--font-size-base);
    color: var(--color-text);
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    transition: border-color var(--transition-fast);
    font-family: inherit;
  }

  .field-input:focus {
    outline: none;
    border-color: var(--color-primary);
  }

  .field-error {
    border-color: var(--color-error);
  }

  .field-row {
    display: flex;
    gap: var(--space-4);
  }

  .inline-field {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .inline-input {
    width: 100px;
  }

  .inline-suffix {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    font-weight: var(--font-weight-medium);
    white-space: nowrap;
  }

  .condition-row {
    display: flex;
    gap: var(--space-2);
    align-items: center;
  }

  .condition-field {
    flex: 2;
  }

  .condition-operator {
    flex: 0 0 70px;
  }

  .condition-value {
    flex: 2;
  }


  .remove-btn {
    padding: var(--space-1) var(--space-2);
    font-size: var(--font-size-lg);
    color: var(--color-error);
    background: none;
    border: none;
    border-radius: var(--radius-sm);
    transition: all var(--transition-fast);
    line-height: 1;
    flex-shrink: 0;
  }

  .remove-btn:hover {
    background: color-mix(in srgb, var(--color-error) 8%, transparent);
  }

  .preview-box {
    padding: var(--space-3) var(--space-4);
    background: color-mix(in srgb, var(--color-success) 6%, transparent);
    border: 1px solid color-mix(in srgb, var(--color-success) 20%, transparent);
    border-radius: var(--radius-md);
  }

  .preview-text {
    font-size: var(--font-size-sm);
    color: var(--color-text);
  }

  .preview-sub {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    margin-top: 2px;
  }

  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: var(--space-3);
    padding-top: var(--space-4);
    border-top: 1px solid var(--color-border);
  }

</style>
