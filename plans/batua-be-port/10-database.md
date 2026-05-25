# Database

## Objective

Confirm that all 30 migrations (`20260318000001_core_enums.sql` →
`20260326000001_referral_code_creation_trigger.sql`) apply cleanly, in
order, against a fresh PostgreSQL database from the ported repo, with
identical results to the source repo.

## Tasks

1. Copy `migrations/` from source repo to `batua-be/migrations/`.
2. Verify count: `ls migrations/ | wc -l` must return `30`.
3. Verify `diff -rq` against the source returns nothing.
4. Provision an isolated verification database:
   ```bash
   psql -U chirag -d postgres -c "DROP DATABASE IF EXISTS batua_be_verify;"
   psql -U chirag -d postgres -c "CREATE DATABASE batua_be_verify;"
   ```
5. Apply migrations in lexicographic order:
   ```bash
   for f in $(ls migrations/*.sql | sort); do
     echo "Applying $f..."
     psql -U chirag -d batua_be_verify -f "$f" || exit 1
   done
   ```
6. Spot-check key tables exist:
   ```bash
   psql -U chirag -d batua_be_verify -c "\dt" | head -40
   ```
   Expect: `merchants`, `customers`, `wallets`, `ledger_entries`,
   `events`, `rules`, `wallet_policies`, `redemption_requests`,
   `connectors`, `notification_templates`, `loyalty_tiers`,
   `gift_cards`, `referral_codes`, `memberships`, `milestones`,
   `streaks`, `spin_wheels`, `merchant_slugs`, `merchant_points_config`,
   `cod_orders`, `newsletter_signups`.
7. Drop verification DB:
   ```bash
   psql -U chirag -d postgres -c "DROP DATABASE batua_be_verify;"
   ```

## Outputs

- All migrations applied without error.
- All expected tables present.
- Verification DB cleaned up.

## Validation

The migration loop in step 5 exits 0 and step 6 lists the expected
tables. If any migration errors, the port has drifted and must be
re-investigated.
