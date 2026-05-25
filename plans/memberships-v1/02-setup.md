# Phase 2: Setup

## Objective

Create the frontend module structure for memberships.

## Tasks

- [ ] Create `frontend/src/lib/client/modules/memberships/` directory
- [ ] Create barrel `index.ts`
- [ ] Create `types.ts` — mirror backend types
- [ ] Create `remote.ts` — API calls to existing endpoints
- [ ] Create `ui/` directory
- [ ] Create admin route `frontend/src/routes/admin/memberships/+page.svelte`
- [ ] Add "Memberships" to sidebar navigation

## No backend setup needed

The `src/services/earn/` service already has:
- Migration: `20260319000006_memberships.sql`
- Types, storage, handlers, helpers
- Router with all 6 endpoints
