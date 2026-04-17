export type CodeCreationTrigger = 'on_registration' | 'on_first_purchase';

export type ReferralProgram = {
  id: string;
  referrer_reward_amount: number;
  referee_reward_amount: number;
  max_referrals_per_customer: number | null;
  is_active: boolean;
  code_creation_trigger: CodeCreationTrigger;
};

export type ReferralCode = {
  id: string;
  code: string;
  customer_id: string;
  customer_phone: string | null;
  customer_name: string | null;
  is_vanity: boolean;
  is_creator: boolean;
  commission_rate: number | null;
  total_referrals: number;
  total_conversions: number;
  is_active: boolean;
};

export type ReferralAnalytics = {
  total_codes: number;
  total_referrals: number;
  total_conversions: number;
  total_suspicious: number;
  conversion_rate: number;
};

export type ReferralConversion = {
  id: string;
  referrer_id: string;
  referee_id: string;
  order_id: string | null;
  is_suspicious: boolean;
  fraud_signals: Array<string>;
  created_at: string;
};
