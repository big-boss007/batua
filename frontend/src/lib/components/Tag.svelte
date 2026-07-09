<script lang="ts">
  import type { Snippet } from 'svelte';

  let {
    label = null,
    tone = 'neutral',
    selected = false,
    onClick = null,
    onRemove = null,
    children
  }: {
    label?: string | null;
    tone?: 'neutral' | 'info' | 'success' | 'warning' | 'critical';
    selected?: boolean;
    onClick?: (() => void) | null;
    onRemove?: (() => void) | null;
    children?: Snippet;
  } = $props();
</script>

{#snippet content()}
  {#if children}{@render children()}{:else}{label}{/if}
{/snippet}

{#if onRemove !== null}
  <!-- Removable filter chip: label + trailing × -->
  <span class="tag tag-{tone}" class:selected>
    <span class="tag-label">{@render content()}</span>
    <button type="button" class="tag-x" aria-label="Remove" onclick={onRemove}>&times;</button>
  </span>
{:else if onClick !== null}
  <!-- Selectable chip -->
  <button type="button" class="tag tag-{tone}" class:selected onclick={onClick}>
    <span class="tag-label">{@render content()}</span>
  </button>
{:else}
  <span class="tag tag-{tone}" class:selected>
    <span class="tag-label">{@render content()}</span>
  </span>
{/if}

<style>
  /* Tag — the interactive chip. Squared (radius 8), distinct from the fully
     round status Badge. Tonal fill when selected. */
  .tag {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 3px 8px 3px 10px;
    border-radius: var(--radius-md);
    border: 1px solid var(--color-border);
    background: var(--color-surface);
    color: var(--color-text);
    font-family: var(--font-sans);
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-medium);
    line-height: 1.5;
    cursor: default;
    transition:
      background var(--transition-fast),
      border-color var(--transition-fast),
      color var(--transition-fast);
  }
  button.tag {
    cursor: pointer;
    padding: 3px 10px;
  }
  button.tag:hover {
    background: var(--color-surface-2);
  }
  .tag-x {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 16px;
    height: 16px;
    border: none;
    background: transparent;
    color: var(--color-text-muted);
    font-size: 15px;
    line-height: 1;
    border-radius: var(--radius-sm);
    cursor: pointer;
    padding: 0;
  }
  .tag-x:hover {
    background: var(--color-surface-2);
    color: var(--color-text);
  }

  /* Selected / toned states — squared tonal fill */
  .tag.selected.tag-neutral {
    background: var(--color-surface-2);
    border-color: var(--color-border);
  }
  .tag.selected.tag-info {
    background: var(--p-100);
    border-color: var(--p-300);
    color: var(--p-700);
  }
  .tag.selected.tag-success {
    background: var(--green-100);
    border-color: var(--green-500);
    color: var(--green-700);
  }
  .tag.selected.tag-warning {
    background: var(--yellow-100);
    border-color: var(--yellow-500);
    color: var(--yellow-700);
  }
  .tag.selected.tag-critical {
    background: var(--red-100);
    border-color: var(--red-500);
    color: var(--red-700);
  }
</style>
