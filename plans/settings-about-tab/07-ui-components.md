# Phase 7: UI Components

## Objective
Build the About tab content.

## Tasks
- In `frontend/src/routes/admin/settings/+page.svelte`, add an
  `{:else if activeTab === 'about'}` branch in the tab-content block.
- Render an "about card" containing:
  - An eyebrow label ("Breeze Retention Suite") and a heading ("About Batua").
  - A short brief paragraph on Batua.
  - Product tags rendered with the existing `Pill` component (Wallet, Loyalty,
    Gift Cards, Referrals, Campaigns, Memberships).
  - A "Visit Website →" `Button` (`btn-primary`) that calls
    `handleOpenLink('/website/index.html')`.
- Add scoped styles for the card using existing design tokens, consistent with the
  page's `.info-card` / `.section-label` styling.

## Component reuse
- `Button` and `Pill` are already imported and used in this file — reused as-is, no
  custom UI elements created.

## Outputs
- About tab markup + styles in `+page.svelte`.

## Validation
- The About tab renders the brief, pills, and button with correct styling in light
  and dark themes.
