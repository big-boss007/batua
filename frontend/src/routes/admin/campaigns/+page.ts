import type { PageLoad } from './$types';
import {
  fetchCampaigns,
  fetchCampaignCalendar,
  fetchFestiveTemplates,
  fetchRules
} from '$lib/client/modules/rules';
import type { Campaign, FestiveTemplate, CampaignCalendarEntry, Rule } from '$lib/client/modules/rules';

export const load: PageLoad = async () => {
  const merchantId = 'default';

  const now = new Date();
  const fromDate = new Date(now.getFullYear(), now.getMonth(), 1).toISOString().split('T')[0];
  const toDate = new Date(now.getFullYear(), now.getMonth() + 3, 0).toISOString().split('T')[0];

  const [campaignsResult, calendarResult, templatesResult, rulesResult] = await Promise.all([
    fetchCampaigns(merchantId),
    fetchCampaignCalendar(merchantId, fromDate, toDate),
    fetchFestiveTemplates(),
    fetchRules(merchantId)
  ]);

  let campaigns: Array<Campaign> = [];
  if (campaignsResult.tag === 'success') {
    campaigns = campaignsResult.data;
  }

  let calendar: Array<CampaignCalendarEntry> = [];
  if (calendarResult.tag === 'success') {
    calendar = calendarResult.data;
  }

  let templates: Array<FestiveTemplate> = [];
  if (templatesResult.tag === 'success') {
    templates = templatesResult.data;
  }

  let rules: Array<Rule> = [];
  if (rulesResult.tag === 'success') {
    rules = rulesResult.data;
  }

  return { campaigns, calendar, templates, rules, merchantId };
};
