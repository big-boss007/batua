import type { PageLoad } from './$types';
import { fetchGeoPolicies } from '$lib/client/modules/platform';
import type { GeoPolicy } from '$lib/client/modules/platform';

export const load: PageLoad = async () => {
  const result = await fetchGeoPolicies();
  const policies: GeoPolicy[] =
    result.tag === 'success' ? result.data : [];
  return { policies };
};
