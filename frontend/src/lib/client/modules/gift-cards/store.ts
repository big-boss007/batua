import { writable } from 'svelte/store';
import type { GiftCard } from './types';

function createGiftCardsStore() {
  const { subscribe, set, update } = writable<Array<GiftCard>>([]);

  return {
    subscribe,
    set(cards: Array<GiftCard>) {
      set(cards);
    },
    add(card: GiftCard) {
      update((cards) => [card, ...cards]);
    },
    addMany(newCards: Array<GiftCard>) {
      update((cards) => [...newCards, ...cards]);
    },
    clear() {
      set([]);
    }
  };
}

export const giftCards = createGiftCardsStore();
