# Phase 10: Database Migration

## Objective
Add points configuration columns to the merchants table.

## Migration: `migrations/20260322000001_merchant_points_config.sql`

```sql
ALTER TABLE merchants
  ADD COLUMN points_name TEXT NOT NULL DEFAULT 'Points',
  ADD COLUMN points_icon TEXT NOT NULL DEFAULT 'pts',
  ADD COLUMN points_to_currency_rate DOUBLE PRECISION NOT NULL DEFAULT 1.0;
```

## Notes
- `points_name`: displayed in full text contexts — "You earned 400 Stars"
- `points_icon`: displayed inline with numbers — "400 ★" or "400 pts"
- `points_to_currency_rate`: 1 point = X ₹. Default 1.0 means 1 point = ₹1 (backward compatible)
- All existing merchants get defaults, no data backfill needed
- No new tables, no FK changes

## Seed Data Update
Update `scripts/seed.sql` to set varied points configs across merchants:
- Chai & Co: points_name="Stars", points_icon="★", rate=0.25 (4★ = ₹1)
- Some merchants keep defaults (1:1)
- Some merchants use custom names: "Coins", "Gems", "Credits"

## Validation
- `psql -d batua -f migrations/20260322000001_merchant_points_config.sql`
- `SELECT name, points_name, points_icon, points_to_currency_rate FROM merchants;`
