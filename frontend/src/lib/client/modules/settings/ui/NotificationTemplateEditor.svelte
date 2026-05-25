<script lang="ts">
  import { Input, Button, Pill, Toggle } from '@juspay/svelte-ui-components';

  import type { NotificationTemplate, UpdateTemplateRequest } from '$lib/client/modules/settings';

  let {
    template,
    onSave
  }: {
    template: NotificationTemplate;
    onSave: (templateId: string, data: UpdateTemplateRequest) => void;
  } = $props();

  // svelte-ignore state_referenced_locally
  let bodyTemplate = $state(template.body_template);
  // svelte-ignore state_referenced_locally
  let isActive = $state(template.is_active);

  let variablePattern = /\{\{(\w+)\}\}/g;

  let variables = $derived.by(() => {
    const matches = bodyTemplate.matchAll(variablePattern);
    const unique = new Set<string>();
    for (const match of matches) {
      unique.add(match[1]);
    }
    return Array.from(unique);
  });

  let previewSegments = $derived.by(() => {
    const parts: Array<{ text: string; isVariable: boolean }> = [];
    let lastIndex = 0;
    const regex = /\{\{(\w+)\}\}/g;
    let match = regex.exec(bodyTemplate);

    while (match !== null) {
      if (match.index > lastIndex) {
        parts.push({ text: bodyTemplate.slice(lastIndex, match.index), isVariable: false });
      }
      parts.push({ text: match[0], isVariable: true });
      lastIndex = regex.lastIndex;
      match = regex.exec(bodyTemplate);
    }

    if (lastIndex < bodyTemplate.length) {
      parts.push({ text: bodyTemplate.slice(lastIndex), isVariable: false });
    }

    return parts;
  });

  function handleSubmit() {
    onSave(template.id, {
      body_template: bodyTemplate,
      is_active: isActive
    });
  }
</script>

<div class="template-editor">
  <div class="editor-header">
    <div class="template-meta">
      <h4 class="template-name">{template.name}</h4>
      <div class="meta-tags">
        <Pill text={template.channel} classes="pill-neutral" />
        <Pill text={template.locale} classes="pill-neutral" />
      </div>
    </div>
    <Toggle
      text={isActive ? 'Active' : 'Inactive'}
      checked={isActive}
      onclick={(checked) => {
        isActive = checked;
      }}
    />
  </div>

  <form class="editor-body" onsubmit={handleSubmit}>
    <Input
      value={bodyTemplate}
      label="Template Body"
      useTextArea
      onInput={(val) => {
        bodyTemplate = val;
      }}
      classes="input-mono"
    />

    {#if variables.length > 0}
      <div class="variables-section">
        <span class="variables-label">Variables:</span>
        <div class="variables-list">
          {#each variables as variable (variable)}
            <Pill text={`{{${variable}}}`} classes="pill-info" />
          {/each}
        </div>
      </div>
    {/if}

    <div class="preview-section">
      <span class="preview-label">Preview</span>
      <div class="preview-body">
        {#each previewSegments as segment (segment.text)}
          {#if segment.isVariable}
            <span class="preview-variable">{segment.text}</span>
          {:else}
            <span>{segment.text}</span>
          {/if}
        {/each}
      </div>
    </div>

    <Button text="Save Template" classes="btn-primary" type="submit" />
  </form>
</div>

<style>
  .template-editor {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    overflow: hidden;
  }

  .editor-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--space-5) var(--space-6);
    border-bottom: 1px solid var(--color-border);
  }

  .template-meta {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .template-name {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .meta-tags {
    display: flex;
    gap: var(--space-2);
  }

  .editor-body {
    padding: var(--space-6);
    display: flex;
    flex-direction: column;
    gap: var(--space-5);
  }

  .variables-section {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    flex-wrap: wrap;
  }

  .variables-label {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    font-weight: var(--font-weight-medium);
    text-transform: uppercase;
  }

  .variables-list {
    display: flex;
    gap: var(--space-2);
    flex-wrap: wrap;
  }

  .preview-section {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .preview-label {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    font-weight: var(--font-weight-medium);
    text-transform: uppercase;
  }

  .preview-body {
    padding: var(--space-4);
    background: var(--color-surface-2);
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
    color: var(--color-text);
    line-height: var(--line-height-loose);
    white-space: pre-wrap;
    word-break: break-word;
  }

  .preview-variable {
    padding: 1px var(--space-1);
    background: color-mix(in srgb, var(--color-primary) 15%, transparent);
    color: var(--color-primary);
    border-radius: 2px;
    font-family: var(--font-mono);
    font-weight: var(--font-weight-medium);
  }

  :global(.input-mono) {
    --input-font-family: var(--font-mono);
    --input-font-size: var(--font-size-sm);
  }

</style>
