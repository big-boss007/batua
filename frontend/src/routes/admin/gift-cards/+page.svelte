<script lang="ts">
  import { Tabs, Table, Pill, Shimmer, Modal } from '@juspay/svelte-ui-components';

  import type { GiftCard, GiftCardStats, BulkIssueInput } from '$lib/client/modules/gift-cards';
  import {
    issueGiftCard,
    bulkIssue,
    fetchGiftCards,
    fetchGiftCardStats,
    giftCards
  } from '$lib/client/modules/gift-cards';
  import {
    IssueGiftCardForm as IssueForm,
    BulkIssueForm as BulkForm,
    GiftCardDetail,
    GiftCardConfirmation
  } from '$lib/client/modules/gift-cards/ui';
  import { currentMerchant, currentMerchantId } from '$lib/client/modules/admin';
  import { MetricCard } from '$lib/client/modules/admin/ui';
  import {
    toastStore,
    formatCurrencyINR,
    formatDateTime,
    MODAL_CLOSE_ICON
  } from '$lib/client/modules/foundation';

  let cards = $state<Array<GiftCard>>([]);
  let selectedCard = $state<GiftCard | null>(null);
  let stats = $state<GiftCardStats | null>(null);
  let merchantId = $state<string | null>(null);
  let merchantName = $state('');
  let loading = $state(false);
  let pendingIssue = $state<{ amount: number; expiresAt: string | null } | null>(null);

  currentMerchant.subscribe((m) => {
    if (m !== null) merchantName = m.name;
  });

  const tabIds = ['list', 'issue', 'bulk'] as const;
  const tabItems = ['All Cards', 'Issue card', 'Bulk issue'];
  let activeTabIndex = $state(0);
  let activeTab = $derived(tabIds[activeTabIndex]);

  const TABLE_HEADERS = ['Code', 'Amount', 'Balance', 'Status', 'Created'];

  const STATUS_PILL_CLASS: Record<string, string> = {
    active: 'pill-success',
    claimed: 'pill-info',
    expired: 'pill-warning',
    inactive: 'pill-neutral'
  };

  function capitalize(s: string): string {
    return s.charAt(0).toUpperCase() + s.slice(1);
  }

  function cardStatus(card: GiftCard): string {
    if (!card.is_active) return 'inactive';
    if (card.is_claimed) return 'claimed';
    if (card.expires_at && new Date(card.expires_at) < new Date()) return 'expired';
    return 'active';
  }

  let tableData = $derived(
    cards.map((card) => [
      `****-****-****-${card.code.slice(-4)}`,
      formatCurrencyINR(card.initial_amount),
      formatCurrencyINR(card.current_amount),
      cardStatus(card),
      formatDateTime(card.created_at)
    ])
  );

  currentMerchantId.subscribe((id) => {
    const prevId = merchantId;
    merchantId = id;
    if (id !== null && id !== prevId) {
      loadData(id);
    }
  });

  async function loadData(mId: string) {
    loading = true;
    const [cardsResult, statsResult] = await Promise.all([
      fetchGiftCards(mId),
      fetchGiftCardStats(mId)
    ]);
    if (cardsResult.tag === 'success') {
      cards = cardsResult.data;
    }
    if (statsResult.tag === 'success') {
      stats = statsResult.data;
    }
    loading = false;
  }

  let statCards = $derived(
    stats !== null
      ? [
          { label: 'Total issued', value: stats.total_issued.toLocaleString('en-IN') },
          { label: 'Outstanding value', value: formatCurrencyINR(stats.total_outstanding_value) },
          { label: 'Active', value: stats.total_active.toLocaleString('en-IN') },
          { label: 'Claimed', value: stats.total_claimed.toLocaleString('en-IN') },
          { label: 'Expired', value: stats.total_expired.toLocaleString('en-IN') }
        ]
      : []
  );

  function handleTabChange(index: number) {
    activeTabIndex = index;
    selectedCard = null;
    pendingIssue = null;
  }

  function handleRowClick(rowIndex: number) {
    const card = cards[rowIndex];
    if (card === undefined) return;
    selectedCard = card;
  }

  function handleCloseDetail() {
    selectedCard = null;
  }

  function handleIssue(amount: number, expiresAt: string | null) {
    pendingIssue = { amount, expiresAt };
  }

  async function handleConfirmIssue(): Promise<GiftCard | null> {
    if (merchantId === null || pendingIssue === null) return null;
    const expiresIso =
      pendingIssue.expiresAt !== null ? new Date(pendingIssue.expiresAt).toISOString() : null;
    try {
      const result = await issueGiftCard({
        merchant_id: merchantId,
        amount: pendingIssue.amount,
        expires_at: expiresIso
      });
      if (result.tag === 'success') {
        cards = [result.data, ...cards];
        giftCards.add(result.data);
        if (merchantId !== null) loadData(merchantId);
        return result.data;
      }
      toastStore.push({ message: result.message, level: 'error' });
      return null;
    } catch {
      return null;
    }
  }

  function handleCancelIssue() {
    pendingIssue = null;
  }

  async function handleBulkIssue(input: BulkIssueInput) {
    if (merchantId === null) return;
    const result = await bulkIssue({
      merchant_id: merchantId,
      batch_id: crypto.randomUUID(),
      cards: input.cards
    });
    if (result.tag === 'success') {
      cards = [...result.data, ...cards];
      giftCards.addMany(result.data);
      toastStore.push({ message: `${result.data.length} gift cards issued`, level: 'success' });
      activeTabIndex = 0;
      if (merchantId !== null) loadData(merchantId);
    } else {
      toastStore.push({ message: result.message, level: 'error' });
    }
  }
</script>

<svelte:head>
  <title>Gift Cards - Batua</title>
</svelte:head>

<div class="page">
  <header class="page-header">
    <h1 class="page-title">Gift cards</h1>
    <p class="page-subtitle">Issue and manage gift cards for your customers</p>
  </header>

  {#if statCards.length > 0}
    <div class="stats-row">
      {#each statCards as stat (stat.label)}
        <MetricCard label={stat.label} displayValue={stat.value} size="compact" />
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

  {#if activeTab === 'list'}
    <div class="cards-list">
      {#if loading}
        <div class="shimmer-rows">
          <Shimmer classes="shimmer-row" />
          <Shimmer classes="shimmer-row" />
          <Shimmer classes="shimmer-row" />
          <Shimmer classes="shimmer-row" />
        </div>
      {:else}
        <Table
          tableHeaders={TABLE_HEADERS}
          {tableData}
          sortable={false}
          onRowClick={handleRowClick}
          --table-row-hover-background="var(--color-surface-2)"
          --table-content-font-size="var(--font-size-sm)"
        >
          {#snippet cell(value, _rowIndex, colIndex)}
            {#if colIndex === 0}
              <code class="code-cell">{value}</code>
            {:else if colIndex === 3}
              <Pill
                text={capitalize(String(value))}
                classes={STATUS_PILL_CLASS[String(value)] ?? ''}
              />
            {:else}
              {value}
            {/if}
          {/snippet}
          {#snippet empty()}
            <p class="table-empty">No gift cards found</p>
          {/snippet}
        </Table>
      {/if}
    </div>

    {#if selectedCard !== null}
      {@const card = selectedCard}
      <Modal
        header={{ text: 'Gift card detail', rightImage: MODAL_CLOSE_ICON }}
        size="fit-content"
        onclose={handleCloseDetail}
        onoverlayClick={handleCloseDetail}
        onheaderRightImageClick={handleCloseDetail}
      >
        {#snippet content()}
          <GiftCardDetail {card} />
        {/snippet}
      </Modal>
    {/if}
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

{#if pendingIssue !== null}
  <GiftCardConfirmation
    amount={pendingIssue.amount}
    expiresAt={pendingIssue.expiresAt}
    {merchantName}
    onConfirm={handleConfirmIssue}
    onCancel={handleCancelIssue}
  />
{/if}

<style>
  .page {
    max-width: 1400px;
    margin: 0 auto;
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
    width: 100%;
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

  .stats-row {
    display: grid;
    grid-template-columns: repeat(5, 1fr);
    gap: var(--space-3);
  }

  .cards-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .code-cell {
    font-family: var(--font-mono);
    font-size: var(--font-size-xs);
    padding: var(--space-1) var(--space-2);
    background: var(--color-surface-2);
    border-radius: var(--radius-sm);
  }

  .table-empty {
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
    text-align: center;
    padding: var(--space-8);
  }

  .form-section {
    max-width: 480px;
  }

  .shimmer-rows {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
    padding: var(--space-4) 0;
  }

  :global(.shimmer-row) {
    --shimmer-width: 100%;
    --shimmer-height: 48px;
    --shimmer-border-radius: 4px;
  }
</style>
