import { derived, writable } from 'svelte/store';
import type { LedgerEntry, TransactionFilters } from './types';

const DEFAULT_FILTERS: TransactionFilters = {
  bucket_type: null,
  movement_type: null,
  page: 1,
  limit: 25
};

const transactionFilters = writable<TransactionFilters>({ ...DEFAULT_FILTERS });

const allEntries = writable<Array<LedgerEntry>>([]);

const filteredEntries = derived([allEntries, transactionFilters], ([$entries, $filters]) => {
  let result = $entries;

  if ($filters.bucket_type !== null) {
    result = result.filter((entry) => entry.bucket_type === $filters.bucket_type);
  }

  if ($filters.movement_type !== null) {
    result = result.filter((entry) => entry.movement_type === $filters.movement_type);
  }

  return result;
});

function resetFilters(): void {
  transactionFilters.set({ ...DEFAULT_FILTERS });
}

export { transactionFilters, allEntries, filteredEntries, resetFilters, DEFAULT_FILTERS };
