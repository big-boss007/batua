# UI Component Library Migration — Checklist

**Status:** COMPLETED

## Preparation

- [x] Audit all frontend modules for custom HTML components replaceable by library
- [x] Identify 13 files across 5 modules (customers, gift-cards, referrals, settings, routes)
- [x] Verify `@juspay/svelte-ui-components` exports: Button, Input, Select, Toggle, Pill, Progress
- [x] Verify `.d.ts` types for actual component APIs (especially Select)

## Migration — customers module

- [x] `LoyaltyProgramForm.svelte` — Button, Input, Select
- [x] `TierForm.svelte` — Button, Input (with useTextArea)
- [x] `TierDistributionChart.svelte` — Progress

## Migration — gift-cards module

- [x] `IssueGiftCardForm.svelte` — Button, Input
- [x] `BulkIssueForm.svelte` — Button, Input
- [x] `GiftCardDetail.svelte` — Pill, Progress

## Migration — referrals module

- [x] `CreateCodeForm.svelte` — Button, Input, Toggle
- [x] `ReferralProgramForm.svelte` — Button, Input, Toggle
- [x] `ConversionsList.svelte` — Pill

## Migration — settings module

- [x] `ConnectorForm.svelte` — Button, Input (with useTextArea), Select
- [x] `WalletPolicyForm.svelte` — Button, Input, Toggle
- [x] `NotificationTemplateEditor.svelte` — Button, Input (with useTextArea), Toggle, Pill

## Migration — routes

- [x] `admin/loyalty/+page.svelte` — Button

## Validation

- [x] Zero TypeScript errors (`npm run check`)
- [x] Visual parity confirmed across all 13 files
- [x] Form submissions work correctly
- [x] Interactive behaviors (toggle, select dropdown) functional
- [x] ~400 lines of duplicated CSS removed
- [x] No regressions in light/dark theme
