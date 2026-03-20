import type { PageLoad } from './$types';
import { fetchSystemHealth } from '$lib/client/modules/platform';
import type { SystemHealth } from '$lib/client/modules/platform';

export const load: PageLoad = async () => {
  const result = await fetchSystemHealth();
  const health: SystemHealth | null = result.tag === 'success' ? result.data : null;
  return { health };
};
