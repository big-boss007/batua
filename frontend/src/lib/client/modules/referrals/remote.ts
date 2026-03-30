import { apiCaller } from '$lib/client/modules/foundation';
import type { APIResult } from '$lib/client/modules/foundation';
import type {
  ReferralProgram,
  ReferralCode,
  ReferralAnalytics,
  ReferralConversion,
  CodeCreationTrigger
} from './types';

function decodeProgram(raw: unknown): ReferralProgram {
  const r = raw as Record<string, unknown>;
  return {
    id: (r['id'] as string) ?? '',
    referrer_reward_amount: (r['referrer_reward_amount'] as number) ?? 0,
    referee_reward_amount: (r['referee_reward_amount'] as number) ?? 0,
    max_referrals_per_customer: (r['max_referrals_per_customer'] as number) ?? null,
    is_active: (r['is_active'] as boolean) ?? false,
    code_creation_trigger: ((r['code_creation_trigger'] as string) ?? 'on_registration') as CodeCreationTrigger
  };
}

function decodeCode(raw: unknown): ReferralCode {
  const r = raw as Record<string, unknown>;
  return {
    id: (r['id'] as string) ?? '',
    code: (r['code'] as string) ?? '',
    customer_id: (r['customer_id'] as string) ?? '',
    is_vanity: (r['is_vanity'] as boolean) ?? false,
    is_creator: (r['is_creator'] as boolean) ?? false,
    commission_rate: (r['commission_rate'] as number) ?? null,
    total_referrals: (r['total_referrals'] as number) ?? 0,
    total_conversions: (r['total_conversions'] as number) ?? 0,
    is_active: (r['is_active'] as boolean) ?? false
  };
}

function decodeCodes(raw: unknown): Array<ReferralCode> {
  const r = raw as Record<string, unknown>;
  const items = (r['codes'] ?? r['data'] ?? raw) as Array<unknown>;
  if (!Array.isArray(items)) return [];
  return items.map(decodeCode);
}

function decodeAnalytics(raw: unknown): ReferralAnalytics {
  const r = raw as Record<string, unknown>;
  return {
    total_codes: (r['total_codes'] as number) ?? 0,
    total_referrals: (r['total_referrals'] as number) ?? 0,
    total_conversions: (r['total_conversions'] as number) ?? 0,
    total_suspicious: (r['total_suspicious'] as number) ?? 0,
    conversion_rate: (r['conversion_rate'] as number) ?? 0
  };
}

function decodeConversion(raw: unknown): ReferralConversion {
  const r = raw as Record<string, unknown>;
  return {
    id: (r['id'] as string) ?? '',
    referrer_id: (r['referrer_id'] as string) ?? '',
    referee_id: (r['referee_id'] as string) ?? '',
    order_id: (r['order_id'] as string) ?? null,
    is_suspicious: (r['is_suspicious'] as boolean) ?? false,
    fraud_signals: Array.isArray(r['fraud_signals']) ? (r['fraud_signals'] as Array<string>) : [],
    created_at: (r['created_at'] as string) ?? ''
  };
}

function decodeConversions(raw: unknown): Array<ReferralConversion> {
  const r = raw as Record<string, unknown>;
  const items = (r['conversions'] ?? r['data'] ?? raw) as Array<unknown>;
  if (!Array.isArray(items)) return [];
  return items.map(decodeConversion);
}

async function fetchProgram(merchantId: string): Promise<APIResult<ReferralProgram>> {
  return apiCaller.get(`/referrals/programs/${merchantId}`, decodeProgram);
}

async function createProgram(
  merchantId: string,
  program: Omit<ReferralProgram, 'id'>
): Promise<APIResult<ReferralProgram>> {
  return apiCaller.post(
    '/referrals/programs',
    {
      merchant_id: merchantId,
      referrer_reward_amount: program.referrer_reward_amount,
      referee_reward_amount: program.referee_reward_amount,
      max_referrals_per_customer: program.max_referrals_per_customer,
      is_active: program.is_active,
      code_creation_trigger: program.code_creation_trigger
    },
    decodeProgram
  );
}

async function updateProgram(
  merchantId: string,
  updates: {
    referrer_reward_amount?: number;
    referee_reward_amount?: number;
    max_referrals_per_customer?: number | null;
    is_active?: boolean;
    code_creation_trigger?: CodeCreationTrigger;
  }
): Promise<APIResult<ReferralProgram>> {
  return apiCaller.put(`/referrals/programs/${merchantId}`, updates as Record<string, unknown>, decodeProgram);
}

async function createCode(params: {
  merchant_id: string;
  customer_id: string;
  code: string | null;
  is_vanity: boolean;
  is_creator: boolean;
  commission_rate: number | null;
}): Promise<APIResult<ReferralCode>> {
  return apiCaller.post(
    '/referrals/codes',
    {
      merchant_id: params.merchant_id,
      customer_id: params.customer_id,
      code: params.code,
      is_vanity: params.is_vanity,
      is_creator: params.is_creator,
      commission_rate: params.commission_rate
    },
    decodeCode
  );
}

async function resolveCustomerByPhone(phone: string): Promise<APIResult<{ customer_id: string }>> {
  return apiCaller.post(
    '/identity/resolve',
    { phone },
    (raw: unknown) => {
      const r = raw as Record<string, unknown>;
      return { customer_id: (r['customer_id'] as string) ?? '' };
    }
  );
}

async function fetchCodeByCode(code: string): Promise<APIResult<ReferralCode>> {
  return apiCaller.get(`/referrals/codes/${code}`, decodeCode);
}

async function processConversion(params: {
  referral_code: string;
  referee_id: string;
  order_id: string | null;
}): Promise<APIResult<ReferralConversion>> {
  return apiCaller.post(
    '/referrals/convert',
    params as unknown as Record<string, unknown>,
    decodeConversion
  );
}

async function fetchAnalytics(merchantId: string): Promise<APIResult<ReferralAnalytics>> {
  return apiCaller.get(`/referrals/analytics/${merchantId}`, decodeAnalytics);
}

async function fetchConversions(merchantId: string): Promise<APIResult<Array<ReferralConversion>>> {
  return apiCaller.get(`/referrals/conversions/${merchantId}`, decodeConversions);
}

async function fetchMerchantCodes(
  merchantId: string,
  page: number,
  limit: number
): Promise<APIResult<Array<ReferralCode>>> {
  return apiCaller.get(`/referrals/merchant/${merchantId}/codes`, decodeCodes, {
    page: String(page),
    limit: String(limit)
  });
}

export {
  fetchProgram,
  createProgram,
  updateProgram,
  createCode,
  fetchCodeByCode,
  processConversion,
  fetchAnalytics,
  fetchConversions,
  fetchMerchantCodes,
  resolveCustomerByPhone
};
