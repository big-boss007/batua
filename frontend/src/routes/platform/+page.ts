import type { PageLoad } from './$types';
import { fetchDashboardStats, fetchRecentEvents } from '$lib/client/modules/platform';
import type { DashboardStats, RecentEvent } from '$lib/client/modules/platform';

export const load: PageLoad = async () => {
  const [statsResult, eventsResult] = await Promise.all([
    fetchDashboardStats(),
    fetchRecentEvents(10)
  ]);

  const stats: DashboardStats | null = statsResult.tag === 'success' ? statsResult.data : null;
  const events: RecentEvent[] = eventsResult.tag === 'success' ? eventsResult.data : [];

  return { stats, events };
};
