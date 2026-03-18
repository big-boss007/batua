import type { PageLoad } from './$types';

export const load: PageLoad = async ({ url }) => {
  const prefilledCode = url.searchParams.get('code');
  return { prefilledCode };
};
