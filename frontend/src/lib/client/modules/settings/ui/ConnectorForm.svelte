<script lang="ts">
  import type { Connector, CreateConnectorRequest } from '$lib/client/modules/settings';

  let {
    connector = null,
    onSave
  }: {
    connector?: Connector | null;
    onSave: (data: CreateConnectorRequest) => void;
  } = $props();

  let capability = $state(connector?.capability ?? '');
  let vendor = $state(connector?.vendor ?? '');
  let configJson = $state(connector ? JSON.stringify(connector.config, null, 2) : '{}');
  let priority = $state(connector?.priority ?? 1);
  let configError = $state<string | null>(null);

  let capabilities = $derived([
    'payment_gateway',
    'sms',
    'email',
    'webhook',
    'shipping',
    'analytics'
  ]);

  function handleConfigInput(event: Event) {
    const target = event.target as HTMLTextAreaElement;
    configJson = target.value;
    try {
      JSON.parse(configJson);
      configError = null;
    } catch {
      configError = 'Invalid JSON';
    }
  }

  function handleSubmit() {
    let parsedConfig: Record<string, unknown>;
    try {
      parsedConfig = JSON.parse(configJson) as Record<string, unknown>;
      configError = null;
    } catch {
      configError = 'Invalid JSON';
      return;
    }

    onSave({
      capability,
      vendor,
      config: parsedConfig,
      priority
    });
  }
</script>

<form class="connector-form" onsubmit={handleSubmit}>
  <h4 class="form-title">{connector ? 'Edit Connector' : 'New Connector'}</h4>

  <div class="form-grid">
    <div class="form-field">
      <label class="field-label" for="connector-capability">Capability</label>
      <select id="connector-capability" class="field-select" bind:value={capability}>
        <option value="" disabled>Select capability</option>
        {#each capabilities as cap (cap)}
          <option value={cap}>{cap.replace(/_/g, ' ')}</option>
        {/each}
      </select>
    </div>

    <div class="form-field">
      <label class="field-label" for="connector-vendor">Vendor</label>
      <input
        id="connector-vendor"
        class="field-input"
        type="text"
        bind:value={vendor}
        placeholder="e.g. razorpay"
      />
    </div>

    <div class="form-field">
      <label class="field-label" for="connector-priority">Priority</label>
      <input
        id="connector-priority"
        class="field-input"
        type="number"
        min="1"
        bind:value={priority}
      />
    </div>
  </div>

  <div class="form-field config-field">
    <label class="field-label" for="connector-config">Configuration (JSON)</label>
    <textarea
      id="connector-config"
      class="field-textarea"
      class:field-error={configError !== null}
      value={configJson}
      oninput={handleConfigInput}
      rows="6"
      spellcheck="false"
    ></textarea>
    {#if configError !== null}
      <span class="error-text">{configError}</span>
    {/if}
  </div>

  <button
    type="submit"
    class="save-button"
    disabled={capability === '' || vendor === '' || configError !== null}
  >
    {connector ? 'Update Connector' : 'Create Connector'}
  </button>
</form>

<style>
  .connector-form {
    padding: var(--space-6);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
  }

  .form-title {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
    margin-bottom: var(--space-5);
  }

  .form-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: var(--space-4);
    margin-bottom: var(--space-4);
  }

  .form-field {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .config-field {
    margin-bottom: var(--space-5);
  }

  .field-label {
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-medium);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .field-input,
  .field-select {
    padding: var(--space-2) var(--space-3);
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    color: var(--color-text);
    font-size: var(--font-size-sm);
    transition: border-color var(--transition-fast);
  }

  .field-input:focus,
  .field-select:focus,
  .field-textarea:focus {
    outline: none;
    border-color: var(--color-primary);
  }

  .field-textarea {
    padding: var(--space-3);
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    color: var(--color-text);
    font-family: var(--font-mono);
    font-size: var(--font-size-sm);
    resize: vertical;
    transition: border-color var(--transition-fast);
  }

  .field-error {
    border-color: var(--color-error);
  }

  .error-text {
    font-size: var(--font-size-xs);
    color: var(--color-error);
  }

  .save-button {
    padding: var(--space-2) var(--space-5);
    background: var(--color-primary);
    color: #ffffff;
    border: none;
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    transition: background var(--transition-fast);
  }

  .save-button:hover:not(:disabled) {
    background: var(--color-primary-hover);
  }

  .save-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
