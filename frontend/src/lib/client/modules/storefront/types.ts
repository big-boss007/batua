export type StorefrontMerchant = {
  id: string;
  name: string;
  slug: string | null;
  domain: string | null;
  currency: string;
};

export type CustomerBalance = {
  wallet_id: string;
  displayed_balance: number;
  spendable_balance: number;
  buckets: Array<BucketBalance>;
};

export type BucketBalance = {
  bucket_type: string;
  displayed: number;
  spendable: number;
  count: number;
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

export type TransactionEntry = {
  id: string;
  bucket_type: string;
  movement_type: string;
  currency_equivalent: number;
  created_at: string;
  state: string;
};

export type GiftCardInfo = {
  code: string;
  initial_amount: number;
  current_amount: number;
  is_claimed: boolean;
  is_active: boolean;
  expires_at: string | null;
  created_at: string;
};

export type ReferralCodeInfo = {
  code: string;
  total_referrals: number;
  total_conversions: number;
  is_creator: boolean;
  commission_rate: number | null;
};

export type ReferralLandingInfo = {
  code: string;
  merchant_name: string;
  referee_reward_amount: number;
};

export type ReferralProgramInfo = {
  referrer_reward_amount: number;
  referee_reward_amount: number;
  is_active: boolean;
};

export type CustomerIdentity = {
  id: string;
  phone: string;
  name: string | null;
};

export type WalletLookup = {
  id: string;
};
