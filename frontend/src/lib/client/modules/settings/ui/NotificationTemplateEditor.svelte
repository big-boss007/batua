<script lang="ts">
  import type { NotificationTemplate, UpdateTemplateRequest } from '$lib/client/modules/settings';

  let {
    template,
    onSave
  }: {
    template: NotificationTemplate;
    onSave: (templateId: string, data: UpdateTemplateRequest) => void;
  } = $props();

  let bodyTemplate = $state(template.body_template);
  let isActive = $state(template.is_active);

  let variablePattern = /\{\{(\w+)\}\}/g;

  let variables = $derived(() => {
    const matches = bodyTemplate.matchAll(variablePattern);
    const unique = new Set<string>();
    for (const match of matches) {
      unique.add(match[1]);
    }
    return Array.from(unique);
  });

  let previewSegments = $derived(() => {
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
        <span class="tag">{template.channel}</span>
        <span class="tag">{template.locale}</span>
      </div>
    </div>
    <label class="toggle-field">
      <input type="checkbox" class="toggle-input" bind:checked={isActive} />
      <span class="toggle-label">{isActive ? 'Active' : 'Inactive'}</span>
    </label>
  </div>

  <form class="editor-body" onsubmit={handleSubmit}>
    <div class="form-field">
      <label class="field-label" for="template-body-{template.id}">Template Body</label>
      <textarea
        id="template-body-{template.id}"
        class="field-textarea"
        bind:value={bodyTemplate}
        rows="8"
        spellcheck="false"
      ></textarea>
    </div>

    {#if variables().length > 0}
      <div class="variables-section">
        <span class="variables-label">Variables:</span>
        <div class="variables-list">
          {#each variables() as variable (variable)}
            <span class="variable-chip">{`{{${variable}}}`}</span>
          {/each}
        </div>
      </div>
    {/if}

    <div class="preview-section">
      <span class="preview-label">Preview</span>
      <div class="preview-body">
        {#each previewSegments() as segment (segment.text)}
          {#if segment.isVariable}
            <span class="preview-variable">{segment.text}</span>
          {:else}
            <span>{segment.text}</span>
          {/if}
        {/each}
      </div>
    </div>

    <button type="submit" class="save-button">Save Template</button>
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

  .tag {
    padding: var(--space-1) var(--space-2);
    background: var(--color-surface-2);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    font-weight: var(--font-weight-medium);
    text-transform: uppercase;
  }

  .toggle-field {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    cursor: pointer;
  }

  .toggle-input {
    width: 16px;
    height: 16px;
    accent-color: var(--color-primary);
    cursor: pointer;
  }

  .toggle-label {
    font-size: var(--font-size-sm);
    color: var(--color-text);
    font-weight: var(--font-weight-medium);
  }

  .editor-body {
    padding: var(--space-6);
    display: flex;
    flex-direction: column;
    gap: var(--space-5);
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

  .field-textarea {
    padding: var(--space-3);
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    color: var(--color-text);
    font-family: var(--font-mono);
    font-size: var(--font-size-sm);
    resize: vertical;
    line-height: var(--line-height-loose);
    transition: border-color var(--transition-fast);
  }

  .field-textarea:focus {
    outline: none;
    border-color: var(--color-primary);
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

  .variable-chip {
    padding: var(--space-1) var(--space-2);
    background: color-mix(in srgb, var(--color-primary) 10%, transparent);
    color: var(--color-primary);
    border-radius: var(--radius-sm);
    font-family: var(--font-mono);
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-medium);
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

  .save-button {
    align-self: flex-start;
    padding: var(--space-2) var(--space-5);
    background: var(--color-primary);
    color: #ffffff;
    border: none;
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    transition: background var(--transition-fast);
  }

  .save-button:hover {
    background: var(--color-primary-hover);
  }
</style>
