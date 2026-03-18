import type { PageLoad } from './$types';
import { fetchTemplates } from '$lib/client/modules/settings';

export const load: PageLoad = async ({ url }) => {
  const merchantId = url.searchParams.get('merchant') ?? 'default';

  const result = await fetchTemplates(merchantId);

  return {
    templates: result.tag === 'success' ? result.data : []
  };
};
