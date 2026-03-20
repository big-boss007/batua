<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';

  import { Tabs, Button, Input, Pill } from '@juspay/svelte-ui-components';

  import type { PageData } from './$types';
  import type { Merchant } from '$lib/client/modules/admin';
  import type {
    WalletPolicy,
    UpdateWalletPolicyRequest,
    CreateConnectorRequest,
    UpdateTemplateRequest
  } from '$lib/client/modules/settings';
  import {
    updateWalletPolicy,
    createConnector,
    updateTemplate,
    walletPoliciesStore,
    connectorsStore,
    templatesStore,
    updateMerchantProfile
  } from '$lib/client/modules/settings';
  import { currentMerchant, currentMerchantId } from '$lib/client/modules/admin';
  import { toastStore } from '$lib/client/modules/foundation';
  import {
    WalletPoliciesList,
    WalletPolicyForm,
    ConnectorsList,
    ConnectorForm,
    NotificationTemplateEditor
  } from '$lib/client/modules/settings/ui';

  let { data }: { data: PageData } = $props();

  const tabIds = ['store', 'policies', 'connectors', 'notifications'] as const;
  const tabItems = ['My Store', 'Wallet Policies', 'Connectors', 'Notifications'];
  let activeTabIndex = $state(
    Math.max(0, tabIds.indexOf(data.activeTab as (typeof tabIds)[number]))
  );
  let activeTab = $derived(tabIds[activeTabIndex]);
  let editingPolicy = $state<WalletPolicy | null>(null);
  let showConnectorForm = $state(false);

  let merchant = $state<Merchant | null>(null);
  let storeName = $state('');
  let storeDomain = $state('');
  let storeSlug = $state('');
  let savingStore = $state(false);

  currentMerchant.subscribe((m) => {
    merchant = m;
    if (m !== null) {
      storeName = m.name;
      storeDomain = m.domain ?? '';
      storeSlug = m.slug ?? '';
    }
  });

  let storefrontUrl = $derived(storeSlug ? `/s/${storeSlug}` : '');

  function handleTabChange(index: number) {
    activeTabIndex = index;
    const url = new URL($page.url);
    url.searchParams.set('tab', tabIds[index]);
    goto(url.toString(), { replaceState: true, noScroll: true });
  }

  function handleEditPolicy(policy: WalletPolicy) {
    editingPolicy = policy;
  }

  async function handleSavePolicy(policyId: string, body: UpdateWalletPolicyRequest) {
    const result = await updateWalletPolicy(policyId, body);
    if (result.tag === 'success') {
      walletPoliciesStore.updatePolicy(result.data);
      editingPolicy = null;
      toastStore.push({ message: 'Policy updated', level: 'success' });
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }
  }

  async function handleCreateConnector(body: CreateConnectorRequest) {
    const merchantId = new URL($page.url).searchParams.get('merchant') ?? '';
    const result = await createConnector(merchantId, body);
    if (result.tag === 'success') {
      connectorsStore.addConnector(result.data);
      showConnectorForm = false;
      toastStore.push({ message: 'Connector created', level: 'success' });
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }
  }

  async function handleSaveTemplate(templateId: string, body: UpdateTemplateRequest) {
    const result = await updateTemplate(templateId, body);
    if (result.tag === 'success') {
      templatesStore.updateTemplate(result.data);
      toastStore.push({ message: 'Template updated', level: 'success' });
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }
  }

  async function handleSaveStore() {
    if (merchant === null) return;
    savingStore = true;
    const payload: { name?: string; domain?: string; slug?: string } = {};
    if (storeName !== merchant.name) payload.name = storeName;
    if (storeDomain !== (merchant.domain ?? '')) payload.domain = storeDomain;
    if (storeSlug !== (merchant.slug ?? '')) payload.slug = storeSlug;

    const result = await updateMerchantProfile(merchant.id, payload);
    if (result.tag === 'success') {
      currentMerchant.set(result.data);
      toastStore.push({ message: 'Store profile updated', level: 'success' });
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }
    savingStore = false;
  }

  function handleCopyStorefrontUrl() {
    if (storefrontUrl) {
      navigator.clipboard.writeText(window.location.origin + storefrontUrl);
      toastStore.push({ message: 'Storefront URL copied', level: 'success' });
    }
  }
</script>

<svelte:head>
  <title>Settings - Batua</title>
</svelte:head>

<div class="settings-page">
  <header class="page-header">
    <h1 class="page-title">Settings</h1>
  </header>

  <Tabs
    items={tabItems}
    activeIndex={activeTabIndex}
    onchange={(idx) => {
      handleTabChange(idx);
    }}
  />

  <div class="tab-content">
    {#if activeTab === 'store'}
      {#if merchant !== null}
        <div class="store-form">
          <Input
            value={storeName}
            label="Merchant Name"
            placeholder="Store name"
            onInput={(val) => {
              storeName = val;
            }}
          />

          <Input
            value={storeDomain}
            label="Domain"
            placeholder="example.com"
            onInput={(val) => {
              storeDomain = val;
            }}
          />

          <Input
            value={storeSlug}
            label="Slug"
            placeholder="my-store"
            onInput={(val) => {
              storeSlug = val;
            }}
          />

          <div class="form-field">
            <span class="field-label">Plan Tier</span>
            <div class="plan-tier-display">
              <Pill text={merchant.plan_tier ?? 'free'} classes="pill-plan" />
            </div>
          </div>

          {#if storefrontUrl}
            <div class="form-field">
              <span class="field-label">Storefront URL</span>
              <div class="storefront-url-row">
                <code class="storefront-url">{storefrontUrl}</code>
                <Button text="Copy" classes="btn-copy" onclick={handleCopyStorefrontUrl} />
              </div>
            </div>
          {/if}

          <div class="form-actions">
            <Button
              text={savingStore ? 'Saving...' : 'Save Changes'}
              classes="btn-primary"
              onclick={handleSaveStore}
            />
          </div>
        </div>
      {:else}
        <p class="empty-state">Select a merchant to edit store settings.</p>
      {/if}
    {:else if activeTab === 'policies'}
      {#if editingPolicy !== null}
        <WalletPolicyForm policy={editingPolicy} onSave={handleSavePolicy} />
        <Button
          text="Cancel"
          classes="btn-cancel"
          onclick={() => {
            editingPolicy = null;
          }}
        />
      {:else}
        <WalletPoliciesList policies={data.policies} onEdit={handleEditPolicy} />
      {/if}
    {:else if activeTab === 'connectors'}
      <ConnectorsList connectors={data.connectors} />
      {#if showConnectorForm}
        <ConnectorForm onSave={handleCreateConnector} />
        <Button
          text="Cancel"
          classes="btn-cancel"
          onclick={() => {
            showConnectorForm = false;
          }}
        />
      {:else}
        <Button
          text="Add Connector"
          classes="btn-primary"
          onclick={() => {
            showConnectorForm = true;
          }}
        />
      {/if}
    {:else if activeTab === 'notifications'}
      {#if data.templates.length === 0}
        <p class="empty-state">No notification templates configured.</p>
      {:else}
        <div class="templates-list">
          {#each data.templates as template (template.id)}
            <NotificationTemplateEditor {template} onSave={handleSaveTemplate} />
          {/each}
        </div>
      {/if}
    {/if}
  </div>
</div>

<style>
  .settings-page {
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
    padding: var(--space-8);
    max-width: 1200px;
  }

  .page-header {
    display: flex;
    align-items: center;
  }

  .page-title {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
  }

  .tab-content {
    display: flex;
    flex-direction: column;
    gap: var(--space-5);
  }

  .store-form {
    display: flex;
    flex-direction: column;
    gap: var(--space-5);
    max-width: 480px;
  }

  .form-field {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .field-label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .plan-tier-display {
    display: flex;
    align-items: center;
  }

  .storefront-url-row {
    display: flex;
    align-items: center;
    gap: var(--space-3);
  }

  .storefront-url {
    font-family: var(--font-mono);
    font-size: var(--font-size-sm);
    padding: var(--space-2) var(--space-3);
    background: var(--color-surface-2);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    color: var(--color-text);
  }

  .form-actions {
    padding-top: var(--space-2);
  }

  .templates-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-5);
  }

  .empty-state {
    padding: var(--space-8);
    text-align: center;
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
  }

  :global(.pill-plan) {
    --pill-color: var(--color-primary);
    --pill-bg: color-mix(in srgb, var(--color-primary) 12%, transparent);
    --pill-font-size: var(--font-size-sm);
    --pill-font-weight: var(--font-weight-semibold);
    text-transform: capitalize;
  }

  :global(.btn-primary) {
    --button-color: var(--color-primary);
    --button-text-color: #ffffff;
    --button-border-radius: var(--radius-md);
    --button-padding: var(--space-2) var(--space-5);
    --button-font-size: var(--font-size-sm);
    --button-font-weight: var(--font-weight-semibold);
    --button-hover-color: var(--color-primary-hover);
  }

  :global(.btn-cancel) {
    --button-color: transparent;
    --button-text-color: var(--color-text-muted);
    --button-border: 1px solid var(--color-border);
    --button-border-radius: var(--radius-md);
    --button-padding: var(--space-2) var(--space-5);
    --button-font-size: var(--font-size-sm);
    --button-font-weight: var(--font-weight-medium);
    --button-hover-color: transparent;
    --button-hover-text-color: var(--color-text);
    --button-hover-border: 1px solid var(--color-text-muted);
  }

  :global(.btn-copy) {
    --button-color: transparent;
    --button-text-color: var(--color-primary);
    --button-border: 1px solid var(--color-primary);
    --button-border-radius: var(--radius-md);
    --button-padding: var(--space-1) var(--space-3);
    --button-font-size: var(--font-size-xs);
    --button-font-weight: var(--font-weight-medium);
    --button-hover-color: color-mix(in srgb, var(--color-primary) 8%, transparent);
  }
</style>
