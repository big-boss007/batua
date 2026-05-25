# Phase 5: Remote (Frontend API Calls)

## `analytics/remote.ts`
- `fetchMerchantAnalytics(merchantId)` -- GET `/admin/merchants/{merchantId}/analytics`
- `fetchCodAnalytics(merchantId, from, to)` -- GET `/cod/analytics/{merchantId}`
- `fetchCampaignPerformance(merchantId, from, to)` -- GET `/admin/dashboard`
- `fetchOverviewMetrics(merchantId, from, to)` -- GET `/admin/dashboard`

## `rules/remote.ts`
- `fetchRules(merchantId)` -- GET `/rules`
- `createRule(req)` -- POST `/rules`
- `updateRule(id, config)` -- PUT `/rules/{id}`
- `fetchCampaigns(merchantId)` -- GET `/campaigns/calendar`
- `createCampaignFromTemplate(req)` -- POST `/campaigns/from-template`
- `fetchFestiveTemplates()` -- GET `/campaigns/templates`
- `fetchCampaignCalendar(merchantId, from, to)` -- GET `/campaigns/calendar`
- `fetchRulePerformance(ruleId)` -- GET `/rules/{ruleId}/performance`

## `settings/remote.ts`
- `fetchWalletPolicies(merchantId)` -- GET `/admin/wallet-policies/{merchantId}`
- `updateWalletPolicy(policyId, body)` -- PUT `/wallets/policies/{policyId}`
- `fetchConnectors(merchantId)` -- GET `/notifications/connectors`
- `createConnector(merchantId, body)` -- POST `/notifications/connectors`
- `fetchTemplates(merchantId)` -- GET `/notifications/templates`
- `updateTemplate(templateId, body)` -- PUT `/notifications/templates/{templateId}`
- `updateMerchantProfile(id, data)` -- PUT `/admin/merchants/{id}`
- `fetchNotificationLogs(merchantId, page, limit)` -- GET `/notifications/logs`

## `platform/remote.ts`
- `fetchDashboardStats()` -- GET `/admin/dashboard`
- `fetchMerchants(page, limit)` -- GET `/admin/merchants`
- `fetchMerchant(id)` -- GET `/admin/merchants/{id}`
- `fetchMerchantStats(id)` -- GET `/admin/merchants/{id}/stats`
- `createMerchant(data)` -- POST `/admin/merchants`
- `updateMerchant(id, data)` -- PUT `/admin/merchants/{id}`
- `updateMerchantPlan(id, plan)` -- PUT `/admin/merchants/{id}/plan`
- `fetchGeoPolicies()` -- GET `/admin/geo-policies`
- `createGeoPolicy(data)` -- POST `/admin/geo-policies`
- `fetchSystemHealth()` -- GET `/admin/system/health`
- `fetchRecentEvents(limit)` -- GET `/admin/events/recent`

## `admin/remote.ts`
- `fetchDashboardStats(merchantId)` -- GET `/admin/dashboard`
- `fetchMerchant(merchantId)` -- GET `/admin/merchants/{merchantId}`
- `fetchMerchants(page, limit)` -- GET `/admin/merchants`
- `fetchMerchantDashboard(merchantId)` -- GET `/admin/merchants/{merchantId}/dashboard`
