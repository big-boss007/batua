export type CustomerMembership = {
  id: string;
  merchant_id: string;
  customer_id: string;
  tier_id: string;
  status: string;
  started_at: string;
  expires_at: string;
  renewed_count: number;
  cancelled_at: string | null;
  created_at: string;
};

export type EnrichedMembership = {
  id: string;
  merchant_id: string;
  customer_id: string;
  tier_id: string;
  status: string;
  started_at: string;
  expires_at: string;
  renewed_count: number;
  cancelled_at: string | null;
  created_at: string;
  customer_name: string | null;
  customer_phone: string;
  tier_name: string;
  earn_rate_multiplier: number;
};

export type AssignMembershipRequest = {
  merchant_id: string;
  customer_id: string;
  tier_id: string;
};

export type AssignMembershipResult = {
  membership: CustomerMembership;
  tier_name: string;
  earn_rate_multiplier: number;
  is_new: boolean;
  message: string;
};

export type MembershipStatus = {
  membership: CustomerMembership | null;
  tier_name: string | null;
  earn_rate_multiplier: number;
  is_active: boolean;
  days_remaining: number;
};
