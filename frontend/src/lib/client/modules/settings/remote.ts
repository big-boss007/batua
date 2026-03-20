import { apiCaller } from '$lib/client/modules/foundation';
import type { APIResult } from '$lib/client/modules/foundation';
import type {
  WalletPolicy,
  Connector,
  NotificationTemplate,
  NotificationLog,
  UpdateWalletPolicyRequest,
  CreateConnectorRequest,
  UpdateTemplateRequest
} from './types';

function decodeWalletPolicies(raw: unknown): Array<WalletPolicy> {
  if (Array.isArray(raw)) return raw as Array<WalletPolicy>;
  const data = raw as Record<string, unknown>;
  const items = data['policies'] ?? data['data'] ?? [];
  return Array.isArray(items) ? (items as Array<WalletPolicy>) : [];
}

function decodeWalletPolicy(raw: unknown): WalletPolicy {
  return raw as WalletPolicy;
}

function decodeConnectors(raw: unknown): Array<Connector> {
  if (Array.isArray(raw)) return raw as Array<Connector>;
  const data = raw as Record<string, unknown>;
  const items = data['connectors'] ?? data['data'] ?? [];
  return Array.isArray(items) ? (items as Array<Connector>) : [];
}

function decodeConnector(raw: unknown): Connector {
  return raw as Connector;
}

function decodeTemplates(raw: unknown): Array<NotificationTemplate> {
  if (Array.isArray(raw)) return raw as Array<NotificationTemplate>;
  const data = raw as Record<string, unknown>;
  const items = data['templates'] ?? data['data'] ?? [];
  return Array.isArray(items) ? (items as Array<NotificationTemplate>) : [];
}

function decodeTemplate(raw: unknown): NotificationTemplate {
  return raw as NotificationTemplate;
}

async function fetchWalletPolicies(merchantId: string): Promise<APIResult<Array<WalletPolicy>>> {
  return apiCaller.get(`/admin/wallet-policies/${merchantId}`, decodeWalletPolicies);
}

async function updateWalletPolicy(
  policyId: string,
  body: UpdateWalletPolicyRequest
): Promise<APIResult<WalletPolicy>> {
  return apiCaller.put(
    `/wallets/policies/${policyId}`,
    body as unknown as Record<string, unknown>,
    decodeWalletPolicy
  );
}

async function fetchConnectors(merchantId: string): Promise<APIResult<Array<Connector>>> {
  return apiCaller.get('/notifications/connectors', decodeConnectors, { merchant_id: merchantId });
}

async function createConnector(
  _merchantId: string,
  body: CreateConnectorRequest
): Promise<APIResult<Connector>> {
  return apiCaller.post(
    '/notifications/connectors',
    body as unknown as Record<string, unknown>,
    decodeConnector
  );
}

async function fetchTemplates(merchantId: string): Promise<APIResult<Array<NotificationTemplate>>> {
  return apiCaller.get('/notifications/templates', decodeTemplates, { merchant_id: merchantId });
}

async function updateTemplate(
  templateId: string,
  body: UpdateTemplateRequest
): Promise<APIResult<NotificationTemplate>> {
  return apiCaller.put(
    `/notifications/templates/${templateId}`,
    body as unknown as Record<string, unknown>,
    decodeTemplate
  );
}

function decodeMerchant(raw: unknown): import('$lib/client/modules/admin').Merchant {
  const r = raw as Record<string, unknown>;
  return {
    id: (r['id'] as string) ?? '',
    external_id: (r['external_id'] as string) ?? '',
    name: (r['name'] as string) ?? '',
    domain: (r['domain'] as string) ?? null,
    slug: (r['slug'] as string) ?? null,
    plan_tier: (r['plan_tier'] as string) ?? null,
    currency: (r['currency'] as string) ?? 'INR',
    timezone: (r['timezone'] as string) ?? 'Asia/Kolkata',
    is_active: (r['is_active'] as boolean) ?? false
  };
}

async function updateMerchantProfile(
  id: string,
  data: { name?: string; domain?: string; slug?: string }
): Promise<APIResult<import('$lib/client/modules/admin').Merchant>> {
  return apiCaller.put(`/admin/merchants/${id}`, data as Record<string, unknown>, decodeMerchant);
}

function decodeNotificationLog(raw: unknown): NotificationLog {
  const r = raw as Record<string, unknown>;
  return {
    id: (r['id'] as string) ?? '',
    customer_id: (r['customer_id'] as string) ?? '',
    channel: (r['channel'] as string) ?? '',
    status: (r['status'] as string) ?? '',
    created_at: (r['created_at'] as string) ?? ''
  };
}

function decodeNotificationLogs(raw: unknown): Array<NotificationLog> {
  const r = raw as Record<string, unknown>;
  const items = (r['logs'] ?? r['data'] ?? raw) as Array<unknown>;
  if (!Array.isArray(items)) return [];
  return items.map(decodeNotificationLog);
}

async function fetchNotificationLogs(
  merchantId: string,
  page: number,
  limit: number
): Promise<APIResult<Array<NotificationLog>>> {
  return apiCaller.get('/notifications/logs', decodeNotificationLogs, {
    merchant_id: merchantId,
    page: String(page),
    limit: String(limit)
  });
}

export {
  fetchWalletPolicies,
  updateWalletPolicy,
  fetchConnectors,
  createConnector,
  fetchTemplates,
  updateTemplate,
  updateMerchantProfile,
  fetchNotificationLogs
};
