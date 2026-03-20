import { apiCaller } from '$lib/client/modules/foundation';
import type { APIResult } from '$lib/client/modules/foundation';
import type { GiftCard, GiftCardStats, IssueGiftCardForm, BulkIssueForm } from './types';

function decodeGiftCard(raw: unknown): GiftCard {
  const r = raw as Record<string, unknown>;
  return {
    id: (r['id'] as string) ?? '',
    code: (r['code'] as string) ?? '',
    initial_amount: (r['initial_amount'] as number) ?? 0,
    current_amount: (r['current_amount'] as number) ?? 0,
    is_claimed: (r['is_claimed'] as boolean) ?? false,
    is_active: (r['is_active'] as boolean) ?? false,
    expires_at: (r['expires_at'] as string) ?? null,
    created_at: (r['created_at'] as string) ?? ''
  };
}

function decodeGiftCards(raw: unknown): Array<GiftCard> {
  const r = raw as Record<string, unknown>;
  const items = (r['gift_cards'] ?? r['data'] ?? raw) as Array<unknown>;
  if (!Array.isArray(items)) return [];
  return items.map(decodeGiftCard);
}

async function issueGiftCard(form: IssueGiftCardForm): Promise<APIResult<GiftCard>> {
  return apiCaller.post(
    '/gift-cards/issue',
    {
      amount: form.amount,
      expires_at: form.expires_at
    },
    decodeGiftCard
  );
}

async function bulkIssue(form: BulkIssueForm): Promise<APIResult<Array<GiftCard>>> {
  return apiCaller.post(
    '/gift-cards/bulk-issue',
    { cards: form.cards as unknown as Record<string, unknown>[] },
    decodeGiftCards
  );
}

async function fetchGiftCards(merchantId: string): Promise<APIResult<Array<GiftCard>>> {
  return apiCaller.get(`/gift-cards/merchant/${merchantId}`, decodeGiftCards);
}

async function getGiftCardByCode(code: string): Promise<APIResult<GiftCard>> {
  return apiCaller.get(`/gift-cards/${code}`, decodeGiftCard);
}

async function claimGiftCard(code: string): Promise<APIResult<GiftCard>> {
  return apiCaller.post('/gift-cards/claim', { code }, decodeGiftCard);
}

async function redeemGiftCard(code: string, amount: number): Promise<APIResult<GiftCard>> {
  return apiCaller.post('/gift-cards/redeem', { code, amount }, decodeGiftCard);
}

function decodeGiftCardStats(raw: unknown): GiftCardStats {
  const r = raw as Record<string, unknown>;
  return {
    total_issued: (r['total_issued'] as number) ?? 0,
    total_outstanding_value: (r['total_outstanding_value'] as number) ?? 0,
    total_redeemed_value: (r['total_redeemed_value'] as number) ?? 0,
    total_expired: (r['total_expired'] as number) ?? 0,
    total_active: (r['total_active'] as number) ?? 0,
    total_claimed: (r['total_claimed'] as number) ?? 0
  };
}

async function fetchGiftCardStats(merchantId: string): Promise<APIResult<GiftCardStats>> {
  return apiCaller.get(`/gift-cards/merchant/${merchantId}/stats`, decodeGiftCardStats);
}

export {
  issueGiftCard,
  bulkIssue,
  fetchGiftCards,
  getGiftCardByCode,
  claimGiftCard,
  redeemGiftCard,
  fetchGiftCardStats
};
