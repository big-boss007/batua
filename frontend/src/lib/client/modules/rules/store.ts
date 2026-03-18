import { writable } from 'svelte/store';
import type { Rule, Campaign } from './types';

function createRulesStore() {
  const { subscribe, set, update } = writable<Array<Rule>>([]);

  return {
    subscribe,
    set,
    addRule(rule: Rule) {
      update((rules) => [...rules, rule]);
    },
    updateRule(updated: Rule) {
      update((rules) => rules.map((r) => (r.id === updated.id ? updated : r)));
    },
    toggleRule(id: string) {
      update((rules) =>
        rules.map((r) => (r.id === id ? { ...r, is_active: !r.is_active } : r))
      );
    }
  };
}

function createCampaignsStore() {
  const { subscribe, set, update } = writable<Array<Campaign>>([]);

  return {
    subscribe,
    set,
    addCampaign(campaign: Campaign) {
      update((campaigns) => [...campaigns, campaign]);
    }
  };
}

function createSelectedRuleStore() {
  const { subscribe, set } = writable<Rule | null>(null);

  return {
    subscribe,
    select(rule: Rule) {
      set(rule);
    },
    clear() {
      set(null);
    }
  };
}

export const rulesStore = createRulesStore();
export const campaignsStore = createCampaignsStore();
export const selectedRuleStore = createSelectedRuleStore();
