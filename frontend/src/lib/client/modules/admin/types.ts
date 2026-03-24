export type DashboardStats = {
  total_merchants: number;
  total_wallets: number;
  total_ledger_entries: number;
  total_value_in_system: number;
};

export type Merchant = {
  id: string;
  external_id: string;
  name: string;
  domain: string | null;
  slug: string | null;
  plan_tier: string | null;
  currency: string;
  timezone: string;
  is_active: boolean;
  points_name: string;
  points_icon: string;
  points_to_currency_rate: number;
};

export type MerchantDashboard = {
  merchant_id: string;
  active_customers: number;
  total_wallets: number;
  total_earned: number;
  total_redeemed: number;
  total_cod_pending: number;
  active_credits: number;
  total_ledger_entries: number;
  redemption_count: number;
};

export type Breadcrumb = {
  label: string;
  href: string | null;
};

export type NavItem = {
  label: string;
  href: string;
  icon: string;
  children?: NavItem[];
};
