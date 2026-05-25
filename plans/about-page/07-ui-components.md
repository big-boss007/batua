# Phase 7: UI Components

## Objective
Build the About page UI.

## Tasks
- Create `frontend/src/routes/admin/about/+page.svelte`.
- Page header ("About" + subtitle) and a `Tabs` bar with 7 items.
- Overview sub-tab:
  - Hero block — eyebrow, headline, intro paragraph, and two `Button`s
    ("Visit Website", "See Pricing") opening `/website/index.html` and
    `/website/pricing.html` in a new tab.
  - "Explore the suite" — a grid of 6 feature cards; clicking a card switches to
    that feature's sub-tab.
- Feature sub-tabs (data-driven from the `features` array):
  - Header — accent icon tile, name, tagline.
  - Lead paragraph.
  - Capabilities checklist (two columns).
  - Context callout.
- Scoped styles using app design tokens; light/dark handled via tokens.

## Component reuse
- `Tabs` and `Button` from `@juspay/svelte-ui-components`. Feature cards are native
  `<button>` elements for accessibility. No custom versions of library components.

## Outputs
- `+page.svelte` with Overview + 6 feature panels and styles.

## Validation
- All 7 sub-tabs render correctly; feature cards navigate; buttons open the site.
