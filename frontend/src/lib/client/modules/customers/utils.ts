import type { LoyaltyTier } from './types';

const tierColors: Record<number, string> = {
  1: 'var(--color-text-muted)',
  2: 'var(--color-info)',
  3: 'var(--color-warning)',
  4: 'var(--color-success)',
  5: 'var(--color-primary)'
};

function getTierColor(rank: number): string {
  return tierColors[rank] ?? 'var(--color-text-muted)';
}

function formatMultiplier(multiplier: number): string {
  return `${multiplier}x`;
}

function sortTiersByRank(tiers: Array<LoyaltyTier>): Array<LoyaltyTier> {
  return [...tiers].sort((a, b) => a.rank - b.rank);
}

function formatMovementType(movement: string): string {
  return movement.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());
}

const BUCKET_LABELS: Record<string, string> = {
  earned_credit: 'Reward Points',
  cod_pending: 'COD Pending Points',
  gift_card: 'Gift Card',
  customer_funded: 'Customer Funded',
  referral_reward: 'Referral Points',
  goodwill_credit: 'Courtesy Points',
  membership_benefit: 'Membership Benefit',
  refund_credit: 'Store Credit',
  EarnedCredit: 'Reward Points',
  CodPending: 'COD Pending Points',
  GiftCard: 'Gift Card',
  CustomerFunded: 'Customer Funded',
  ReferralReward: 'Referral Points',
  GoodwillCredit: 'Courtesy Points',
  MembershipBenefit: 'Membership Benefit',
  RefundCredit: 'Store Credit'
};

function formatBucketType(bucket: string): string {
  return BUCKET_LABELS[bucket] ?? bucket.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());
}

export { getTierColor, formatMultiplier, sortTiersByRank, formatMovementType, formatBucketType };
