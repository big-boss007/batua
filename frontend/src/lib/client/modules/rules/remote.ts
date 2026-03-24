import { apiCaller } from '$lib/client/modules/foundation';
import type { APIResult } from '$lib/client/modules/foundation';
import type {
  Rule,
  RulePerformance,
  Campaign,
  FestiveTemplate,
  CampaignCalendarEntry,
  CampaignStackingConfig,
  CreateRuleRequest,
  UpdateRuleRequest,
  CreateCampaignFromTemplateRequest,
  CreateCampaignDirectRequest
} from './types';

function decodeRules(raw: unknown): Array<Rule> {
  if (Array.isArray(raw)) return raw as Array<Rule>;
  const data = raw as Record<string, unknown>;
  const items = data['rules'] ?? data['data'] ?? [];
  return Array.isArray(items) ? (items as Array<Rule>) : [];
}

function decodeRule(raw: unknown): Rule {
  return raw as Rule;
}

function decodeCampaigns(raw: unknown): Array<Campaign> {
  if (Array.isArray(raw)) return raw as Array<Campaign>;
  const data = raw as Record<string, unknown>;
  const items = data['campaigns'] ?? data['data'] ?? [];
  return Array.isArray(items) ? (items as Array<Campaign>) : [];
}

function decodeCampaign(raw: unknown): Campaign {
  return raw as Campaign;
}

function decodeTemplates(raw: unknown): Array<FestiveTemplate> {
  if (Array.isArray(raw)) return raw as Array<FestiveTemplate>;
  const data = raw as Record<string, unknown>;
  const items = data['templates'] ?? data['data'] ?? [];
  return Array.isArray(items) ? (items as Array<FestiveTemplate>) : [];
}

function decodeCalendar(raw: unknown): Array<CampaignCalendarEntry> {
  if (Array.isArray(raw)) return raw as Array<CampaignCalendarEntry>;
  const data = raw as Record<string, unknown>;
  const items = data['entries'] ?? data['data'] ?? [];
  return Array.isArray(items) ? (items as Array<CampaignCalendarEntry>) : [];
}

export async function fetchRules(merchantId: string): Promise<APIResult<Array<Rule>>> {
  return apiCaller.get('/rules', decodeRules, { merchant_id: merchantId });
}

export async function createRule(req: CreateRuleRequest): Promise<APIResult<Rule>> {
  return apiCaller.post('/rules', req as unknown as Record<string, unknown>, decodeRule);
}

export async function updateRule(id: string, config: UpdateRuleRequest): Promise<APIResult<Rule>> {
  return apiCaller.put(`/rules/${id}`, config as unknown as Record<string, unknown>, decodeRule);
}

export async function fetchCampaigns(merchantId: string): Promise<APIResult<Array<Campaign>>> {
  return apiCaller.get('/campaigns/calendar', decodeCampaigns, { merchant_id: merchantId });
}

export async function createCampaignFromTemplate(
  req: CreateCampaignFromTemplateRequest
): Promise<APIResult<Campaign>> {
  return apiCaller.post(
    '/campaigns/from-template',
    req as unknown as Record<string, unknown>,
    decodeCampaign
  );
}

export async function fetchFestiveTemplates(): Promise<APIResult<Array<FestiveTemplate>>> {
  return apiCaller.get('/campaigns/templates', decodeTemplates);
}

export async function fetchCampaignCalendar(
  merchantId: string,
  from: string,
  to: string
): Promise<APIResult<Array<CampaignCalendarEntry>>> {
  return apiCaller.get('/campaigns/calendar', decodeCalendar, {
    merchant_id: merchantId,
    from,
    to
  });
}

function decodeRulePerformance(raw: unknown): RulePerformance {
  const r = raw as Record<string, unknown>;
  return {
    rule_id: (r['rule_id'] as string) ?? '',
    total_entries: (r['total_entries'] as number) ?? 0,
    total_value: (r['total_value'] as number) ?? 0,
    unique_customers: (r['unique_customers'] as number) ?? 0
  };
}

export async function fetchRulePerformance(ruleId: string): Promise<APIResult<RulePerformance>> {
  return apiCaller.get(`/rules/${ruleId}/performance`, decodeRulePerformance);
}

export async function createCampaignDirect(
  req: CreateCampaignDirectRequest
): Promise<APIResult<Campaign>> {
  return apiCaller.post(
    '/campaigns/create',
    req as unknown as Record<string, unknown>,
    decodeCampaign
  );
}

export async function deactivateCampaign(campaignId: string): Promise<APIResult<Campaign>> {
  return apiCaller.post(`/campaigns/${campaignId}/deactivate`, {}, decodeCampaign);
}

function decodeStackingConfig(raw: unknown): CampaignStackingConfig {
  const r = raw as Record<string, unknown>;
  return {
    campaign_stacking_mode: (r['campaign_stacking_mode'] as string) ?? 'multiplicative',
    max_campaign_multiplier: (r['max_campaign_multiplier'] as number) ?? 10.0
  };
}

export async function getCampaignConfig(
  merchantId: string
): Promise<APIResult<CampaignStackingConfig>> {
  return apiCaller.get(
    `/admin/merchants/${merchantId}/campaign-config`,
    decodeStackingConfig
  );
}

export async function updateCampaignConfig(
  merchantId: string,
  config: CampaignStackingConfig
): Promise<APIResult<CampaignStackingConfig>> {
  return apiCaller.put(
    `/admin/merchants/${merchantId}/campaign-config`,
    config as unknown as Record<string, unknown>,
    decodeStackingConfig
  );
}
