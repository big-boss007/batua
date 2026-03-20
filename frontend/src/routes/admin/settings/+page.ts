import type { PageLoad } from './$types';
import { fetchWalletPolicies, fetchConnectors, fetchTemplates } from '$lib/client/modules/settings';

export const load: PageLoad = async ({ url }) => {
  const merchantId = url.searchParams.get('merchant') ?? 'default';

  const [policiesResult, connectorsResult, templatesResult] = await Promise.all([
    fetchWalletPolicies(merchantId),
    fetchConnectors(merchantId),
    fetchTemplates(merchantId)
  ]);

  return {
    policies: policiesResult.tag === 'success' ? policiesResult.data : [],
    connectors: connectorsResult.tag === 'success' ? connectorsResult.data : [],
    templates: templatesResult.tag === 'success' ? templatesResult.data : [],
    activeTab: url.searchParams.get('tab') ?? 'store'
  };
};
