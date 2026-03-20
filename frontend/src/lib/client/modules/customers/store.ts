import { writable } from 'svelte/store';
import type {
  Customer,
  CustomerDetail,
  LoyaltyProgram,
  LoyaltyTier,
  TierDistribution
} from './types';

function createCustomerSearchStore() {
  const { subscribe, set, update } = writable<{
    results: Array<Customer>;
    loading: boolean;
    query: string;
  }>({
    results: [],
    loading: false,
    query: ''
  });

  return {
    subscribe,
    setQuery(query: string) {
      update((state) => ({ ...state, query }));
    },
    setLoading(loading: boolean) {
      update((state) => ({ ...state, loading }));
    },
    setResults(results: Array<Customer>) {
      update((state) => ({ ...state, results, loading: false }));
    },
    clear() {
      set({ results: [], loading: false, query: '' });
    }
  };
}

function createCustomerDetailStore() {
  const { subscribe, set } = writable<{
    detail: CustomerDetail | null;
    loading: boolean;
  }>({
    detail: null,
    loading: false
  });

  return {
    subscribe,
    setDetail(detail: CustomerDetail) {
      set({ detail, loading: false });
    },
    setLoading() {
      set({ detail: null, loading: true });
    },
    clear() {
      set({ detail: null, loading: false });
    }
  };
}

function createLoyaltyStore() {
  const { subscribe, set, update } = writable<{
    program: LoyaltyProgram | null;
    tiers: Array<LoyaltyTier>;
    distribution: Array<TierDistribution>;
    loading: boolean;
  }>({
    program: null,
    tiers: [],
    distribution: [],
    loading: false
  });

  return {
    subscribe,
    setProgram(program: LoyaltyProgram) {
      update((state) => ({ ...state, program }));
    },
    setTiers(tiers: Array<LoyaltyTier>) {
      update((state) => ({ ...state, tiers }));
    },
    setDistribution(distribution: Array<TierDistribution>) {
      update((state) => ({ ...state, distribution }));
    },
    setLoading(loading: boolean) {
      update((state) => ({ ...state, loading }));
    },
    clear() {
      set({ program: null, tiers: [], distribution: [], loading: false });
    }
  };
}

export const customerSearchStore = createCustomerSearchStore();
export const customerDetailStore = createCustomerDetailStore();
export const loyaltyStore = createLoyaltyStore();
