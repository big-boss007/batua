import type { PageLoad } from './$types';
import { fetchMerchant, fetchMerchantStats } from '$lib/client/modules/platform';
import type { PlatformMerchant, MerchantStats } from '$lib/client/modules/platform';

export const load: PageLoad = async ({ params }) => {
  const [merchantResult, statsResult] = await Promise.all([
    fetchMerchant(params.id),
    fetchMerchantStats(params.id)
  ]);

  const merchant: PlatformMerchant | null =
    merchantResult.tag === 'success' ? merchantResult.data : null;
  const stats: MerchantStats | null =
    statsResult.tag === 'success' ? statsResult.data : null;

  return { merchant, stats };
};
