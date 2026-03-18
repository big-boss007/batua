export type PlatformMerchant = {
  id: string;
  name: string;
  slug: string | null;
  domain: string | null;
  external_id: string;
  currency: string;
  timezone: string;
  plan_tier: string;
  is_active: boolean;
  geo_policy_id: string | null;
  created_at: string;
};

export type MerchantStats = {
  merchant_id: string;
  total_wallets: number;
  total_customers: number;
  total_ledger_entries: number;
  active_credits: number;
  total_redeemed: number;
};

export type SystemHealth = {
  unprocessed_events: number;
  failed_events: number;
  pending_cod_orders: number;
  expiring_7d_count: number;
  expiring_7d_value: number;
  expiring_30d_count: number;
  expiring_30d_value: number;
};

export type GeoPolicy = {
  id: string;
  geo_code: string;
  name: string;
  config: Record<string, unknown>;
  is_active: boolean;
  created_at: string;
};

export type RecentEvent = {
  id: string;
  merchant_id: string;
  merchant_name: string;
  event_type: string;
  event_source: string;
  state: string;
  created_at: string;
};

export type DashboardStats = {
  total_merchants: number;
  total_wallets: number;
  total_ledger_entries: number;
  total_value_in_system: number;
};

export type OnboardMerchantForm = {
  name: string;
  external_id: string;
  domain: string | null;
  slug: string | null;
  plan_tier: string;
};
