import { apiCaller } from '$lib/client/modules/foundation';
import type { APIResult } from '$lib/client/modules/foundation';
import type {
  Customer,
  CustomerDetail,
  CustomerMembershipInfo,
  CustomerReferralInfo,
  WalletSummary,
  CustomerTierInfo,
  TierProgress,
  LedgerEntrySummary,
  LoyaltyProgram,
  LoyaltyTier,
  TierDistribution,
  MerchantCustomerRow,
  AddRequest,
  RemoveRequest,
  ExpireRequest,
  WalletActionResult
} from './types';

function decodeTierProgress(raw: unknown): TierProgress | null {
  if (raw === null || raw === undefined) return null;
  const r = raw as Record<string, unknown>;
  return {
    next_tier_name: (r['next_tier_name'] as string) ?? '',
    current_value: (r['current_value'] as number) ?? 0,
    threshold: (r['threshold'] as number) ?? 0,
    percentage: (r['percentage'] as number) ?? 0
  };
}

function decodeCustomerTierInfo(raw: unknown): CustomerTierInfo | null {
  if (raw === null || raw === undefined) return null;
  const r = raw as Record<string, unknown>;
  const tierObj = (r['tier'] ?? r) as Record<string, unknown>;
  return {
    tier_name: (tierObj['name'] as string) ?? (r['tier_name'] as string) ?? '',
    rank: (tierObj['rank'] as number) ?? (r['rank'] as number) ?? 0,
    earn_rate_multiplier: (tierObj['earn_rate_multiplier'] as number) ?? (r['earn_rate_multiplier'] as number) ?? 1,
    progress_to_next: decodeTierProgress(r['progress_to_next'] ?? null)
  };
}

function decodeWalletSummary(raw: unknown): WalletSummary | null {
  if (raw === null || raw === undefined) return null;
  const r = raw as Record<string, unknown>;
  const rawBuckets = Array.isArray(r['buckets']) ? (r['buckets'] as Array<Record<string, unknown>>) : [];
  return {
    id: (r['id'] as string) ?? '',
    displayed_balance: (r['displayed_balance'] as number) ?? 0,
    spendable_balance: (r['spendable_balance'] as number) ?? 0,
    points_balance: (r['points_balance'] as number) ?? 0,
    cash_balance: (r['cash_balance'] as number) ?? 0,
    buckets: rawBuckets.map((b) => ({
      bucket_type: (b['bucket_type'] as string) ?? '',
      spendable: (b['spendable'] as number) ?? 0
    }))
  };
}

function decodeLedgerEntry(raw: unknown): LedgerEntrySummary {
  const r = raw as Record<string, unknown>;
  return {
    id: (r['id'] as string) ?? '',
    bucket_type: (r['bucket_type'] as string) ?? '',
    movement_type: (r['movement_type'] as string) ?? '',
    earning_unit: (r['earning_unit'] as number) ?? 0,
    currency_equivalent: (r['currency_equivalent'] as number) ?? 0,
    created_at: (r['created_at'] as string) ?? ''
  };
}

function decodeCustomer(raw: unknown): Customer {
  const r = raw as Record<string, unknown>;
  return {
    id: (r['id'] as string) ?? '',
    phone: (r['phone'] as string) ?? '',
    email: (r['email'] as string) ?? null,
    name: (r['name'] as string) ?? null,
    is_verified: (r['is_verified'] as boolean) ?? false,
    created_at: (r['created_at'] as string) ?? ''
  };
}

function decodeCustomerList(raw: unknown): Array<Customer> {
  if (Array.isArray(raw)) return raw.map(decodeCustomer);
  const r = raw as Record<string, unknown>;
  const items = (r['customers'] ?? r['data'] ?? []) as Array<unknown>;
  if (!Array.isArray(items)) return [];
  return items.map(decodeCustomer);
}

function decodeCustomerDetail(raw: unknown): CustomerDetail {
  const r = raw as Record<string, unknown>;
  return {
    customer: decodeCustomer(r['customer'] ?? r),
    wallet: decodeWalletSummary(r['wallet'] ?? null),
    tier: decodeCustomerTierInfo(r['tier'] ?? null),
    membership: null,
    referral: null,
    recent_entries: Array.isArray(r['recent_entries'])
      ? (r['recent_entries'] as Array<unknown>).map(decodeLedgerEntry)
      : []
  };
}

function decodeLoyaltyProgram(raw: unknown): LoyaltyProgram {
  const r = raw as Record<string, unknown>;
  return {
    id: (r['id'] as string) ?? '',
    name: (r['name'] as string) ?? '',
    evaluation_criteria: (r['evaluation_criteria'] as string) ?? '',
    evaluation_period_days: (r['evaluation_period_days'] as number) ?? null,
    is_active: (r['is_active'] as boolean) ?? false
  };
}

function decodeLoyaltyTier(raw: unknown): LoyaltyTier {
  const r = raw as Record<string, unknown>;
  return {
    id: (r['id'] as string) ?? '',
    name: (r['name'] as string) ?? '',
    rank: (r['rank'] as number) ?? 0,
    threshold: (r['threshold'] as number) ?? 0,
    earn_rate_multiplier: (r['earn_rate_multiplier'] as number) ?? 1,
    benefits: (r['benefits'] as Record<string, unknown>) ?? {}
  };
}

function decodeTierList(raw: unknown): Array<LoyaltyTier> {
  const r = raw as Record<string, unknown>;
  const items = (r['tiers'] ?? r['data'] ?? raw) as Array<unknown>;
  if (!Array.isArray(items)) return [];
  return items.map(decodeLoyaltyTier);
}

function decodeTierDistribution(raw: unknown): Array<TierDistribution> {
  const r = raw as Record<string, unknown>;
  const items = (r['distribution'] ?? r['data'] ?? raw) as Array<unknown>;
  if (!Array.isArray(items)) return [];
  return items.map((item) => {
    if (Array.isArray(item)) {
      return {
        tier_name: String(item[0] ?? ''),
        count: Number(item[1] ?? 0)
      };
    }
    const d = item as Record<string, unknown>;
    return {
      tier_name: (d['tier_name'] as string) ?? '',
      count: (d['count'] as number) ?? 0
    };
  });
}

async function searchCustomers(
  _merchantId: string,
  query: string
): Promise<APIResult<Array<Customer>>> {
  const isPhone = /^\+?\d[\d\s-]{5,}$/.test(query.trim());
  const params: Record<string, string> = {};
  if (isPhone) {
    params['phone'] = query.trim();
  } else {
    params['external_id'] = query.trim();
  }
  return apiCaller.get('/identity/customers', decodeCustomerList, params);
}

async function getCustomerDetail(
  merchantId: string,
  customerId: string
): Promise<APIResult<CustomerDetail>> {
  const customerResult = await apiCaller.get(`/identity/customers/${customerId}`, decodeCustomer);
  if (customerResult.tag === 'error') {
    return {
      tag: 'error',
      message: customerResult.message,
      status: customerResult.status,
      body: customerResult.body
    };
  }
  const customer = customerResult.data;

  let wallet: WalletSummary | null = null;
  let recentEntries: Array<LedgerEntrySummary> = [];

  const walletResult = await apiCaller.get('/wallets/lookup', (raw: unknown) => raw, {
    merchant_id: merchantId,
    customer_id: customerId
  });
  if (walletResult.tag === 'success' && walletResult.data !== null) {
    const w = walletResult.data as Record<string, unknown>;
    const walletObj = (w['wallet'] ?? w) as Record<string, unknown>;
    const walletId = walletObj['id'] as string;
    if (walletId) {
      const balanceResult = await apiCaller.get(
        `/wallets/${walletId}/balance`,
        (raw: unknown) => raw as Record<string, unknown>
      );
      if (balanceResult.tag === 'success') {
        const rawBuckets = Array.isArray(balanceResult.data['buckets'])
          ? (balanceResult.data['buckets'] as Array<Record<string, unknown>>)
          : [];
        wallet = {
          id: walletId,
          displayed_balance: (balanceResult.data['displayed_balance'] as number) ?? 0,
          spendable_balance: (balanceResult.data['spendable_balance'] as number) ?? 0,
          points_balance: (balanceResult.data['points_balance'] as number) ?? 0,
          cash_balance: (balanceResult.data['cash_balance'] as number) ?? 0,
          buckets: rawBuckets.map((b) => ({
            bucket_type: (b['bucket_type'] as string) ?? '',
            spendable: (b['spendable'] as number) ?? 0
          }))
        };
      }

      const entriesResult = await apiCaller.get(
        `/wallets/${walletId}/entries`,
        (raw: unknown) => {
          if (Array.isArray(raw)) return raw;
          return [];
        },
        { limit: '5' }
      );
      if (entriesResult.tag === 'success') {
        recentEntries = entriesResult.data.map(decodeLedgerEntry);
      }
    }
  }

  let tier: CustomerTierInfo | null = null;
  try {
    const tierResult = await apiCaller.get(
      `/loyalty/customers/${merchantId}/${customerId}`,
      decodeCustomerTierInfo
    );
    if (tierResult.tag === 'success') {
      tier = tierResult.data;
    }
  } catch {
    // no tier
  }

  let membership: CustomerMembershipInfo | null = null;
  try {
    const memResult = await apiCaller.get(
      `/earn/memberships/status/${merchantId}/${customerId}`,
      (raw: unknown) => raw as Record<string, unknown>
    );
    if (memResult.tag === 'success' && memResult.data['is_active'] === true) {
      const m = memResult.data;
      const mem = m['membership'] as Record<string, unknown> | null;
      membership = {
        tier_name: (m['tier_name'] as string) ?? '',
        earn_rate_multiplier: (m['earn_rate_multiplier'] as number) ?? 1,
        status: 'active',
        expires_at: (mem?.['expires_at'] as string) ?? '',
        days_remaining: (m['days_remaining'] as number) ?? 0,
        renewed_count: (mem?.['renewed_count'] as number) ?? 0
      };
    }
  } catch {
    // no membership
  }

  let referral: CustomerReferralInfo | null = null;
  try {
    const refResult = await apiCaller.get(
      `/referrals/codes/customer/${merchantId}/${customerId}`,
      (raw: unknown) => raw as Record<string, unknown>
    );
    if (refResult.tag === 'success' && refResult.data !== null) {
      const r = refResult.data;
      referral = {
        code: (r['code'] as string) ?? null,
        referrals_made: (r['total_conversions'] as number) ?? 0,
        rewards_earned: (r['total_referrals'] as number) ?? 0
      };
    }
  } catch {
    // no referral data
  }

  return {
    tag: 'success',
    data: { customer, wallet, tier, membership, referral, recent_entries: recentEntries },
    status: 200
  };
}

async function fetchLoyaltyProgram(merchantId: string): Promise<APIResult<LoyaltyProgram>> {
  return apiCaller.get(`/loyalty/programs/${merchantId}`, decodeLoyaltyProgram);
}

async function fetchTiers(programId: string): Promise<APIResult<Array<LoyaltyTier>>> {
  return apiCaller.get(`/loyalty/programs/${programId}/tiers`, decodeTierList);
}

async function fetchTierDistribution(
  merchantId: string
): Promise<APIResult<Array<TierDistribution>>> {
  return apiCaller.get(`/loyalty/distribution/${merchantId}`, decodeTierDistribution);
}

async function createProgram(
  merchantId: string,
  body: { name: string; evaluation_criteria: string }
): Promise<APIResult<LoyaltyProgram>> {
  return apiCaller.post(
    '/loyalty/programs',
    { merchant_id: merchantId, ...body },
    decodeLoyaltyProgram
  );
}

async function createTier(
  programId: string,
  body: {
    name: string;
    rank: number;
    threshold: number;
    earn_rate_multiplier: number;
    benefits: Record<string, unknown>;
  }
): Promise<APIResult<LoyaltyTier>> {
  return apiCaller.post('/loyalty/tiers', { program_id: programId, ...body }, decodeLoyaltyTier);
}

async function updateProgram(
  programId: string,
  body: {
    name: string;
    evaluation_criteria: string;
    evaluation_period_days: number | null;
    is_active: boolean;
  }
): Promise<APIResult<LoyaltyProgram>> {
  return apiCaller.put(`/loyalty/programs/${programId}/update`, body, decodeLoyaltyProgram);
}

async function updateTier(
  tierId: string,
  body: {
    name?: string;
    rank?: number;
    threshold?: number;
    earn_rate_multiplier?: number;
    benefits?: Record<string, unknown>;
  }
): Promise<APIResult<LoyaltyTier>> {
  return apiCaller.put(`/loyalty/tiers/${tierId}`, body, decodeLoyaltyTier);
}

async function deleteTier(tierId: string): Promise<APIResult<null>> {
  return apiCaller.delete(`/loyalty/tiers/${tierId}`, () => null);
}

async function evaluateTier(merchantId: string): Promise<APIResult<{ evaluated: number }>> {
  return apiCaller.post(`/loyalty/programs/${merchantId}/evaluate`, {}, (raw: unknown) => {
    const r = raw as Record<string, unknown>;
    return { evaluated: (r['evaluated'] as number) ?? 0 };
  });
}

function decodeMerchantCustomerRow(raw: unknown): MerchantCustomerRow {
  const r = raw as Record<string, unknown>;
  return {
    customer_id: (r['customer_id'] as string) ?? '',
    customer_name: (r['customer_name'] as string) ?? null,
    customer_phone: (r['customer_phone'] as string) ?? '',
    customer_email: (r['customer_email'] as string) ?? null,
    wallet_id: (r['wallet_id'] as string) ?? '',
    created_at: (r['created_at'] as string) ?? ''
  };
}

function decodeMerchantCustomers(raw: unknown): Array<MerchantCustomerRow> {
  if (Array.isArray(raw)) return raw.map(decodeMerchantCustomerRow);
  const r = raw as Record<string, unknown>;
  const items = (r['customers'] ?? r['data'] ?? []) as Array<unknown>;
  if (!Array.isArray(items)) return [];
  return items.map(decodeMerchantCustomerRow);
}

async function fetchMerchantCustomers(
  merchantId: string,
  search: string | null,
  page: number,
  limit: number
): Promise<APIResult<Array<MerchantCustomerRow>>> {
  const params: Record<string, string> = { page: String(page), limit: String(limit) };
  if (search !== null && search.length > 0) {
    params['search'] = search;
  }
  return apiCaller.get(`/admin/merchants/${merchantId}/customers`, decodeMerchantCustomers, params);
}

function decodeWalletActionResult(raw: unknown): WalletActionResult {
  const r = raw as Record<string, unknown>;
  return {
    success: (r['success'] as boolean) ?? false,
    ledger_entry_id: (r['ledger_entry_id'] as string) ?? null,
    entries_affected: (r['entries_expired'] as number) ?? (r['entries_affected'] as number) ?? 1,
    amount_affected: (r['amount_removed'] as number) ?? (r['amount_expired'] as number) ?? 0,
    new_balance: (r['new_total_balance'] as number) ?? (r['new_balance'] as number) ?? 0,
    error: (r['error'] as string) ?? null
  };
}

async function addCredit(
  merchantId: string,
  req: AddRequest
): Promise<APIResult<WalletActionResult>> {
  const body = {
    merchant_id: merchantId,
    customer_ids: [req.customer_id],
    amount: req.amount,
    bucket_type: req.bucket_type,
    reason: `[${req.reason_category}] ${req.reason_text}`,
    actor_id: 'admin'
  };
  return apiCaller.post('/admin/bulk-credit', body, (raw: unknown) => {
    const r = raw as Record<string, unknown>;
    const results = r['results'] as Array<Record<string, unknown>> | undefined;
    const first = results?.[0];
    return {
      success: (first?.['success'] as boolean) ?? false,
      ledger_entry_id: (first?.['ledger_entry_id'] as string) ?? null,
      entries_affected: 1,
      amount_affected: req.amount,
      new_balance: 0,
      error: (first?.['error'] as string) ?? null
    };
  });
}

async function removeBalance(
  merchantId: string,
  req: RemoveRequest
): Promise<APIResult<WalletActionResult>> {
  const body = {
    merchant_id: merchantId,
    customer_id: req.customer_id,
    bucket_type: req.bucket_type,
    amount: req.amount,
    reason_category: req.reason_category,
    reason_text: req.reason_text,
    reference: req.reference,
    actor_id: 'admin'
  };
  return apiCaller.post('/admin/debit', body, decodeWalletActionResult);
}

async function expireBalance(
  merchantId: string,
  req: ExpireRequest
): Promise<APIResult<WalletActionResult>> {
  const body = {
    merchant_id: merchantId,
    customer_id: req.customer_id,
    bucket_types: req.bucket_types,
    reason_category: req.reason_category,
    reason_text: req.reason_text,
    reference: req.reference,
    actor_id: 'admin'
  };
  return apiCaller.post('/admin/force-expire', body, decodeWalletActionResult);
}

async function updateCustomer(
  merchantId: string,
  customerId: string,
  body: { name?: string; email?: string }
): Promise<APIResult<MerchantCustomerRow>> {
  return apiCaller.put(
    `/admin/merchants/${merchantId}/customers/${customerId}`,
    body,
    decodeMerchantCustomerRow
  );
}

export {
  searchCustomers,
  getCustomerDetail,
  fetchMerchantCustomers,
  updateCustomer,
  fetchLoyaltyProgram,
  fetchTiers,
  fetchTierDistribution,
  createProgram,
  updateProgram,
  createTier,
  updateTier,
  deleteTier,
  evaluateTier,
  addCredit,
  removeBalance,
  expireBalance
};
