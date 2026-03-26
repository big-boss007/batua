<script lang="ts">
  import type { Snippet } from 'svelte';
  import { ThemeSwitcher } from '@juspay/svelte-ui-components';
  import { sidebarStore, themeStore } from '$lib/client/modules/foundation';
  import type { NavItem } from '$lib/client/modules/admin';
  import { currentMerchant, fetchMerchants, getCurrentMerchantId } from '$lib/client/modules/admin';
  import { Sidebar } from '$lib/client/modules/admin/ui';

  let { children }: { children: Snippet } = $props();

  const navItems: NavItem[] = [
    {
      label: 'Wallet',
      href: '/admin',
      icon: '\u25A6',
      children: [
        { label: 'Overview', href: '/admin', icon: '' },
        { label: 'Wallet Policies', href: '/admin/wallet-policies', icon: '' }
      ]
    },
    {
      label: 'Loyalty',
      href: '/admin/loyalty',
      icon: '\u2665',
      children: [
        { label: 'Earn Rules', href: '/admin/rules', icon: '' },
        { label: 'Tiers', href: '/admin/loyalty', icon: '' },
        { label: 'Memberships', href: '/admin/memberships', icon: '' },
        { label: 'Campaigns', href: '/admin/campaigns', icon: '' }
      ]
    },
    { label: 'Transactions', href: '/admin/transactions', icon: '\u21C4' },
    { label: 'Customers', href: '/admin/customers', icon: '\u2616' },
    { label: 'Gift Cards', href: '/admin/gift-cards', icon: '\u2606' },
    {
      label: 'Referrals',
      href: '/admin/referrals',
      icon: '\u2192',
      children: [
        { label: 'Referral Program', href: '/admin/referrals', icon: '' },
        { label: 'Influencers', href: '/admin/influencers', icon: '' }
      ]
    },
    { label: 'Notifications', href: '/admin/notifications', icon: '\u2709' },
    { label: 'Settings', href: '/admin/settings', icon: '\u2638' }
  ];

  let sidebarCollapsed = $state(false);
  let merchantName = $state<string | null>(null);

  sidebarStore.subscribe((state) => {
    sidebarCollapsed = state.collapsed;
  });

  currentMerchant.subscribe((m) => {
    merchantName = m !== null ? m.name : null;
  });

  async function ensureMerchantSelected() {
    const savedId = getCurrentMerchantId();
    if (savedId !== null) {
      const result = await fetchMerchants(1, 50);
      if (result.tag === 'success') {
        const saved = result.data.find((m) => m.id === savedId) ?? null;
        if (saved !== null) {
          currentMerchant.set(saved);
          return;
        }
      }
    }
    const result = await fetchMerchants(1, 1);
    if (result.tag === 'success' && result.data.length > 0) {
      currentMerchant.set(result.data[0]);
    }
  }

  ensureMerchantSelected();

  function handleToggleSidebar() {
    sidebarStore.toggle();
  }

  function handleThemeChange(value: string, _resolved: string) {
    themeStore.setTheme(value as 'light' | 'dark');
  }
</script>

<svelte:head>
  <title>Batua Admin</title>
</svelte:head>

<div class="admin-layout">
  <Sidebar items={navItems} collapsed={sidebarCollapsed} ontoggle={handleToggleSidebar} />

  <div class="admin-main">
    <header class="top-bar">
      <div class="top-bar-left">
        <button class="mobile-toggle" onclick={handleToggleSidebar} aria-label="Toggle menu">
          <svg
            width="20"
            height="20"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <line x1="3" y1="12" x2="21" y2="12"></line>
            <line x1="3" y1="6" x2="21" y2="6"></line>
            <line x1="3" y1="18" x2="21" y2="18"></line>
          </svg>
        </button>
        <span class="merchant-name">Merchant Admin</span>
        {#if merchantName}
          <span class="merchant-name-badge">{merchantName}</span>
        {/if}
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
  .admin-layout {
    display: flex;
    min-height: 100vh;
    background: var(--color-bg);
  }

  .admin-main {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-width: 0;
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

  .merchant-name {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .merchant-name-badge {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-primary);
    background: var(--color-surface-2);
    padding: var(--space-1) var(--space-3);
    border-radius: var(--radius-full, 9999px);
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
