<script lang="ts">
  import type {
    FestiveTemplate,
    Rule,
    Campaign,
    CreateCampaignFromTemplateRequest
  } from '$lib/client/modules/rules';
  import {
    campaignsStore,
    createCampaignFromTemplate,
    fetchCampaigns,
    fetchFestiveTemplates,
    fetchRules
  } from '$lib/client/modules/rules';
  import { currentMerchantId } from '$lib/client/modules/admin';
  import { toastStore } from '$lib/client/modules/foundation';
  import { CampaignsList, FestiveTemplateGrid, CampaignForm } from '$lib/client/modules/rules/ui';

  let merchantId = $state<string | null>(null);
  let templates = $state<Array<FestiveTemplate>>([]);
  let rules = $state<Array<Rule>>([]);
  let selectedTemplate = $state<FestiveTemplate | null>(null);

  campaignsStore.set([]);

  currentMerchantId.subscribe((id) => {
    const prevId = merchantId;
    merchantId = id;
    if (id !== null && id !== prevId) {
      loadData(id);
    }
  });

  async function loadData(id: string) {
    const [campaignsResult, templatesResult, rulesResult] = await Promise.all([
      fetchCampaigns(id),
      fetchFestiveTemplates(),
      fetchRules(id)
    ]);

    if (campaignsResult.tag === 'success') {
      campaignsStore.set(campaignsResult.data);
    }
    if (templatesResult.tag === 'success') {
      templates = templatesResult.data;
    }
    if (rulesResult.tag === 'success') {
      rules = rulesResult.data;
    }
  }

  function handleSelectTemplate(template: FestiveTemplate) {
    selectedTemplate = template;
  }

  async function handleCreateCampaign(req: CreateCampaignFromTemplateRequest) {
    if (merchantId === null) return;
    const payload = { ...req, merchant_id: merchantId };
    const result = await createCampaignFromTemplate(payload);

    if (result.tag === 'success') {
      campaignsStore.addCampaign(result.data);
      toastStore.push({ message: 'Campaign created successfully', level: 'success' });
      selectedTemplate = null;
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }
  }

  function handleCancelCreate() {
    selectedTemplate = null;
  }
</script>

<svelte:head>
  <title>Campaigns - Batua Admin</title>
</svelte:head>

<div class="campaigns-page">
  <header class="page-header">
    <div class="header-content">
      <h1 class="page-title">Campaigns</h1>
      <p class="page-description">Boost engagement with festive and seasonal reward campaigns</p>
    </div>
  </header>

  <section class="section">
    <h2 class="section-title">Active & Upcoming Campaigns</h2>
    <CampaignsList campaigns={$campaignsStore} />
  </section>

  <section class="section">
    <h2 class="section-title">Festive Templates</h2>
    <p class="section-description">
      Select a template to create a new campaign with pre-configured settings
    </p>
    <FestiveTemplateGrid {templates} onSelect={handleSelectTemplate} />
  </section>

  {#if selectedTemplate !== null}
    <div class="modal-overlay" role="presentation" onclick={handleCancelCreate}>
      <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
      <div
        class="modal-panel"
        role="dialog"
        aria-label="Create campaign"
        onclick={(e) => e.stopPropagation()}
      >
        <CampaignForm
          template={selectedTemplate}
          {rules}
          onSave={handleCreateCampaign}
          onCancel={handleCancelCreate}
        />
      </div>
    </div>
  {/if}
</div>

<style>
  .campaigns-page {
    max-width: 1200px;
    margin: 0 auto;
    padding: var(--space-6) var(--space-8);
  }

  .page-header {
    margin-bottom: var(--space-8);
  }

  .header-content {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .page-title {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
  }

  .page-description {
    font-size: var(--font-size-base);
    color: var(--color-text-muted);
  }

  .section {
    margin-bottom: var(--space-8);
  }

  .section-title {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
    margin-bottom: var(--space-2);
  }

  .section-description {
    font-size: var(--font-size-base);
    color: var(--color-text-muted);
    margin-bottom: var(--space-4);
  }

  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: flex-start;
    justify-content: center;
    padding-top: var(--space-12);
    z-index: var(--z-modal);
    overflow-y: auto;
  }

  .modal-panel {
    background: var(--color-bg);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-lg);
    max-height: calc(100vh - var(--space-16));
    overflow-y: auto;
  }
</style>
