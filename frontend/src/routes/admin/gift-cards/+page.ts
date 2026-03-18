import type { PageLoad } from './$types';
import { fetchGiftCards } from '$lib/client/modules/gift-cards';
import type { GiftCard } from '$lib/client/modules/gift-cards';

export const load: PageLoad = async () => {
  const merchantId = 'default';
  const result = await fetchGiftCards(merchantId);

  const cards: Array<GiftCard> = result.tag === 'success' ? result.data : [];

  return { cards, merchantId };
};
