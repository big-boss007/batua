# Customer Edit Profile

## Goal
Add ability for merchants to edit customer name and email from the Customer Detail modal (Option B: dedicated edit section).

## Scope
- Backend: `PUT /admin/merchants/{merchant_id}/customers/{customer_id}` endpoint
- Frontend: Edit form section in CustomerDetail.svelte with name, email fields (phone read-only)
- Update customer name/email in the `customers` table

## Out of Scope
- Phone number editing (immutable identifier)
- Customer creation from admin
- Batch editing

## Success Criteria
- Merchant can click "Edit customer details" link in Customer Detail modal
- Form expands with name, email inputs and disabled phone field
- Save updates the customer record and refreshes the modal
- Cancel collapses the form without changes

## Dependencies
- Existing admin service (`src/services/admin/`)
- Existing CustomerDetail.svelte component
- Existing customers module remote.ts
