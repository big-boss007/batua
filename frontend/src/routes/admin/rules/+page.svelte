<script lang="ts">
  import type { Rule, RulePerformance, RewardRuleConfig } from '$lib/client/modules/rules';
  import { Button } from '@juspay/svelte-ui-components';
  import {
    rulesStore,
    selectedRuleStore,
    fetchRules,
    createRule,
    updateRule,
    fetchRulePerformance
  } from '$lib/client/modules/rules';
  import { currentMerchantId } from '$lib/client/modules/admin';
  import { toastStore, formatCurrencyINR } from '$lib/client/modules/foundation';
  import { RulesList, RuleForm } from '$lib/client/modules/rules/ui';

  rulesStore.set([]);

  let showForm = $state(false);
  let editingRule = $state<Rule | null>(null);
  let merchantId = $state<string | null>(null);
  let performanceMap = $state<Record<string, RulePerformance>>({});

  currentMerchantId.subscribe((id) => {
    const prevId = merchantId;
    merchantId = id;
    if (id !== null && id !== prevId) {
      loadRules(id);
    }
  });

  async function loadRules(mId: string) {
    const result = await fetchRules(mId);
    if (result.tag === 'success') {
      rulesStore.set(result.data);
      loadPerformance(result.data);
    }
  }

  async function loadPerformance(rules: Array<Rule>) {
    const results = await Promise.all(rules.map((r) => fetchRulePerformance(r.id)));
    const map: Record<string, RulePerformance> = {};
    for (const result of results) {
      if (result.tag === 'success') {
        map[result.data.rule_id] = result.data;
      }
    }
    performanceMap = map;
  }

  function handleCreate() {
    editingRule = null;
    showForm = true;
  }

  function handleEdit(rule: Rule) {
    editingRule = rule;
    selectedRuleStore.select(rule);
    showForm = true;
  }

  async function handleToggle(rule: Rule) {
    const result = await updateRule(rule.id, {
      config: rule.config,
      is_active: !rule.is_active
    });

    if (result.tag === 'success') {
      rulesStore.updateRule(result.data);
      toastStore.push({
        message: `Rule "${rule.name}" ${rule.is_active ? 'deactivated' : 'activated'}`,
        level: 'success'
      });
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }
  }

  async function handleSave(name: string, ruleType: string, config: RewardRuleConfig) {
    if (editingRule !== null) {
      const result = await updateRule(editingRule.id, { name, config });
      if (result.tag === 'success') {
        rulesStore.updateRule(result.data);
        toastStore.push({ message: 'Rule updated successfully', level: 'success' });
        closeForm();
      } else {
        toastStore.push({ message: result.message, level: 'error' });
      }
    } else {
      if (merchantId === null) {
        toastStore.push({ message: 'No merchant selected', level: 'error' });
        return;
      }
      const result = await createRule({
        merchant_id: merchantId,
        rule_type: ruleType,
        name,
        config
      });
      if (result.tag === 'success') {
        rulesStore.addRule(result.data);
        toastStore.push({ message: 'Rule created successfully', level: 'success' });
        closeForm();
      } else {
        toastStore.push({ message: result.message, level: 'error' });
      }
    }
  }

  function closeForm() {
    showForm = false;
    editingRule = null;
    selectedRuleStore.clear();
  }

  function formatPerformance(ruleId: string): string | null {
    const perf = performanceMap[ruleId];
    if (perf === undefined) return null;
    const entries = perf.total_entries.toLocaleString('en-IN');
    const value = formatCurrencyINR(perf.total_value);
    const customers = perf.unique_customers.toLocaleString('en-IN');
    return `${entries} entries \u00B7 ${value} \u00B7 ${customers} customers`;
  }
</script>

<svelte:head>
  <title>Rules - Batua Admin</title>
</svelte:head>

<div class="rules-page">
  <header class="page-header">
    <div class="header-content">
      <h1 class="page-title">Reward Rules</h1>
      <p class="page-description">Configure how customers earn and burn rewards</p>
    </div>
    <Button text="+ New Rule" classes="btn-primary" onclick={handleCreate} />
  </header>

  {#if merchantId === null}
    <div class="empty-state">
      <p class="empty-text">Select a merchant to view rules</p>
    </div>
  {:else}
    <section class="rules-section">
      {#if $rulesStore.length === 0}
        <div class="empty-rules">
          <p class="empty-text">
            No rules configured yet. Create your first reward rule to get started.
          </p>
        </div>
      {:else}
        <div class="rules-list">
          {#each $rulesStore as rule (rule.id)}
            {@const perf = formatPerformance(rule.id)}
            <div class="rule-row">
              <button class="rule-row-main" onclick={() => handleEdit(rule)}>
                <div class="rule-info">
                  <span class="rule-name">{rule.name}</span>
                  {#if perf !== null}
                    <span class="rule-perf">{perf}</span>
                  {/if}
                </div>
                <div class="rule-meta">
                  <span class="rule-type-badge">{rule.rule_type}</span>
                  <span class="rule-event">{rule.config.event_type}</span>
                  <span class="rule-version">v{rule.version}</span>
                  <span
                    class="rule-status"
                    class:status-active={rule.is_active}
                    class:status-inactive={!rule.is_active}
                  >
                    {rule.is_active ? 'Active' : 'Inactive'}
                  </span>
                </div>
              </button>
              <div class="rule-actions">
                <button class="toggle-btn" onclick={() => handleToggle(rule)}>
                  {rule.is_active ? 'Deactivate' : 'Activate'}
                </button>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </section>
  {/if}

  {#if showForm}
    <div class="modal-overlay" onclick={closeForm} onkeydown={(e) => { if (e.key === 'Escape') closeForm(); }} role="button" tabindex="-1">
      <div class="modal-card" onclick={(e) => e.stopPropagation()} role="dialog">
        <div class="modal-header">
          <h3 class="modal-title">{editingRule ? 'Edit Rule' : 'New Rule'}</h3>
          <button class="modal-close" onclick={closeForm}>&times;</button>
        </div>
        <div class="modal-body">
          <RuleForm rule={editingRule} onSave={handleSave} onCancel={closeForm} />
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: var(--z-modal, 400);
  }

  .modal-card {
    background: var(--color-bg);
    border-radius: var(--radius-lg);
    width: 620px;
    max-width: 90vw;
    max-height: 85vh;
    display: flex;
    flex-direction: column;
    box-shadow: var(--shadow-lg);
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--space-4) var(--space-6);
    border-bottom: 1px solid var(--color-border);
    flex-shrink: 0;
  }

  .modal-title {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .modal-close {
    background: none;
    border: none;
    font-size: var(--font-size-xl);
    color: var(--color-text-muted);
    cursor: pointer;
    padding: var(--space-1) var(--space-2);
    border-radius: var(--radius-sm);
    line-height: 1;
  }

  .modal-close:hover {
    color: var(--color-text);
    background: var(--color-surface-2);
  }

  .modal-body {
    overflow-y: auto;
    flex: 1;
  }

  .rules-page {
    max-width: 1200px;
    margin: 0 auto;
    padding: var(--space-6) var(--space-8);
  }

  .page-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: var(--space-6);
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

  .rules-section {
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    overflow: hidden;
  }

  .rules-list {
    display: flex;
    flex-direction: column;
  }

  .rule-row {
    display: flex;
    align-items: center;
    border-bottom: 1px solid var(--color-border);
    transition: background var(--transition-fast);
  }

  .rule-row:last-child {
    border-bottom: none;
  }

  .rule-row:hover {
    background: var(--color-surface);
  }

  .rule-row-main {
    flex: 1;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--space-4) var(--space-5);
    background: none;
    border: none;
    text-align: left;
    cursor: pointer;
    gap: var(--space-4);
  }

  .rule-info {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    min-width: 0;
  }

  .rule-name {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .rule-perf {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    font-weight: var(--font-weight-medium);
  }

  .rule-meta {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    flex-shrink: 0;
  }

  .rule-type-badge {
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-medium);
    color: var(--color-info);
    background: color-mix(in srgb, var(--color-info) 12%, transparent);
    padding: var(--space-1) var(--space-2);
    border-radius: var(--radius-full);
  }

  .rule-event {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .rule-version {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    font-family: var(--font-mono);
  }

  .rule-status {
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-semibold);
    padding: var(--space-1) var(--space-2);
    border-radius: var(--radius-full);
  }

  .status-active {
    color: var(--color-success);
    background: color-mix(in srgb, var(--color-success) 12%, transparent);
  }

  .status-inactive {
    color: var(--color-text-muted);
    background: var(--color-surface-2);
  }

  .rule-actions {
    padding-right: var(--space-4);
  }

  .toggle-btn {
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-medium);
    color: var(--color-text-muted);
    background: none;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    padding: var(--space-1) var(--space-3);
    cursor: pointer;
    transition:
      color var(--transition-fast),
      border-color var(--transition-fast);
  }

  .toggle-btn:hover {
    color: var(--color-text);
    border-color: var(--color-text-muted);
  }

  .empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--space-16);
    background: var(--color-surface);
    border: 1px dashed var(--color-border);
    border-radius: var(--radius-lg);
  }

  .empty-rules {
    padding: var(--space-12);
    text-align: center;
  }

  .empty-text {
    color: var(--color-text-muted);
    font-size: var(--font-size-base);
  }
</style>
