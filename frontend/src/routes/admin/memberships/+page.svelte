<script lang="ts">
  import { Tabs, Shimmer, Input, Select, Pagination } from '@juspay/svelte-ui-components';
  import { currentMerchantId } from '$lib/client/modules/admin';
  import {
    toastStore,
    formatDateTime,
    formatDate,
    formatPhone,
    normalizePhoneE164
  } from '$lib/client/modules/foundation';
  import { lookupCustomer } from '$lib/client/modules/storefront';
  import type { EnrichedMembership, MembershipStatus } from '$lib/client/modules/memberships';
  import {
    assignMembership,
    cancelMembership,
    listSubscribersEnriched,
    getMembershipStatus
  } from '$lib/client/modules/memberships';
  import { AssignForm } from '$lib/client/modules/memberships/ui';

  type TierOption = { id: string; name: string; rank: number; earn_rate_multiplier: number };

  let merchantId = $state<string | null>(null);
  let tiers = $state<Array<TierOption>>([]);
  let subscribers = $state<Array<EnrichedMembership>>([]);
  let loading = $state(false);
  let formLoading = $state(false);
  let formLookingUp = $state(false);
  let formLookupError = $state<string | null>(null);

  let resolvedCustomerId = $state<string | null>(null);
  let resolvedCustomer = $state<{
    name: string;
    phone: string;
    walletBalance: number | null;
    currentTierName: string | null;
    currentTierRank: number;
    currentTierMultiplier: number;
    currentExpiry: string | null;
    joinedAt: string | null;
  } | null>(null);
  let assignResult = $state<{
    tierName: string;
    multiplier: number;
    expiresAt: string;
    previousTierName: string | null;
  } | null>(null);

  // Detail modal
  let selectedMembership = $state<EnrichedMembership | null>(null);
  let selectedStatus = $state<MembershipStatus | null>(null);
  let detailLoading = $state(false);
  let selectedEarnedTier = $state<string | null>(null);

  // Tabs
  const tabItems = ['Subscribers', 'Assign Membership'];
  const tabIds = ['subscribers', 'assign'] as const;
  let activeTabIndex = $state(0);
  let activeTab = $derived(tabIds[activeTabIndex]);

  // Filters
  let searchQuery = $state('');
  let filterTier = $state('all');
  let filterStatus = $state('all');
  let currentPage = $state(1);
  const pageSize = 10;

  let uniqueTierNames = $derived([...new Set(subscribers.map((s) => s.tier_name))].sort());
  let tierFilterItems = $derived([
    { id: 'all', label: 'All Tiers' },
    ...uniqueTierNames.map((t) => ({ id: t, label: t }))
  ]);
  let statusFilterItems = [
    { id: 'all', label: 'All Status' },
    { id: 'active', label: 'Active' },
    { id: 'expiring', label: 'Expiring Soon' },
    { id: 'cancelled', label: 'Cancelled' },
    { id: 'expired', label: 'Expired' }
  ];

  function daysRemaining(expiresAt: string): number {
    const diff = new Date(expiresAt).getTime() - Date.now();
    return Math.ceil(diff / (1000 * 60 * 60 * 24));
  }

  function memberStatus(sub: EnrichedMembership): 'active' | 'expiring' | 'cancelled' | 'expired' {
    if (sub.status === 'cancelled') return 'cancelled';
    const days = daysRemaining(sub.expires_at);
    if (days <= 0) return 'expired';
    if (days <= 30) return 'expiring';
    return 'active';
  }

  function tierColorClass(tierName: string): string {
    const lower = tierName.toLowerCase();
    if (lower === 'bronze') return 't-bronze';
    if (lower === 'silver') return 't-silver';
    if (lower === 'gold') return 't-gold';
    if (lower === 'platinum') return 't-plat';
    return 't-default';
  }

  let filtered = $derived(
    subscribers.filter((s) => {
      const status = memberStatus(s);
      if (filterTier !== 'all' && s.tier_name !== filterTier) return false;
      if (filterStatus !== 'all' && status !== filterStatus) return false;
      if (searchQuery.trim().length > 0) {
        const q = searchQuery.toLowerCase();
        const name = (s.customer_name ?? '').toLowerCase();
        const phone = s.customer_phone.toLowerCase();
        if (!name.includes(q) && !phone.includes(q)) return false;
      }
      return true;
    })
  );

  let totalPages = $derived(Math.max(1, Math.ceil(filtered.length / pageSize)));
  let pagedData = $derived(filtered.slice((currentPage - 1) * pageSize, currentPage * pageSize));

  currentMerchantId.subscribe((id) => {
    const prevId = merchantId;
    merchantId = id;
    if (id !== null && id !== prevId) {
      if (typeof window !== 'undefined') loadData(id);
    }
  });

  async function loadData(mId: string) {
    loading = true;
    const [subsResult, tiersResult] = await Promise.all([
      listSubscribersEnriched(mId),
      fetchLoyaltyTiers(mId)
    ]);
    if (subsResult.tag === 'success') subscribers = subsResult.data;
    tiers = tiersResult;
    loading = false;
  }

  async function fetchLoyaltyTiers(mId: string): Promise<Array<TierOption>> {
    try {
      const programRes = await fetch(`http://localhost:3000/loyalty/programs/${mId}`);
      if (!programRes.ok) return [];
      const program = await programRes.json();
      const tiersRes = await fetch(`http://localhost:3000/loyalty/programs/${program.id}/tiers`);
      if (!tiersRes.ok) return [];
      const tiersData = await tiersRes.json();
      if (!Array.isArray(tiersData)) return [];
      return tiersData.map((t: Record<string, unknown>) => ({
        id: (t['id'] as string) ?? '',
        name: (t['name'] as string) ?? '',
        rank: (t['rank'] as number) ?? 0,
        earn_rate_multiplier: (t['earn_rate_multiplier'] as number) ?? 1.0
      }));
    } catch {
      return [];
    }
  }

  async function handleRowClick(sub: EnrichedMembership) {
    selectedMembership = sub;
    selectedStatus = null;
    selectedEarnedTier = null;
    detailLoading = true;
    if (merchantId !== null) {
      const result = await getMembershipStatus(merchantId, sub.customer_id);
      if (result.tag === 'success') {
        selectedStatus = result.data;
      }
      try {
        const tierRes = await fetch(`http://localhost:3000/loyalty/customers/${merchantId}/${sub.customer_id}`);
        if (tierRes.ok) {
          const tierData = await tierRes.json();
          selectedEarnedTier = (tierData?.tier?.name as string) ?? null;
        }
      } catch { /* no earned tier */ }
    }
    detailLoading = false;
  }

  function closeDetail() {
    selectedMembership = null;
    selectedStatus = null;
    selectedEarnedTier = null;
  }

  async function handleCancelFromDetail() {
    if (selectedMembership === null) return;
    const result = await cancelMembership(selectedMembership.id);
    if (result.tag === 'success') {
      subscribers = subscribers.map((s) =>
        s.id === selectedMembership!.id
          ? { ...s, status: 'cancelled', cancelled_at: new Date().toISOString() }
          : s
      );
      selectedMembership = { ...selectedMembership, status: 'cancelled', cancelled_at: new Date().toISOString() };
      toastStore.push({ message: 'Membership cancelled', level: 'success' });
    } else {
      toastStore.push({ message: 'Failed to cancel membership', level: 'error' });
    }
  }

  async function handleCustomerLookup(phone: string) {
    if (merchantId === null) return;
    formLookingUp = true;
    formLookupError = null;
    resolvedCustomerId = null;
    resolvedCustomer = null;
    assignResult = null;

    const normalized = normalizePhoneE164(phone);
    const customerResult = await lookupCustomer(normalized);
    if (customerResult.tag === 'error' || customerResult.data.length === 0) {
      formLookupError = 'No registered customer with this phone number.';
      formLookingUp = false;
      return;
    }

    const cust = customerResult.data[0];
    resolvedCustomerId = cust.id;
    let tierName: string | null = null;
    let tierRank = 0;
    let tierMult = 1;
    let tierExpiry: string | null = null;

    try {
      const tierRes = await fetch(`http://localhost:3000/loyalty/customers/${merchantId}/${cust.id}`);
      if (tierRes.ok) {
        const tierData = await tierRes.json();
        tierName = (tierData?.tier?.name as string) ?? null;
        tierRank = (tierData?.tier?.rank as number) ?? 0;
        tierMult = (tierData?.tier?.earn_rate_multiplier as number) ?? 1;
      }
    } catch { /* no tier */ }

    try {
      const statusRes = await fetch(`http://localhost:3000/earn/memberships/status/${merchantId}/${cust.id}`);
      if (statusRes.ok) {
        const statusData = await statusRes.json();
        if (statusData?.membership?.expires_at) {
          tierExpiry = formatDateTime(statusData.membership.expires_at as string);
        }
        const memMult = (statusData?.earn_rate_multiplier as number) ?? 0;
        if (statusData?.is_active && memMult > tierMult) {
          tierName = (statusData.tier_name as string) ?? tierName;
          tierMult = memMult;
        }
      }
    } catch { /* no membership status */ }

    resolvedCustomer = {
      name: cust.name ?? cust.phone,
      phone: cust.phone,
      walletBalance: null,
      currentTierName: tierName,
      currentTierRank: tierRank,
      currentTierMultiplier: tierMult,
      currentExpiry: tierExpiry,
      joinedAt: null
    };
    formLookingUp = false;
  }

  async function handleAssign(data: { tier_id: string }) {
    if (merchantId === null || resolvedCustomerId === null) return;
    formLoading = true;
    const result = await assignMembership({
      merchant_id: merchantId,
      customer_id: resolvedCustomerId,
      tier_id: data.tier_id
    });
    formLoading = false;
    if (result.tag === 'success') {
      if (merchantId !== null) loadData(merchantId);
      assignResult = {
        tierName: result.data.tier_name,
        multiplier: result.data.earn_rate_multiplier,
        expiresAt: formatDateTime(result.data.membership.expires_at),
        previousTierName: resolvedCustomer?.currentTierName ?? null
      };
    } else {
      toastStore.push({ message: 'Failed to assign membership', level: 'error' });
    }
  }

  function handleTabChange(index: number) {
    activeTabIndex = index;
  }
</script>

<svelte:head>
  <title>Memberships - Batua</title>
</svelte:head>

<div class="memberships-page">
  <div class="page-header">
    <h1 class="page-title">Memberships</h1>
    <p class="page-subtitle">Assign loyalty tiers to customers with a 1-year expiry</p>
  </div>

  <Tabs items={tabItems} activeIndex={activeTabIndex} onchange={handleTabChange} />

  <div class="tab-content">
    {#if loading}
      <div class="loading">
        <Shimmer classes="shimmer-table" />
        <Shimmer classes="shimmer-table" />
        <Shimmer classes="shimmer-table" />
      </div>
    {:else if activeTab === 'subscribers'}
      {#if subscribers.length === 0 && !searchQuery && filterTier === 'all' && filterStatus === 'all'}
        <div class="empty-state">
          <div class="empty-icon">👥</div>
          <p>No memberships assigned yet.</p>
          <button class="link-btn" onclick={() => { activeTabIndex = 1; }}>Assign a membership →</button>
        </div>
      {:else}
        <div class="table-card">
          <div class="filters-bar">
            <input
              class="filter-input"
              placeholder="Search by name or phone..."
              value={searchQuery}
              oninput={(e) => { searchQuery = (e.target as HTMLInputElement).value; currentPage = 1; }}
            />
            <select class="filter-select" onchange={(e) => { filterTier = (e.target as HTMLSelectElement).value; currentPage = 1; }}>
              {#each tierFilterItems as item}
                <option value={item.id} selected={filterTier === item.id}>{item.label}</option>
              {/each}
            </select>
            <select class="filter-select" onchange={(e) => { filterStatus = (e.target as HTMLSelectElement).value; currentPage = 1; }}>
              {#each statusFilterItems as item}
                <option value={item.id} selected={filterStatus === item.id}>{item.label}</option>
              {/each}
            </select>
            <span class="filter-count">
              {#if filtered.length === subscribers.length}
                {filtered.length} memberships
              {:else}
                Showing {filtered.length} of {subscribers.length}
              {/if}
            </span>
          </div>

          {#if filtered.length === 0}
            <div class="empty-state" style="padding: 32px 16px;">
              <p>No memberships match your filters.</p>
            </div>
          {:else}
            <table>
              <thead>
                <tr>
                  <th>Customer</th>
                  <th>Tier</th>
                  <th>Multiplier</th>
                  <th>Status</th>
                  <th>Started</th>
                  <th>Expires</th>
                </tr>
              </thead>
              <tbody>
                {#each pagedData as sub (sub.id)}
                  {@const status = memberStatus(sub)}
                  {@const days = daysRemaining(sub.expires_at)}
                  <tr onclick={() => handleRowClick(sub)}>
                    <td>
                      <div class="cell-customer">
                        <span class="cell-name">{sub.customer_name ?? 'Unnamed'}</span>
                        <span class="cell-phone">{formatPhone(sub.customer_phone)}</span>
                      </div>
                    </td>
                    <td><span class="tier-badge {tierColorClass(sub.tier_name)}">{sub.tier_name}</span></td>
                    <td><span class="mult-badge">{sub.earn_rate_multiplier}x</span></td>
                    <td>
                      <span class="status-pill s-{status}">
                        {status === 'active' ? 'Active' : status === 'expiring' ? 'Expiring Soon' : status === 'cancelled' ? 'Cancelled' : 'Expired'}
                      </span>
                    </td>
                    <td><span class="cell-date">{formatDate(sub.started_at)}</span></td>
                    <td>
                      {#if status === 'cancelled'}
                        <span class="cell-date strikethrough muted">{formatDate(sub.expires_at)}</span>
                        <div class="cell-sub">cancelled {sub.cancelled_at ? formatDate(sub.cancelled_at) : ''}</div>
                      {:else if status === 'expired'}
                        <span class="cell-date muted">{formatDate(sub.expires_at)}</span>
                        <div class="cell-sub">expired {Math.abs(days)} days ago</div>
                      {:else if status === 'expiring'}
                        <span class="cell-date warn">{formatDate(sub.expires_at)}</span>
                        <div class="cell-sub warn">in {days} days</div>
                      {:else}
                        <span class="cell-date">{formatDate(sub.expires_at)}</span>
                        <div class="cell-sub">in {days} days</div>
                      {/if}
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>

            {#if totalPages > 1}
              <div class="pagination-bar">
                <span class="pagination-info">Showing {(currentPage - 1) * pageSize + 1}–{Math.min(currentPage * pageSize, filtered.length)} of {filtered.length} memberships</span>
                <div class="page-btns">
                  <button class="page-btn" disabled={currentPage <= 1} onclick={() => { currentPage = currentPage - 1; }}>‹</button>
                  {#each Array.from({ length: totalPages }, (_, i) => i + 1) as p}
                    <button class="page-btn" class:active={currentPage === p} onclick={() => { currentPage = p; }}>{p}</button>
                  {/each}
                  <button class="page-btn" disabled={currentPage >= totalPages} onclick={() => { currentPage = currentPage + 1; }}>›</button>
                </div>
              </div>
            {/if}
          {/if}
        </div>
      {/if}

    {:else if activeTab === 'assign'}
      <div class="form-container">
        <AssignForm
          {tiers}
          onLookup={handleCustomerLookup}
          onSubmit={handleAssign}
          onReset={() => { resolvedCustomerId = null; resolvedCustomer = null; assignResult = null; formLookupError = null; }}
          loading={formLoading}
          lookingUp={formLookingUp}
          lookupError={formLookupError}
          customer={resolvedCustomer}
          result={assignResult}
        />
      </div>
    {/if}
  </div>
</div>

<!-- ═══════════════════════════════════════════ -->
<!-- Detail Modal                               -->
<!-- ═══════════════════════════════════════════ -->
{#if selectedMembership !== null}
  {@const status = memberStatus(selectedMembership)}
  {@const days = daysRemaining(selectedMembership.expires_at)}
  {@const progressPct = status === 'active' || status === 'expiring' ? Math.min(100, Math.max(0, (days / 365) * 100)) : 0}

  <div class="modal-overlay" onclick={closeDetail} onkeydown={(e) => { if (e.key === 'Escape') closeDetail(); }} role="button" tabindex="-1">
    <div class="modal-panel" onclick={(e) => e.stopPropagation()} role="dialog">
      <div class="modal-header">
        <span class="modal-title">Membership Details</span>
        <button class="modal-close" onclick={closeDetail}>&times;</button>
      </div>
      <div class="modal-body">
        <!-- Customer header -->
        <div class="detail-header">
          <div class="detail-customer">
            <span class="detail-name">{selectedMembership.customer_name ?? 'Unnamed'}</span>
            <span class="detail-phone">{formatPhone(selectedMembership.customer_phone)}</span>
          </div>
          <span class="status-pill s-{status}">
            {status === 'active' ? 'Active' : status === 'expiring' ? 'Expiring Soon' : status === 'cancelled' ? 'Cancelled' : 'Expired'}
          </span>
        </div>

        <!-- Expiring warning banner -->
        {#if status === 'expiring'}
          <div class="detail-banner amber">
            <strong>Expires in {days} days.</strong> Renew to keep this customer at {selectedMembership.tier_name} with {selectedMembership.earn_rate_multiplier}x earn rate.
          </div>
        {/if}

        <!-- Cancelled banner -->
        {#if status === 'cancelled'}
          <div class="detail-banner red">
            Membership was cancelled on <strong>{selectedMembership.cancelled_at ? formatDate(selectedMembership.cancelled_at) : 'unknown'}</strong>. The customer has fallen back to their earned tier.
          </div>
        {/if}

        <!-- Detail grid -->
        <div class="detail-grid">
          {#if status === 'cancelled'}
            <div class="detail-field">
              <span class="detail-label">Tier (was)</span>
              <span class="detail-value strikethrough muted"><span class="tier-badge {tierColorClass(selectedMembership.tier_name)}" style="opacity: 0.5;">{selectedMembership.tier_name}</span></span>
            </div>
            <div class="detail-field">
              <span class="detail-label">Current Earned Tier</span>
              <span class="detail-value">
                {#if selectedEarnedTier}
                  <span class="tier-badge {tierColorClass(selectedEarnedTier)}">{selectedEarnedTier}</span>
                {:else}
                  <span class="muted">None</span>
                {/if}
              </span>
            </div>
          {:else}
            <div class="detail-field">
              <span class="detail-label">Tier</span>
              <span class="detail-value"><span class="tier-badge {tierColorClass(selectedMembership.tier_name)}">{selectedMembership.tier_name}</span></span>
            </div>
            <div class="detail-field">
              <span class="detail-label">Multiplier</span>
              <span class="detail-value mono">{selectedMembership.earn_rate_multiplier}x</span>
            </div>
          {/if}
          <div class="detail-field">
            <span class="detail-label">Started</span>
            <span class="detail-value">{formatDate(selectedMembership.started_at)}</span>
          </div>
          <div class="detail-field">
            <span class="detail-label">{status === 'cancelled' ? 'Cancelled' : 'Expires'}</span>
            <span class="detail-value" class:warn={status === 'expiring'} class:error={status === 'cancelled'}>
              {#if status === 'cancelled'}
                {selectedMembership.cancelled_at ? formatDate(selectedMembership.cancelled_at) : '—'}
              {:else}
                {formatDate(selectedMembership.expires_at)}
              {/if}
            </span>
          </div>
          <div class="detail-field">
            <span class="detail-label">Renewed</span>
            <span class="detail-value">{selectedMembership.renewed_count} {selectedMembership.renewed_count === 1 ? 'time' : 'times'}</span>
          </div>
          {#if status === 'expiring' && selectedEarnedTier}
            <div class="detail-field">
              <span class="detail-label">Earned Tier (fallback)</span>
              <span class="detail-value"><span class="tier-badge {tierColorClass(selectedEarnedTier)}" style="font-size: 10px;">{selectedEarnedTier}</span></span>
            </div>
          {/if}
          {#if status === 'cancelled'}
            <div class="detail-field">
              <span class="detail-label">Was set to expire</span>
              <span class="detail-value strikethrough muted">{formatDate(selectedMembership.expires_at)}</span>
            </div>
          {/if}
        </div>

        <!-- Progress bar (active/expiring only) -->
        {#if status === 'active' || status === 'expiring'}
          <div class="detail-progress">
            <div class="detail-progress-label">
              <span>Time remaining</span>
              <span class="detail-progress-value" class:warn={status === 'expiring'}>{days} days left</span>
            </div>
            <div class="detail-progress-bar">
              <div
                class="detail-progress-fill"
                class:green={status === 'active'}
                class:amber={status === 'expiring'}
                style="width: {progressPct}%;"
              ></div>
            </div>
          </div>
        {/if}

        <!-- Loading status -->
        {#if detailLoading}
          <Shimmer classes="shimmer-detail" />
        {/if}

        <!-- Actions -->
        <div class="detail-actions">
          {#if status === 'expiring'}
            <button class="action-btn action-renew">↻ Renew for 1 Year</button>
            <button class="action-btn action-upgrade">⬆ Upgrade Tier</button>
            <button class="action-btn action-cancel" onclick={handleCancelFromDetail}>Cancel</button>
          {:else if status === 'active'}
            <button class="action-btn action-upgrade">⬆ Upgrade Tier</button>
            <button class="action-btn action-extend">+ Extend</button>
            <button class="action-btn action-cancel" onclick={handleCancelFromDetail}>Cancel Membership</button>
          {:else if status === 'cancelled'}
            <button class="action-btn action-upgrade" style="border-color: var(--color-primary);">↻ Re-assign Membership</button>
          {/if}
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .memberships-page {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    max-width: 1100px;
  }

  .page-header { margin-bottom: var(--space-2); }
  .page-title {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    margin-bottom: var(--space-1);
  }
  .page-subtitle { font-size: var(--font-size-base); color: var(--color-text-muted); }
  .tab-content { min-height: 300px; }
  .form-container { max-width: 480px; padding-top: var(--space-4); }

  /* ── Table card ── */
  .table-card {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    overflow: hidden;
  }

  .filters-bar {
    display: flex;
    gap: var(--space-3);
    align-items: center;
    padding: var(--space-4);
    border-bottom: 1px solid var(--color-border);
  }

  .filter-input {
    flex: 1;
    max-width: 260px;
    padding: 8px 12px;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
    color: var(--color-text);
    outline: none;
    background: var(--color-surface);
  }
  .filter-input:focus { border-color: var(--color-primary); }
  .filter-input::placeholder { color: var(--color-text-muted); }

  .filter-select {
    padding: 8px 12px;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
    color: var(--color-text);
    background: var(--color-surface);
    outline: none;
  }

  .filter-count { font-size: var(--font-size-xs); color: var(--color-text-muted); margin-left: auto; white-space: nowrap; }

  /* ── Table ── */
  table { width: 100%; border-collapse: collapse; font-size: var(--font-size-sm); }
  th {
    text-align: left;
    padding: 10px 16px;
    font-size: 11px;
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.04em;
    border-bottom: 1px solid var(--color-border);
    background: var(--color-surface-2, #f9fafb);
  }
  td { padding: 14px 16px; border-bottom: 1px solid var(--color-border-light, #f3f4f6); vertical-align: middle; }
  tr:last-child td { border-bottom: none; }
  tbody tr { cursor: pointer; transition: background 0.1s; }
  tbody tr:hover td { background: var(--color-surface-2, #fafbfc); }

  .cell-customer { display: flex; flex-direction: column; gap: 1px; }
  .cell-name { font-weight: var(--font-weight-medium); color: var(--color-text); }
  .cell-phone { font-size: 11px; color: var(--color-text-muted); font-family: var(--font-mono); }

  .tier-badge {
    display: inline-block;
    padding: 3px 10px;
    border-radius: 12px;
    font-size: 11px;
    font-weight: var(--font-weight-semibold);
  }
  .t-bronze { background: #fef3c7; color: #92400e; }
  .t-silver { background: #f3f4f6; color: #374151; }
  .t-gold { background: #fef9c3; color: #854d0e; }
  .t-plat { background: #ede9fe; color: #5b21b6; }
  .t-default { background: var(--color-surface-2); color: var(--color-text); }

  .status-pill {
    display: inline-block;
    padding: 3px 10px;
    border-radius: 12px;
    font-size: 11px;
    font-weight: var(--font-weight-semibold);
  }
  .s-active { background: #d1fae5; color: #065f46; }
  .s-expiring { background: #fef3c7; color: #92400e; }
  .s-cancelled { background: #fee2e2; color: #991b1b; }
  .s-expired { background: #f3f4f6; color: #6b7280; }

  .mult-badge { font-size: 11px; font-weight: var(--font-weight-semibold); color: var(--color-text-muted); font-family: var(--font-mono); }
  .cell-date { font-size: var(--font-size-xs); color: var(--color-text); }
  .cell-sub { font-size: 11px; color: var(--color-text-muted); }
  .cell-sub.warn, .cell-date.warn { color: #f59e0b; font-weight: var(--font-weight-medium); }
  .strikethrough { text-decoration: line-through; }
  .muted { color: var(--color-text-muted); }
  .warn { color: #f59e0b; font-weight: var(--font-weight-semibold); }
  .error { color: #ef4444; }
  .mono { font-family: var(--font-mono); }

  /* ── Pagination ── */
  .pagination-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    border-top: 1px solid var(--color-border);
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }
  .pagination-info { font-size: var(--font-size-xs); }
  .page-btns { display: flex; gap: 4px; }
  .page-btn {
    width: 28px; height: 28px;
    display: flex; align-items: center; justify-content: center;
    border: 1px solid var(--color-border); border-radius: 6px;
    background: var(--color-surface); cursor: pointer;
    font-size: var(--font-size-xs); color: var(--color-text);
  }
  .page-btn.active { background: var(--color-primary); color: #fff; border-color: var(--color-primary); }
  .page-btn:hover:not(.active) { background: var(--color-surface-2); }
  .page-btn:disabled { opacity: 0.4; cursor: default; }

  /* ── Empty state ── */
  .empty-state {
    display: flex; flex-direction: column; align-items: center; gap: var(--space-3);
    padding: var(--space-12) var(--space-4); color: var(--color-text-muted); font-size: var(--font-size-base);
  }
  .empty-icon { font-size: 36px; }
  .link-btn {
    background: none; border: none; color: var(--color-primary);
    font-size: var(--font-size-base); font-weight: var(--font-weight-medium);
    cursor: pointer; padding: var(--space-2) var(--space-4); border-radius: var(--radius-md);
  }
  .link-btn:hover { background: color-mix(in srgb, var(--color-primary) 6%, transparent); }

  .loading { display: flex; flex-direction: column; gap: var(--space-3); padding-top: var(--space-4); }
  :global(.shimmer-table) { --shimmer-width: 100%; --shimmer-height: 48px; --shimmer-border-radius: 8px; }
  :global(.shimmer-detail) { --shimmer-width: 100%; --shimmer-height: 60px; --shimmer-border-radius: 8px; }

  /* ── Modal ── */
  .modal-overlay {
    position: fixed; inset: 0; background: rgba(0, 0, 0, 0.4);
    display: flex; align-items: center; justify-content: center; z-index: var(--z-modal, 400);
  }
  .modal-panel {
    background: var(--color-bg); border-radius: var(--radius-lg);
    width: 520px; max-width: 90vw; max-height: 85vh;
    box-shadow: var(--shadow-lg); display: flex; flex-direction: column;
  }
  .modal-header {
    display: flex; justify-content: space-between; align-items: center;
    padding: var(--space-4) var(--space-5); border-bottom: 1px solid var(--color-border);
  }
  .modal-title { font-size: var(--font-size-md); font-weight: var(--font-weight-semibold); color: var(--color-text); }
  .modal-close {
    background: none; border: none; font-size: 20px; color: var(--color-text-muted);
    cursor: pointer; padding: 2px 6px; border-radius: var(--radius-sm); line-height: 1;
  }
  .modal-close:hover { color: var(--color-text); background: var(--color-surface-2); }
  .modal-body { padding: var(--space-5); overflow-y: auto; flex: 1; }

  /* ── Detail modal content ── */
  .detail-header {
    display: flex; align-items: center; justify-content: space-between; margin-bottom: var(--space-5);
  }
  .detail-customer { display: flex; flex-direction: column; gap: 2px; }
  .detail-name { font-size: var(--font-size-md); font-weight: var(--font-weight-semibold); color: var(--color-text); }
  .detail-phone { font-size: var(--font-size-xs); color: var(--color-text-muted); font-family: var(--font-mono); }

  .detail-banner {
    padding: 10px 14px; border-radius: var(--radius-md); margin-bottom: var(--space-5);
    font-size: var(--font-size-xs); line-height: 1.5;
  }
  .detail-banner.amber { background: #fffbeb; border: 1px solid #fde68a; color: #92400e; }
  .detail-banner.red { background: #fef2f2; border: 1px solid #fecaca; color: #991b1b; }

  .detail-grid {
    display: grid; grid-template-columns: 1fr 1fr; gap: 14px;
    padding: var(--space-4); background: var(--color-surface-2, #f9fafb);
    border-radius: var(--radius-md); margin-bottom: var(--space-5);
  }
  .detail-field { display: flex; flex-direction: column; gap: 2px; }
  .detail-label { font-size: 11px; color: var(--color-text-muted); text-transform: uppercase; letter-spacing: 0.04em; }
  .detail-value { font-size: var(--font-size-sm); font-weight: var(--font-weight-medium); color: var(--color-text); }

  .detail-progress { margin-bottom: var(--space-5); }
  .detail-progress-label {
    font-size: var(--font-size-xs); color: var(--color-text-muted);
    margin-bottom: 6px; display: flex; justify-content: space-between;
  }
  .detail-progress-value { font-weight: var(--font-weight-medium); color: var(--color-text); }
  .detail-progress-value.warn { color: #f59e0b; }
  .detail-progress-bar { height: 6px; border-radius: 3px; background: var(--color-border); overflow: hidden; }
  .detail-progress-fill { height: 100%; border-radius: 3px; transition: width 0.3s; }
  .detail-progress-fill.green { background: #22c55e; }
  .detail-progress-fill.amber { background: #f59e0b; }

  .detail-actions { display: flex; gap: 8px; flex-wrap: wrap; }
  .action-btn {
    padding: 8px 16px; border-radius: var(--radius-md); font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium); cursor: pointer;
    border: 1px solid; display: inline-flex; align-items: center; gap: 6px;
    background: var(--color-surface);
  }
  .action-renew { background: #22c55e; color: #fff; border-color: #22c55e; font-weight: var(--font-weight-semibold); }
  .action-renew:hover { background: #16a34a; }
  .action-upgrade { color: var(--color-primary); border-color: #c7d2fe; }
  .action-upgrade:hover { background: color-mix(in srgb, var(--color-primary) 6%, transparent); }
  .action-extend { color: #22c55e; border-color: #bbf7d0; }
  .action-extend:hover { background: #f0fdf4; }
  .action-cancel { color: #ef4444; border-color: #fecaca; }
  .action-cancel:hover { background: #fef2f2; }
</style>
