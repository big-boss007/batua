# Phase 4: Frontend API Integration

## Objective
Add frontend API calls for direct campaign creation, stacking config, campaign performance, and deactivation.

## `frontend/src/lib/client/modules/rules/remote.ts`

### New functions:

```typescript
async function createCampaignDirect(req: CreateCampaignRequest): Promise<APIResult<Campaign>>
  // POST /campaigns/create

async function getCampaignPerformance(campaignId: string): Promise<APIResult<CampaignPerformance>>
  // GET /campaigns/{campaignId}/performance

async function deactivateCampaign(campaignId: string): Promise<APIResult<Campaign>>
  // POST /campaigns/{campaignId}/deactivate

async function getCampaignConfig(merchantId: string): Promise<APIResult<CampaignStackingConfig>>
  // GET /admin/merchants/{merchantId}/campaign-config

async function updateCampaignConfig(merchantId: string, config: CampaignStackingConfig): Promise<APIResult<CampaignStackingConfig>>
  // PUT /admin/merchants/{merchantId}/campaign-config
```

### Decoders:
- `decodeCampaignPerformance(raw)`
- `decodeCampaignStackingConfig(raw)`

### Barrel exports:
Update `index.ts` to export new functions and types.

## Validation
- `npx svelte-check --threshold error` passes
- API calls work from browser network tab
