export type GiftCard = {
  id: string;
  code: string;
  initial_amount: number;
  current_amount: number;
  is_claimed: boolean;
  is_active: boolean;
  expires_at: string | null;
  created_at: string;
};

export type IssueGiftCardForm = {
  merchant_id: string;
  amount: number;
  expires_at: string | null;
};

export type BulkIssueForm = {
  merchant_id: string;
  batch_id: string;
  cards: Array<{ amount: number; recipient_phone: string | null }>;
};

export type BulkIssueInput = {
  cards: Array<{ amount: number; recipient_phone: string | null }>;
};

export type GiftCardStats = {
  total_issued: number;
  total_outstanding_value: number;
  total_redeemed_value: number;
  total_expired: number;
  total_active: number;
  total_claimed: number;
};
