<script lang="ts">
  import { Tabs, Table, Pill } from '@juspay/svelte-ui-components';

  import type { PageData } from './$types';
  import type { UpdateTemplateRequest, NotificationLog } from '$lib/client/modules/settings';
  import { updateTemplate, templatesStore, fetchNotificationLogs } from '$lib/client/modules/settings';
  import { currentMerchantId } from '$lib/client/modules/admin';
  import { toastStore, formatDateTime } from '$lib/client/modules/foundation';
  import { NotificationTemplateEditor } from '$lib/client/modules/settings/ui';

  let { data }: { data: PageData } = $props();

  let selectedId = $state<string | null>(null);
  let merchantId = $state<string | null>(null);
  let logs = $state<Array<NotificationLog>>([]);
  let logsLoading = $state(false);

  const tabIds = ['templates', 'logs'] as const;
  const tabItems = ['Templates', 'Logs'];
  let activeTabIndex = $state(0);
  let activeTab = $derived(tabIds[activeTabIndex]);

  let selectedTemplate = $derived(
    selectedId !== null ? data.templates.find((t) => t.id === selectedId) ?? null : null
  );

  currentMerchantId.subscribe((id) => {
    const prevId = merchantId;
    merchantId = id;
    if (id !== null && id !== prevId) {
      loadLogs(id);
    }
  });

  async function loadLogs(mId: string) {
    logsLoading = true;
    const result = await fetchNotificationLogs(mId, 1, 50);
    if (result.tag === 'success') {
      logs = result.data;
    }
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
  </header>

  <Tabs items={tabItems} activeIndex={activeTabIndex} onchange={(idx) => { handleTabChange(idx) }} />

  {#if activeTab === 'templates'}
    {#if data.templates.length === 0}
      <p class="empty-state">No notification templates configured.</p>
    {:else}
      <div class="notifications-layout">
        <aside class="templates-sidebar">
          {#each data.templates as template (template.id)}
            <button
              class="template-item"
              class:template-selected={selectedId === template.id}
              onclick={() => (selectedId = template.id)}
            >
              <span class="template-item-name">{template.name}</span>
              <div class="template-item-meta">
                <span class="template-item-channel">{template.channel}</span>
                <span
                  class="template-item-status"
                  class:status-active={template.is_active}
                  class:status-inactive={!template.is_active}
                >
                  {template.is_active ? 'Active' : 'Inactive'}
                </span>
              </div>
            </button>
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
                classes={String(value) === 'sent' ? 'pill-log-success' : String(value) === 'failed' ? 'pill-log-error' : 'pill-log-default'}
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
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: var(--space-3);
    background: var(--color-surface);
    overflow-y: auto;
  }

  .template-item {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    padding: var(--space-3) var(--space-4);
    background: none;
    border: 1px solid transparent;
    border-radius: var(--radius-md);
    text-align: left;
    transition:
      background var(--transition-fast),
      border-color var(--transition-fast);
  }

  .template-item:hover {
    background: var(--color-surface-2);
  }

  .template-selected {
    background: var(--color-surface-2);
    border-color: var(--color-primary);
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

  .template-item-status {
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-semibold);
  }

  .status-active {
    color: var(--color-success);
  }

  .status-inactive {
    color: var(--color-text-muted);
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
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    overflow: hidden;
  }

  .channel-label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    text-transform: uppercase;
    color: var(--color-text);
  }

  :global(.pill-log-success) {
    --pill-color: var(--color-success);
    --pill-bg: color-mix(in srgb, var(--color-success) 12%, transparent);
  }

  :global(.pill-log-error) {
    --pill-color: var(--color-error);
    --pill-bg: color-mix(in srgb, var(--color-error) 12%, transparent);
  }

  :global(.pill-log-default) {
    --pill-color: var(--color-text-muted);
    --pill-bg: var(--color-surface-2);
  }
</style>
