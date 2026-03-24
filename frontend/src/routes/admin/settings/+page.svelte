<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';

  import { Tabs, Button, Input, Pill } from '@juspay/svelte-ui-components';

  import type { PageData } from './$types';
  import type { Merchant } from '$lib/client/modules/admin';
  import type {
    Connector,
    NotificationTemplate,
    CreateConnectorRequest,
    UpdateTemplateRequest
  } from '$lib/client/modules/settings';
  import {
    fetchConnectors,
    fetchTemplates,
    createConnector,
    updateTemplate,
    connectorsStore,
    templatesStore,
    updateMerchantProfile
  } from '$lib/client/modules/settings';
  import { currentMerchant, currentMerchantId } from '$lib/client/modules/admin';
  import { toastStore } from '$lib/client/modules/foundation';
  import {
    ConnectorsList,
    ConnectorForm,
    NotificationTemplateEditor
  } from '$lib/client/modules/settings/ui';

  let { data }: { data: PageData } = $props();

  const tabIds = ['store', 'connectors', 'notifications'] as const;
  const tabItems = ['My Store', 'Connectors', 'Notifications'];
  let activeTabIndex = $state(
    Math.max(0, tabIds.indexOf(data.activeTab as (typeof tabIds)[number]))
  );
  let activeTab = $derived(tabIds[activeTabIndex]);
  let showConnectorForm = $state(false);

  let merchant = $state<Merchant | null>(null);
  let mId = $state<string | null>(null);
  let connectors = $state<Array<Connector>>([]);
  let templates = $state<Array<NotificationTemplate>>([]);
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

  currentMerchantId.subscribe((id) => {
    const prevId = mId;
    mId = id;
    if (id !== null && id !== prevId) {
      loadSettingsData(id);
    }
  });

  async function loadSettingsData(id: string) {
    const [connectorsResult, templatesResult] = await Promise.all([
      fetchConnectors(id),
      fetchTemplates(id)
    ]);

    if (connectorsResult.tag === 'success') connectors = connectorsResult.data;
    if (templatesResult.tag === 'success') templates = templatesResult.data;
  }

  let storefrontUrl = $derived(storeSlug ? `/s/${storeSlug}` : '');

  function handleTabChange(index: number) {
    activeTabIndex = index;
    const url = new URL($page.url);
    url.searchParams.set('tab', tabIds[index]);
    goto(url.toString(), { replaceState: true, noScroll: true });
  }

  async function handleCreateConnector(body: CreateConnectorRequest) {
    if (mId === null) return;
    const result = await createConnector(mId, body);
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

  let customerLinks = $derived(
    storefrontUrl
      ? [
          { name: 'Rewards Hub', path: storefrontUrl, icon: '💰' },
          { name: 'Gift Card Check', path: `${storefrontUrl}/gift-cards/check`, icon: '🎁' },
          { name: 'Referral Program', path: `${storefrontUrl}/refer`, icon: '🔗' }
        ]
      : []
  );

  let copiedLink: string | null = $state(null);

  function handleCopyLink(path: string, label: string) {
    navigator.clipboard.writeText(window.location.origin + path);
    copiedLink = path;
    toastStore.push({ message: `${label} link copied`, level: 'success' });
    setTimeout(() => {
      copiedLink = null;
    }, 2000);
  }

  function handleOpenLink(path: string) {
    window.open(path, '_blank');
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
        <div class="store-split">
          <div class="store-left">
            <div class="section-label">Store Details</div>
            <div class="store-fields">
              <Input
                value={storeName}
                label="Store Name"
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
              <div class="store-save">
                <Button
                  text={savingStore ? 'Saving...' : 'Save Changes'}
                  classes="btn-primary"
                  onclick={handleSaveStore}
                />
              </div>
            </div>
          </div>

          <div class="store-right">
            <div class="info-card">
              <div class="info-row">
                <span class="info-label">Plan</span>
                <Pill text={merchant.plan_tier ?? 'free'} classes="pill-plan" />
              </div>
              {#if storefrontUrl}
                <div class="info-row">
                  <span class="info-label">Storefront</span>
                  <code class="info-url">{storefrontUrl}</code>
                </div>
              {/if}
            </div>

            {#if customerLinks.length > 0}
              <div class="section-label">Customer Links</div>
              <div class="links-list">
                {#each customerLinks as link (link.path)}
                  <div class="link-row">
                    <div class="link-left">
                      <div class="link-icon">{link.icon}</div>
                      <div>
                        <div class="link-name">{link.name}</div>
                        <code class="link-path">{link.path}</code>
                      </div>
                    </div>
                    <div class="link-actions">
                      <button
                        class="icon-btn"
                        class:copied={copiedLink === link.path}
                        onclick={() => handleCopyLink(link.path, link.name)}
                        aria-label={copiedLink === link.path ? 'Copied' : 'Copy link'}
                        title={copiedLink === link.path ? 'Copied!' : 'Copy link'}
                      >
                        {#if copiedLink === link.path}
                          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="20 6 9 17 4 12"/>
                          </svg>
                        {:else}
                          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
                            <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
                          </svg>
                        {/if}
                      </button>
                      <button class="icon-btn" onclick={() => handleOpenLink(link.path)} aria-label="Open in new tab" title="Open in new tab">
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                          <path d="M18 13v6a2 2 0 01-2 2H5a2 2 0 01-2-2V8a2 2 0 012-2h6"/>
                          <polyline points="15 3 21 3 21 9"/>
                          <line x1="10" y1="14" x2="21" y2="3"/>
                        </svg>
                      </button>
                    </div>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        </div>
      {:else}
        <p class="empty-state">Select a merchant to edit store settings.</p>
      {/if}
    {:else if activeTab === 'connectors'}
      <ConnectorsList connectors={connectors} />
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
      {#if templates.length === 0}
        <p class="empty-state">No notification templates configured.</p>
      {:else}
        <div class="templates-list">
          {#each templates as template (template.id)}
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

  .store-split {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    overflow: hidden;
    max-width: 720px;
  }

  .store-left {
    padding: var(--space-6);
    border-right: 1px solid var(--color-border);
  }

  .store-right {
    padding: var(--space-6);
  }

  .section-label {
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.8px;
    margin-bottom: var(--space-4);
  }

  .store-fields {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .store-save {
    padding-top: var(--space-2);
  }

  .info-card {
    padding: var(--space-4);
    background: var(--color-surface-2);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    margin-bottom: var(--space-5);
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .info-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .info-label {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .info-url-row {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .info-url {
    font-family: var(--font-mono);
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .links-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .link-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-3) var(--space-4);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    transition: border-color var(--transition-fast);
  }

  .link-row:hover {
    border-color: var(--color-primary);
  }

  .link-left {
    display: flex;
    align-items: center;
    gap: var(--space-3);
  }

  .link-icon {
    width: 32px;
    height: 32px;
    border-radius: var(--radius-md);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 14px;
    background: var(--color-surface-2);
    flex-shrink: 0;
  }

  .link-name {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .link-path {
    font-size: 11px;
    color: var(--color-text-muted);
    font-family: var(--font-mono);
    margin-top: 1px;
    display: block;
  }

  .link-actions {
    display: flex;
    gap: 4px;
    flex-shrink: 0;
  }

  .icon-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    padding: 0;
    background: none;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    color: var(--color-text-muted);
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .icon-btn:hover {
    color: var(--color-primary);
    border-color: var(--color-primary);
  }

  .icon-btn.copied {
    color: var(--color-success);
    border-color: var(--color-success);
    background: color-mix(in srgb, var(--color-success) 8%, transparent);
  }

  @media (max-width: 768px) {
    .store-split {
      grid-template-columns: 1fr;
    }

    .store-left {
      border-right: none;
      border-bottom: 1px solid var(--color-border);
    }
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
