import type { PageLoad } from './$types';
import { fetchMerchants } from '$lib/client/modules/platform';
import type { PlatformMerchant } from '$lib/client/modules/platform';

export const load: PageLoad = async () => {
  const result = await fetchMerchants(1, 100);
  const merchants: PlatformMerchant[] = result.tag === 'success' ? result.data : [];
  return { merchants };
};
