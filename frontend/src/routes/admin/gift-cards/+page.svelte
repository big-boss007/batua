<script lang="ts">
  import { Tabs, Button } from '@juspay/svelte-ui-components';

  import type { GiftCard, GiftCardStats, BulkIssueForm } from '$lib/client/modules/gift-cards';
  import {
    issueGiftCard,
    bulkIssue,
    fetchGiftCardStats,
    giftCards
  } from '$lib/client/modules/gift-cards';
  import {
    GiftCardsList,
    IssueGiftCardForm as IssueForm,
    BulkIssueForm as BulkForm,
    GiftCardDetail
  } from '$lib/client/modules/gift-cards/ui';
  import { currentMerchantId } from '$lib/client/modules/admin';
  import { toastStore, formatCurrencyINR } from '$lib/client/modules/foundation';

  let { data } = $props();

  let cards = $state<Array<GiftCard>>(data.cards);
  let selectedCard = $state<GiftCard | null>(null);
  let stats = $state<GiftCardStats | null>(null);
  let merchantId = $state<string | null>(null);

  const tabIds = ['list', 'issue', 'bulk'] as const;
  const tabItems = ['All Cards', 'Issue Card', 'Bulk Issue'];
  let activeTabIndex = $state(0);
  let activeTab = $derived(tabIds[activeTabIndex]);

  currentMerchantId.subscribe((id) => {
    const prevId = merchantId;
    merchantId = id;
    if (id !== null && id !== prevId) {
      loadStats(id);
    }
  });

  async function loadStats(mId: string) {
    const result = await fetchGiftCardStats(mId);
    if (result.tag === 'success') {
      stats = result.data;
    }
  }

  let statCards = $derived(
    stats !== null
      ? [
          { label: 'Total Issued', value: stats.total_issued.toLocaleString('en-IN') },
          { label: 'Outstanding Value', value: formatCurrencyINR(stats.total_outstanding_value) },
          { label: 'Active', value: stats.total_active.toLocaleString('en-IN') },
          { label: 'Claimed', value: stats.total_claimed.toLocaleString('en-IN') },
          { label: 'Expired', value: stats.total_expired.toLocaleString('en-IN') }
        ]
      : []
  );

  function handleTabChange(index: number) {
    activeTabIndex = index;
    selectedCard = null;
  }

  async function handleIssue(amount: number, expiresAt: string | null) {
    const result = await issueGiftCard({ amount, expires_at: expiresAt });
    if (result.tag === 'success') {
      cards = [result.data, ...cards];
      giftCards.add(result.data);
      toastStore.push({ message: 'Gift card issued', level: 'success' });
      activeTabIndex = 0;
      if (merchantId !== null) loadStats(merchantId);
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }
  }

  async function handleBulkIssue(form: BulkIssueForm) {
    const result = await bulkIssue(form);
    if (result.tag === 'success') {
      cards = [...result.data, ...cards];
      giftCards.addMany(result.data);
      toastStore.push({ message: `${result.data.length} gift cards issued`, level: 'success' });
      activeTabIndex = 0;
      if (merchantId !== null) loadStats(merchantId);
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }
  }

  function handleCardClick(card: GiftCard) {
    selectedCard = card;
  }
</script>

<div class="page">
  <div class="page-header">
    <h1 class="page-title">Gift Cards</h1>
  </div>

  {#if statCards.length > 0}
    <div class="stats-row">
      {#each statCards as stat (stat.label)}
        <div class="stat-card">
          <span class="stat-value">{stat.value}</span>
          <span class="stat-label">{stat.label}</span>
        </div>
      {/each}
    </div>
  {/if}

  <Tabs
    items={tabItems}
    activeIndex={activeTabIndex}
    onchange={(idx) => {
      handleTabChange(idx);
    }}
  />

  {#if selectedCard}
    <div class="detail-section">
      <Button
        text="Back to list"
        classes="btn-back"
        onclick={() => {
          selectedCard = null;
        }}
      />
      <GiftCardDetail card={selectedCard} />
    </div>
  {:else if activeTab === 'list'}
    <div class="card-list-wrapper" role="list">
      {#each cards as card (card.id)}
        <button class="card-row" onclick={() => handleCardClick(card)}>
          <GiftCardDetail {card} />
        </button>
      {/each}
      {#if cards.length === 0}
        <GiftCardsList {cards} />
      {/if}
    </div>
  {:else if activeTab === 'issue'}
    <div class="form-section">
      <IssueForm onIssue={handleIssue} />
    </div>
  {:else if activeTab === 'bulk'}
    <div class="form-section">
      <BulkForm onBulkIssue={handleBulkIssue} />
    </div>
  {/if}
</div>

<style>
  .page {
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
    max-width: 960px;
    padding: var(--space-8);
  }

  .page-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .page-title {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
  }

  .stats-row {
    display: grid;
    grid-template-columns: repeat(5, 1fr);
    gap: var(--space-3);
  }

  .stat-card {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    padding: var(--space-4) var(--space-5);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-sm);
    transition: box-shadow var(--transition-fast);
  }

  .stat-card:hover {
    box-shadow: var(--shadow-md);
  }

  .stat-value {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    line-height: var(--line-height-tight);
  }

  .stat-label {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    font-weight: var(--font-weight-medium);
  }

  .form-section {
    max-width: 480px;
  }

  .detail-section {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .card-list-wrapper {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .card-row {
    background: none;
    border: none;
    padding: 0;
    text-align: left;
    width: 100%;
    cursor: pointer;
  }

  :global(.btn-back) {
    --button-color: transparent;
    --button-text-color: var(--color-primary);
    --button-padding: 0;
    --button-font-size: var(--font-size-sm);
    --button-font-weight: var(--font-weight-medium);
    --button-hover-color: transparent;
    --button-hover-text-color: var(--color-primary-hover);
  }
</style>
