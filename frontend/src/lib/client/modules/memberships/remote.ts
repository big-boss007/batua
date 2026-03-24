import { apiCaller } from '$lib/client/modules/foundation';
import type { APIResult } from '$lib/client/modules/foundation';
import type {
  CustomerMembership,
  EnrichedMembership,
  AssignMembershipRequest,
  AssignMembershipResult,
  MembershipStatus
} from './types';

function decodeMembership(raw: unknown): CustomerMembership {
  const r = raw as Record<string, unknown>;
  return {
    id: (r['id'] as string) ?? '',
    merchant_id: (r['merchant_id'] as string) ?? '',
    customer_id: (r['customer_id'] as string) ?? '',
    tier_id: (r['tier_id'] as string) ?? '',
    status: (r['status'] as string) ?? '',
    started_at: (r['started_at'] as string) ?? '',
    expires_at: (r['expires_at'] as string) ?? '',
    renewed_count: (r['renewed_count'] as number) ?? 0,
    cancelled_at: (r['cancelled_at'] as string) ?? null,
    created_at: (r['created_at'] as string) ?? ''
  };
}

function decodeMemberships(raw: unknown): Array<CustomerMembership> {
  if (Array.isArray(raw)) return raw.map(decodeMembership);
  const r = raw as Record<string, unknown>;
  const items = (r['memberships'] ?? r['data'] ?? []) as Array<unknown>;
  if (!Array.isArray(items)) return [];
  return items.map(decodeMembership);
}

function decodeAssignResult(raw: unknown): AssignMembershipResult {
  const r = raw as Record<string, unknown>;
  return {
    membership: decodeMembership(r['membership'] ?? {}),
    tier_name: (r['tier_name'] as string) ?? '',
    earn_rate_multiplier: (r['earn_rate_multiplier'] as number) ?? 1.0,
    is_new: (r['is_new'] as boolean) ?? false,
    message: (r['message'] as string) ?? ''
  };
}

function decodeMembershipStatus(raw: unknown): MembershipStatus {
  const r = raw as Record<string, unknown>;
  return {
    membership:
      r['membership'] !== null && r['membership'] !== undefined
        ? decodeMembership(r['membership'])
        : null,
    tier_name: (r['tier_name'] as string) ?? null,
    earn_rate_multiplier: (r['earn_rate_multiplier'] as number) ?? 1.0,
    is_active: (r['is_active'] as boolean) ?? false,
    days_remaining: (r['days_remaining'] as number) ?? 0
  };
}

async function assignMembership(
  req: AssignMembershipRequest
): Promise<APIResult<AssignMembershipResult>> {
  return apiCaller.post('/earn/memberships/assign', req, decodeAssignResult);
}

async function cancelMembership(
  membershipId: string
): Promise<APIResult<CustomerMembership>> {
  return apiCaller.post(`/earn/memberships/cancel/${membershipId}`, {}, decodeMembership);
}

async function listSubscribers(
  merchantId: string
): Promise<APIResult<Array<CustomerMembership>>> {
  return apiCaller.get(`/earn/memberships/subscribers/${merchantId}`, decodeMemberships);
}

function decodeEnrichedMembership(raw: unknown): EnrichedMembership {
  const r = raw as Record<string, unknown>;
  return {
    id: (r['id'] as string) ?? '',
    merchant_id: (r['merchant_id'] as string) ?? '',
    customer_id: (r['customer_id'] as string) ?? '',
    tier_id: (r['tier_id'] as string) ?? '',
    status: (r['status'] as string) ?? '',
    started_at: (r['started_at'] as string) ?? '',
    expires_at: (r['expires_at'] as string) ?? '',
    renewed_count: (r['renewed_count'] as number) ?? 0,
    cancelled_at: (r['cancelled_at'] as string) ?? null,
    created_at: (r['created_at'] as string) ?? '',
    customer_name: (r['customer_name'] as string) ?? null,
    customer_phone: (r['customer_phone'] as string) ?? '',
    tier_name: (r['tier_name'] as string) ?? '',
    earn_rate_multiplier: (r['earn_rate_multiplier'] as number) ?? 1.0
  };
}

function decodeEnrichedMemberships(raw: unknown): Array<EnrichedMembership> {
  if (Array.isArray(raw)) return raw.map(decodeEnrichedMembership);
  return [];
}

async function listSubscribersEnriched(
  merchantId: string
): Promise<APIResult<Array<EnrichedMembership>>> {
  return apiCaller.get(
    `/earn/memberships/subscribers/${merchantId}/enriched`,
    decodeEnrichedMemberships
  );
}

async function getMembershipStatus(
  merchantId: string,
  customerId: string
): Promise<APIResult<MembershipStatus>> {
  return apiCaller.get(
    `/earn/memberships/status/${merchantId}/${customerId}`,
    decodeMembershipStatus
  );
}

export {
  assignMembership,
  cancelMembership,
  listSubscribers,
  listSubscribersEnriched,
  getMembershipStatus
};
