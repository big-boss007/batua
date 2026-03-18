import type { LayoutLoad } from './$types';
import { fetchMerchantBySlug } from '$lib/client/modules/storefront';

export const load: LayoutLoad = async ({ params }) => {
  const result = await fetchMerchantBySlug(params.slug);
  if (result.tag === 'success') {
    return { merchant: result.data };
  }
  return { merchant: null };
};
