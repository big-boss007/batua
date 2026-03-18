import { writable } from 'svelte/store';
import type { StorefrontMerchant } from './types';

function createCustomerPhoneStore() {
  const stored = typeof window !== 'undefined'
    ? sessionStorage.getItem('batua-storefront-phone')
    : null;
  const { subscribe, set } = writable<string | null>(stored);

  return {
    subscribe,
    set(phone: string | null) {
      if (typeof window !== 'undefined') {
        if (phone !== null) {
          sessionStorage.setItem('batua-storefront-phone', phone);
        } else {
          sessionStorage.removeItem('batua-storefront-phone');
        }
      }
      set(phone);
    },
    clear() {
      if (typeof window !== 'undefined') {
        sessionStorage.removeItem('batua-storefront-phone');
      }
      set(null);
    }
  };
}

function createMerchantContextStore() {
  const { subscribe, set } = writable<StorefrontMerchant | null>(null);

  return {
    subscribe,
    set(merchant: StorefrontMerchant | null) {
      set(merchant);
    },
    clear() {
      set(null);
    }
  };
}

export const customerPhone = createCustomerPhoneStore();
export const merchantContext = createMerchantContextStore();
