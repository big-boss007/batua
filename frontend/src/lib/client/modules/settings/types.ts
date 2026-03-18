export type WalletPolicy = {
  id: string;
  merchant_id: string;
  bucket_type: string;
  min_redemption: number | null;
  step_size: number | null;
  max_per_order_pct: number | null;
  max_per_order_fixed: number | null;
  stackable_with_discounts: boolean;
  default_expiry_days: number | null;
  is_transferable: boolean;
};

export type Connector = {
  id: string;
  capability: string;
  vendor: string;
  config: Record<string, unknown>;
  is_active: boolean;
  priority: number;
};

export type NotificationTemplate = {
  id: string;
  name: string;
  channel: string;
  locale: string;
  body_template: string;
  is_active: boolean;
};

export type UpdateWalletPolicyRequest = {
  min_redemption: number | null;
  step_size: number | null;
  max_per_order_pct: number | null;
  max_per_order_fixed: number | null;
  stackable_with_discounts: boolean;
  default_expiry_days: number | null;
  is_transferable: boolean;
};

export type CreateConnectorRequest = {
  capability: string;
  vendor: string;
  config: Record<string, unknown>;
  priority: number;
};

export type UpdateTemplateRequest = {
  body_template: string;
  is_active: boolean;
};

export type NotificationLog = {
  id: string;
  customer_id: string;
  channel: string;
  status: string;
  created_at: string;
};
