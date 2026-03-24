import { apiCaller } from '$lib/client/modules/foundation';
import type { APIResult } from '$lib/client/modules/foundation';
import type {
  StorefrontMerchant,
  CustomerBalance,
  BucketBalance,
  CustomerTierInfo,
  TierProgress,
  TransactionEntry,
  GiftCardInfo,
  ReferralCodeInfo,
  ReferralProgramInfo,
  CustomerIdentity,
  WalletLookup
} from './types';

function decodeMerchant(raw: unknown): StorefrontMerchant {
  const r = raw as Record<string, unknown>;
  return {
    id: (r['id'] as string) ?? '',
    name: (r['name'] as string) ?? '',
    slug: (r['slug'] as string) ?? null,
    domain: (r['domain'] as string) ?? null,
    currency: (r['currency'] as string) ?? 'INR',
    points_name: (r['points_name'] as string) ?? 'Points',
    points_icon: (r['points_icon'] as string) ?? 'pts',
    points_to_currency_rate: (r['points_to_currency_rate'] as number) ?? 1.0
  };
}

function decodeCustomerIdentity(raw: unknown): CustomerIdentity {
  const r = raw as Record<string, unknown>;
  return {
    id: (r['id'] as string) ?? '',
    phone: (r['phone'] as string) ?? '',
    name: (r['name'] as string) ?? null
  };
}

function decodeCustomerList(raw: unknown): Array<CustomerIdentity> {
  if (Array.isArray(raw)) return raw.map(decodeCustomerIdentity);
  const r = raw as Record<string, unknown>;
  const items = (r['customers'] ?? r['data'] ?? []) as Array<unknown>;
  if (!Array.isArray(items)) return [];
  return items.map(decodeCustomerIdentity);
}

function decodeWalletLookup(raw: unknown): WalletLookup {
  const r = raw as Record<string, unknown>;
  const wallet = (r['wallet'] ?? r) as Record<string, unknown>;
  return {
    id: (wallet['id'] as string) ?? ''
  };
}

function decodeBucket(raw: unknown): BucketBalance {
  const r = raw as Record<string, unknown>;
  return {
    bucket_type: (r['bucket_type'] as string) ?? '',
    displayed: (r['displayed'] as number) ?? 0,
    spendable: (r['spendable'] as number) ?? 0,
    count: (r['count'] as number) ?? 0
  };
}

function decodeBalance(raw: unknown): CustomerBalance {
  const r = raw as Record<string, unknown>;
  const rawBuckets = r['buckets'];
  const buckets: Array<BucketBalance> = Array.isArray(rawBuckets)
    ? rawBuckets.map(decodeBucket)
    : [];
  const rawExpiring = r['expiring_soon'];
  let expiring_soon = null;
  if (rawExpiring !== null && rawExpiring !== undefined) {
    const e = rawExpiring as Record<string, unknown>;
    expiring_soon = {
      amount: (e['amount'] as number) ?? 0,
      currency: (e['currency'] as number) ?? 0,
      days: (e['days'] as number) ?? 0,
      count: (e['count'] as number) ?? 0
    };
  }
  return {
    wallet_id: (r['wallet_id'] as string) ?? '',
    displayed_balance: (r['displayed_balance'] as number) ?? 0,
    spendable_balance: (r['spendable_balance'] as number) ?? 0,
    points_balance: (r['points_balance'] as number) ?? 0,
    cash_balance: (r['cash_balance'] as number) ?? 0,
    buckets,
    expiring_soon
  };
}

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

function decodeTierInfo(raw: unknown): CustomerTierInfo {
  const r = raw as Record<string, unknown>;
  const tier = (r['tier'] ?? {}) as Record<string, unknown>;
  return {
    tier_name: (tier['name'] as string) ?? (r['tier_name'] as string) ?? '',
    rank: (tier['rank'] as number) ?? (r['rank'] as number) ?? 0,
    earn_rate_multiplier: (tier['earn_rate_multiplier'] as number) ?? (r['earn_rate_multiplier'] as number) ?? 1,
    progress_to_next: decodeTierProgress(r['progress_to_next'] ?? null)
  };
}

function decodeEntry(raw: unknown): TransactionEntry {
  const r = raw as Record<string, unknown>;
  return {
    id: (r['id'] as string) ?? '',
    bucket_type: (r['bucket_type'] as string) ?? '',
    movement_type: (r['movement_type'] as string) ?? '',
    earning_unit: (r['earning_unit'] as number) ?? 0,
    currency_equivalent: (r['currency_equivalent'] as number) ?? 0,
    created_at: (r['created_at'] as string) ?? '',
    state: (r['state'] as string) ?? ''
  };
}

function decodeEntries(raw: unknown): Array<TransactionEntry> {
  if (Array.isArray(raw)) return raw.map(decodeEntry);
  const r = raw as Record<string, unknown>;
  const items = (r['entries'] ?? r['data'] ?? []) as Array<unknown>;
  if (!Array.isArray(items)) return [];
  return items.map(decodeEntry);
}

function decodeGiftCard(raw: unknown): GiftCardInfo {
  const r = raw as Record<string, unknown>;
  return {
    code: (r['code'] as string) ?? '',
    initial_amount: (r['initial_amount'] as number) ?? 0,
    current_amount: (r['current_amount'] as number) ?? 0,
    is_claimed: (r['is_claimed'] as boolean) ?? false,
    is_active: (r['is_active'] as boolean) ?? false,
    expires_at: (r['expires_at'] as string) ?? null,
    created_at: (r['created_at'] as string) ?? ''
  };
}

function decodeReferralCode(raw: unknown): ReferralCodeInfo {
  const r = raw as Record<string, unknown>;
  return {
    code: (r['code'] as string) ?? '',
    total_referrals: (r['total_referrals'] as number) ?? 0,
    total_conversions: (r['total_conversions'] as number) ?? 0,
    is_creator: (r['is_creator'] as boolean) ?? false,
    commission_rate: (r['commission_rate'] as number) ?? null
  };
}

function decodeReferralProgram(raw: unknown): ReferralProgramInfo {
  const r = raw as Record<string, unknown>;
  return {
    referrer_reward_amount: (r['referrer_reward_amount'] as number) ?? 0,
    referee_reward_amount: (r['referee_reward_amount'] as number) ?? 0,
    is_active: (r['is_active'] as boolean) ?? false
  };
}

async function fetchMerchantBySlug(slug: string): Promise<APIResult<StorefrontMerchant>> {
  return apiCaller.get(`/admin/merchants/by-slug/${slug}`, decodeMerchant);
}

async function lookupCustomer(phone: string): Promise<APIResult<Array<CustomerIdentity>>> {
  return apiCaller.get('/identity/customers', decodeCustomerList, { phone });
}

async function lookupWallet(
  merchantId: string,
  customerId: string
): Promise<APIResult<WalletLookup>> {
  return apiCaller.get('/wallets/lookup', decodeWalletLookup, {
    merchant_id: merchantId,
    customer_id: customerId
  });
}

async function fetchBalance(walletId: string): Promise<APIResult<CustomerBalance>> {
  return apiCaller.get(`/wallets/${walletId}/balance`, decodeBalance);
}

async function fetchEntries(
  walletId: string,
  limit: number
): Promise<APIResult<Array<TransactionEntry>>> {
  return apiCaller.get(`/wallets/${walletId}/entries`, decodeEntries, {
    limit: String(limit)
  });
}

async function fetchCustomerTier(
  merchantId: string,
  customerId: string
): Promise<APIResult<CustomerTierInfo>> {
  return apiCaller.get(`/loyalty/customers/${merchantId}/${customerId}`, decodeTierInfo);
}

async function fetchGiftCard(code: string): Promise<APIResult<GiftCardInfo>> {
  return apiCaller.get(`/gift-cards/${code}`, decodeGiftCard);
}

async function claimGiftCardForCustomer(
  code: string,
  customerId: string
): Promise<APIResult<GiftCardInfo>> {
  return apiCaller.post(
    '/gift-cards/claim',
    { code, customer_id: customerId },
    decodeGiftCard
  );
}

async function fetchReferralCode(code: string): Promise<APIResult<ReferralCodeInfo>> {
  return apiCaller.get(`/referrals/codes/${code}`, decodeReferralCode);
}

async function fetchCustomerReferralCode(
  merchantId: string,
  customerId: string
): Promise<APIResult<ReferralCodeInfo>> {
  return apiCaller.get(`/referrals/codes/customer/${merchantId}/${customerId}`, decodeReferralCode);
}

async function fetchReferralProgram(merchantId: string): Promise<APIResult<ReferralProgramInfo>> {
  return apiCaller.get(`/referrals/programs/${merchantId}`, decodeReferralProgram);
}

export {
  fetchMerchantBySlug,
  lookupCustomer,
  lookupWallet,
  fetchBalance,
  fetchEntries,
  fetchCustomerTier,
  fetchGiftCard,
  claimGiftCardForCustomer,
  fetchReferralCode,
  fetchCustomerReferralCode,
  fetchReferralProgram
};
