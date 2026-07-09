<script lang="ts">
  import type {
    FestiveTemplate,
    Rule,
    Campaign,
    CampaignStackingConfig,
    CreateCampaignFromTemplateRequest,
    CreateCampaignDirectRequest
  } from '$lib/client/modules/rules';
  import {
    campaignsStore,
    createCampaignFromTemplate,
    createCampaignDirect,
    deactivateCampaign,
    fetchCampaigns,
    fetchFestiveTemplates,
    fetchRules,
    getCampaignConfig,
    updateCampaignConfig
  } from '$lib/client/modules/rules';
  import { Button } from '@juspay/svelte-ui-components';
  import { currentMerchantId } from '$lib/client/modules/admin';
  import { toastStore } from '$lib/client/modules/foundation';
  import {
    CampaignsList,
    FestiveTemplateGrid,
    CampaignForm,
    EarnFormulaBanner,
    CampaignDetailModal,
    StackingConfigModal
  } from '$lib/client/modules/rules/ui';

  let merchantId = $state<string | null>(null);
  let templates = $state<Array<FestiveTemplate>>([]);
  let rules = $state<Array<Rule>>([]);
  let selectedTemplate = $state<FestiveTemplate | null>(null);
  let selectedCampaign = $state<Campaign | null>(null);
  let showCreateModal = $state(false);
  let showSettingsModal = $state(false);
  let stackingConfig = $state<CampaignStackingConfig>({
    campaign_stacking_mode: 'multiplicative',
    max_campaign_multiplier: 10
  });

  campaignsStore.set([]);

  currentMerchantId.subscribe((id) => {
    const prevId = merchantId;
    merchantId = id;
    if (id !== null && id !== prevId) {
      loadData(id);
    }
  });

  let activeCampaign = $derived(
    $campaignsStore.find((c) => {
      const now = new Date();
      return c.is_active && new Date(c.starts_at) <= now && new Date(c.ends_at) > now;
    }) ?? null
  );

  async function loadData(id: string) {
    const [campaignsResult, templatesResult, rulesResult, configResult] = await Promise.all([
      fetchCampaigns(id),
      fetchFestiveTemplates(),
      fetchRules(id),
      getCampaignConfig(id)
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
    if (configResult.tag === 'success') {
      stackingConfig = configResult.data;
    }
  }

  function handleSelectTemplate(template: FestiveTemplate) {
    selectedTemplate = template;
  }

  function handleSelectCampaign(campaign: Campaign) {
    selectedCampaign = campaign;
  }

  async function handleCreateFromTemplate(req: CreateCampaignFromTemplateRequest) {
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

  async function handleCreateDirect(req: CreateCampaignDirectRequest) {
    if (merchantId === null) return;
    const payload = { ...req, merchant_id: merchantId };
    const result = await createCampaignDirect(payload);

    if (result.tag === 'success') {
      campaignsStore.addCampaign(result.data);
      toastStore.push({ message: 'Campaign created successfully', level: 'success' });
      showCreateModal = false;
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }
  }

  async function handleDeactivate(campaignId: string) {
    const result = await deactivateCampaign(campaignId);
    if (result.tag === 'success') {
      campaignsStore.set($campaignsStore.filter((c) => c.id !== campaignId));
      toastStore.push({ message: 'Campaign deactivated', level: 'success' });
      selectedCampaign = null;
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }
  }

  async function handleSaveSettings(config: CampaignStackingConfig) {
    if (merchantId === null) return;
    const result = await updateCampaignConfig(merchantId, config);
    if (result.tag === 'success') {
      stackingConfig = result.data;
      toastStore.push({ message: 'Campaign settings saved', level: 'success' });
      showSettingsModal = false;
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }
  }
</script>

<svelte:head>
  <title>Campaigns - Batua Admin</title>
</svelte:head>

<div class="campaigns-page">
  <header class="page-header">
    <h1 class="page-title">Campaigns</h1>
    <p class="page-subtitle">Boost engagement with festive and seasonal reward campaigns</p>
  </header>

  {#if activeCampaign !== null && activeCampaign.multiplier !== null}
    <div class="earn-banner-wrapper">
      <EarnFormulaBanner
        multiplier={activeCampaign.multiplier}
        campaignName={activeCampaign.name}
        stackingMode={stackingConfig.campaign_stacking_mode}
        maxCap={stackingConfig.max_campaign_multiplier}
      />
    </div>
  {/if}

  <section class="section">
    <div class="section-header">
      <h2 class="section-title">Active & Upcoming Campaigns</h2>
      <div class="section-actions">
        <Button
          text="Settings"
          classes="btn-secondary"
          onclick={() => {
            showSettingsModal = true;
          }}
        />
        <Button
          text="+ Create campaign"
          classes="btn-primary"
          onclick={() => {
            showCreateModal = true;
          }}
        />
      </div>
    </div>
    <CampaignsList campaigns={$campaignsStore} {rules} onSelect={handleSelectCampaign} />
  </section>

  <section class="section">
    <h2 class="section-title">Festive templates</h2>
    <p class="section-description">Quick-start from a pre-configured template</p>
    <FestiveTemplateGrid {templates} onSelect={handleSelectTemplate} />
  </section>

  {#if selectedTemplate !== null}
    <div
      class="modal-overlay"
      role="presentation"
      onclick={() => {
        selectedTemplate = null;
      }}
    >
      <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
      <div
        class="modal-panel"
        role="dialog"
        aria-label="Create campaign from template"
        tabindex="-1"
        onclick={(e) => e.stopPropagation()}
        onkeydown={(e) => {
          if (e.key === 'Escape') {
            selectedTemplate = null;
          }
        }}
      >
        <CampaignForm
          template={selectedTemplate}
          {rules}
          existingCampaigns={$campaignsStore}
          onSaveTemplate={handleCreateFromTemplate}
          onCancel={() => {
            selectedTemplate = null;
          }}
        />
      </div>
    </div>
  {/if}

  {#if showCreateModal}
    <div
      class="modal-overlay"
      role="presentation"
      onclick={() => {
        showCreateModal = false;
      }}
    >
      <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
      <div
        class="modal-panel"
        role="dialog"
        aria-label="Create campaign"
        tabindex="-1"
        onclick={(e) => e.stopPropagation()}
        onkeydown={(e) => {
          if (e.key === 'Escape') {
            showCreateModal = false;
          }
        }}
      >
        <CampaignForm
          {rules}
          existingCampaigns={$campaignsStore}
          onSaveDirect={handleCreateDirect}
          onCancel={() => {
            showCreateModal = false;
          }}
        />
      </div>
    </div>
  {/if}

  {#if selectedCampaign !== null}
    <CampaignDetailModal
      campaign={selectedCampaign}
      {rules}
      {stackingConfig}
      onDeactivate={handleDeactivate}
      onClose={() => {
        selectedCampaign = null;
      }}
    />
  {/if}

  {#if showSettingsModal}
    <StackingConfigModal
      config={stackingConfig}
      onSave={handleSaveSettings}
      onCancel={() => {
        showSettingsModal = false;
      }}
    />
  {/if}
</div>

<style>
  .campaigns-page {
    max-width: 1200px;
    margin: 0 auto;
  }

  .page-header {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    margin-bottom: var(--space-5);
  }

  .page-title {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
  }

  .page-subtitle {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .earn-banner-wrapper {
    margin-bottom: var(--space-6);
  }

  .section {
    margin-bottom: var(--space-8);
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: var(--space-4);
  }

  .section-title {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .section-actions {
    display: flex;
    gap: var(--space-2);
  }

  .section-description {
    font-size: var(--font-size-sm);
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
