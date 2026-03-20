import type { PageLoad } from './$types';

export const load: PageLoad = async ({ url }) => {
  return {
    activeTab: url.searchParams.get('tab') ?? 'store'
  };
};
