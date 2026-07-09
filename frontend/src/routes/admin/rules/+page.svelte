<script lang="ts">
  import type { Rule, RulePerformance, RewardRuleConfig } from '$lib/client/modules/rules';
  import { Button, Pill, Modal, Toggle } from '@juspay/svelte-ui-components';

  import {
    rulesStore,
    selectedRuleStore,
    fetchRules,
    createRule,
    updateRule,
    fetchRulePerformance
  } from '$lib/client/modules/rules';
  import { currentMerchantId } from '$lib/client/modules/admin';
  import { toastStore, formatCurrencyINR, MODAL_CLOSE_ICON } from '$lib/client/modules/foundation';
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
  <title>Earn Rules - Batua Admin</title>
</svelte:head>

<div class="rules-page">
  <header class="page-header">
    <div class="header-content">
      <h1 class="page-title">Earn rules</h1>
      <p class="page-description">Configure how customers earn and burn points</p>
    </div>
    <Button text="+ New rule" classes="btn-primary" onclick={handleCreate} />
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
            No earn rules configured yet. Create your first rule to get started.
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
                  <Pill text={rule.rule_type} classes="pill-info" />
                  <span class="rule-event">{rule.config.event_type}</span>
                  <span class="rule-version">v{rule.version}</span>
                  <Pill
                    text={rule.is_active ? 'Active' : 'Inactive'}
                    classes={rule.is_active ? 'pill-success' : 'pill-neutral'}
                  />
                </div>
              </button>
              <div class="rule-actions">
                <Toggle
                  checked={rule.is_active}
                  classes="rule-toggle"
                  onclick={() => handleToggle(rule)}
                />
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </section>
  {/if}

  {#if showForm}
    <Modal
      header={{ text: editingRule ? 'Edit rule' : 'New rule', rightImage: MODAL_CLOSE_ICON }}
      size="fit-content"
      onclose={closeForm}
      onoverlayClick={closeForm}
      onheaderRightImageClick={closeForm}
    >
      {#snippet content()}
        <RuleForm rule={editingRule} onSave={handleSave} onCancel={closeForm} />
      {/snippet}
    </Modal>
  {/if}
</div>

<style>
  .rules-page {
    max-width: 1200px;
    margin: 0 auto;
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
    background: var(--color-surface-2);
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

  .rule-event {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .rule-version {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    font-family: var(--font-mono);
  }

  .rule-actions {
    padding-right: var(--space-4);
  }

  :global(.rule-toggle) {
    --slider-checked-color: var(--color-primary);
    --slider-unchecked-color: var(--color-border);
    --toggle-switch-width: 36px;
    --toggle-switch-height: 20px;
    --toggle-ball-width: 16px;
    --toggle-ball-height: 16px;
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
