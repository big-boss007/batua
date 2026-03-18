import { apiCaller } from '$lib/client/modules/foundation';
import type { APIResult } from '$lib/client/modules/foundation';
import type {
  DashboardStats,
  PlatformMerchant,
  MerchantStats,
  GeoPolicy,
  SystemHealth,
  RecentEvent
} from './types';

function decodeDashboardStats(raw: unknown): DashboardStats {
  const r = raw as Record<string, unknown>;
  return {
    total_merchants: (r['total_merchants'] as number) ?? 0,
    total_wallets: (r['total_wallets'] as number) ?? 0,
    total_ledger_entries: (r['total_ledger_entries'] as number) ?? 0,
    total_value_in_system: (r['total_value_in_system'] as number) ?? 0
  };
}

function decodePlatformMerchant(raw: unknown): PlatformMerchant {
  const r = raw as Record<string, unknown>;
  return {
    id: (r['id'] as string) ?? '',
    name: (r['name'] as string) ?? '',
    slug: (r['slug'] as string) ?? null,
    domain: (r['domain'] as string) ?? null,
    external_id: (r['external_id'] as string) ?? '',
    currency: (r['currency'] as string) ?? 'INR',
    timezone: (r['timezone'] as string) ?? 'Asia/Kolkata',
    plan_tier: (r['plan_tier'] as string) ?? 'free',
    is_active: (r['is_active'] as boolean) ?? false,
    geo_policy_id: (r['geo_policy_id'] as string) ?? null,
    created_at: (r['created_at'] as string) ?? ''
  };
}

function decodeMerchantList(raw: unknown): Array<PlatformMerchant> {
  if (Array.isArray(raw)) return raw.map(decodePlatformMerchant);
  const r = raw as Record<string, unknown>;
  const items = (r['merchants'] ?? r['data'] ?? []) as Array<unknown>;
  if (!Array.isArray(items)) return [];
  return items.map(decodePlatformMerchant);
}

function decodeMerchantStats(raw: unknown): MerchantStats {
  const r = raw as Record<string, unknown>;
  return {
    merchant_id: (r['merchant_id'] as string) ?? '',
    total_wallets: (r['total_wallets'] as number) ?? 0,
    total_customers: (r['total_customers'] as number) ?? 0,
    total_ledger_entries: (r['total_ledger_entries'] as number) ?? 0,
    active_credits: (r['active_credits'] as number) ?? 0,
    total_redeemed: (r['total_redeemed'] as number) ?? 0
  };
}

function decodeGeoPolicy(raw: unknown): GeoPolicy {
  const r = raw as Record<string, unknown>;
  return {
    id: (r['id'] as string) ?? '',
    geo_code: (r['geo_code'] as string) ?? '',
    name: (r['name'] as string) ?? '',
    config: (r['config'] as Record<string, unknown>) ?? {},
    is_active: (r['is_active'] as boolean) ?? false,
    created_at: (r['created_at'] as string) ?? ''
  };
}

function decodeGeoPolicyList(raw: unknown): Array<GeoPolicy> {
  if (Array.isArray(raw)) return raw.map(decodeGeoPolicy);
  const r = raw as Record<string, unknown>;
  const items = (r['policies'] ?? r['data'] ?? []) as Array<unknown>;
  if (!Array.isArray(items)) return [];
  return items.map(decodeGeoPolicy);
}

function decodeSystemHealth(raw: unknown): SystemHealth {
  const r = raw as Record<string, unknown>;
  return {
    unprocessed_events: (r['unprocessed_events'] as number) ?? 0,
    failed_events: (r['failed_events'] as number) ?? 0,
    pending_cod_orders: (r['pending_cod_orders'] as number) ?? 0,
    expiring_7d_count: (r['expiring_7d_count'] as number) ?? 0,
    expiring_7d_value: (r['expiring_7d_value'] as number) ?? 0,
    expiring_30d_count: (r['expiring_30d_count'] as number) ?? 0,
    expiring_30d_value: (r['expiring_30d_value'] as number) ?? 0
  };
}

function decodeRecentEvent(raw: unknown): RecentEvent {
  const r = raw as Record<string, unknown>;
  return {
    id: (r['id'] as string) ?? '',
    merchant_id: (r['merchant_id'] as string) ?? '',
    merchant_name: (r['merchant_name'] as string) ?? '',
    event_type: (r['event_type'] as string) ?? '',
    event_source: (r['event_source'] as string) ?? '',
    state: (r['state'] as string) ?? '',
    created_at: (r['created_at'] as string) ?? ''
  };
}

function decodeRecentEventList(raw: unknown): Array<RecentEvent> {
  if (Array.isArray(raw)) return raw.map(decodeRecentEvent);
  const r = raw as Record<string, unknown>;
  const items = (r['events'] ?? r['data'] ?? []) as Array<unknown>;
  if (!Array.isArray(items)) return [];
  return items.map(decodeRecentEvent);
}

async function fetchDashboardStats(): Promise<APIResult<DashboardStats>> {
  return apiCaller.get('/admin/dashboard', decodeDashboardStats);
}

async function fetchMerchants(
  page: number = 1,
  limit: number = 50
): Promise<APIResult<Array<PlatformMerchant>>> {
  return apiCaller.get('/admin/merchants', decodeMerchantList, {
    page: String(page),
    limit: String(limit)
  });
}

async function fetchMerchant(id: string): Promise<APIResult<PlatformMerchant>> {
  return apiCaller.get(`/admin/merchants/${id}`, decodePlatformMerchant);
}

async function fetchMerchantStats(id: string): Promise<APIResult<MerchantStats>> {
  return apiCaller.get(`/admin/merchants/${id}/stats`, decodeMerchantStats);
}

async function createMerchant(
  data: Record<string, unknown>
): Promise<APIResult<PlatformMerchant>> {
  return apiCaller.post('/admin/merchants', data, decodePlatformMerchant);
}

async function updateMerchant(
  id: string,
  data: Record<string, unknown>
): Promise<APIResult<PlatformMerchant>> {
  return apiCaller.put(`/admin/merchants/${id}`, data, decodePlatformMerchant);
}

async function updateMerchantPlan(
  id: string,
  plan: string
): Promise<APIResult<PlatformMerchant>> {
  return apiCaller.put(`/admin/merchants/${id}/plan`, { plan_tier: plan }, decodePlatformMerchant);
}

async function fetchGeoPolicies(): Promise<APIResult<Array<GeoPolicy>>> {
  return apiCaller.get('/admin/geo-policies', decodeGeoPolicyList);
}

async function createGeoPolicy(
  data: Record<string, unknown>
): Promise<APIResult<GeoPolicy>> {
  return apiCaller.post('/admin/geo-policies', data, decodeGeoPolicy);
}

async function fetchSystemHealth(): Promise<APIResult<SystemHealth>> {
  return apiCaller.get('/admin/system/health', decodeSystemHealth);
}

async function fetchRecentEvents(
  limit: number = 10
): Promise<APIResult<Array<RecentEvent>>> {
  return apiCaller.get('/admin/events/recent', decodeRecentEventList, {
    limit: String(limit)
  });
}

export {
  fetchDashboardStats,
  fetchMerchants,
  fetchMerchant,
  fetchMerchantStats,
  createMerchant,
  updateMerchant,
  updateMerchantPlan,
  fetchGeoPolicies,
  createGeoPolicy,
  fetchSystemHealth,
  fetchRecentEvents
};
