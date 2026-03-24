<script lang="ts">
  import { browser } from '$app/environment';
  import type { Snippet } from 'svelte';
  import type { StorefrontMerchant } from '$lib/client/modules/storefront';
  import { merchantContext } from '$lib/client/modules/storefront';
  import { MerchantHeader } from '$lib/client/modules/storefront/ui';

  if (browser) {
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    document.documentElement.setAttribute('data-theme', prefersDark ? 'dark' : 'light');
  }

  let { data, children }: { data: { merchant: StorefrontMerchant | null }; children: Snippet } =
    $props();

  let merchant = $derived(data.merchant);

  let _syncContext = $derived.by(() => {
    if (merchant !== null) {
      merchantContext.set(merchant);
    }
    return merchant;
  });
</script>

<svelte:head>
  {#if merchant !== null}
    <title>{merchant.name} Rewards</title>
  {:else}
    <title>Rewards</title>
  {/if}
</svelte:head>

<div class="storefront-page">
  <div class="storefront-shell">
    {#if merchant !== null}
      <MerchantHeader {merchant} />
      <main class="storefront-content">
        {@render children()}
      </main>
    {:else}
      <main class="storefront-content">
        <div class="storefront-error">
          <div class="error-icon">?</div>
          <h2 class="error-title">Store not found</h2>
          <p class="error-text">
            We couldn't find a rewards program at this address. Please check the URL and try again.
          </p>
        </div>
      </main>
    {/if}
  </div>
</div>

<style>
  .storefront-page {
    min-height: 100vh;
    background: #0f1117;
    padding: 32px 16px 48px;
  }

  .storefront-shell {
    max-width: 440px;
    margin: 0 auto;
    background: #1a1d27;
    border: 1px solid #2a2d3a;
    border-radius: 16px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.4);
    overflow: hidden;
  }

  @media (max-width: 480px) {
    .storefront-page {
      background: #0f1117;
      padding: 0;
    }

    .storefront-shell {
      max-width: 100%;
      border: none;
      border-radius: 0;
      box-shadow: none;
      min-height: 100vh;
    }
  }

  .storefront-content {
    padding: 0;
  }

  .storefront-error {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: var(--space-16) var(--space-4);
  }

  .error-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 64px;
    height: 64px;
    border-radius: var(--radius-full);
    background: var(--color-surface-2);
    color: var(--color-text-muted);
    font-size: var(--font-size-3xl);
    font-weight: var(--font-weight-bold);
    margin-bottom: var(--space-4);
  }

  .error-title {
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
    margin-bottom: var(--space-2);
  }

  .error-text {
    font-size: var(--font-size-base);
    color: var(--color-text-muted);
    line-height: var(--line-height-base);
    max-width: 300px;
  }
</style>
