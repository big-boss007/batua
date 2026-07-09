<script lang="ts">
  import type { FestiveTemplate } from '../types';

  let {
    templates,
    onSelect
  }: {
    templates: Array<FestiveTemplate>;
    onSelect: (template: FestiveTemplate) => void;
  } = $props();

  const CATEGORY_COLORS: Record<string, string> = {
    hindu: 'var(--color-warning)',
    national: 'var(--color-info)',
    seasonal: 'var(--color-success)',
    commercial: 'var(--color-primary)'
  };

  function getCategoryColor(category: string): string {
    return CATEGORY_COLORS[category] ?? 'var(--color-text-muted)';
  }
</script>

<div class="template-grid">
  {#if templates.length === 0}
    <div class="empty-state">
      <p class="empty-title">No templates available</p>
    </div>
  {:else}
    <div class="grid">
      {#each templates as template (template.name)}
        <button class="template-card" onclick={() => onSelect(template)}>
          <div class="card-accent" style="background: {getCategoryColor(template.category)}"></div>
          <div class="card-body">
            <div class="card-header">
              <h3 class="template-name">{template.display_name}</h3>
              <span
                class="category-tag"
                style="color: {getCategoryColor(
                  template.category
                )}; background: color-mix(in srgb, {getCategoryColor(
                  template.category
                )} 12%, transparent)"
              >
                {template.category}
              </span>
            </div>
            <p class="template-description">{template.description}</p>
            <div class="template-meta">
              <div class="meta-item">
                <span class="meta-label">Multiplier</span>
                <span class="meta-value">{template.default_multiplier}x</span>
              </div>
              <div class="meta-item">
                <span class="meta-label">Duration</span>
                <span class="meta-value">{template.default_duration_days} days</span>
              </div>
            </div>
          </div>
        </button>
      {/each}
    </div>
  {/if}
</div>

<style>
  .template-grid {
    width: 100%;
  }

  .empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--space-8);
  }

  .empty-title {
    font-size: var(--font-size-base);
    color: var(--color-text-muted);
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: var(--space-4);
  }

  .template-card {
    display: flex;
    flex-direction: column;
    text-align: left;
    background: var(--color-surface);
    border-radius: var(--radius-lg);
    overflow: hidden;
    transition: all var(--transition-fast);
    padding: 0;
    box-shadow: var(--shadow-card);
  }

  .template-card:hover {
    border-color: var(--color-primary);
    box-shadow: var(--shadow-md);
    transform: translateY(-2px);
  }

  .card-accent {
    height: 4px;
    width: 100%;
  }

  .card-body {
    padding: var(--space-4);
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-2);
  }

  .template-name {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .category-tag {
    padding: var(--space-1) var(--space-2);
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-medium);
    border-radius: var(--radius-sm);
    white-space: nowrap;
  }

  .template-description {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    line-height: var(--line-height-base);
  }

  .template-meta {
    display: flex;
    gap: var(--space-4);
    padding-top: var(--space-3);
    border-top: 1px solid var(--color-border);
  }

  .meta-item {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .meta-label {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .meta-value {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }
</style>
