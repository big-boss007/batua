<script lang="ts">
  import { page } from '$app/stores';

  type NavItem = {
    label: string;
    href: string;
    icon: string;
  };

  let { collapsed, ontoggle }: { collapsed: boolean; ontoggle: () => void } = $props();

  const navItems: NavItem[] = [
    { label: 'Dashboard', href: '/platform', icon: '\u25A6' },
    { label: 'Merchants', href: '/platform/merchants', icon: '\u2616' },
    { label: 'Geo Policies', href: '/platform/geo-policies', icon: '\u2637' },
    { label: 'Defaults', href: '/platform/defaults', icon: '\u2699' },
    { label: 'System', href: '/platform/system', icon: '\u2638' }
  ];

  let currentPath = $derived($page.url.pathname);

  function isActive(href: string): boolean {
    if (href === '/platform') {
      return currentPath === '/platform';
    }
    return currentPath.startsWith(href);
  }
</script>

<aside class="sidebar" class:collapsed>
  <div class="sidebar-header">
    {#if !collapsed}
      <span class="logo-text">Breeze</span>
    {:else}
      <span class="logo-icon">B</span>
    {/if}
    <button class="toggle-btn" onclick={ontoggle} aria-label="Toggle sidebar">
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
    </button>
  </div>

  {#if !collapsed}
    <div class="sidebar-badge">Platform</div>
  {/if}

  <nav class="sidebar-nav">
    {#each navItems as item (item.href)}
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
    {/each}
  </nav>
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

  .sidebar-badge {
    padding: var(--space-1) var(--space-4);
    margin: var(--space-2) var(--space-3);
    background: var(--color-primary);
    color: #ffffff;
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-semibold);
    border-radius: var(--radius-sm);
    text-align: center;
    text-transform: uppercase;
    letter-spacing: 0.06em;
  }

  .toggle-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border: none;
    background: transparent;
    color: var(--color-text-muted);
    border-radius: var(--radius-sm);
    transition:
      background var(--transition-fast),
      color var(--transition-fast);
    flex-shrink: 0;
  }

  .toggle-btn:hover {
    background: var(--color-surface-2);
    color: var(--color-text);
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

  .sidebar.collapsed .toggle-btn {
    margin: 0 auto;
  }

  .sidebar.collapsed .nav-item {
    justify-content: center;
    padding: var(--space-2);
  }
</style>
