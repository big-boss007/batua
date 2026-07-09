<script lang="ts">
  import { Input, Select, Button } from '@juspay/svelte-ui-components';

  import type { Connector, CreateConnectorRequest } from '$lib/client/modules/settings';

  let {
    connector = null,
    onSave
  }: {
    connector?: Connector | null;
    onSave: (data: CreateConnectorRequest) => void;
  } = $props();

  // svelte-ignore state_referenced_locally
  let capability = $state(connector?.capability ?? '');
  // svelte-ignore state_referenced_locally
  let vendor = $state(connector?.vendor ?? '');
  // svelte-ignore state_referenced_locally
  let configJson = $state(connector ? JSON.stringify(connector.config, null, 2) : '{}');
  // svelte-ignore state_referenced_locally
  let priority = $state(String(connector?.priority ?? 1));
  let configError = $state<string | null>(null);

  const capabilityItems = [
    { id: 'payment_gateway', label: 'Payment gateway' },
    { id: 'sms', label: 'SMS' },
    { id: 'email', label: 'Email' },
    { id: 'webhook', label: 'Webhook' },
    { id: 'shipping', label: 'Shipping' },
    { id: 'analytics', label: 'Analytics' }
  ];

  function handleConfigInput(val: string) {
    configJson = val;
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
      priority: Number(priority)
    });
  }
</script>

<form class="connector-form" onsubmit={handleSubmit}>
  <h4 class="form-title">{connector ? 'Edit connector' : 'New connector'}</h4>

  <div class="form-grid">
    <div class="form-field">
      <span class="field-label">Capability</span>
      <Select
        items={capabilityItems}
        value={capability ? [capability] : []}
        placeholder="Select capability"
        onchange={(val) => {
          capability = val[0] ?? '';
        }}
      />
    </div>

    <Input
      value={vendor}
      label="Vendor"
      placeholder="e.g. razorpay"
      onInput={(val) => {
        vendor = val;
      }}
    />

    <Input
      value={priority}
      label="Priority"
      dataType="number"
      onInput={(val) => {
        priority = val;
      }}
    />
  </div>

  <div class="config-field">
    <Input
      value={configJson}
      label="Configuration (JSON)"
      useTextArea
      onInput={(val) => {
        handleConfigInput(val);
      }}
      onErrorMessage={configError}
      classes="input-mono"
    />
  </div>

  <Button
    text={connector ? 'Update connector' : 'Create connector'}
    classes="btn-primary"
    disabled={capability === '' || vendor === '' || configError !== null}
    type="submit"
  />
</form>

<style>
  .connector-form {
    padding: var(--space-6);
    background: var(--color-surface);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-card);
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

  .config-field {
    margin-bottom: var(--space-5);
  }

  .form-field {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .field-label {
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-medium);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  :global(.input-mono) {
    --input-font-family: var(--font-mono);
    --input-font-size: var(--font-size-sm);
  }
</style>
