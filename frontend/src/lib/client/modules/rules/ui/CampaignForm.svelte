<script lang="ts">
  import type {
    FestiveTemplate,
    Rule,
    Campaign,
    CreateCampaignFromTemplateRequest,
    CreateCampaignDirectRequest
  } from '../types';
  import { Button, Input, Select } from '@juspay/svelte-ui-components';
  import Icon from '$lib/components/Icon.svelte';

  let {
    template = null,
    rules,
    existingCampaigns = [],
    onSaveTemplate,
    onSaveDirect,
    onCancel
  }: {
    template?: FestiveTemplate | null;
    rules: Array<Rule>;
    existingCampaigns?: Array<Campaign>;
    onSaveTemplate?: (req: CreateCampaignFromTemplateRequest) => void;
    onSaveDirect?: (req: CreateCampaignDirectRequest) => void;
    onCancel: () => void;
  } = $props();

  let isTemplate = $derived(template !== null);

  // svelte-ignore state_referenced_locally
  let campaignName = $state(template ? `${template.display_name} Campaign` : '');
  // svelte-ignore state_referenced_locally
  let baseRuleId = $state(
    rules.filter((r) => r.is_active).length > 0 ? rules.filter((r) => r.is_active)[0].id : ''
  );
  // svelte-ignore state_referenced_locally
  let multiplier = $state(template?.default_multiplier ?? 2);
  let startsAt = $state(todayISO());
  // svelte-ignore state_referenced_locally
  let endsAt = $state(addDaysISO(todayISO(), template?.default_duration_days ?? 7));

  let activeRules = $derived(rules.filter((r) => r.is_active));
  let nameError = $derived(campaignName.trim().length === 0 ? 'Campaign name is required' : null);
  let ruleError = $derived(baseRuleId.length === 0 ? 'Select a reward rule' : null);
  let dateError = $derived(
    new Date(endsAt) <= new Date(startsAt) ? 'End date must be after start date' : null
  );
  let isValid = $derived(
    nameError === null && ruleError === null && dateError === null && multiplier > 0
  );

  let durationDays = $derived(() => {
    const diff = new Date(endsAt).getTime() - new Date(startsAt).getTime();
    return Math.max(0, Math.ceil(diff / 86400000));
  });

  let overlappingCampaign = $derived(() => {
    if (baseRuleId === '' || startsAt === '' || endsAt === '') return null;
    const start = new Date(startsAt).getTime();
    const end = new Date(endsAt).getTime();
    return (
      existingCampaigns.find((c) => {
        if (!c.is_active) return false;
        const cStart = new Date(c.starts_at).getTime();
        const cEnd = new Date(c.ends_at).getTime();
        return start < cEnd && end > cStart;
      }) ?? null
    );
  });

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
    if (isTemplate && onSaveTemplate && template) {
      onSaveTemplate({
        merchant_id: '',
        template_name: template.name,
        base_rule_id: baseRuleId,
        name: campaignName.trim(),
        multiplier,
        starts_at: new Date(startsAt).toISOString(),
        ends_at: new Date(endsAt).toISOString()
      });
    } else if (onSaveDirect) {
      onSaveDirect({
        merchant_id: '',
        name: campaignName.trim(),
        base_rule_id: baseRuleId,
        multiplier,
        starts_at: new Date(startsAt).toISOString(),
        ends_at: new Date(endsAt).toISOString()
      });
    }
  }
</script>

<form
  class="campaign-form"
  onsubmit={(e) => {
    e.preventDefault();
    handleSubmit();
  }}
>
  <h2 class="form-title">{isTemplate ? 'Create Campaign from Template' : 'Create campaign'}</h2>

  {#if isTemplate && template}
    <div class="template-banner">
      <span class="template-banner-icon"><Icon name="star" size={18} /></span>
      <div>
        <div class="template-banner-name">{template.display_name} Template</div>
        <div class="template-banner-desc">
          {template.description} — Default: {template.default_multiplier}x for {template.default_duration_days}
          days
        </div>
      </div>
    </div>
  {/if}

  <section class="form-section">
    <div class="field-group">
      <Input
        value={campaignName}
        label="Campaign name"
        placeholder="e.g. Weekend Double Points"
        onInput={(val) => {
          campaignName = val;
        }}
        classes="field-input{nameError !== null && campaignName.length > 0 ? ' field-error' : ''}"
      />
    </div>

    <div class="field-row">
      <div class="field-group">
        <label class="field-label" for="multiplier">Multiplier</label>
        <div class="multiplier-input-row">
          <Input
            value={String(multiplier)}
            onInput={(val) => {
              multiplier = Number(val) || 1;
            }}
            classes="field-input multiplier-field"
          />
          <span class="multiplier-suffix">x</span>
        </div>
        {#if isTemplate && template}
          <span class="field-hint">Template default: {template.default_multiplier}x</span>
        {:else}
          <span class="field-hint">How much to multiply the base earning</span>
        {/if}
      </div>

      <div class="field-group">
        <label class="field-label" for="base-rule">Linked reward rule</label>
        {#if activeRules.length === 0}
          <p class="no-rules-text">No active rules. Create a rule first.</p>
        {:else}
          <Select
            items={activeRules.map((r) => ({ id: r.id, label: `${r.name} (${r.rule_type})` }))}
            value={[baseRuleId]}
            onchange={(vals) => {
              baseRuleId = vals[0] ?? '';
            }}
            classes="field-input"
          />
          <span class="field-hint">Multiplier applies when this rule fires</span>
        {/if}
      </div>
    </div>

    <div class="field-row">
      <div class="field-group">
        <Input
          value={startsAt}
          label="Start date"
          onInput={(val) => {
            startsAt = val;
          }}
          classes="field-input"
        />
      </div>
      <div class="field-group">
        <Input
          value={endsAt}
          label="End date"
          onInput={(val) => {
            endsAt = val;
          }}
          classes="field-input{dateError !== null ? ' field-error' : ''}"
        />
        {#if dateError !== null}
          <span class="error-text">{dateError}</span>
        {:else}
          <span class="field-hint">{durationDays()} days</span>
        {/if}
      </div>
    </div>
  </section>

  <div class="earning-preview">
    <div class="preview-title">Earning preview</div>
    <div class="preview-formula">
      <div class="pf-block"><span class="pf-label">Base</span><span class="pf-val">100</span></div>
      <span class="pf-op">&times;</span>
      <div class="pf-block">
        <span class="pf-label">Campaign</span><span class="pf-val blue">{multiplier}x</span>
      </div>
      <span class="pf-op">&times;</span>
      <div class="pf-block">
        <span class="pf-label">Tier</span><span class="pf-val purple">1x–2x</span>
      </div>
      <span class="pf-op">=</span>
      <div class="pf-result">
        <span class="pf-label">Effective</span><span class="pf-val"
          >{multiplier}x–{multiplier * 2}x</span
        >
      </div>
    </div>
    <div class="preview-note">Stacking: Multiplicative &middot; Max cap: 10x</div>
  </div>

  {#if overlappingCampaign() !== null}
    {@const overlap = overlappingCampaign()}
    {#if overlap}
      <div class="overlap-warning">
        <strong>Campaign overlap detected.</strong> The selected dates overlap with:
        <div class="overlap-campaign">
          <div>
            <div class="overlap-name">{overlap.name}</div>
            <div class="overlap-dates">
              {new Date(overlap.starts_at).toLocaleDateString('en-IN', {
                day: 'numeric',
                month: 'short'
              })} – {new Date(overlap.ends_at).toLocaleDateString('en-IN', {
                day: 'numeric',
                month: 'short',
                year: 'numeric'
              })}
            </div>
          </div>
          {#if overlap.multiplier}
            <span class="overlap-mult">{overlap.multiplier}x</span>
          {/if}
        </div>
        <div class="overlap-note">
          During the overlap period, only the campaign with the <strong>higher multiplier</strong> will
          apply.
        </div>
      </div>
    {/if}
  {/if}

  <div class="form-actions">
    <Button text="Cancel" onclick={onCancel} classes="btn-secondary" />
    <Button
      text={overlappingCampaign() !== null
        ? 'Create anyway'
        : isTemplate && template
          ? `Create ${template.display_name} Campaign`
          : 'Create campaign'}
      onclick={handleSubmit}
      disabled={!isValid}
      classes="btn-primary"
    />
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
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
  }

  .template-banner {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-3) var(--space-4);
    background: var(--yellow-100);
    border: 1px solid var(--yellow-100);
    border-radius: var(--radius-md);
  }
  .template-banner-icon {
    font-size: var(--font-size-2xl);
  }
  .template-banner-name {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--yellow-700);
  }
  .template-banner-desc {
    font-size: var(--font-size-xs);
    color: var(--yellow-700);
  }

  .form-section {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    padding: var(--space-5);
    background: var(--color-surface);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-card);
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

  .earning-preview {
    padding: var(--space-3) var(--space-4);
    background: var(--color-surface);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-card);
  }
  .preview-title {
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-muted);
    margin-bottom: var(--space-2);
  }
  .preview-formula {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    flex-wrap: wrap;
  }
  .pf-block {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2px;
    padding: 4px 10px;
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    min-width: 50px;
  }
  .pf-label {
    font-size: 8px;
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.03em;
  }
  .pf-val {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-bold);
  }
  .pf-val.blue {
    color: var(--p-600);
  }
  .pf-val.purple {
    color: var(--purple-500);
  }
  .pf-op {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-normal);
    color: var(--color-text-muted);
  }
  .pf-result {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2px;
    padding: 4px 10px;
    background: var(--green-700);
    border-radius: var(--radius-md);
    min-width: 50px;
  }
  .pf-result .pf-label {
    color: rgba(255, 255, 255, 0.7);
  }
  .pf-result .pf-val {
    color: white;
  }
  .preview-note {
    font-size: 10px;
    color: var(--color-text-muted);
    margin-top: var(--space-2);
  }

  .overlap-warning {
    padding: var(--space-3) var(--space-4);
    background: var(--yellow-100);
    border: 1px solid var(--yellow-100);
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
    color: var(--yellow-700);
    line-height: 1.5;
  }
  .overlap-campaign {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin: var(--space-2) 0;
    padding: var(--space-2) var(--space-3);
    background: white;
    border-radius: var(--radius-sm);
  }
  .overlap-name {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }
  .overlap-dates {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }
  .overlap-mult {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-bold);
    color: var(--color-primary);
    font-family: var(--font-mono);
  }
  .overlap-note {
    margin-top: var(--space-2);
  }

  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: var(--space-3);
    padding-top: var(--space-4);
    border-top: 1px solid var(--color-border);
  }
</style>
