import { apiCaller } from '$lib/client/modules/foundation';
import type { APIResult } from '$lib/client/modules/foundation';
import type { DashboardStats, Merchant, MerchantDashboard } from './types';

function decodeDashboardStats(raw: unknown): DashboardStats {
  const r = raw as Record<string, unknown>;
  return {
    total_merchants: (r['total_merchants'] as number) ?? 0,
    total_wallets: (r['total_wallets'] as number) ?? 0,
    total_ledger_entries: (r['total_ledger_entries'] as number) ?? 0,
    total_value_in_system: (r['total_value_in_system'] as number) ?? 0
  };
}

function decodeMerchant(raw: unknown): Merchant {
  const r = raw as Record<string, unknown>;
  return {
    id: (r['id'] as string) ?? '',
    external_id: (r['external_id'] as string) ?? '',
    name: (r['name'] as string) ?? '',
    domain: (r['domain'] as string) ?? null,
    slug: (r['slug'] as string) ?? null,
    plan_tier: (r['plan_tier'] as string) ?? null,
    currency: (r['currency'] as string) ?? 'INR',
    timezone: (r['timezone'] as string) ?? 'Asia/Kolkata',
    is_active: (r['is_active'] as boolean) ?? false
  };
}

async function fetchDashboardStats(_merchantId: string): Promise<APIResult<DashboardStats>> {
  return apiCaller.get('/admin/dashboard', decodeDashboardStats);
}

async function fetchMerchant(merchantId: string): Promise<APIResult<Merchant>> {
  return apiCaller.get(`/admin/merchants/${merchantId}`, decodeMerchant);
}

function decodeMerchantList(raw: unknown): Array<Merchant> {
  if (Array.isArray(raw)) return raw.map(decodeMerchant);
  const r = raw as Record<string, unknown>;
  const items = (r['merchants'] ?? r['data'] ?? []) as Array<unknown>;
  if (!Array.isArray(items)) return [];
  return items.map(decodeMerchant);
}

async function fetchMerchants(
  page: number = 1,
  limit: number = 50
): Promise<APIResult<Array<Merchant>>> {
  return apiCaller.get('/admin/merchants', decodeMerchantList, {
    page: String(page),
    limit: String(limit)
  });
}

function decodeMerchantDashboard(raw: unknown): MerchantDashboard {
  const r = raw as Record<string, unknown>;
  return {
    merchant_id: (r['merchant_id'] as string) ?? '',
    active_customers: (r['active_customers'] as number) ?? 0,
    total_wallets: (r['total_wallets'] as number) ?? 0,
    total_earned: (r['total_earned'] as number) ?? 0,
    total_redeemed: (r['total_redeemed'] as number) ?? 0,
    total_cod_pending: (r['total_cod_pending'] as number) ?? 0,
    active_credits: (r['active_credits'] as number) ?? 0,
    total_ledger_entries: (r['total_ledger_entries'] as number) ?? 0,
    redemption_count: (r['redemption_count'] as number) ?? 0
  };
}

async function fetchMerchantDashboard(merchantId: string): Promise<APIResult<MerchantDashboard>> {
  return apiCaller.get(`/admin/merchants/${merchantId}/dashboard`, decodeMerchantDashboard);
}

export { fetchDashboardStats, fetchMerchant, fetchMerchants, fetchMerchantDashboard };
