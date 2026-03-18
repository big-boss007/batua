import { writable } from 'svelte/store';
import type { ReferralProgram, ReferralCode } from './types';

function createReferralProgramStore() {
  const { subscribe, set } = writable<ReferralProgram | null>(null);

  return {
    subscribe,
    set(program: ReferralProgram) {
      set(program);
    },
    clear() {
      set(null);
    }
  };
}

function createReferralCodesStore() {
  const { subscribe, set, update } = writable<Array<ReferralCode>>([]);

  return {
    subscribe,
    set(codes: Array<ReferralCode>) {
      set(codes);
    },
    add(code: ReferralCode) {
      update((codes) => [code, ...codes]);
    },
    clear() {
      set([]);
    }
  };
}

export const referralProgram = createReferralProgramStore();
export const referralCodes = createReferralCodesStore();
