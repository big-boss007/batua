<script lang="ts">
  import type {
    FestiveTemplate,
    Rule,
    Campaign,
    CreateCampaignFromTemplateRequest,
    CreateCampaignDirectRequest
  } from '../types';

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

  let campaignName = $state(template ? `${template.display_name} Campaign` : '');
  let baseRuleId = $state(rules.filter((r) => r.is_active).length > 0 ? rules.filter((r) => r.is_active)[0].id : '');
  let multiplier = $state(template?.default_multiplier ?? 2);
  let startsAt = $state(todayISO());
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
    return existingCampaigns.find((c) => {
      if (!c.is_active) return false;
      const cStart = new Date(c.starts_at).getTime();
      const cEnd = new Date(c.ends_at).getTime();
      return start < cEnd && end > cStart;
    }) ?? null;
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

<form class="campaign-form" onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
  <h2 class="form-title">{isTemplate ? 'Create Campaign from Template' : 'Create Campaign'}</h2>

  {#if isTemplate && template}
    <div class="template-banner">
      <span class="template-banner-icon">🎉</span>
      <div>
        <div class="template-banner-name">{template.display_name} Template</div>
        <div class="template-banner-desc">{template.description} — Default: {template.default_multiplier}x for {template.default_duration_days} days</div>
      </div>
    </div>
  {/if}

  <section class="form-section">
    <div class="field-group">
      <label class="field-label" for="campaign-name">Campaign Name</label>
      <input
        id="campaign-name"
        type="text"
        class="field-input"
        class:field-error={nameError !== null && campaignName.length > 0}
        placeholder="e.g. Weekend Double Points"
        value={campaignName}
        oninput={(e) => { campaignName = e.currentTarget.value; }}
      />
    </div>

    <div class="field-row">
      <div class="field-group">
        <label class="field-label" for="multiplier">Multiplier</label>
        <div class="multiplier-input-row">
          <input
            id="multiplier"
            type="number"
            class="field-input multiplier-field"
            value={multiplier}
            oninput={(e) => { multiplier = Number(e.currentTarget.value); }}
            min="1"
            max="20"
            step="0.5"
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
        <label class="field-label" for="base-rule">Linked Reward Rule</label>
        {#if activeRules.length === 0}
          <p class="no-rules-text">No active rules. Create a rule first.</p>
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
          <span class="field-hint">Multiplier applies when this rule fires</span>
        {/if}
      </div>
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
        {:else}
          <span class="field-hint">{durationDays()} days</span>
        {/if}
      </div>
    </div>
  </section>

  <div class="earning-preview">
    <div class="preview-title">Earning Preview</div>
    <div class="preview-formula">
      <div class="pf-block"><span class="pf-label">Base</span><span class="pf-val">100</span></div>
      <span class="pf-op">&times;</span>
      <div class="pf-block"><span class="pf-label">Campaign</span><span class="pf-val blue">{multiplier}x</span></div>
      <span class="pf-op">&times;</span>
      <div class="pf-block"><span class="pf-label">Tier</span><span class="pf-val purple">1x–2x</span></div>
      <span class="pf-op">=</span>
      <div class="pf-result"><span class="pf-label">Effective</span><span class="pf-val">{multiplier}x–{multiplier * 2}x</span></div>
    </div>
    <div class="preview-note">Stacking: Multiplicative &middot; Max cap: 10x</div>
  </div>

  {#if overlappingCampaign() !== null}
    {@const overlap = overlappingCampaign()}
    {#if overlap}
      <div class="overlap-warning">
        <strong>⚠ Campaign overlap detected.</strong> The selected dates overlap with:
        <div class="overlap-campaign">
          <div>
            <div class="overlap-name">{overlap.name}</div>
            <div class="overlap-dates">{new Date(overlap.starts_at).toLocaleDateString('en-IN', { day: 'numeric', month: 'short' })} – {new Date(overlap.ends_at).toLocaleDateString('en-IN', { day: 'numeric', month: 'short', year: 'numeric' })}</div>
          </div>
          {#if overlap.multiplier}
            <span class="overlap-mult">{overlap.multiplier}x</span>
          {/if}
        </div>
        <div class="overlap-note">During the overlap period, only the campaign with the <strong>higher multiplier</strong> will apply.</div>
      </div>
    {/if}
  {/if}

  <div class="form-actions">
    <button type="button" class="btn-cancel" onclick={onCancel}>Cancel</button>
    <button type="submit" class="btn-save" disabled={!isValid}>
      {overlappingCampaign() !== null ? 'Create Anyway' : isTemplate && template ? `Create ${template.display_name} Campaign` : 'Create Campaign'}
    </button>
  </div>
</form>

<style>
  .campaign-form { display: flex; flex-direction: column; gap: var(--space-5); padding: var(--space-6); max-width: 560px; }
  .form-title { font-size: var(--font-size-lg); font-weight: var(--font-weight-bold); color: var(--color-text); }

  .template-banner {
    display: flex; align-items: center; gap: var(--space-3);
    padding: var(--space-3) var(--space-4); background: #fef9c3; border: 1px solid #fde68a; border-radius: var(--radius-md);
  }
  .template-banner-icon { font-size: 24px; }
  .template-banner-name { font-size: var(--font-size-sm); font-weight: var(--font-weight-semibold); color: #854d0e; }
  .template-banner-desc { font-size: var(--font-size-xs); color: #92400e; }

  .form-section { display: flex; flex-direction: column; gap: var(--space-4); padding: var(--space-5); background: var(--color-surface); border: 1px solid var(--color-border); border-radius: var(--radius-md); }
  .field-group { display: flex; flex-direction: column; gap: var(--space-1); flex: 1; }
  .field-label { font-size: var(--font-size-sm); font-weight: var(--font-weight-medium); color: var(--color-text-muted); }
  .field-input { padding: var(--space-2) var(--space-3); font-size: var(--font-size-base); color: var(--color-text); background: var(--color-bg); border: 1px solid var(--color-border); border-radius: var(--radius-sm); font-family: inherit; }
  .field-input:focus { outline: none; border-color: var(--color-primary); }
  .field-error { border-color: var(--color-error); }
  .error-text { font-size: var(--font-size-xs); color: var(--color-error); }
  .field-hint { font-size: var(--font-size-xs); color: var(--color-text-muted); }
  .field-row { display: flex; gap: var(--space-4); }

  .multiplier-input-row { display: flex; align-items: center; gap: var(--space-2); }
  .multiplier-field { max-width: 100px; font-family: var(--font-mono); font-weight: var(--font-weight-semibold); }
  .multiplier-suffix { font-size: var(--font-size-sm); color: var(--color-text-muted); font-weight: var(--font-weight-medium); }
  .no-rules-text { font-size: var(--font-size-sm); color: var(--color-warning); padding: var(--space-2) 0; }

  .earning-preview {
    padding: var(--space-3) var(--space-4); background: var(--color-surface); border: 1px solid var(--color-border); border-radius: var(--radius-md);
  }
  .preview-title { font-size: var(--font-size-xs); font-weight: var(--font-weight-semibold); color: var(--color-text-muted); margin-bottom: var(--space-2); }
  .preview-formula { display: flex; align-items: center; gap: var(--space-2); flex-wrap: wrap; }
  .pf-block { display: flex; flex-direction: column; align-items: center; gap: 2px; padding: 4px 10px; background: var(--color-bg); border: 1px solid var(--color-border); border-radius: var(--radius-md); min-width: 50px; }
  .pf-label { font-size: 8px; color: var(--color-text-muted); text-transform: uppercase; letter-spacing: 0.03em; }
  .pf-val { font-size: var(--font-size-sm); font-weight: var(--font-weight-bold); }
  .pf-val.blue { color: #3b82f6; }
  .pf-val.purple { color: #6366f1; }
  .pf-op { font-size: var(--font-size-sm); font-weight: 300; color: var(--color-text-muted); }
  .pf-result { display: flex; flex-direction: column; align-items: center; gap: 2px; padding: 4px 10px; background: #065f46; border-radius: var(--radius-md); min-width: 50px; }
  .pf-result .pf-label { color: rgba(255,255,255,0.7); }
  .pf-result .pf-val { color: white; }
  .preview-note { font-size: 10px; color: var(--color-text-muted); margin-top: var(--space-2); }

  .overlap-warning {
    padding: var(--space-3) var(--space-4); background: #fffbeb; border: 1px solid #fde68a;
    border-radius: var(--radius-md); font-size: var(--font-size-sm); color: #92400e; line-height: 1.5;
  }
  .overlap-campaign {
    display: flex; justify-content: space-between; align-items: center;
    margin: var(--space-2) 0; padding: var(--space-2) var(--space-3);
    background: white; border-radius: var(--radius-sm);
  }
  .overlap-name { font-size: var(--font-size-sm); font-weight: var(--font-weight-semibold); color: #1a1a2e; }
  .overlap-dates { font-size: var(--font-size-xs); color: var(--color-text-muted); }
  .overlap-mult { font-size: var(--font-size-sm); font-weight: var(--font-weight-bold); color: var(--color-primary); font-family: var(--font-mono); }
  .overlap-note { margin-top: var(--space-2); }

  .form-actions { display: flex; justify-content: flex-end; gap: var(--space-3); padding-top: var(--space-4); border-top: 1px solid var(--color-border); }
  .btn-cancel { padding: var(--space-2) var(--space-5); font-size: var(--font-size-base); font-weight: var(--font-weight-medium); color: var(--color-text-muted); background: none; border: 1px solid var(--color-border); border-radius: var(--radius-md); cursor: pointer; }
  .btn-cancel:hover { color: var(--color-text); border-color: var(--color-text-muted); }
  .btn-save { padding: var(--space-2) var(--space-5); font-size: var(--font-size-base); font-weight: var(--font-weight-medium); color: white; background: var(--color-primary); border: none; border-radius: var(--radius-md); cursor: pointer; }
  .btn-save:hover:not(:disabled) { background: var(--color-primary-hover); }
  .btn-save:disabled { opacity: 0.5; cursor: not-allowed; }
</style>
