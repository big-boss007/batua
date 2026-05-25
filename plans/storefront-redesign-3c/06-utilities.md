# Phase 6: Utilities

## Objective

Add helper functions for the new UI calculations.

## New utils (in `utils.ts` or inline in page):

### `groupEntriesByDate(entries: TransactionEntry[]): DateGroup[]`
- Groups entries by calendar date from `created_at`
- Returns label: "Today", "Yesterday", or formatted date ("Mar 17")
- Entries within each group maintain their original order

### `computeRunningBalances(entries: TransactionEntry[], currentBalance: number): Map<string, number>`
- Takes entries (newest-first) and current spendable balance
- Walks backward, computing balance after each transaction
- Returns map of entry ID → balance-after

### `getInitials(name: string): string`
- "Priya Sharma" → "PS"
- "Priya" → "P"
- Fallback to "?" if null/empty

### `formatDateLabel(date: Date): string`
- Today → "Today"
- Yesterday → "Yesterday"
- This year → "Mar 17"
- Older → "Mar 17, 2025"

## Tasks

- [ ] Implement `groupEntriesByDate`
- [ ] Implement `computeRunningBalances`
- [ ] Implement `getInitials`
- [ ] Implement `formatDateLabel`
