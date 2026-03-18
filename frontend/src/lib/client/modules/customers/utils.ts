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

function formatBucketType(bucket: string): string {
  return bucket.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());
}

export { getTierColor, formatMultiplier, sortTiersByRank, formatMovementType, formatBucketType };
