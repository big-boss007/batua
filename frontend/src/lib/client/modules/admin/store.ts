import { writable, derived } from 'svelte/store';
import type { Merchant, Breadcrumb } from './types';
import { getCurrentMerchantId, setCurrentMerchantId } from './utils';

function createMerchantStore() {
  const { subscribe, set } = writable<Merchant | null>(null);

  return {
    subscribe,
    set(merchant: Merchant) {
      setCurrentMerchantId(merchant.id);
      set(merchant);
    },
    clear() {
      set(null);
    }
  };
}

function createBreadcrumbStore() {
  const { subscribe, set } = writable<Breadcrumb[]>([]);

  return {
    subscribe,
    set(crumbs: Breadcrumb[]) {
      set(crumbs);
    },
    clear() {
      set([]);
    }
  };
}

export const currentMerchant = createMerchantStore();

export const currentMerchantId = derived(currentMerchant, ($merchant) => {
  if ($merchant !== null) return $merchant.id;
  return getCurrentMerchantId();
});

export const breadcrumbs = createBreadcrumbStore();
