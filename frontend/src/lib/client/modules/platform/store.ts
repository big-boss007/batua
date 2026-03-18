import { writable } from 'svelte/store';
import type { PlatformMerchant } from './types';

function createMerchantsStore() {
  const { subscribe, set, update } = writable<PlatformMerchant[]>([]);

  return {
    subscribe,
    set(merchants: PlatformMerchant[]) {
      set(merchants);
    },
    clear() {
      set([]);
    }
  };
}

function createSelectedMerchantStore() {
  const { subscribe, set } = writable<PlatformMerchant | null>(null);

  return {
    subscribe,
    set(merchant: PlatformMerchant | null) {
      set(merchant);
    },
    clear() {
      set(null);
    }
  };
}

export const merchantsList = createMerchantsStore();
export const selectedMerchant = createSelectedMerchantStore();
