<script lang="ts">
  import type { Snippet } from 'svelte';
  import { ThemeSwitcher } from '@juspay/svelte-ui-components';
  import { sidebarStore, themeStore } from '$lib/client/modules/foundation';
  import { PlatformSidebar } from '$lib/client/modules/platform/ui';

  let { children }: { children: Snippet } = $props();

  let sidebarCollapsed = $state(false);

  sidebarStore.subscribe((state) => {
    sidebarCollapsed = state.collapsed;
  });

  function handleToggleSidebar() {
    sidebarStore.toggle();
  }

  function handleThemeChange(value: string, _resolved: string) {
    themeStore.setTheme(value as 'light' | 'dark');
  }
</script>

<svelte:head>
  <title>Breeze Platform</title>
</svelte:head>

<div class="platform-layout">
  <PlatformSidebar collapsed={sidebarCollapsed} ontoggle={handleToggleSidebar} />

  <div class="platform-main">
    <div class="accent-bar">Breeze Platform</div>

    <header class="top-bar">
      <div class="top-bar-left">
        <button class="mobile-toggle" onclick={handleToggleSidebar} aria-label="Toggle menu">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="3" y1="12" x2="21" y2="12"></line>
            <line x1="3" y1="6" x2="21" y2="6"></line>
            <line x1="3" y1="18" x2="21" y2="18"></line>
          </svg>
        </button>
        <span class="platform-title">Platform Admin</span>
      </div>
      <div class="top-bar-right">
        <ThemeSwitcher mode="toggle" onchange={handleThemeChange} />
      </div>
    </header>

    <main class="content">
      {@render children()}
    </main>
  </div>
</div>

<style>
  .platform-layout {
    display: flex;
    min-height: 100vh;
    background: var(--color-bg);
  }

  .platform-main {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-width: 0;
  }

  .accent-bar {
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--color-primary);
    color: #ffffff;
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-semibold);
    letter-spacing: 0.06em;
    text-transform: uppercase;
  }

  .top-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 56px;
    padding: 0 var(--space-6);
    background: var(--color-surface);
    border-bottom: 1px solid var(--color-border);
    position: sticky;
    top: 0;
    z-index: var(--z-sticky);
  }

  .top-bar-left {
    display: flex;
    align-items: center;
    gap: var(--space-3);
  }

  .top-bar-right {
    display: flex;
    align-items: center;
    gap: var(--space-3);
  }

  .mobile-toggle {
    display: none;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    border: none;
    background: transparent;
    color: var(--color-text-muted);
    border-radius: var(--radius-sm);
    transition: background var(--transition-fast);
  }

  .mobile-toggle:hover {
    background: var(--color-surface-2);
  }

  .platform-title {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .content {
    flex: 1;
    padding: var(--space-8);
    overflow-y: auto;
  }

  @media (max-width: 768px) {
    .mobile-toggle {
      display: flex;
    }

    .content {
      padding: var(--space-4);
    }
  }
</style>
