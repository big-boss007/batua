export type CodAnalytics = {
  total_pending: number;
  total_delivered: number;
  total_rto: number;
  pending_amount: number;
  released_amount: number;
  cancelled_amount: number;
};

export type CampaignPerformance = {
  campaign_id: string;
  name: string;
  total_entries: number;
  total_value: number;
  unique_customers: number;
  average_reward: number;
};

export type OverviewMetrics = {
  total_wallets: number;
  total_active_credits: number;
  total_redeemed: number;
  total_expired: number;
  rto_rate_loyalty: number;
  rto_rate_non_loyalty: number;
};

export type DateRange = {
  from: string;
  to: string;
};

export type MerchantAnalytics = {
  total_earned: number;
  total_redeemed: number;
  total_expired: number;
  active_credits: number;
  cod_pending: number;
  cod_delivered: number;
  cod_rto: number;
  total_orders: number;
  prepaid_orders: number;
  cod_orders: number;
  loyalty_rto_rate: number;
  non_loyalty_rto_rate: number;
  repeat_purchase_rate: number;
};
