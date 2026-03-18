export type LedgerEntry = {
  id: string;
  wallet_id: string;
  bucket_type: string;
  movement_type: string;
  earning_unit: number;
  currency_equivalent: number;
  conversion_rate: number;
  state: string;
  event_id: string | null;
  actor_type: string;
  created_at: string;
  expires_at: string | null;
};

export type WalletBalance = {
  wallet_id: string;
  displayed_balance: number;
  spendable_balance: number;
  buckets: Array<BucketBalance>;
};

export type BucketBalance = {
  bucket_type: string;
  displayed: number;
  spendable: number;
  count: number;
};

export type RedemptionRequest = {
  id: string;
  wallet_id: string;
  state: string;
  requested_amount: number;
  applied_amount: number | null;
  order_id: string;
  created_at: string;
};

export type TransactionFilters = {
  bucket_type: string | null;
  movement_type: string | null;
  page: number;
  limit: number;
};

export type LedgerResponse = {
  entries: Array<LedgerEntry>;
  total: number;
  page: number;
  limit: number;
};

export type RedemptionResponse = {
  redemptions: Array<RedemptionRequest>;
  total: number;
  page: number;
  limit: number;
};

export type WalletLookupResult = {
  wallet_id: string;
  customer_id: string;
  merchant_id: string;
  created_at: string;
};

export type MerchantTransactionRow = {
  entry_id: string;
  wallet_id: string;
  customer_name: string | null;
  customer_phone: string;
  bucket_type: string;
  movement_type: string;
  currency_equivalent: number;
  state: string;
  created_at: string;
};
