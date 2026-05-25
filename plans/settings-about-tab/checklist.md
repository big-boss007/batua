## Implementation Checklist

### Phase 1: Planning
- [x] Approach decided (bundle site into `frontend/static/website/`)
- [x] Plan approved by user

### Phase 2: Setup
- [x] Create `frontend/static/website/`
- [x] Copy `index.html`, `pricing.html`, `styles.css`, `script.js` from `site/`
- [x] Fix nav logo `href="/"` -> `href="index.html"` in copied index.html
- [x] Verify `/website/*` assets return 200

### Phase 3: Type Definitions
- [x] SKIPPED — no types needed

### Phase 4: State Management
- [x] SKIPPED — tab state is existing local component state

### Phase 5: API Integration
- [x] SKIPPED — no API calls

### Phase 6: Utilities
- [x] SKIPPED — no utilities needed

### Phase 7: UI Components
- [x] Add `{:else if activeTab === 'about'}` branch
- [x] Render about card: eyebrow, heading, brief, Pills, Visit Website button
- [x] Add scoped styles using design tokens

### Phase 8: Integration
- [x] Add `'about'` to `tabIds`
- [x] Add `'About'` to `tabItems`
- [x] Confirm `+page.ts` unchanged

### Verification
- [x] `npx svelte-check --threshold error` passes (settings page clean; 1 pre-existing
      unrelated error in `routes/admin/rules/+page.svelte`)
- [x] `svelte-autofixer` reports no issues in the modified component
- [x] `/admin/settings?tab=about` renders the About tab
- [x] "Visit Website" opens `/website/index.html` in a new tab
- [x] Bundled marketing site renders with full styling at `/website/`
- [x] Existing tabs unaffected
