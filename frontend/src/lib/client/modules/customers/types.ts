export type Customer = {
  id: string;
  phone: string;
  email: string | null;
  name: string | null;
  is_verified: boolean;
  created_at: string;
};

export type CustomerMembershipInfo = {
  tier_name: string;
  earn_rate_multiplier: number;
  status: string;
  expires_at: string;
  days_remaining: number;
  renewed_count: number;
};

export type CustomerReferralInfo = {
  code: string | null;
  referrals_made: number;
  rewards_earned: number;
};

export type CustomerDetail = {
  customer: Customer;
  wallet: WalletSummary | null;
  tier: CustomerTierInfo | null;
  membership: CustomerMembershipInfo | null;
  referral: CustomerReferralInfo | null;
  recent_entries: Array<LedgerEntrySummary>;
};

export type WalletSummary = {
  id: string;
  displayed_balance: number;
  spendable_balance: number;
};

export type CustomerTierInfo = {
  tier_name: string;
  rank: number;
  earn_rate_multiplier: number;
  progress_to_next: TierProgress | null;
};

export type TierProgress = {
  next_tier_name: string;
  current_value: number;
  threshold: number;
  percentage: number;
};

export type LedgerEntrySummary = {
  id: string;
  bucket_type: string;
  movement_type: string;
  earning_unit: number;
  currency_equivalent: number;
  created_at: string;
};

export type LoyaltyProgram = {
  id: string;
  name: string;
  evaluation_criteria: string;
  evaluation_period_days: number | null;
  is_active: boolean;
};

export type LoyaltyTier = {
  id: string;
  name: string;
  rank: number;
  threshold: number;
  earn_rate_multiplier: number;
  benefits: Record<string, unknown>;
};

export type TierDistribution = {
  tier_name: string;
  count: number;
};

export type MerchantCustomerRow = {
  customer_id: string;
  customer_name: string | null;
  customer_phone: string;
  customer_email: string | null;
  wallet_id: string;
  created_at: string;
};
