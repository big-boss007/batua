<script lang="ts">
  import { page } from '$app/stores';
  import { Button } from '@juspay/svelte-ui-components';
  import type { NavItem } from '$lib/client/modules/admin';

  let {
    items,
    collapsed,
    ontoggle
  }: {
    items: NavItem[];
    collapsed: boolean;
    ontoggle: () => void;
  } = $props();

  let currentPath = $derived($page.url.pathname);
  let expandedParent = $state<string | null>(null);

  function isActive(href: string): boolean {
    if (href === '/admin') {
      return currentPath === '/admin';
    }
    return currentPath.startsWith(href);
  }

  function isParentActive(item: NavItem): boolean {
    if (item.children === undefined) return false;
    return isActive(item.href) || item.children.some((c) => isActive(c.href));
  }

  function isExpanded(item: NavItem): boolean {
    if (expandedParent === item.href) return true;
    if (expandedParent === null && isParentActive(item)) return true;
    return false;
  }

  function toggleParent(item: NavItem) {
    if (isExpanded(item)) {
      expandedParent = '__none__';
    } else {
      expandedParent = item.href;
    }
  }
</script>

<aside class="sidebar" class:collapsed>
  <div class="sidebar-header">
    {#if !collapsed}
      <span class="logo-text">Batua</span>
    {:else}
      <span class="logo-icon">B</span>
    {/if}
    <Button classes="toggle-btn" onclick={ontoggle}>
      {#snippet icon()}
        <svg
          width="18"
          height="18"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          {#if collapsed}
            <polyline points="9 18 15 12 9 6"></polyline>
          {:else}
            <polyline points="15 18 9 12 15 6"></polyline>
          {/if}
        </svg>
      {/snippet}
    </Button>
  </div>

  <nav class="sidebar-nav">
    {#each items as item}
      {#if item.children !== undefined && item.children.length > 0}
        <button
          class="nav-item nav-parent"
          class:parent-active={isParentActive(item)}
          class:open={isExpanded(item)}
          onclick={() => toggleParent(item)}
          title={collapsed ? item.label : null}
        >
          <span class="nav-icon">{item.icon}</span>
          {#if !collapsed}
            <span class="nav-label">{item.label}</span>
            <span class="nav-chevron">{isExpanded(item) ? '▾' : '▸'}</span>
          {/if}
        </button>
        {#if isExpanded(item) && !collapsed}
          <div class="nav-children">
            {#each item.children as child}
              <a
                href={child.href}
                class="nav-child"
                class:active={isActive(child.href)}
                title={collapsed ? child.label : null}
              >
                <span class="nav-child-dot"></span>
                <span class="nav-label">{child.label}</span>
              </a>
            {/each}
          </div>
        {/if}
      {:else}
        <a
          href={item.href}
          class="nav-item"
          class:active={isActive(item.href)}
          title={collapsed ? item.label : null}
        >
          <span class="nav-icon">{item.icon}</span>
          {#if !collapsed}
            <span class="nav-label">{item.label}</span>
          {/if}
        </a>
      {/if}
    {/each}
  </nav>

  <div class="sidebar-footer">
    <a
      href="/platform/merchants"
      class="switch-merchant"
      title={collapsed ? 'Switch Merchant' : null}
    >
      {#if collapsed}
        <span class="nav-icon">&rArr;</span>
      {:else}
        Switch Merchant &rarr;
      {/if}
    </a>
  </div>
</aside>

<style>
  .sidebar {
    display: flex;
    flex-direction: column;
    width: 240px;
    height: 100vh;
    background: var(--color-surface);
    border-right: 1px solid var(--color-border);
    transition: width var(--transition-base);
    position: sticky;
    top: 0;
    flex-shrink: 0;
    overflow-y: auto;
    overflow-x: hidden;
    z-index: var(--z-sticky);
  }

  .sidebar.collapsed {
    width: 60px;
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-4);
    border-bottom: 1px solid var(--color-border);
    min-height: 56px;
  }

  .logo-text {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-bold);
    color: var(--color-primary);
    white-space: nowrap;
    overflow: hidden;
  }

  .logo-icon {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-bold);
    color: var(--color-primary);
  }

  :global(.toggle-btn) {
    --button-color: transparent;
    --button-text-color: var(--color-text-muted);
    --button-border-radius: var(--radius-sm);
    --button-padding: 0;
    --button-hover-color: var(--color-surface-2);
    --button-hover-text-color: var(--color-text);
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    flex-shrink: 0;
  }

  .sidebar-nav {
    display: flex;
    flex-direction: column;
    padding: var(--space-2);
    gap: var(--space-1);
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-2) var(--space-3);
    border-radius: var(--radius-md);
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    text-decoration: none;
    white-space: nowrap;
    transition:
      background var(--transition-fast),
      color var(--transition-fast);
  }

  .nav-item:hover {
    background: var(--color-surface-2);
    color: var(--color-text);
  }

  .nav-item.active {
    background: var(--color-primary);
    color: #ffffff;
  }

  /* Parent nav item */
  .nav-parent {
    border: none;
    width: 100%;
    text-align: left;
    cursor: pointer;
    background: none;
  }

  .nav-parent.parent-active {
    color: var(--color-primary);
    font-weight: var(--font-weight-semibold);
  }

  .nav-parent.open {
    color: var(--color-primary);
  }

  .nav-chevron {
    margin-left: auto;
    font-size: 10px;
    color: var(--color-text-muted);
  }

  .nav-parent.parent-active .nav-chevron,
  .nav-parent.open .nav-chevron {
    color: var(--color-primary);
  }

  /* Children container */
  .nav-children {
    display: flex;
    flex-direction: column;
    gap: 0;
    padding-left: var(--space-2);
  }

  .nav-child {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: 6px var(--space-3) 6px 36px;
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-medium);
    color: var(--color-text-muted);
    text-decoration: none;
    border-radius: var(--radius-md);
    position: relative;
    transition:
      background var(--transition-fast),
      color var(--transition-fast);
  }

  .nav-child::before {
    content: '';
    position: absolute;
    left: 22px;
    top: 0;
    bottom: 0;
    width: 1px;
    background: var(--color-border);
  }

  .nav-child:hover {
    background: var(--color-surface-2);
    color: var(--color-text);
  }

  .nav-child.active {
    color: var(--color-primary);
    font-weight: var(--font-weight-semibold);
    background: color-mix(in srgb, var(--color-primary) 8%, transparent);
  }

  .nav-child.active::before {
    background: var(--color-primary);
    width: 2px;
  }

  .nav-child-dot {
    width: 5px;
    height: 5px;
    border-radius: 50%;
    background: var(--color-border);
    flex-shrink: 0;
  }

  .nav-child.active .nav-child-dot {
    background: var(--color-primary);
  }

  .nav-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
    font-size: var(--font-size-md);
    flex-shrink: 0;
  }

  .nav-label {
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .sidebar.collapsed .sidebar-header {
    justify-content: center;
    padding: var(--space-4) var(--space-2);
  }

  .sidebar.collapsed :global(.toggle-btn) {
    margin: 0 auto;
  }

  .sidebar.collapsed .nav-item {
    justify-content: center;
    padding: var(--space-2);
  }

  .sidebar-footer {
    margin-top: auto;
    padding: var(--space-3);
    border-top: 1px solid var(--color-border);
  }

  .switch-merchant {
    display: block;
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    text-decoration: none;
    text-align: center;
    padding: var(--space-2);
    border-radius: var(--radius-sm);
    transition:
      color var(--transition-fast),
      background var(--transition-fast);
  }

  .switch-merchant:hover {
    color: var(--color-primary);
    background: var(--color-surface-2);
  }

  @media (max-width: 768px) {
    .sidebar {
      width: 60px;
    }

    .sidebar .nav-label,
    .sidebar .logo-text {
      display: none;
    }

    .sidebar .logo-icon {
      display: inline;
    }

    .sidebar .sidebar-header {
      justify-content: center;
      padding: var(--space-4) var(--space-2);
    }

    .sidebar :global(.toggle-btn) {
      display: none;
    }

    .sidebar .nav-item {
      justify-content: center;
      padding: var(--space-2);
    }
  }
</style>
