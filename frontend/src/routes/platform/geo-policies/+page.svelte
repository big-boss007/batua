<script lang="ts">
  import { Button, Modal } from '@juspay/svelte-ui-components';
  import type { PageData } from './$types';
  import type { GeoPolicy } from '$lib/client/modules/platform';
  import { createGeoPolicy } from '$lib/client/modules/platform';
  import { toastStore, MODAL_CLOSE_ICON } from '$lib/client/modules/foundation';
  import { GeoPolicyTable, GeoPolicyForm } from '$lib/client/modules/platform/ui';
  import { invalidateAll } from '$app/navigation';

  let { data }: { data: PageData } = $props();

  let policies: GeoPolicy[] = $derived(data.policies ?? []);
  let showCreateModal = $state(false);
  let createLoading = $state(false);

  function handleOpenModal() {
    showCreateModal = true;
  }

  function handleCloseModal() {
    showCreateModal = false;
  }

  async function handleCreate(formData: {
    geo_code: string;
    name: string;
    config: Record<string, unknown>;
  }) {
    createLoading = true;
    const result = await createGeoPolicy(formData as Record<string, unknown>);
    createLoading = false;

    if (result.tag === 'success') {
      toastStore.push({ message: 'Geo policy created', level: 'success' });
      showCreateModal = false;
      await invalidateAll();
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }
  }
</script>

<svelte:head>
  <title>Geo Policies - Breeze Platform</title>
</svelte:head>

<div class="geo-page">
  <div class="page-header">
    <div class="header-text">
      <h1>Geo policies</h1>
      <p class="subtitle">Manage geography-specific configuration layers</p>
    </div>
    <Button text="+ Create geo policy" classes="btn-primary" onclick={handleOpenModal} />
  </div>

  <GeoPolicyTable {policies} />
</div>

{#if showCreateModal}
  <Modal
    header={{ text: 'Create geo policy', rightImage: MODAL_CLOSE_ICON }}
    size="fit-content"
    onheaderRightImageClick={handleCloseModal}
    footer={{
      primaryButton: { text: 'Create policy', enable: !createLoading },
      secondaryButton: { text: 'Cancel' }
    }}
    onprimaryButtonClick={() => {
      const form = document.querySelector('.geo-form') as HTMLFormElement | null;
      if (form) {
        form.requestSubmit();
      }
    }}
    onsecondaryButtonClick={handleCloseModal}
    onoverlayClick={handleCloseModal}
  >
    {#snippet content()}
      <GeoPolicyForm onsubmit={handleCreate} loading={createLoading} />
    {/snippet}
  </Modal>
{/if}

<style>
  .geo-page {
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
  }

  .page-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: var(--space-4);
  }

  .header-text {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .page-header h1 {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    line-height: var(--line-height-tight);
  }

  .subtitle {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }
</style>
