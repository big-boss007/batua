<script lang="ts">
  import type { CampaignStackingConfig } from '../types';
  import { Modal, Input } from '@juspay/svelte-ui-components';
  import { MODAL_CLOSE_ICON } from '$lib/client/modules/foundation';

  let {
    config,
    onSave,
    onCancel
  }: {
    config: CampaignStackingConfig;
    onSave: (config: CampaignStackingConfig) => void;
    onCancel: () => void;
  } = $props();

  // svelte-ignore state_referenced_locally
  let stackingMode = $state(config.campaign_stacking_mode);
  // svelte-ignore state_referenced_locally
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

<Modal
  size="medium"
  header={{ text: 'Campaign Settings', rightImage: MODAL_CLOSE_ICON }}
  onclose={onCancel}
  onoverlayClick={onCancel}
  onheaderRightImageClick={onCancel}
  footer={{
    primaryButton: { text: 'Save Settings' },
    secondaryButton: { text: 'Cancel' }
  }}
  onprimaryButtonClick={handleSave}
  onsecondaryButtonClick={onCancel}
>
  {#snippet content()}
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
            <Input
              value={String(maxMultiplier)}
              onInput={(val) => { maxMultiplier = Number(val) || 1; }}
              classes="cap-input"
            />
            <span class="cap-suffix">x</span>
          </div>
        </div>
      </div>

      <div class="info-note">
        <strong>Note:</strong> When multiple campaigns overlap on the same rule, the campaign with the higher multiplier automatically wins. This is a platform-level safety rule and cannot be changed.
      </div>
    </div>
  {/snippet}
</Modal>

<style>
  .modal-body {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
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

</style>
