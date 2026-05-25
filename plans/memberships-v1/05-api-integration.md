# Phase 5: API Integration

## Objective

Create `remote.ts` with API calls to existing backend endpoints.

## Endpoints to integrate

| Function | Method | Path | Backend handler |
|----------|--------|------|-----------------|
| `createPlan` | POST | `/earn/memberships/plans` | `create_membership_plan` |
| `listPlans` | GET | `/earn/memberships/plans/{merchant_id}` | `list_membership_plans` |
| `subscribeMembership` | POST | `/earn/memberships/subscribe` | `subscribe_membership` |
| `renewMembership` | POST | `/earn/memberships/renew` | `renew_membership` |
| `cancelMembership` | POST | `/earn/memberships/cancel/{membership_id}` | `cancel_membership` |
| `getMembershipStatus` | GET | `/earn/memberships/status/{merchant_id}/{customer_id}` | `membership_status` |

## Additional endpoint needed

The existing backend has no "list all subscribers" endpoint. For the admin subscriber list, we need either:
- **Option A**: New endpoint `GET /earn/memberships/subscribers/{merchant_id}` — lists all customer_memberships for a merchant (with pagination)
- **Option B**: Query from client by iterating customers — not viable

**Decision: Need to add a list subscribers endpoint to the backend.**

## Tasks

- [ ] Create `remote.ts` with decoder functions + API calls
- [ ] Add `listSubscribers` endpoint to backend (`src/services/earn/`)
- [ ] Export from barrel `index.ts`
