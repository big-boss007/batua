import type { PageLoad } from './$types';
import { fetchProgram, fetchAnalytics, fetchConversions } from '$lib/client/modules/referrals';
import type {
  ReferralProgram,
  ReferralAnalytics,
  ReferralConversion
} from '$lib/client/modules/referrals';

export const load: PageLoad = async () => {
  const merchantId = 'default';

  const [programResult, analyticsResult, conversionsResult] = await Promise.all([
    fetchProgram(merchantId),
    fetchAnalytics(merchantId),
    fetchConversions(merchantId)
  ]);

  const program: ReferralProgram | null =
    programResult.tag === 'success' ? programResult.data : null;
  const analytics: ReferralAnalytics | null =
    analyticsResult.tag === 'success' ? analyticsResult.data : null;
  const conversions: Array<ReferralConversion> =
    conversionsResult.tag === 'success' ? conversionsResult.data : [];

  return { program, analytics, conversions, merchantId };
};
