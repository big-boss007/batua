use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;

use super::types::{
    BucketBalance, BucketType, GetEntriesQuery, LedgerEntry, MovementType, NewLedgerEntry,
    WalletBalance,
};

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_entry(
    pool: &PgPool,
    new_entry: NewLedgerEntry,
    idempotency_key: String,
) -> Result<LedgerEntry, AppError> {
    let existing: Option<LedgerEntry> = sqlx::query_as(
        "SELECT * FROM ledger_entries WHERE idempotency_key = $1",
    )
    .bind(&idempotency_key)
    .fetch_optional(pool)
    .await?;

    if let Some(entry) = existing {
        return Ok(entry);
    }

    let entry: LedgerEntry = sqlx::query_as(
        r#"
        INSERT INTO ledger_entries (
            wallet_id, bucket_type, movement_type,
            earning_unit, currency_equivalent, conversion_rate,
            idempotency_key, event_id, rule_snapshot_id, campaign_snapshot_id,
            actor_type, actor_id, payment_reference, transfer_id,
            constraints, expires_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
        RETURNING *
        "#,
    )
    .bind(new_entry.wallet_id)
    .bind(&new_entry.bucket_type)
    .bind(&new_entry.movement_type)
    .bind(new_entry.earning_unit)
    .bind(new_entry.currency_equivalent)
    .bind(new_entry.conversion_rate)
    .bind(&idempotency_key)
    .bind(new_entry.event_id)
    .bind(new_entry.rule_snapshot_id)
    .bind(new_entry.campaign_snapshot_id)
    .bind(&new_entry.actor_type)
    .bind(&new_entry.actor_id)
    .bind(&new_entry.payment_reference)
    .bind(new_entry.transfer_id)
    .bind(&new_entry.constraints)
    .bind(new_entry.expires_at)
    .fetch_one(pool)
    .await?;

    Ok(entry)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_across_movement(
    pool: &PgPool,
    out_entry: NewLedgerEntry,
    out_idempotency_key: String,
    in_entry: NewLedgerEntry,
    in_idempotency_key: String,
) -> Result<(LedgerEntry, LedgerEntry), AppError> {
    let transfer_id = Uuid::new_v4();
    let mut tx = pool.begin().await?;

    let out_result: LedgerEntry = sqlx::query_as(
        r#"
        INSERT INTO ledger_entries (
            wallet_id, bucket_type, movement_type,
            earning_unit, currency_equivalent, conversion_rate,
            idempotency_key, event_id, rule_snapshot_id, campaign_snapshot_id,
            actor_type, actor_id, payment_reference, transfer_id,
            constraints, expires_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
        ON CONFLICT (idempotency_key) DO UPDATE SET idempotency_key = EXCLUDED.idempotency_key
        RETURNING *
        "#,
    )
    .bind(out_entry.wallet_id)
    .bind(&out_entry.bucket_type)
    .bind(&MovementType::Out)
    .bind(out_entry.earning_unit)
    .bind(out_entry.currency_equivalent)
    .bind(out_entry.conversion_rate)
    .bind(&out_idempotency_key)
    .bind(out_entry.event_id)
    .bind(out_entry.rule_snapshot_id)
    .bind(out_entry.campaign_snapshot_id)
    .bind(&out_entry.actor_type)
    .bind(&out_entry.actor_id)
    .bind(&out_entry.payment_reference)
    .bind(transfer_id)
    .bind(&out_entry.constraints)
    .bind(out_entry.expires_at)
    .fetch_one(&mut *tx)
    .await?;

    let in_result: LedgerEntry = sqlx::query_as(
        r#"
        INSERT INTO ledger_entries (
            wallet_id, bucket_type, movement_type,
            earning_unit, currency_equivalent, conversion_rate,
            idempotency_key, event_id, rule_snapshot_id, campaign_snapshot_id,
            actor_type, actor_id, payment_reference, transfer_id,
            constraints, expires_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
        ON CONFLICT (idempotency_key) DO UPDATE SET idempotency_key = EXCLUDED.idempotency_key
        RETURNING *
        "#,
    )
    .bind(in_entry.wallet_id)
    .bind(&in_entry.bucket_type)
    .bind(&MovementType::In)
    .bind(in_entry.earning_unit)
    .bind(in_entry.currency_equivalent)
    .bind(in_entry.conversion_rate)
    .bind(&in_idempotency_key)
    .bind(in_entry.event_id)
    .bind(in_entry.rule_snapshot_id)
    .bind(in_entry.campaign_snapshot_id)
    .bind(&in_entry.actor_type)
    .bind(&in_entry.actor_id)
    .bind(&in_entry.payment_reference)
    .bind(transfer_id)
    .bind(&in_entry.constraints)
    .bind(in_entry.expires_at)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok((out_result, in_result))
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_entries(
    pool: &PgPool,
    wallet_id: Uuid,
    query: GetEntriesQuery,
) -> Result<Vec<LedgerEntry>, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(50).clamp(1, 100);
    let offset = (page - 1) * limit;

    let entries: Vec<LedgerEntry> = match (&query.bucket_type, &query.movement_type) {
        (Some(bt), Some(mt)) => {
            sqlx::query_as(
                r#"
                SELECT * FROM ledger_entries
                WHERE wallet_id = $1 AND bucket_type = $2 AND movement_type = $3
                ORDER BY created_at DESC
                LIMIT $4 OFFSET $5
                "#,
            )
            .bind(wallet_id)
            .bind(bt)
            .bind(mt)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(pool)
            .await?
        }
        (Some(bt), None) => {
            sqlx::query_as(
                r#"
                SELECT * FROM ledger_entries
                WHERE wallet_id = $1 AND bucket_type = $2
                ORDER BY created_at DESC
                LIMIT $3 OFFSET $4
                "#,
            )
            .bind(wallet_id)
            .bind(bt)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(pool)
            .await?
        }
        (None, Some(mt)) => {
            sqlx::query_as(
                r#"
                SELECT * FROM ledger_entries
                WHERE wallet_id = $1 AND movement_type = $2
                ORDER BY created_at DESC
                LIMIT $3 OFFSET $4
                "#,
            )
            .bind(wallet_id)
            .bind(mt)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(pool)
            .await?
        }
        (None, None) => {
            sqlx::query_as(
                r#"
                SELECT * FROM ledger_entries
                WHERE wallet_id = $1
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#,
            )
            .bind(wallet_id)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(pool)
            .await?
        }
    };

    Ok(entries)
}

#[derive(Debug, sqlx::FromRow)]
struct BucketAggregateRow {
    bucket_type: BucketType,
    movement_type: MovementType,
    total_units: Option<f64>,
    entry_count: Option<i64>,
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_balance(
    pool: &PgPool,
    wallet_id: Uuid,
) -> Result<WalletBalance, AppError> {
    let rows: Vec<BucketAggregateRow> = sqlx::query_as(
        r#"
        SELECT
            bucket_type,
            movement_type,
            COALESCE(SUM(earning_unit), 0)::float8 AS total_units,
            COUNT(*)::int8 AS entry_count
        FROM ledger_entries
        WHERE wallet_id = $1
            AND state = 'active'
            AND (expires_at IS NULL OR expires_at > now())
        GROUP BY bucket_type, movement_type
        ORDER BY bucket_type, movement_type
        "#,
    )
    .bind(wallet_id)
    .fetch_all(pool)
    .await?;

    build_wallet_balance(wallet_id, &rows)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_balance_at(
    pool: &PgPool,
    wallet_id: Uuid,
    at: DateTime<Utc>,
) -> Result<WalletBalance, AppError> {
    let rows: Vec<BucketAggregateRow> = sqlx::query_as(
        r#"
        SELECT
            bucket_type,
            movement_type,
            COALESCE(SUM(earning_unit), 0)::float8 AS total_units,
            COUNT(*)::int8 AS entry_count
        FROM ledger_entries
        WHERE wallet_id = $1
            AND state = 'active'
            AND created_at <= $2
            AND (expires_at IS NULL OR expires_at > $2)
        GROUP BY bucket_type, movement_type
        ORDER BY bucket_type, movement_type
        "#,
    )
    .bind(wallet_id)
    .bind(at)
    .fetch_all(pool)
    .await?;

    build_wallet_balance(wallet_id, &rows)
}

fn build_wallet_balance(
    wallet_id: Uuid,
    rows: &[BucketAggregateRow],
) -> Result<WalletBalance, AppError> {
    let mut bucket_map: std::collections::HashMap<String, BucketBalance> =
        std::collections::HashMap::new();

    for row in rows {
        let key = format!("{:?}", row.bucket_type);
        let bucket = bucket_map.entry(key).or_insert_with(|| BucketBalance {
            bucket_type: row.bucket_type.clone(),
            displayed: 0.0,
            spendable: 0.0,
            count: 0,
        });

        let total = row.total_units.unwrap_or(0.0);
        let count = row.entry_count.unwrap_or(0);

        match row.movement_type {
            MovementType::In => {
                bucket.displayed += total;
                bucket.spendable += total;
                bucket.count += count;
            }
            MovementType::Held => {
                bucket.displayed += total;
                bucket.count += count;
            }
            MovementType::Out => {
                bucket.displayed -= total;
                bucket.spendable -= total;
            }
            MovementType::Across => {}
        }
    }

    let buckets: Vec<BucketBalance> = bucket_map
        .into_values()
        .map(|mut b| {
            b.spendable = b.spendable.max(0.0);
            b.displayed = b.displayed.max(0.0);
            b
        })
        .collect();

    let displayed_balance: f64 = buckets.iter().map(|b| b.displayed).sum();
    let spendable_balance: f64 = buckets.iter().map(|b| b.spendable).sum();

    Ok(WalletBalance {
        wallet_id,
        displayed_balance,
        spendable_balance,
        buckets,
    })
}
