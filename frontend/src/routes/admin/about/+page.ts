import type { PageLoad } from './$types';

export const load: PageLoad = ({ url }) => {
  return {
    activeTab: url.searchParams.get('tab') ?? 'overview'
  };
};
