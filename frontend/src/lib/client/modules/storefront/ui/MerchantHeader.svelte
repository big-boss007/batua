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
    background: #1a1d27;
  }

  .merchant-brand-bar {
    height: 3px;
    background: linear-gradient(90deg, #6366f1, #818cf8);
  }

  .merchant-header-content {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 14px 24px;
    border-bottom: 1px solid #2a2d3a;
    background: linear-gradient(90deg, rgba(99, 102, 241, 0.08), transparent);
  }

  .merchant-logo {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border-radius: 6px;
    background: #6366f1;
    color: #ffffff;
    font-size: 12px;
    font-weight: 700;
    flex-shrink: 0;
  }

  .merchant-name {
    font-size: 14px;
    font-weight: 600;
    color: #ffffff;
  }
</style>
