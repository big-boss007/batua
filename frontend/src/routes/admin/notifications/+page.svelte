<script lang="ts">
  import { Tabs, Table, Pill, Button } from '@juspay/svelte-ui-components';

  import type {
    NotificationTemplate,
    UpdateTemplateRequest,
    NotificationLog
  } from '$lib/client/modules/settings';
  import {
    updateTemplate,
    fetchTemplates,
    templatesStore,
    fetchNotificationLogs
  } from '$lib/client/modules/settings';
  import { currentMerchantId } from '$lib/client/modules/admin';
  import { toastStore, formatDateTime } from '$lib/client/modules/foundation';
  import { NotificationTemplateEditor } from '$lib/client/modules/settings/ui';

  let selectedId = $state<string | null>(null);
  let merchantId = $state<string | null>(null);
  let templates = $state<Array<NotificationTemplate>>([]);
  let logs = $state<Array<NotificationLog>>([]);
  let logsLoading = $state(false);

  const tabIds = ['templates', 'logs'] as const;
  const tabItems = ['Templates', 'Logs'];
  let activeTabIndex = $state(0);
  let activeTab = $derived(tabIds[activeTabIndex]);

  let selectedTemplate = $derived(
    selectedId !== null ? (templates.find((t) => t.id === selectedId) ?? null) : null
  );

  currentMerchantId.subscribe((id) => {
    const prevId = merchantId;
    merchantId = id;
    if (id !== null && id !== prevId) {
      loadData(id);
    }
  });

  async function loadData(mId: string) {
    logsLoading = true;
    const [templatesResult, logsResult] = await Promise.all([
      fetchTemplates(mId),
      fetchNotificationLogs(mId, 1, 50)
    ]);

    if (templatesResult.tag === 'success') templates = templatesResult.data;
    if (logsResult.tag === 'success') logs = logsResult.data;
    logsLoading = false;
  }

  function handleTabChange(index: number) {
    activeTabIndex = index;
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

  let logsTableData = $derived(
    logs.map((log) => [
      log.channel,
      log.status,
      log.created_at ? formatDateTime(log.created_at) : ''
    ])
  );
</script>

<svelte:head>
  <title>Notifications - Batua</title>
</svelte:head>

<div class="notifications-page">
  <header class="page-header">
    <h1 class="page-title">Notifications</h1>
    <p class="page-subtitle">Manage notification templates and delivery logs</p>
  </header>

  <Tabs
    items={tabItems}
    activeIndex={activeTabIndex}
    onchange={(idx) => {
      handleTabChange(idx);
    }}
  />

  {#if activeTab === 'templates'}
    {#if templates.length === 0}
      <p class="empty-state">No notification templates configured.</p>
    {:else}
      <div class="notifications-layout">
        <aside class="templates-sidebar">
          {#each templates as template (template.id)}
            <Button
              classes="template-item{selectedId === template.id ? ' template-selected' : ''}"
              onclick={() => (selectedId = template.id)}
            >
              {#snippet children()}
                <span class="template-item-name">{template.name}</span>
                <div class="template-item-meta">
                  <span class="template-item-channel">{template.channel}</span>
                  <Pill
                    text={template.is_active ? 'Active' : 'Inactive'}
                    classes={template.is_active ? 'pill-success' : 'pill-neutral'}
                  />
                </div>
              {/snippet}
            </Button>
          {/each}
        </aside>

        <div class="editor-panel">
          {#if selectedTemplate !== null}
            <NotificationTemplateEditor template={selectedTemplate} onSave={handleSaveTemplate} />
          {:else}
            <div class="no-selection">
              <p>Select a template to edit</p>
            </div>
          {/if}
        </div>
      </div>
    {/if}
  {:else if activeTab === 'logs'}
    {#if merchantId === null}
      <p class="empty-state">Select a merchant to view notification logs.</p>
    {:else if logsLoading}
      <p class="empty-state">Loading logs...</p>
    {:else}
      <div class="logs-section">
        <Table
          tableHeaders={['Channel', 'Status', 'Sent At']}
          tableData={logsTableData}
          sortable={false}
        >
          {#snippet cell(value, rowIndex, colIndex)}
            {#if colIndex === 0}
              <span class="channel-label">{value}</span>
            {:else if colIndex === 1}
              <Pill
                text={String(value)}
                classes={String(value) === 'sent'
                  ? 'pill-success'
                  : String(value) === 'failed'
                    ? 'pill-error'
                    : 'pill-neutral'}
              />
            {:else}
              {value}
            {/if}
          {/snippet}
          {#snippet empty()}
            <p class="empty-state">No notification logs yet.</p>
          {/snippet}
        </Table>
      </div>
    {/if}
  {/if}
</div>

<style>
  .notifications-page {
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
    max-width: 1200px;
  }

  .page-header {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
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

  .empty-state {
    padding: var(--space-8);
    text-align: center;
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
  }

  .notifications-layout {
    display: grid;
    grid-template-columns: 280px 1fr;
    gap: var(--space-6);
    min-height: 500px;
  }

  .templates-sidebar {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    border-radius: var(--radius-lg);
    padding: var(--space-3);
    background: var(--color-surface);
    overflow-y: auto;
    box-shadow: var(--shadow-card);
  }

  :global(.template-item) {
    --button-color: transparent;
    --button-text-color: var(--color-text);
    --button-border: 1px solid transparent;
    --button-border-radius: var(--radius-md);
    --button-padding: var(--space-3) var(--space-4);
    --button-hover-color: var(--color-surface-2);
    --button-justify-content: flex-start;
    --button-width: 100%;
    --button-content-flex-direction: column;
    --button-content-gap: var(--space-1);
  }

  :global(.template-item button) {
    align-items: flex-start !important;
    text-align: left;
  }

  :global(.template-selected) {
    --button-color: var(--color-surface-2);
    --button-border: 1px solid var(--color-primary);
  }

  .template-item-name {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .template-item-meta {
    display: flex;
    gap: var(--space-2);
    align-items: center;
  }

  .template-item-channel {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    text-transform: uppercase;
  }

  .editor-panel {
    min-width: 0;
  }

  .no-selection {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    border: 1px dashed var(--color-border);
    border-radius: var(--radius-lg);
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
  }

  .logs-section {
    background: var(--color-surface);
    border-radius: var(--radius-lg);
    overflow: hidden;
    box-shadow: var(--shadow-card);
  }

  .channel-label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    text-transform: uppercase;
    color: var(--color-text);
  }
</style>
