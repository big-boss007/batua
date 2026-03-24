export type Rule = {
  id: string;
  merchant_id: string;
  rule_type: string;
  name: string;
  config: RewardRuleConfig;
  version: number;
  is_active: boolean;
  created_at: string;
};

export type RewardRuleConfig = {
  event_type: string;
  conditions: Array<Condition>;
  action: RewardAction;
};

export type Condition = {
  field: string;
  operator: string;
  value: unknown;
};

export type RewardAction = {
  bucket_type: string;
  calculation: string;
  value: number;
  max_amount: number | null;
  expiry_days: number | null;
};

export type Campaign = {
  id: string;
  merchant_id: string;
  name: string;
  campaign_type: string;
  base_rule_id: string | null;
  multiplier: number | null;
  starts_at: string;
  ends_at: string;
  is_active: boolean;
};

export type CampaignPerformance = {
  total_orders: number;
  unique_customers: number;
  extra_points_issued: number;
  extra_value: number;
};

export type FestiveTemplate = {
  name: string;
  display_name: string;
  description: string;
  default_multiplier: number;
  default_duration_days: number;
  category: string;
};

export type CreateRuleRequest = {
  merchant_id: string;
  rule_type: string;
  name: string;
  config: RewardRuleConfig;
};

export type UpdateRuleRequest = {
  name?: string;
  config: RewardRuleConfig;
  is_active?: boolean;
};

export type CreateCampaignFromTemplateRequest = {
  merchant_id: string;
  template_name: string;
  base_rule_id: string;
  name: string;
  multiplier: number;
  starts_at: string;
  ends_at: string;
};

export type CampaignCalendarEntry = {
  id: string;
  name: string;
  campaign_type: string;
  starts_at: string;
  ends_at: string;
  is_active: boolean;
};

export type RulePerformance = {
  rule_id: string;
  total_entries: number;
  total_value: number;
  unique_customers: number;
};

export type CreateCampaignDirectRequest = {
  merchant_id: string;
  name: string;
  base_rule_id: string;
  multiplier: number;
  starts_at: string;
  ends_at: string;
};

export type CampaignStackingConfig = {
  campaign_stacking_mode: string;
  max_campaign_multiplier: number;
};
