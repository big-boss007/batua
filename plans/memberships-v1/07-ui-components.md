# Phase 7: UI Components

## Objective

Build admin UI components and storefront integration.

---

### Admin Page: `routes/admin/memberships/+page.svelte`

Single page with two sections/tabs:

#### Section 1: Plans

- List of plans in a table: Name, Type (Monthly/Annual), Price, Multiplier, Status, Actions
- "Create Plan" button → opens modal/form
- Edit/deactivate actions per plan
- **PlanForm.svelte** — form for creating/editing a plan:
  - Name (text)
  - Type (select: Monthly / Annual)
  - Price (number)
  - Earn Rate Multiplier (number, default 1.5)
  - Active toggle

#### Section 2: Subscribers

- List of memberships in a table: Customer (phone/name), Plan, Status, Started, Expires, Days Left, Actions
- Filter by status (Active / Expired / Cancelled / All)
- "Assign Membership" button → opens modal/form
- Cancel action per row
- **AssignForm.svelte** — form for assigning a membership:
  - Customer search (phone number lookup)
  - Plan select (from active plans)
  - Submit → calls `subscribeMembership`

---

### Storefront Integration

Update the storefront `+page.svelte` to:
1. Call `getMembershipStatus(merchant_id, customer_id)` alongside other data fetches
2. Pass membership status to `ProfileBar`

Update `ProfileBar.svelte` to:
- Accept optional `membershipPlan` and `membershipDaysRemaining` props
- If active membership: show badge below tier name — e.g., "Gold Member · 47 days left"

---

### Sidebar Navigation

Add "Memberships" item to the admin sidebar (between "Loyalty" and "Campaigns" or at appropriate position).

## Component list

| Component | Location | Purpose |
|-----------|----------|---------|
| PlanForm.svelte | memberships/ui/ | Create/edit plan modal |
| AssignForm.svelte | memberships/ui/ | Assign customer to plan |
| +page.svelte | routes/admin/memberships/ | Admin memberships page |
| ProfileBar.svelte | storefront/ui/ (update) | Add membership badge |
| +page.svelte | routes/s/[slug]/ (update) | Fetch membership status |

## Tasks

- [ ] Create PlanForm component
- [ ] Create AssignForm component
- [ ] Create admin memberships page with plans + subscribers sections
- [ ] Add sidebar nav item
- [ ] Update storefront ProfileBar with membership badge
- [ ] Update storefront page to fetch membership status
