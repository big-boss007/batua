<script lang="ts">
  import {
    Tabs,
    Shimmer,
    Input,
    Select,
    Pagination,
    Pill,
    Progress,
    Modal,
    Button
  } from '@juspay/svelte-ui-components';
  import { currentMerchantId } from '$lib/client/modules/admin';
  import {
    toastStore,
    formatDateTime,
    formatDate,
    formatPhone,
    normalizePhoneE164,
    MODAL_CLOSE_ICON
  } from '$lib/client/modules/foundation';
  import { lookupCustomer } from '$lib/client/modules/storefront';
  import type { EnrichedMembership, MembershipStatus } from '$lib/client/modules/memberships';
  import {
    assignMembership,
    cancelMembership,
    listSubscribersEnriched,
    getMembershipStatus,
    upgradeMembership,
    extendMembership,
    renewMembership
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
  let modalMode = $state<'default' | 'cancel-confirm' | 'upgrade' | 'extend' | 'renew-confirm'>(
    'default'
  );
  let selectedUpgradeTierId = $state('');
  let extendDays = $state(365);
  let actionLoading = $state(false);

  // Tabs
  const tabItems = ['Subscribers', 'Assign membership'];
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
    { id: 'all', label: 'All tiers' },
    ...uniqueTierNames.map((t) => ({ id: t, label: t }))
  ]);
  let statusFilterItems = [
    { id: 'all', label: 'All status' },
    { id: 'active', label: 'Active' },
    { id: 'expiring', label: 'Expiring soon' },
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
    modalMode = 'default';
    selectedUpgradeTierId = '';
    extendDays = 365;
    detailLoading = true;
    if (merchantId !== null) {
      const result = await getMembershipStatus(merchantId, sub.customer_id);
      if (result.tag === 'success') {
        selectedStatus = result.data;
      }
      try {
        const tierRes = await fetch(
          `http://localhost:3000/loyalty/customers/${merchantId}/${sub.customer_id}`
        );
        if (tierRes.ok) {
          const tierData = await tierRes.json();
          selectedEarnedTier = (tierData?.tier?.name as string) ?? null;
        }
      } catch {
        /* no earned tier */
      }
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
      selectedMembership = {
        ...selectedMembership,
        status: 'cancelled',
        cancelled_at: new Date().toISOString()
      };
      toastStore.push({ message: 'Membership cancelled', level: 'success' });
    } else {
      toastStore.push({ message: 'Failed to cancel membership', level: 'error' });
    }
  }

  async function handleUpgradeFromDetail() {
    if (selectedMembership === null || !selectedUpgradeTierId) return;
    actionLoading = true;
    const result = await upgradeMembership(selectedMembership.id, selectedUpgradeTierId);
    actionLoading = false;
    if (result.tag === 'success') {
      toastStore.push({ message: 'Membership upgraded', level: 'success' });
      if (merchantId !== null) loadData(merchantId);
      closeDetail();
    } else {
      toastStore.push({ message: 'Failed to upgrade membership', level: 'error' });
    }
  }

  async function handleExtendFromDetail() {
    if (selectedMembership === null || extendDays <= 0) return;
    actionLoading = true;
    const result = await extendMembership(selectedMembership.id, extendDays);
    actionLoading = false;
    if (result.tag === 'success') {
      toastStore.push({ message: `Membership extended by ${extendDays} days`, level: 'success' });
      if (merchantId !== null) loadData(merchantId);
      closeDetail();
    } else {
      toastStore.push({ message: 'Failed to extend membership', level: 'error' });
    }
  }

  async function handleRenewFromDetail() {
    if (selectedMembership === null) return;
    actionLoading = true;
    const result = await renewMembership(selectedMembership.id);
    actionLoading = false;
    if (result.tag === 'success') {
      toastStore.push({ message: 'Membership renewed for 1 year', level: 'success' });
      if (merchantId !== null) loadData(merchantId);
      closeDetail();
    } else {
      toastStore.push({ message: 'Failed to renew membership', level: 'error' });
    }
  }

  async function handleReassignFromDetail() {
    if (selectedMembership === null || merchantId === null || !selectedUpgradeTierId) return;
    actionLoading = true;
    const result = await assignMembership({
      merchant_id: merchantId,
      customer_id: selectedMembership.customer_id,
      tier_id: selectedUpgradeTierId
    });
    actionLoading = false;
    if (result.tag === 'success') {
      toastStore.push({ message: 'Membership re-assigned', level: 'success' });
      if (merchantId !== null) loadData(merchantId);
      closeDetail();
    } else {
      toastStore.push({ message: 'Failed to re-assign membership', level: 'error' });
    }
  }

  function computeExtendedExpiry(baseExpiry: string, days: number): string {
    const date = new Date(baseExpiry);
    date.setDate(date.getDate() + days);
    return formatDate(date.toISOString());
  }

  function computeRenewExpiry(): string {
    const date = new Date();
    date.setFullYear(date.getFullYear() + 1);
    return formatDate(date.toISOString());
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
      const tierRes = await fetch(
        `http://localhost:3000/loyalty/customers/${merchantId}/${cust.id}`
      );
      if (tierRes.ok) {
        const tierData = await tierRes.json();
        tierName = (tierData?.tier?.name as string) ?? null;
        tierRank = (tierData?.tier?.rank as number) ?? 0;
        tierMult = (tierData?.tier?.earn_rate_multiplier as number) ?? 1;
      }
    } catch {
      /* no tier */
    }

    try {
      const statusRes = await fetch(
        `http://localhost:3000/earn/memberships/status/${merchantId}/${cust.id}`
      );
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
    } catch {
      /* no membership status */
    }

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
          <Button
            text="Assign a membership →"
            classes="btn-ghost"
            onclick={() => {
              activeTabIndex = 1;
            }}
          />
        </div>
      {:else}
        <div class="table-card">
          <div class="filters-bar">
            <Input
              value={searchQuery}
              placeholder="Search by name or phone..."
              onInput={(val) => {
                searchQuery = val;
                currentPage = 1;
              }}
              classes="filter-input"
            />
            <Select
              items={tierFilterItems}
              value={[filterTier]}
              onchange={(vals) => {
                filterTier = vals[0] ?? 'all';
                currentPage = 1;
              }}
              classes="filter-select"
            />
            <Select
              items={statusFilterItems}
              value={[filterStatus]}
              onchange={(vals) => {
                filterStatus = vals[0] ?? 'all';
                currentPage = 1;
              }}
              classes="filter-select"
            />
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
                    <td
                      ><Pill
                        text={sub.tier_name}
                        classes="tier-pill {tierColorClass(sub.tier_name)}"
                      /></td
                    >
                    <td
                      ><Pill
                        text="{sub.earn_rate_multiplier}x"
                        classes="pill-neutral mult-pill"
                      /></td
                    >
                    <td>
                      <Pill
                        text={status === 'active'
                          ? 'Active'
                          : status === 'expiring'
                            ? 'Expiring Soon'
                            : status === 'cancelled'
                              ? 'Cancelled'
                              : 'Expired'}
                        classes={status === 'active'
                          ? 'pill-success'
                          : status === 'expiring'
                            ? 'pill-warning'
                            : status === 'cancelled' || status === 'expired'
                              ? 'pill-error'
                              : 'pill-neutral'}
                      />
                    </td>
                    <td><span class="cell-date">{formatDate(sub.started_at)}</span></td>
                    <td>
                      {#if status === 'cancelled'}
                        <span class="cell-date strikethrough muted"
                          >{formatDate(sub.expires_at)}</span
                        >
                        <div class="cell-sub">
                          cancelled {sub.cancelled_at ? formatDate(sub.cancelled_at) : ''}
                        </div>
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
                <span class="pagination-info"
                  >Showing {(currentPage - 1) * pageSize + 1}–{Math.min(
                    currentPage * pageSize,
                    filtered.length
                  )} of {filtered.length} memberships</span
                >
                <Pagination
                  {totalPages}
                  {currentPage}
                  onchange={(p) => {
                    currentPage = p;
                  }}
                />
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
          onReset={() => {
            resolvedCustomerId = null;
            resolvedCustomer = null;
            assignResult = null;
            formLookupError = null;
          }}
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
  <Modal
    size="large"
    showOverlay={true}
    header={{ text: 'Membership details', rightImage: MODAL_CLOSE_ICON }}
    onclose={closeDetail}
    onoverlayClick={closeDetail}
    onheaderRightImageClick={closeDetail}
  >
    {#snippet content()}
      {#if selectedMembership !== null}
        {@const ms = selectedMembership}
        {@const st = memberStatus(ms)}
        {@const d = daysRemaining(ms.expires_at)}
        {@const pPct =
          st === 'active' || st === 'expiring' ? Math.min(100, Math.max(0, (d / 365) * 100)) : 0}
        <div class="modal-body">
          <div class="detail-header">
            <div class="detail-customer">
              <span class="detail-name">{ms.customer_name ?? 'Unnamed'}</span>
              <span class="detail-phone">{formatPhone(ms.customer_phone)}</span>
            </div>
            <Pill
              text={st === 'active'
                ? 'Active'
                : st === 'expiring'
                  ? 'Expiring Soon'
                  : st === 'cancelled'
                    ? 'Cancelled'
                    : 'Expired'}
              classes={st === 'active'
                ? 'pill-success'
                : st === 'expiring'
                  ? 'pill-warning'
                  : st === 'cancelled' || st === 'expired'
                    ? 'pill-error'
                    : 'pill-neutral'}
            />
          </div>

          {#if st === 'expiring'}
            <div class="detail-banner amber">
              <strong>Expires in {d} days.</strong> Renew to keep this customer at {ms.tier_name} with
              {ms.earn_rate_multiplier}x earn rate.
            </div>
          {/if}

          {#if st === 'cancelled'}
            <div class="detail-banner red">
              Membership was cancelled on <strong
                >{ms.cancelled_at ? formatDate(ms.cancelled_at) : 'unknown'}</strong
              >. The customer has fallen back to their earned tier.
            </div>
          {/if}

          <div class="detail-grid">
            {#if st === 'cancelled'}
              <div class="detail-field">
                <span class="detail-label">Tier (was)</span>
                <span class="detail-value strikethrough muted"
                  ><Pill
                    text={ms.tier_name}
                    classes="tier-pill {tierColorClass(ms.tier_name)} pill-faded"
                  /></span
                >
              </div>
              <div class="detail-field">
                <span class="detail-label">Current earned tier</span>
                <span class="detail-value">
                  {#if selectedEarnedTier}
                    <Pill
                      text={selectedEarnedTier}
                      classes="tier-pill {tierColorClass(selectedEarnedTier)}"
                    />
                  {:else}
                    <span class="muted">None</span>
                  {/if}
                </span>
              </div>
            {:else}
              <div class="detail-field">
                <span class="detail-label">Tier</span>
                <span class="detail-value"
                  ><Pill
                    text={ms.tier_name}
                    classes="tier-pill {tierColorClass(ms.tier_name)}"
                  /></span
                >
              </div>
              <div class="detail-field">
                <span class="detail-label">Multiplier</span>
                <span class="detail-value mono">{ms.earn_rate_multiplier}x</span>
              </div>
            {/if}
            <div class="detail-field">
              <span class="detail-label">Started</span>
              <span class="detail-value">{formatDate(ms.started_at)}</span>
            </div>
            <div class="detail-field">
              <span class="detail-label">{st === 'cancelled' ? 'Cancelled' : 'Expires'}</span>
              <span
                class="detail-value"
                class:warn={st === 'expiring'}
                class:error={st === 'cancelled'}
              >
                {#if st === 'cancelled'}
                  {ms.cancelled_at ? formatDate(ms.cancelled_at) : '—'}
                {:else}
                  {formatDate(ms.expires_at)}
                {/if}
              </span>
            </div>
            <div class="detail-field">
              <span class="detail-label">Renewed</span>
              <span class="detail-value"
                >{ms.renewed_count} {ms.renewed_count === 1 ? 'time' : 'times'}</span
              >
            </div>
            {#if st === 'expiring' && selectedEarnedTier}
              <div class="detail-field">
                <span class="detail-label">Earned Tier (fallback)</span>
                <span class="detail-value"
                  ><Pill
                    text={selectedEarnedTier}
                    classes="tier-pill {tierColorClass(selectedEarnedTier)} pill-sm"
                  /></span
                >
              </div>
            {/if}
            {#if st === 'cancelled'}
              <div class="detail-field">
                <span class="detail-label">Was set to expire</span>
                <span class="detail-value strikethrough muted">{formatDate(ms.expires_at)}</span>
              </div>
            {/if}
          </div>

          {#if st === 'active' || st === 'expiring'}
            <div class="detail-progress">
              <div class="detail-progress-label">
                <span>Time remaining</span>
                <span class="detail-progress-value" class:warn={st === 'expiring'}
                  >{d} days left</span
                >
              </div>
              <Progress
                value={pPct}
                classes={st === 'active'
                  ? 'progress-green'
                  : st === 'expiring'
                    ? 'progress-amber'
                    : ''}
              />
            </div>
          {/if}

          {#if detailLoading}
            <Shimmer classes="shimmer-detail" />
          {/if}

          <div class="detail-actions" class:actions-dimmed={modalMode !== 'default'}>
            {#if st === 'expiring'}
              <Button
                text="↻ Renew for 1 Year"
                classes="btn-primary"
                onclick={() => {
                  modalMode = 'renew-confirm';
                }}
              />
              <Button
                text="⬆ Upgrade Tier"
                classes="btn-secondary"
                onclick={() => {
                  modalMode = 'upgrade';
                  selectedUpgradeTierId = '';
                }}
              />
              <Button
                text="Cancel"
                classes="btn-danger"
                onclick={() => {
                  modalMode = 'cancel-confirm';
                }}
              />
            {:else if st === 'active'}
              <Button
                text="⬆ Upgrade Tier"
                classes="btn-secondary"
                onclick={() => {
                  modalMode = 'upgrade';
                  selectedUpgradeTierId = '';
                }}
              />
              <Button
                text="+ Extend"
                classes="btn-secondary"
                onclick={() => {
                  modalMode = 'extend';
                  extendDays = 365;
                }}
              />
              <Button
                text="Cancel membership"
                classes="btn-danger"
                onclick={() => {
                  modalMode = 'cancel-confirm';
                }}
              />
            {:else if st === 'cancelled'}
              <Button
                text="↻ Re-assign Membership"
                classes="btn-primary"
                onclick={() => {
                  modalMode = 'upgrade';
                  selectedUpgradeTierId = '';
                }}
              />
            {/if}
          </div>

          {#if modalMode === 'cancel-confirm'}
            <div class="confirm-box confirm-danger">
              <p class="confirm-text danger">
                Are you sure you want to cancel this membership? {ms.customer_name ??
                  'This customer'} will lose their {ms.tier_name} tier ({ms.earn_rate_multiplier}x
                earn rate) and fall back to their earned tier. This cannot be undone.
              </p>
              <div class="confirm-actions">
                <Button
                  text="Yes, Cancel Membership"
                  classes="btn-danger"
                  disabled={actionLoading}
                  onclick={async () => {
                    actionLoading = true;
                    await handleCancelFromDetail();
                    actionLoading = false;
                    modalMode = 'default';
                  }}
                />
                <Button
                  text="Keep membership"
                  classes="btn-ghost"
                  onclick={() => {
                    modalMode = 'default';
                  }}
                />
              </div>
            </div>
          {/if}

          {#if modalMode === 'upgrade'}
            {@const currentTierId = ms.tier_id ?? ''}
            {@const currentTierRank = tiers.find((t) => t.id === currentTierId)?.rank ?? 0}
            <div class="confirm-box confirm-info">
              <div class="tier-select-row">
                {#each tiers as tier (tier.id)}
                  {@const isCurrent = st !== 'cancelled' && tier.id === currentTierId}
                  {@const isLower = st !== 'cancelled' && tier.rank < currentTierRank}
                  <button
                    class="tier-option"
                    class:selected={selectedUpgradeTierId === tier.id}
                    class:current={isCurrent}
                    class:disabled={isCurrent || isLower}
                    onclick={() => {
                      selectedUpgradeTierId = tier.id;
                    }}
                  >
                    <div class="tier-option-name">
                      {tier.name}{#if isCurrent}
                        ✓{/if}
                    </div>
                    <div class="tier-option-mult">{tier.earn_rate_multiplier}x</div>
                  </button>
                {/each}
              </div>
              <div class="confirm-actions">
                {#if st === 'cancelled'}
                  <Button
                    text="Assign {tiers.find((t) => t.id === selectedUpgradeTierId)?.name ??
                      ''} Membership"
                    classes="btn-primary"
                    disabled={!selectedUpgradeTierId || actionLoading}
                    onclick={handleReassignFromDetail}
                  />
                {:else}
                  <Button
                    text="Upgrade to {tiers.find((t) => t.id === selectedUpgradeTierId)?.name ??
                      '...'}"
                    classes="btn-primary"
                    disabled={!selectedUpgradeTierId || actionLoading}
                    onclick={handleUpgradeFromDetail}
                  />
                {/if}
                <Button
                  text="Cancel"
                  classes="btn-ghost"
                  onclick={() => {
                    modalMode = 'default';
                  }}
                />
              </div>
            </div>
          {/if}

          {#if modalMode === 'extend'}
            <div class="confirm-box confirm-success">
              <div class="extend-input-row">
                <span class="confirm-text success">Extend by</span>
                <input
                  type="number"
                  class="extend-input"
                  min="1"
                  max="3650"
                  value={extendDays}
                  oninput={(e) => {
                    extendDays = parseInt((e.target as HTMLInputElement).value) || 0;
                  }}
                />
                <span class="confirm-text success">days</span>
              </div>
              {#if extendDays > 0}
                <div class="extend-preview green">
                  New expiry: {computeExtendedExpiry(ms.expires_at, extendDays)}
                </div>
              {/if}
              <div class="confirm-actions">
                <Button
                  text="Extend membership"
                  classes="btn-primary"
                  disabled={extendDays <= 0 || actionLoading}
                  onclick={handleExtendFromDetail}
                />
                <Button
                  text="Cancel"
                  classes="btn-ghost"
                  onclick={() => {
                    modalMode = 'default';
                  }}
                />
              </div>
            </div>
          {/if}

          {#if modalMode === 'renew-confirm'}
            <div class="confirm-box confirm-success">
              <p class="confirm-text success">
                Renew {ms.tier_name} membership for {ms.customer_name ?? 'this customer'}? This will
                extend by 1 year from today. New expiry: {computeRenewExpiry()}
              </p>
              <div class="confirm-actions">
                <Button
                  text="Confirm renewal"
                  classes="btn-primary"
                  disabled={actionLoading}
                  onclick={handleRenewFromDetail}
                />
                <Button
                  text="Not now"
                  classes="btn-ghost"
                  onclick={() => {
                    modalMode = 'default';
                  }}
                />
              </div>
            </div>
          {/if}
        </div>
      {/if}
    {/snippet}
  </Modal>
{/if}

<style>
  .memberships-page {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    max-width: 1100px;
  }

  .page-header {
    margin-bottom: var(--space-2);
  }
  .page-title {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    margin-bottom: var(--space-1);
  }
  .page-subtitle {
    font-size: var(--font-size-base);
    color: var(--color-text-muted);
  }
  .tab-content {
    min-height: 300px;
  }
  .form-container {
    max-width: 480px;
    padding-top: var(--space-4);
  }

  /* ── Table card ── */
  .table-card {
    background: var(--color-surface);
    border-radius: var(--radius-lg);
    overflow: hidden;
    box-shadow: var(--shadow-card);
  }

  .filters-bar {
    display: flex;
    gap: var(--space-3);
    align-items: center;
    padding: var(--space-4);
    border-bottom: 1px solid var(--color-border);
  }

  :global(.filter-input) {
    flex: 1;
    max-width: 260px;
  }
  :global(.filter-select) {
    min-width: 140px;
  }

  .filter-count {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    margin-left: auto;
    white-space: nowrap;
  }

  /* ── Table ── */
  table {
    width: 100%;
    border-collapse: collapse;
    font-size: var(--font-size-sm);
  }
  th {
    text-align: left;
    padding: 10px 16px;
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.04em;
    border-bottom: 1px solid var(--color-border);
    background: var(--color-surface-2, var(--g-100));
  }
  td {
    padding: 14px 16px;
    border-bottom: 1px solid var(--color-border-light, var(--color-surface-2));
    vertical-align: middle;
  }
  tr:last-child td {
    border-bottom: none;
  }
  tbody tr {
    cursor: pointer;
    transition: background 0.1s;
  }
  tbody tr:hover td {
    background: var(--color-surface-2, var(--g-100));
  }

  .cell-customer {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }
  .cell-name {
    font-weight: var(--font-weight-medium);
    color: var(--color-text);
  }
  .cell-phone {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    font-family: var(--font-mono);
  }

  :global(.tier-pill.t-bronze) {
    --pill-bg: var(--yellow-100);
    --pill-color: var(--yellow-700);
  }
  :global(.tier-pill.t-silver) {
    --pill-bg: var(--color-surface-2);
    --pill-color: var(--g-1700);
  }
  :global(.tier-pill.t-gold) {
    --pill-bg: var(--yellow-100);
    --pill-color: var(--yellow-700);
  }
  :global(.tier-pill.t-plat) {
    --pill-bg: color-mix(in srgb, var(--purple-500) 12%, #fff);
    --pill-color: var(--purple-500);
  }
  :global(.tier-pill.t-default) {
    --pill-bg: var(--color-surface-2);
    --pill-color: var(--color-text);
  }
  :global(.tier-pill.pill-faded) {
    opacity: 0.5;
  }
  :global(.tier-pill.pill-sm) {
    font-size: 10px;
  }
  :global(.mult-pill) {
    font-family: var(--font-mono);
  }
  .cell-date {
    font-size: var(--font-size-xs);
    color: var(--color-text);
  }
  .cell-sub {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }
  .cell-sub.warn,
  .cell-date.warn {
    color: var(--yellow-500);
    font-weight: var(--font-weight-medium);
  }
  .strikethrough {
    text-decoration: line-through;
  }
  .muted {
    color: var(--color-text-muted);
  }
  .warn {
    color: var(--yellow-500);
    font-weight: var(--font-weight-semibold);
  }
  .error {
    color: var(--red-500);
  }
  .mono {
    font-family: var(--font-mono);
  }

  .pagination-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    border-top: 1px solid var(--color-border);
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }
  .pagination-info {
    font-size: var(--font-size-xs);
  }

  /* ── Empty state ── */
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-12) var(--space-4);
    color: var(--color-text-muted);
    font-size: var(--font-size-base);
  }
  .empty-icon {
    font-size: 36px;
  }

  .loading {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
    padding-top: var(--space-4);
  }
  :global(.shimmer-table) {
    --shimmer-width: 100%;
    --shimmer-height: 48px;
    --shimmer-border-radius: var(--radius-md);
  }
  :global(.shimmer-detail) {
    --shimmer-width: 100%;
    --shimmer-height: 60px;
    --shimmer-border-radius: var(--radius-md);
  }

  .modal-body {
    padding: var(--space-5);
    overflow-y: auto;
    flex: 1;
  }

  /* ── Detail modal content ── */
  .detail-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: var(--space-5);
  }
  .detail-customer {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  .detail-name {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }
  .detail-phone {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    font-family: var(--font-mono);
  }

  .detail-banner {
    padding: 10px 14px;
    border-radius: var(--radius-md);
    margin-bottom: var(--space-5);
    font-size: var(--font-size-xs);
    line-height: 1.5;
  }
  .detail-banner.amber {
    background: var(--yellow-100);
    border: 1px solid var(--yellow-100);
    color: var(--yellow-700);
  }
  .detail-banner.red {
    background: var(--red-100);
    border: 1px solid var(--red-100);
    color: var(--red-700);
  }

  .detail-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 14px;
    padding: var(--space-4);
    background: var(--color-surface-2, var(--g-100));
    border-radius: var(--radius-md);
    margin-bottom: var(--space-5);
  }
  .detail-field {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  .detail-label {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }
  .detail-value {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-text);
  }

  .detail-progress {
    margin-bottom: var(--space-5);
  }
  .detail-progress-label {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    margin-bottom: 6px;
    display: flex;
    justify-content: space-between;
  }
  .detail-progress-value {
    font-weight: var(--font-weight-medium);
    color: var(--color-text);
  }
  .detail-progress-value.warn {
    color: var(--yellow-500);
  }
  :global(.progress-green) {
    --progress-fill-color: var(--green-500);
  }
  :global(.progress-amber) {
    --progress-fill-color: var(--yellow-500);
  }

  .detail-actions {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
  }

  .confirm-box {
    margin-top: 16px;
    padding: 14px;
    border-radius: var(--radius-md);
    border: 1px solid;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
  .confirm-danger {
    border-color: var(--red-100);
    background: var(--red-100);
  }
  .confirm-success {
    border-color: var(--green-100);
    background: var(--green-100);
  }
  .confirm-info {
    border-color: color-mix(in srgb, var(--purple-500) 25%, #fff);
    background: color-mix(in srgb, var(--purple-500) 7%, #fff);
  }
  .confirm-text {
    font-size: var(--font-size-sm);
    line-height: 1.5;
  }
  .confirm-text.danger {
    color: var(--red-700);
  }
  .confirm-text.success {
    color: var(--green-700);
  }
  .confirm-actions {
    display: flex;
    gap: 8px;
  }

  .tier-select-row {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
    margin-bottom: 8px;
  }
  .tier-option {
    padding: 8px 14px;
    border: 2px solid var(--color-border);
    border-radius: var(--radius-md);
    cursor: pointer;
    text-align: center;
    font-size: var(--font-size-xs);
    background: white;
    transition: all 0.15s;
    min-width: 80px;
  }
  .tier-option:hover:not(.disabled) {
    border-color: color-mix(in srgb, var(--purple-500) 25%, #fff);
  }
  .tier-option.selected {
    border-color: var(--purple-500);
    background: color-mix(in srgb, var(--purple-500) 7%, #fff);
  }
  .tier-option.current {
    border-color: var(--yellow-100);
    background: var(--yellow-100);
  }
  .tier-option.disabled {
    opacity: 0.4;
    pointer-events: none;
  }
  .tier-option-name {
    font-weight: 600;
    color: var(--color-text);
  }
  .tier-option-mult {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .extend-input-row {
    display: flex;
    gap: 8px;
    align-items: center;
    margin-bottom: 8px;
  }
  .extend-input {
    width: 80px;
    padding: 7px 10px;
    border: 1px solid var(--g-500);
    border-radius: 7px;
    font-size: var(--font-size-sm);
    text-align: center;
    outline: none;
  }
  .extend-input:focus {
    border-color: var(--purple-500);
  }
  .extend-preview {
    font-size: var(--font-size-xs);
    padding: 8px 12px;
    border-radius: var(--radius-sm);
    margin-bottom: 4px;
  }
  .extend-preview.green {
    color: var(--green-700);
    background: var(--green-100);
  }
  .actions-dimmed {
    opacity: 0.4;
    pointer-events: none;
  }
</style>
