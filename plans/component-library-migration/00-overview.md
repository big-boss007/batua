# Component Library Migration Plan

## Goal
Replace all 48 custom UI element implementations with their `@juspay/svelte-ui-components` equivalents across 22 files.

## Motivation
CLAUDE.md mandates: "Before writing ANY UI element, call list_components. If the library has it, use it. Do not build custom versions."

## Approach
5 parallel workstreams, each handling non-overlapping files:

### Workstream 1: Toast + Layout + Sidebars + Small Admin Pages
- Add Toast renderer to root layout (P0 - currently all toasts silently discarded)
- Fix admin layout merchant badge → Pill
- Fix Sidebar/PlatformSidebar toggle buttons → Button
- Fix campaigns, notifications, settings page buttons → Button

### Workstream 2: Memberships Page + AssignForm
- Replace custom modal → Modal
- Replace custom pagination → Pagination
- Replace custom progress bar → Progress
- Replace custom pills/badges → Pill
- Replace filter input → Input
- Replace filter selects → Select
- Replace custom spinner → Loader
- Replace custom shimmer → Shimmer (in AssignForm)
- Replace native select → Select (in AssignForm)

### Workstream 3: Rules Module (6 files)
- RuleForm: inputs → Input, selects → Select
- CampaignForm: inputs → Input, selects → Select
- CampaignDetailModal: modal-overlay → Modal, buttons → Button
- StackingConfigModal: modal-overlay → Modal, buttons → Button/Choicebox, radios
- FestiveTemplateGrid: template card buttons
- EarnFormulaBanner: info banner → Banner

### Workstream 4: Gift Cards + Referrals + Storefront
- Gift cards page modal → Modal
- GiftCardConfirmation modal → Modal
- IssueGiftCardForm date input → Input
- ReferralCodesList badges → Pill, table
- ConversionsList table
- Referrals page badges → Pill, radios → Radio
- ProfileBar avatar → Avatar, button → Button
- TierCard progress → Progress
- GiftCardStatus buttons → Button
- CampaignBanner → Banner (if applicable)
- Gift cards check page button → Button

### Workstream 5: Customer + Remaining Admin Pages
- Customers page modal → Modal
- Transactions page modal → Modal
- Rules page modal → Modal, pill
- Setup page tier badges → Pill
- CustomerSearch spinner → Loader
- CustomerDetail progress → Progress
- TierWizard info banners → Banner
