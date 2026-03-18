import type { PageLoad } from './$types';
import {
  fetchLoyaltyProgram,
  fetchTiers,
  fetchTierDistribution
} from '$lib/client/modules/customers';
import type { LoyaltyProgram, LoyaltyTier, TierDistribution } from '$lib/client/modules/customers';

export type LoyaltyPageData = {
  program: LoyaltyProgram | null;
  tiers: Array<LoyaltyTier>;
  distribution: Array<TierDistribution>;
};

export const load: PageLoad = async (): Promise<LoyaltyPageData> => {
  const merchantId = 'default';

  const [programResult, tiersResult, distributionResult] = await Promise.all([
    fetchLoyaltyProgram(merchantId),
    fetchTiers(merchantId),
    fetchTierDistribution(merchantId)
  ]);

  return {
    program: programResult.tag === 'success' ? programResult.data : null,
    tiers: tiersResult.tag === 'success' ? tiersResult.data : [],
    distribution: distributionResult.tag === 'success' ? distributionResult.data : []
  };
};
