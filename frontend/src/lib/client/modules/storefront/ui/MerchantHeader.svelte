<script lang="ts">
  import type { StorefrontMerchant } from '../types';

  let { merchant }: { merchant: StorefrontMerchant } = $props();
</script>

<header class="merchant-header">
  <div class="merchant-brand-bar"></div>
  <div class="merchant-header-content">
    <div class="merchant-logo">
      {merchant.name.charAt(0).toUpperCase()}
    </div>
    <h1 class="merchant-name">{merchant.name}</h1>
  </div>
</header>

<style>
  .merchant-header {
    position: sticky;
    top: 0;
    z-index: var(--z-sticky);
    background: var(--color-bg);
  }

  .merchant-brand-bar {
    height: 3px;
    background: linear-gradient(90deg, var(--color-primary) 0%, var(--color-primary-hover) 100%);
  }

  .merchant-header-content {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-3) var(--space-4);
    border-bottom: 1px solid var(--color-border);
  }

  .merchant-logo {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    border-radius: var(--radius-md);
    background: var(--color-primary);
    color: #ffffff;
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-bold);
    flex-shrink: 0;
  }

  .merchant-name {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
    line-height: var(--line-height-tight);
  }
</style>
