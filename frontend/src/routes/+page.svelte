<script lang="ts">
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();
</script>

<svelte:head>
  <title>Batua</title>
</svelte:head>

<main>
  <h1>Batua</h1>
  <p>Wallet, Loyalty, Gift Cards & Passes</p>
</main>

<style>
  main {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 100vh;
    gap: var(--space-4);
  }

  h1 {
    font-size: var(--font-size-3xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-primary);
  }

  p {
    font-size: var(--font-size-md);
    color: var(--color-text-muted);
  }
</style>
