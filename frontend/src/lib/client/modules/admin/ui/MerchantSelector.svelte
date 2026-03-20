<script lang="ts">
  import { Select } from '@juspay/svelte-ui-components';
  import type { Merchant } from '$lib/client/modules/admin';
  import {
    currentMerchant,
    currentMerchantId,
    fetchMerchants,
    getCurrentMerchantId
  } from '$lib/client/modules/admin';

  let merchants = $state<Array<Merchant>>([]);
  let loaded = $state(false);

  let merchantItems = $derived(
    merchants.map((m) => ({ id: m.id, label: m.name || m.external_id }))
  );

  let selectedValue = $state<string[]>([]);

  currentMerchantId.subscribe((id) => {
    if (id !== null) {
      selectedValue = [id];
    }
  });

  async function loadMerchants() {
    if (loaded) return;
    const result = await fetchMerchants();
    if (result.tag === 'success') {
      merchants = result.data;
      loaded = true;

      const savedId = getCurrentMerchantId();
      const savedMerchant =
        savedId !== null ? (merchants.find((m) => m.id === savedId) ?? null) : null;

      if (savedMerchant !== null) {
        currentMerchant.set(savedMerchant);
      } else if (merchants.length > 0) {
        currentMerchant.set(merchants[0]);
      }
    }
  }

  loadMerchants();

  function handleChange(value: string[]) {
    if (value.length === 0) return;
    const selected = merchants.find((m) => m.id === value[0]) ?? null;
    if (selected !== null) {
      currentMerchant.set(selected);
    }
  }
</script>

<div class="merchant-selector">
  {#if merchantItems.length > 0}
    <Select
      items={merchantItems}
      value={selectedValue}
      onchange={handleChange}
      placeholder="Select merchant"
    />
  {:else if loaded}
    <span class="no-merchants">No merchants available</span>
  {:else}
    <span class="loading-merchants">Loading...</span>
  {/if}
</div>

<style>
  .merchant-selector {
    min-width: 200px;
  }

  .no-merchants,
  .loading-merchants {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }
</style>
