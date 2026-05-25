# About Page — Overview

## Goal
Add a top-level "About" page to the merchant admin: a sidebar entry opening a full
page that introduces Batua and explains every feature of the retention suite via
sub-tabs.

## Scope
In scope:
- New "About" item in the admin sidebar navigation.
- New route `/admin/about` rendering a full page with sub-tabs.
- Sub-tabs: Overview + one per feature (Wallet, Loyalty, Gift Cards, Referrals,
  Campaigns, Memberships).
- Overview sub-tab: product intro + links to the bundled marketing website.
- Feature sub-tabs: per-feature brief with capabilities.
- Remove the superseded "About" tab previously added inside Settings.

Out of scope:
- Changes to the marketing site content.
- Backend / API / store work.

## Success Criteria
- "About" appears in the sidebar and routes to `/admin/about`.
- The page shows 7 sub-tabs; Overview is default and deep-linkable via `?tab=`.
- Overview links open the bundled site at `/website/`.
- Each feature sub-tab renders its brief.
- The Settings page no longer has an About tab.
- `svelte-check --threshold error` passes for changed files.

## Dependencies
- Marketing site already bundled at `frontend/static/website/` (prior iteration).
- `Tabs` / `Button` from `@juspay/svelte-ui-components`.
- Sidebar `NavItem` list in `frontend/src/routes/admin/+layout.svelte`.

## Note
Supersedes `plans/settings-about-tab/` — the About tab is moving out of Settings
into its own top-level page.
