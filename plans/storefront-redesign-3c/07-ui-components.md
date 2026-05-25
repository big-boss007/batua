# Phase 7: UI Components

## Objective

Rework existing components to match Concept 3C design.

---

### 1. MerchantHeader.svelte — Add Avatar

**Changes:**
- Add `customerName` prop (string | null)
- When name exists: show avatar circle (initials, amber gradient) on the right
- When no name: show nothing on right (or keep as-is)
- Keep brand bar + logo + merchant name

---

### 2. ProfileBar.svelte — NEW Component

**Purpose:** Displays customer name, tier, order count, member-since, and lifetime saved.

**Props:**
- `name: string`
- `tierName: string | null`
- `orderCount: number | null` (derive from entry count or omit if not available)
- `lifetimeSaved: number | null`
- `currency: string`

**Layout:**
- Left: name (20px bold), tier pill + "· X orders", member-since
- Right: "Lifetime Saved" label + green amount
- Divider below

---

### 3. BalanceCard.svelte → BalanceHero (rework)

**Changes:**
- Remove gradient card background entirely
- Center-aligned layout
- "SPENDABLE BALANCE" eyebrow (12px, uppercase, muted)
- Amount: 48px, bold, white
- Sub: "₹662 total · ₹171 pending" (pending in amber)
- No bucket bars — those move to StatGrid

---

### 4. StatGrid.svelte — NEW Component

**Purpose:** 2x2 grid of bucket stats.

**Props:**
- `buckets: BucketBalance[]`
- `currency: string`

**Layout:**
- 2-column CSS grid, gap 8px
- Each cell: surface background, 12px radius, padding 14px
- Value: 18px bold, color-coded (green for earned, amber for pending, purple for expiring)
- Label: 10px uppercase muted
- 4th cell: "Expiring Soon" (derived or show ₹0 if not available)

---

### 5. TierCard.svelte → Segmented Progress (rework)

**Changes:**
- Remove card background/shadow
- Tier row: dot + "Silver" label (left), "491 / 3,000" (right)
- 6 discrete segments (flex row, gap 3px, 4px height)
- Segments filled proportionally: `Math.ceil(percentage / (100/6))`
- Hint text: "2,509 points to Silver · unlocks 1.5x cashback"
- Dividers above and below

---

### 6. TransactionList.svelte → Date-Grouped (rework)

**Changes:**
- Accept `runningBalances: Map<string, number>` prop
- Group entries by date using `groupEntriesByDate()`
- Render date label header for each group
- Pass running balance to each TransactionCard

---

### 7. TransactionCard.svelte → Color Bar Style (rework)

**Changes:**
- Replace 36px icon circle with 4px × 32px vertical color bar
- Colors: green (credit), red (debit), purple (transfer), gray (neutral)
- Remove icon text (+, -, ⇄)
- Add `runningBalance: number | null` prop
- Show "bal ₹491.00" below amount (10px, muted)
- Date inline in meta: "Order #1042 · Mar 17"
- Remove border-bottom from last item in each date group

---

## Tasks

- [ ] Rework MerchantHeader — add avatar
- [ ] Create ProfileBar component
- [ ] Rework BalanceCard → hero style
- [ ] Create StatGrid component
- [ ] Rework TierCard → segmented progress
- [ ] Rework TransactionList → date-grouped
- [ ] Rework TransactionCard → color bar + running balance
- [ ] Update UI barrel (ui/index.ts) with new exports
