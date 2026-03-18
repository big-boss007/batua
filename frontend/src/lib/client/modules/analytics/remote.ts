import { apiCaller } from '$lib/client/modules/foundation';
import type { APIResult } from '$lib/client/modules/foundation';
import type { MerchantAnalytics, CodAnalytics, CampaignPerformance, OverviewMetrics } from './types';

function decodeMerchantAnalytics(raw: unknown): MerchantAnalytics {
  const r = raw as Record<string, unknown>;
  return {
    total_earned: (r['total_earned'] as number) ?? 0,
    total_redeemed: (r['total_redeemed'] as number) ?? 0,
    total_expired: (r['total_expired'] as number) ?? 0,
    active_credits: (r['active_credits'] as number) ?? 0,
    cod_pending: (r['cod_pending'] as number) ?? 0,
    cod_delivered: (r['cod_delivered'] as number) ?? 0,
    cod_rto: (r['cod_rto'] as number) ?? 0,
    total_orders: (r['total_orders'] as number) ?? 0,
    prepaid_orders: (r['prepaid_orders'] as number) ?? 0,
    cod_orders: (r['cod_orders'] as number) ?? 0,
    loyalty_rto_rate: (r['loyalty_rto_rate'] as number) ?? 0,
    non_loyalty_rto_rate: (r['non_loyalty_rto_rate'] as number) ?? 0,
    repeat_purchase_rate: (r['repeat_purchase_rate'] as number) ?? 0
  };
}

function decodeCodAnalytics(raw: unknown): CodAnalytics {
  const r = raw as Record<string, unknown>;
  return {
    total_pending: (r['total_pending'] as number) ?? 0,
    total_delivered: (r['total_delivered'] as number) ?? 0,
    total_rto: (r['total_rto'] as number) ?? 0,
    pending_amount: (r['pending_amount'] as number) ?? 0,
    released_amount: (r['released_amount'] as number) ?? 0,
    cancelled_amount: (r['cancelled_amount'] as number) ?? 0
  };
}

function decodeCampaignPerformance(raw: unknown): Array<CampaignPerformance> {
  const data = raw as Record<string, unknown>;
  return (data['campaigns'] ?? []) as Array<CampaignPerformance>;
}

function decodeOverviewMetrics(raw: unknown): OverviewMetrics {
  const r = raw as Record<string, unknown>;
  return {
    total_wallets: (r['total_wallets'] as number) ?? 0,
    total_active_credits: (r['total_active_credits'] as number) ?? 0,
    total_redeemed: (r['total_redeemed'] as number) ?? 0,
    total_expired: (r['total_expired'] as number) ?? 0,
    rto_rate_loyalty: (r['rto_rate_loyalty'] as number) ?? 0,
    rto_rate_non_loyalty: (r['rto_rate_non_loyalty'] as number) ?? 0
  };
}

async function fetchMerchantAnalytics(
  merchantId: string
): Promise<APIResult<MerchantAnalytics>> {
  return apiCaller.get(`/admin/merchants/${merchantId}/analytics`, decodeMerchantAnalytics);
}

async function fetchCodAnalytics(
  merchantId: string,
  from: string,
  to: string
): Promise<APIResult<CodAnalytics>> {
  return apiCaller.get(`/cod/analytics/${merchantId}`, decodeCodAnalytics, {
    from,
    to
  });
}

async function fetchCampaignPerformance(
  _merchantId: string,
  from: string,
  to: string
): Promise<APIResult<Array<CampaignPerformance>>> {
  return apiCaller.get(
    '/admin/dashboard',
    decodeCampaignPerformance,
    { from, to }
  );
}

async function fetchOverviewMetrics(
  _merchantId: string,
  from: string,
  to: string
): Promise<APIResult<OverviewMetrics>> {
  return apiCaller.get(
    '/admin/dashboard',
    decodeOverviewMetrics,
    { from, to }
  );
}

export {
  fetchMerchantAnalytics,
  fetchCodAnalytics,
  fetchCampaignPerformance,
  fetchOverviewMetrics
};
