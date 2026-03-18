<script lang="ts">
  import { Button, Modal } from '@juspay/svelte-ui-components';
  import type { PageData } from './$types';
  import type { PlatformMerchant } from '$lib/client/modules/platform';
  import { createMerchant } from '$lib/client/modules/platform';
  import { toastStore } from '$lib/client/modules/foundation';
  import { MerchantTable, OnboardForm } from '$lib/client/modules/platform/ui';
  import { invalidateAll } from '$app/navigation';

  let { data }: { data: PageData } = $props();

  let merchants: PlatformMerchant[] = $derived(data.merchants ?? []);
  let showOnboardModal = $state(false);
  let onboardLoading = $state(false);

  function handleOpenModal() {
    showOnboardModal = true;
  }

  function handleCloseModal() {
    showOnboardModal = false;
  }

  async function handleOnboard(formData: {
    name: string;
    external_id: string;
    domain: string | null;
    slug: string | null;
    plan_tier: string;
  }) {
    onboardLoading = true;
    const result = await createMerchant(formData as unknown as Record<string, unknown>);
    onboardLoading = false;

    if (result.tag === 'success') {
      toastStore.push({ message: 'Merchant onboarded successfully', level: 'success' });
      showOnboardModal = false;
      await invalidateAll();
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }
  }
</script>

<svelte:head>
  <title>Merchants - Breeze Platform</title>
</svelte:head>

<div class="merchants-page">
  <div class="page-header">
    <div class="header-text">
      <h1>Merchants</h1>
      <p class="subtitle">Manage all merchants on the Breeze platform</p>
    </div>
    <Button text="+ Onboard Merchant" classes="btn-primary" onclick={handleOpenModal} />
  </div>

  <MerchantTable merchants={merchants} />
</div>

{#if showOnboardModal}
  <Modal
    header={{ text: 'Onboard New Merchant' }}
    footer={{
      primaryButton: { text: 'Create Merchant', enable: !onboardLoading },
      secondaryButton: { text: 'Cancel' }
    }}
    onprimaryButtonClick={() => {
      const form = document.querySelector('.onboard-form') as HTMLFormElement | null;
      if (form) {
        form.requestSubmit();
      }
    }}
    onsecondaryButtonClick={handleCloseModal}
    onoverlayClick={handleCloseModal}
  >
    {#snippet content()}
      <OnboardForm onsubmit={handleOnboard} loading={onboardLoading} />
    {/snippet}
  </Modal>
{/if}

<style>
  .merchants-page {
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
