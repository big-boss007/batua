# Settings "About" Tab — Overview

## Goal
Add an "About" tab to the merchant Settings page (`/admin/settings`) that shows a short
brief on Batua and links to the bundled marketing website.

## Scope
In scope:
- Bundle the existing static marketing site (`site/`) into the SvelteKit app's static
  assets so it is served at `/website/`.
- Add a fourth tab, "About", to `frontend/src/routes/admin/settings/+page.svelte`.
- Render a brief on Batua + a "Visit Website" button that opens `/website/` in a new tab.

Out of scope:
- Changes to the marketing site content itself.
- Any backend, type, store, or API work.
- Linking the website from anywhere other than the About tab.

## Success Criteria
- `/admin/settings?tab=about` shows the About tab with the brief.
- The "Visit Website" button opens the bundled site at `/website/index.html` in a new tab.
- The bundled site renders correctly (styles, script, pricing page, internal links).
- Existing tabs (My Store, Connectors, Notifications) are unaffected.
- `svelte-check --threshold error` passes.

## Dependencies
- Existing static marketing site at `site/` (index.html, pricing.html, styles.css, script.js).
- SvelteKit serves `static/` at the URL root (default `kit.files.assets`).
