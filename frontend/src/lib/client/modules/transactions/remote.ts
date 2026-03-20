import { apiCaller } from '$lib/client/modules/foundation';
import type { APIResult } from '$lib/client/modules/foundation';
import type {
  LedgerResponse,
  LedgerEntryDetail,
  RedemptionResponse,
  TransactionFilters,
  WalletBalance,
  WalletLookupResult,
  MerchantTransactionRow
} from './types';

function decodeLedgerResponse(raw: unknown): LedgerResponse {
  if (Array.isArray(raw)) {
    return { entries: raw, total: raw.length } as LedgerResponse;
  }
  return raw as LedgerResponse;
}

function decodeWalletBalance(raw: unknown): WalletBalance {
  return raw as WalletBalance;
}

function decodeRedemptionResponse(raw: unknown): RedemptionResponse {
  if (Array.isArray(raw)) {
    return { redemptions: raw, total: raw.length } as RedemptionResponse;
  }
  return raw as RedemptionResponse;
}

function decodeWalletLookupResult(raw: unknown): WalletLookupResult {
  return raw as WalletLookupResult;
}

async function fetchEntries(
  walletId: string,
  filters: TransactionFilters
): Promise<APIResult<LedgerResponse>> {
  const params: Record<string, string> = {
    page: String(filters.page),
    limit: String(filters.limit)
  };

  if (filters.bucket_type !== null) {
    params['bucket_type'] = filters.bucket_type;
  }

  if (filters.movement_type !== null) {
    params['movement_type'] = filters.movement_type;
  }

  return apiCaller.get(`/wallets/${walletId}/entries`, decodeLedgerResponse, params);
}

async function fetchBalance(walletId: string): Promise<APIResult<WalletBalance>> {
  return apiCaller.get(`/wallets/${walletId}/balance`, decodeWalletBalance);
}

async function fetchRedemptions(
  merchantId: string,
  page: number,
  limit: number
): Promise<APIResult<RedemptionResponse>> {
  const params: Record<string, string> = {
    page: String(page),
    limit: String(limit)
  };

  return apiCaller.get(`/redemptions`, decodeRedemptionResponse, params);
}

async function lookupWallet(
  merchantId: string,
  customerId: string
): Promise<APIResult<WalletLookupResult>> {
  const params: Record<string, string> = {
    merchant_id: merchantId,
    customer_id: customerId
  };

  return apiCaller.get('/wallets/lookup', decodeWalletLookupResult, params);
}

function decodeMerchantTransactionRow(raw: unknown): MerchantTransactionRow {
  const r = raw as Record<string, unknown>;
  return {
    entry_id: (r['entry_id'] as string) ?? '',
    wallet_id: (r['wallet_id'] as string) ?? '',
    customer_name: (r['customer_name'] as string) ?? null,
    customer_phone: (r['customer_phone'] as string) ?? '',
    bucket_type: (r['bucket_type'] as string) ?? '',
    movement_type: (r['movement_type'] as string) ?? '',
    currency_equivalent: (r['currency_equivalent'] as number) ?? 0,
    state: (r['state'] as string) ?? '',
    created_at: (r['created_at'] as string) ?? ''
  };
}

function decodeMerchantTransactions(raw: unknown): Array<MerchantTransactionRow> {
  if (Array.isArray(raw)) return raw.map(decodeMerchantTransactionRow);
  const r = raw as Record<string, unknown>;
  const items = (r['entries'] ?? r['data'] ?? []) as Array<unknown>;
  if (!Array.isArray(items)) return [];
  return items.map(decodeMerchantTransactionRow);
}

async function fetchMerchantTransactions(
  merchantId: string,
  search: string | null,
  bucketType: string | null,
  movementType: string | null,
  page: number,
  limit: number
): Promise<APIResult<Array<MerchantTransactionRow>>> {
  const params: Record<string, string> = { page: String(page), limit: String(limit) };
  if (search !== null && search.length > 0) {
    params['search'] = search;
  }
  if (bucketType !== null) {
    params['bucket_type'] = bucketType;
  }
  if (movementType !== null) {
    params['movement_type'] = movementType;
  }
  return apiCaller.get(
    `/admin/merchants/${merchantId}/transactions`,
    decodeMerchantTransactions,
    params
  );
}

function decodeLedgerEntryDetail(raw: unknown): LedgerEntryDetail {
  return raw as LedgerEntryDetail;
}

async function fetchEntryDetail(entryId: string): Promise<APIResult<LedgerEntryDetail>> {
  return apiCaller.get(`/entries/${entryId}`, decodeLedgerEntryDetail);
}

export {
  fetchEntries,
  fetchBalance,
  fetchRedemptions,
  lookupWallet,
  fetchMerchantTransactions,
  fetchEntryDetail
};
