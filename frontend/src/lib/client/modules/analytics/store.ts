import { writable } from 'svelte/store';
import type { CodAnalytics, CampaignPerformance, OverviewMetrics, DateRange } from './types';

function createAnalyticsStore() {
  const { subscribe, set, update } = writable<{
    cod: CodAnalytics | null;
    campaigns: Array<CampaignPerformance>;
    overview: OverviewMetrics | null;
    loading: boolean;
  }>({
    cod: null,
    campaigns: [],
    overview: null,
    loading: false
  });

  return {
    subscribe,
    setCod(cod: CodAnalytics) {
      update((state) => ({ ...state, cod }));
    },
    setCampaigns(campaigns: Array<CampaignPerformance>) {
      update((state) => ({ ...state, campaigns }));
    },
    setOverview(overview: OverviewMetrics) {
      update((state) => ({ ...state, overview }));
    },
    setLoading(loading: boolean) {
      update((state) => ({ ...state, loading }));
    },
    clear() {
      set({ cod: null, campaigns: [], overview: null, loading: false });
    }
  };
}

function createDateRangeStore() {
  const today = new Date();
  const thirtyDaysAgo = new Date(today);
  thirtyDaysAgo.setDate(thirtyDaysAgo.getDate() - 30);

  const initial: DateRange = {
    from: thirtyDaysAgo.toISOString().slice(0, 10),
    to: today.toISOString().slice(0, 10)
  };

  const { subscribe, set } = writable<DateRange>(initial);

  return {
    subscribe,
    set(range: DateRange) {
      set(range);
    }
  };
}

export const analyticsStore = createAnalyticsStore();
export const dateRangeStore = createDateRangeStore();
