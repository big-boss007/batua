<script lang="ts">
  import { Input } from '@juspay/svelte-ui-components';

  let {
    onsubmit,
    loading = false
  }: {
    onsubmit: (data: { geo_code: string; name: string; config: Record<string, unknown> }) => void;
    loading?: boolean;
  } = $props();

  let geoCode = $state('');
  let name = $state('');
  let configJson = $state(
    '{\n  "cod_enabled": false,\n  "default_currency": "INR",\n  "whatsapp_default": false,\n  "upi_topup_enabled": false,\n  "default_timezone": "Asia/Kolkata"\n}'
  );
  let configError = $state<string | null>(null);

  function handleGeoCodeInput(value: string) {
    geoCode = value.toLowerCase().replace(/[^a-z0-9_-]/g, '');
  }

  function handleNameInput(value: string) {
    name = value;
  }

  function handleConfigInput(value: string) {
    configJson = value;
    try {
      JSON.parse(value);
      configError = null;
    } catch {
      configError = 'Invalid JSON';
    }
  }

  function handleSubmit() {
    if (geoCode.trim() === '' || name.trim() === '') return;
    try {
      const config = JSON.parse(configJson) as Record<string, unknown>;
      configError = null;
      onsubmit({
        geo_code: geoCode.trim(),
        name: name.trim(),
        config
      });
    } catch {
      configError = 'Invalid JSON';
    }
  }
</script>

<form
  class="geo-form"
  onsubmit={(e) => {
    e.preventDefault();
    handleSubmit();
  }}
>
  <div class="form-field">
    <Input value={geoCode} label="Geo code" placeholder="india" onInput={handleGeoCodeInput} />
  </div>

  <div class="form-field">
    <Input value={name} label="Policy name" placeholder="India default" onInput={handleNameInput} />
  </div>

  <div class="form-field">
    <label class="textarea-label" for="geo-config-json">Configuration (JSON)</label>
    <textarea
      id="geo-config-json"
      class="config-textarea"
      class:has-error={configError !== null}
      value={configJson}
      oninput={(e) => handleConfigInput(e.currentTarget.value)}
      rows="8"
      spellcheck="false"
    ></textarea>
    {#if configError !== null}
      <span class="error-msg">{configError}</span>
    {/if}
  </div>
</form>

<style>
  .geo-form {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    padding: var(--space-4);
  }

  .form-field {
    --input-width: 100%;
    --input-margin: 0;
  }

  .textarea-label {
    display: block;
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-normal);
    color: var(--color-text-muted);
    margin-bottom: var(--space-2);
  }

  .config-textarea {
    width: 100%;
    padding: var(--space-3) var(--space-4);
    background: var(--color-surface);
    border-radius: var(--radius-md);
    color: var(--color-text);
    font-family: var(--font-mono);
    font-size: var(--font-size-sm);
    resize: vertical;
    line-height: var(--line-height-base);
    border: 1px solid var(--color-border);
  }

  .config-textarea:focus {
    outline: none;
    border-color: var(--color-primary);
  }

  .config-textarea.has-error {
    border-color: var(--color-error);
  }

  .error-msg {
    font-size: var(--font-size-xs);
    color: var(--color-error);
    margin-top: var(--space-1);
  }
</style>
