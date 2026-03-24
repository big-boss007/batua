<script lang="ts">
  import type { CampaignStackingConfig } from '../types';

  let {
    config,
    onSave,
    onCancel
  }: {
    config: CampaignStackingConfig;
    onSave: (config: CampaignStackingConfig) => void;
    onCancel: () => void;
  } = $props();

  let stackingMode = $state(config.campaign_stacking_mode);
  let maxMultiplier = $state(config.max_campaign_multiplier);

  type StackingOption = {
    value: string;
    name: string;
    recommended: boolean;
    description: string;
    example: string;
  };

  const STACKING_OPTIONS: Array<StackingOption> = [
    {
      value: 'multiplicative',
      name: 'Multiplicative',
      recommended: true,
      description: 'Campaign and tier multipliers multiply together. Higher-tier members benefit more from campaigns.',
      example: 'Gold 2x × Campaign 2x = 4x total'
    },
    {
      value: 'best_of',
      name: 'Best-of',
      recommended: false,
      description: 'Customer gets the higher of campaign or tier multiplier, not both. More conservative.',
      example: 'Gold 2x vs Campaign 3x = 3x total'
    },
    {
      value: 'additive',
      name: 'Additive',
      recommended: false,
      description: 'Campaign bonus is added to tier multiplier. Predictable costs, moderate reward boost.',
      example: 'Gold 2x + Campaign 2x = 3x total'
    }
  ];

  function handleSave() {
    onSave({
      campaign_stacking_mode: stackingMode,
      max_campaign_multiplier: maxMultiplier
    });
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<div class="modal-overlay" role="presentation" onclick={onCancel}>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div class="modal-panel" role="dialog" aria-label="Campaign settings" tabindex="-1" onclick={(e) => e.stopPropagation()}>
    <div class="modal-header">
      <span class="modal-title">Campaign Settings</span>
      <button class="modal-close" onclick={onCancel}>&times;</button>
    </div>

    <div class="modal-body">
      <div class="config-section">
        <div class="config-title">Multiplier Stacking Mode</div>
        <p class="config-desc">How campaign multipliers combine with tier/membership multipliers</p>
        <div class="stacking-options">
          {#each STACKING_OPTIONS as option (option.value)}
            <button
              class="stacking-option"
              class:selected={stackingMode === option.value}
              onclick={() => { stackingMode = option.value; }}
            >
              <div class="stacking-radio" class:selected={stackingMode === option.value}></div>
              <div class="stacking-info">
                <div class="stacking-name">
                  {option.name}
                  {#if option.recommended}
                    <span class="recommended-tag">(Recommended)</span>
                  {/if}
                </div>
                <div class="stacking-desc">{option.description}</div>
                <div class="stacking-example">{option.example}</div>
              </div>
            </button>
          {/each}
        </div>
      </div>

      <div class="config-section">
        <div class="config-title">Safety Limits</div>
        <div class="config-row">
          <div>
            <div class="config-label">Maximum Effective Multiplier</div>
            <div class="config-sublabel">Cap the total multiplier regardless of stacking mode</div>
          </div>
          <div class="cap-input-row">
            <input
              class="cap-input"
              type="number"
              value={maxMultiplier}
              oninput={(e) => { maxMultiplier = Number(e.currentTarget.value); }}
              min="1"
              max="50"
            />
            <span class="cap-suffix">x</span>
          </div>
        </div>
      </div>

      <div class="info-note">
        <strong>Note:</strong> When multiple campaigns overlap on the same rule, the campaign with the higher multiplier automatically wins. This is a platform-level safety rule and cannot be changed.
      </div>
    </div>

    <div class="modal-footer">
      <button class="btn-cancel" onclick={onCancel}>Cancel</button>
      <button class="btn-save" onclick={handleSave}>Save Settings</button>
    </div>
  </div>
</div>

<style>
  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: flex-start;
    justify-content: center;
    padding-top: var(--space-12);
    z-index: var(--z-modal);
    overflow-y: auto;
  }

  .modal-panel {
    background: var(--color-bg);
    border-radius: var(--radius-lg);
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.12);
    width: 560px;
    max-width: 100%;
    overflow: hidden;
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--space-4) var(--space-5);
    border-bottom: 1px solid var(--color-border);
  }

  .modal-title {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
  }

  .modal-close {
    background: none;
    border: none;
    font-size: 20px;
    color: var(--color-text-muted);
    cursor: pointer;
    padding: 2px 6px;
    border-radius: var(--radius-sm);
  }

  .modal-body {
    padding: var(--space-5);
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: var(--space-3);
    padding: var(--space-3) var(--space-5);
    border-top: 1px solid var(--color-border);
  }

  .config-section {
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: var(--space-4);
    background: white;
  }

  .config-title {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
    margin-bottom: var(--space-3);
  }

  .config-desc {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    margin-bottom: var(--space-4);
  }

  .stacking-options {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .stacking-option {
    display: flex;
    align-items: flex-start;
    gap: var(--space-3);
    padding: var(--space-3);
    border: 2px solid var(--color-border);
    border-radius: var(--radius-lg);
    cursor: pointer;
    transition: all 0.15s;
    background: none;
    text-align: left;
    width: 100%;
    font-family: inherit;
  }

  .stacking-option:hover { border-color: #c7d2fe; }
  .stacking-option.selected { border-color: #6366f1; background: #f5f3ff; }

  .stacking-radio {
    width: 18px;
    height: 18px;
    border-radius: 50%;
    border: 2px solid #d1d5db;
    margin-top: 2px;
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .stacking-radio.selected { border-color: #6366f1; }
  .stacking-radio.selected::after {
    content: '';
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #6366f1;
  }

  .stacking-info { flex: 1; }

  .stacking-name {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .recommended-tag {
    font-weight: var(--font-weight-normal);
    color: var(--color-text-muted);
  }

  .stacking-desc {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    margin-top: 2px;
    line-height: 1.4;
  }

  .stacking-example {
    font-size: 10px;
    color: var(--color-text-muted);
    margin-top: var(--space-1);
    font-family: var(--font-mono);
    background: var(--color-surface);
    padding: 4px 8px;
    border-radius: var(--radius-sm);
    display: inline-block;
  }

  .config-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-3) 0;
  }

  .config-label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-text);
  }

  .config-sublabel {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    margin-top: 1px;
  }

  .cap-input-row {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .cap-input {
    width: 80px;
    padding: var(--space-2) var(--space-3);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
    text-align: center;
    font-family: var(--font-mono);
  }

  .cap-suffix {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .info-note {
    padding: var(--space-3);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    line-height: 1.5;
  }

  .info-note strong { color: var(--color-text); }

  .btn-cancel {
    padding: var(--space-2) var(--space-5);
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-text-muted);
    background: none;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    cursor: pointer;
    font-family: inherit;
  }

  .btn-cancel:hover { color: var(--color-text); border-color: var(--color-text-muted); }

  .btn-save {
    padding: var(--space-2) var(--space-5);
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: white;
    background: var(--color-primary);
    border: none;
    border-radius: var(--radius-md);
    cursor: pointer;
    font-family: inherit;
  }

  .btn-save:hover { background: var(--color-primary-hover); }
</style>
