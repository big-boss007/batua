# Phase 8: Integration

## Objective
Wire the page into the admin shell and remove the superseded Settings tab.

## Tasks
- Add an "About" `NavItem` (`/admin/about`) to `navItems` in
  `frontend/src/routes/admin/+layout.svelte`, after "Settings".
- Create `frontend/src/routes/admin/about/+page.ts` — load returns the `tab`
  query param (default `overview`).
- Remove the About tab from `frontend/src/routes/admin/settings/+page.svelte`:
  the `'about'` entries in `tabIds` / `tabItems`, the `{:else if activeTab ===
  'about'}` block, and the `.about-*` styles.

## Outputs
- Updated sidebar, new route, reverted Settings page.

## Validation
- "About" shows in the sidebar and is active on `/admin/about`.
- `?tab=<id>` deep-links and survives reload.
- Settings shows only My Store / Connectors / Notifications.
- `npx svelte-check --threshold error` passes; `svelte-autofixer` clean.
