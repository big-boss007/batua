# UI Component Library Migration — Overview

**Status:** COMPLETED

## Goal

Maximize usage of `@juspay/svelte-ui-components` across the Batua admin frontend by replacing custom HTML+CSS implementations with library components. Eliminate duplicated styling code and establish consistent component patterns.

## Scope

### Files Modified (13)

| Module | File | Components Replaced |
|--------|------|-------------------|
| customers | `LoyaltyProgramForm.svelte` | Button, Input, Select |
| customers | `TierForm.svelte` | Button, Input (textarea) |
| customers | `TierDistributionChart.svelte` | Progress |
| gift-cards | `IssueGiftCardForm.svelte` | Button, Input |
| gift-cards | `BulkIssueForm.svelte` | Button, Input |
| gift-cards | `GiftCardDetail.svelte` | Pill, Progress |
| referrals | `CreateCodeForm.svelte` | Button, Input, Toggle |
| referrals | `ReferralProgramForm.svelte` | Button, Input, Toggle |
| referrals | `ConversionsList.svelte` | Pill |
| settings | `ConnectorForm.svelte` | Button, Input (textarea), Select |
| settings | `WalletPolicyForm.svelte` | Button, Input, Toggle |
| settings | `NotificationTemplateEditor.svelte` | Button, Input (textarea), Toggle, Pill |
| routes | `admin/loyalty/+page.svelte` | Button |

### Replacements Summary

| Custom Pattern | Library Component | Instances |
|---------------|------------------|-----------|
| `<button>` | `Button` | 13 |
| `<input>` | `Input` | 18 |
| `<textarea>` | `Input useTextArea` | 3 |
| `<select>` | `Select` | 2 |
| `<input type="checkbox">` | `Toggle` | 6 |
| `.badge` spans | `Pill` | 8 |
| Custom progress divs | `Progress` | 3 |

### Impact

- ~400 lines of duplicated CSS eliminated
- 7 distinct component patterns standardized

### Not In Scope

Domain-specific components kept as custom implementations:
- MetricCard, TierCard, ReferralCard (data-display cards)
- Sidebar, CustomerSearch
- Complex forms (CampaignForm, RuleForm)

## Success Criteria

- [x] All 13 files migrated to library components
- [x] Zero TypeScript errors after migration
- [x] Visual parity with previous custom implementations
- [x] Global theme classes applied via `app.css` custom properties
- [x] No regressions in form submission or interactive behavior

## Dependencies

- `@juspay/svelte-ui-components` package installed
- Global CSS custom properties defined in `app.css`

## Key Learnings

1. **Select API mismatch:** The `Select` component's actual TypeScript API (`items: SelectItem[]`, `value: string[]`, `onchange`) differs from MCP docs (`allItems`, `selectedItem`, `onselect`). Always verify against `.d.ts` files.
2. **Global theming:** Classes like `btn-primary`, `btn-secondary`, `pill-success` were already defined in `app.css`, so library components picked up project styling automatically.
3. **Textarea pattern:** `Input` with `useTextArea` prop replaces `<textarea>` without needing a separate component.
