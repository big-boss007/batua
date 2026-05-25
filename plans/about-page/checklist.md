## Implementation Checklist

### Phase 1: Planning
- [x] Approach decided (top-level route + sub-tabs)
- [x] Design mockup approved by user (`docs/about-page-design.html`)

### Phase 2: Setup
- [x] Marketing site bundled at `frontend/static/website/` (prior iteration, reused)

### Phase 3: Type Definitions
- [x] SKIPPED — component-local `Feature` type only

### Phase 4: State Management
- [x] SKIPPED — local tab state + URL query param

### Phase 5: API Integration
- [x] SKIPPED — static content

### Phase 6: Utilities
- [x] SKIPPED — component-local `openWebsite` helper only

### Phase 7: UI Components
- [x] Create `/admin/about/+page.svelte`
- [x] Tabs bar + page header
- [x] Overview: hero + Visit Website / See Pricing buttons
- [x] Overview: "Explore the suite" feature-card grid (clickable)
- [x] Feature panels data-driven from `features` array
- [x] Scoped styles using design tokens

### Phase 8: Integration
- [x] Add "About" NavItem to sidebar `navItems`
- [x] Create `/admin/about/+page.ts` (tab query param)
- [x] Remove the About tab from `settings/+page.svelte`

### Verification
- [x] `npx svelte-check --threshold error` passes (about/settings clean; 1 pre-existing
      unrelated error in `routes/admin/rules/+page.svelte`)
- [x] `svelte-autofixer` clean on the new component
- [x] Sidebar "About" routes to `/admin/about`
- [x] All 7 sub-tabs render; feature cards navigate; buttons open `/website/`
- [x] `?tab=<id>` deep-links survive reload (verified `?tab=campaigns`)
- [x] Settings no longer shows an About tab (only My Store / Connectors / Notifications)
