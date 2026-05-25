# Phase 7: UI Components — Part B (Wizard)

## Objective
Build a 3-step wizard for loyalty tier setup that works in two modes:
1. **Fresh setup** — when no program exists (program === null)
2. **Reconfigure** — when program exists, triggered by "Reconfigure" button

## Component: `TierWizard.svelte`

### Props
```typescript
{
  mode: 'fresh' | 'reconfigure';
  existingProgram: LoyaltyProgram | null;
  existingTiers: Array<LoyaltyTier>;
  merchantId: string;
  onComplete: () => void;  // called after save, parent reloads data
  onCancel: () => void;    // back to existing view (reconfigure only)
}
```

### Internal State
- `step`: 1 | 2 | 3
- `programData`: { name, evaluation_criteria, evaluation_period_days }
- `wizardTiers`: Array<WizardTier> where WizardTier = LoyaltyTier + `_status: 'existing' | 'modified' | 'new' | 'removed'` + `_original` snapshot

### Step 1: Program
- Pre-fill from existingProgram in reconfigure mode
- Fields: name, evaluation_criteria (select), evaluation_period_days (select)
- Reconfigure shows amber banner
- Cancel button (reconfigure) or empty space (fresh)

### Step 2: Tiers
- Load existing tiers into wizardTiers (reconfigure) or start empty (fresh)
- Preset quick-add: Bronze/Silver/Gold/Platinum with defaults
- Presets already added/existing are disabled
- Custom tier inline form: name, rank, threshold, multiplier + Add button
- Tier list with edit/remove per row
- In reconfigure mode, track changes: modified (amber), new (green), removed (strikethrough)
- Minimum 2 tiers to proceed

### Step 3: Review
- Fresh: show program summary + tier stack + green info box + "Activate Program"
- Reconfigure: show changes summary (diff list) + final tier structure with change badges + amber note + "Save Changes"

### Save Logic (on confirm)
Fresh mode:
1. createProgram(merchantId, programData)
2. For each tier: createTier(programId, tierData)
3. Call onComplete()

Reconfigure mode:
1. updateProgram if program fields changed (need to check if endpoint exists)
2. For new tiers: createTier(programId, tierData)
3. For modified tiers: updateTier(tierId, changes)
4. For removed tiers: deleteTier(tierId)
5. Call onComplete()

## Page Changes (`+page.svelte`)
- Add `showWizard` state and `wizardMode` state
- If program === null → auto-show wizard in 'fresh' mode
- If program !== null → show existing view with "Reconfigure" button
- "Reconfigure" sets showWizard=true, wizardMode='reconfigure'
- onComplete reloads data and hides wizard
- onCancel hides wizard

## Validation
- Wizard flow works end-to-end for fresh setup
- Wizard flow works for reconfigure (edit tier, add tier, remove tier)
- Cancel from reconfigure returns to existing view without changes
- Page refresh after setup shows existing view
