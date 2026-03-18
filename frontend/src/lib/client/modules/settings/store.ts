import { writable } from 'svelte/store';
import type { WalletPolicy, Connector, NotificationTemplate } from './types';

function createWalletPoliciesStore() {
  const { subscribe, set, update } = writable<Array<WalletPolicy>>([]);

  return {
    subscribe,
    set,
    updatePolicy(updated: WalletPolicy) {
      update((policies) => policies.map((p) => (p.id === updated.id ? updated : p)));
    }
  };
}

function createConnectorsStore() {
  const { subscribe, set, update } = writable<Array<Connector>>([]);

  return {
    subscribe,
    set,
    addConnector(connector: Connector) {
      update((connectors) => [...connectors, connector]);
    },
    updateConnector(updated: Connector) {
      update((connectors) => connectors.map((c) => (c.id === updated.id ? updated : c)));
    }
  };
}

function createTemplatesStore() {
  const { subscribe, set, update } = writable<Array<NotificationTemplate>>([]);

  return {
    subscribe,
    set,
    updateTemplate(updated: NotificationTemplate) {
      update((templates) => templates.map((t) => (t.id === updated.id ? updated : t)));
    }
  };
}

export const walletPoliciesStore = createWalletPoliciesStore();
export const connectorsStore = createConnectorsStore();
export const templatesStore = createTemplatesStore();
