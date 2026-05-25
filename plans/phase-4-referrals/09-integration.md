# Phase 4: Referrals — Integration

**Status:** COMPLETED

## Backend Integration

### Inbound Dependencies (what referrals uses)
- **Wallets service**: `get_or_create_wallet` to ensure referrer and referee have wallets for reward distribution
- **Ledger service**: `create_entry` with `BucketType::ReferralReward` and `MovementType::In` for both referrer and referee rewards
- **Identity service**: `get_customer` for customer name lookup during vanity code generation; `customers` table queried for account age in fraud checks

### Outbound Dependencies (what uses referrals)
- Storefront APIs can trigger conversion processing
- Admin dashboard uses analytics, code listing, and conversion listing endpoints

### Idempotency Keys
| Operation | Key Format |
|-----------|-----------|
| Referrer reward | `referral_referrer_{code}_{referee_id}_{order_part}_{wallet_id}` |
| Referee reward | `referral_referee_{code}_{referee_id}_{order_part}_{wallet_id}` |

### Fraud Signal → Reward Logic
- If ANY fraud signal is detected: `is_suspicious = true`, no rewards issued
- Conversion record is still created with `is_suspicious = true` and `fraud_signals` populated
- Referral stats: `total_referrals` always incremented; `total_conversions` only incremented if NOT suspicious
- Customer referral limit only counts non-suspicious conversions

## Frontend Integration

### Module: `referrals/`
- **remote.ts**: API calls for programs, codes, conversions, analytics
- **store.ts**: `referralProgram` (single program) and `referralCodes` (array) stores

### UI Components
| Component | Purpose |
|-----------|---------|
| `ReferralProgramForm` | Reward amounts, max-referral toggle, active toggle |
| `CreateCodeForm` | Customer ID, vanity toggle with code input, creator toggle with commission rate |
| `ReferralCodesList` | Table with code, customer, type badges (vanity/creator/auto), stats, status |
| `ReferralAnalyticsCard` | Metric grid: total codes, referrals, conversions, conversion rate, suspicious count (highlighted) |
| `ConversionsList` | Table with referrer, referee, order, fraud signals (pills), date; suspicious rows highlighted |

### Component Library Usage
- `Input`, `Button`, `Toggle` for forms
- `Pill` for fraud signals and status in conversion/code lists
- Custom table styling with badges using `color-mix()` for transparent badge backgrounds
