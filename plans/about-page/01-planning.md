# Phase 1: Planning

## Objective
Decide the page structure, routing, and how it integrates with the existing
admin shell.

## Approach
"About" becomes a top-level admin page, not a Settings tab. It is a new SvelteKit
route `/admin/about` that renders inside the existing admin layout (sidebar + top
bar). The page uses the `Tabs` component for sub-tabs, mirroring the Settings
page's tab pattern: a `+page.ts` load returns the `tab` query param, the component
resolves it to an index, and tab changes update the URL via `goto`.

The six feature sub-tabs share an identical layout, so they are data-driven from a
single `features` array; the Overview sub-tab is rendered separately.

## Decisions
- Full route over a Settings tab — the content (7 sub-tabs) needs a full page.
- Sub-tabs deep-linkable via `?tab=<id>`, consistent with Settings.
- Feature panels rendered from a typed `features` array to avoid six near-identical
  markup blocks.
- The Overview feature cards double as navigation — clicking one switches to that
  feature's sub-tab.
- Reuse the website bundle from the prior iteration; remove the old Settings tab.

## Outputs
- Approved design mockup: `docs/about-page-design.html`.

## Validation
- Mockup reviewed and approved by the user before implementation.
