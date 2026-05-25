# Phase 1: Planning

## Objective
Decide how the marketing site is made reachable from the running app and how the
About tab is wired in.

## Approach
The marketing site in `site/` is self-contained static HTML/CSS/JS but is not served
anywhere. SvelteKit serves the `static/` directory at the URL root, so the site is
bundled by copying it to `frontend/static/website/`, making it available at
`/website/index.html`.

The Settings page already renders tabs from two parallel arrays (`tabIds`, `tabItems`)
and a chained `{#if}` on `activeTab`. Adding a tab means extending both arrays and
adding one `{:else if}` branch — no routing or load-function change, since `+page.ts`
reads the `tab` query param as-is and `tabIds.indexOf` resolves it.

## Decisions
- Bundle (copy) the site rather than serve it separately — keeps it part of the app
  deployment, no extra process or port.
- The site's relative links (`styles.css`, `script.js`, `pricing.html`, `index.html`)
  resolve correctly under `/website/`. The single absolute `href="/"` (nav logo) is
  changed to `href="index.html"` so it stays within the bundled site.
- The "Visit Website" button opens `/website/index.html` in a new tab, reusing the
  page's existing `handleOpenLink` helper.

## Outputs
- This plan.

## Validation
- Approach reviewed and approved before implementation.
