use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;

use super::helpers::generate_event_idempotency_key;
use super::types::{Event, EventState, IngestEventRequest};

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn store_event(
    pool: &PgPool,
    req: &IngestEventRequest,
) -> Result<(Event, bool), AppError> {
    let idempotency_key = generate_event_idempotency_key(
        req.merchant_id,
        &req.event_source,
        &req.external_event_id,
    );

    let existing: Option<Event> = sqlx::query_as(
        "SELECT id, merchant_id, event_type, event_source, external_event_id,
                payload, state, idempotency_key, created_at, processed_at
         FROM events
         WHERE idempotency_key = $1",
    )
    .bind(&idempotency_key)
    .fetch_optional(pool)
    .await?;

    if let Some(event) = existing {
        return Ok((event, true));
    }

    let event: Event = sqlx::query_as(
        "INSERT INTO events (merchant_id, event_type, event_source, external_event_id,
                             payload, state, idempotency_key)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING id, merchant_id, event_type, event_source, external_event_id,
                   payload, state, idempotency_key, created_at, processed_at",
    )
    .bind(req.merchant_id)
    .bind(&req.event_type)
    .bind(&req.event_source)
    .bind(&req.external_event_id)
    .bind(&req.payload)
    .bind(EventState::Received)
    .bind(&idempotency_key)
    .fetch_one(pool)
    .await?;

    Ok((event, false))
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_event(pool: &PgPool, id: Uuid) -> Result<Event, AppError> {
    let event: Event = sqlx::query_as(
        "SELECT id, merchant_id, event_type, event_source, external_event_id,
                payload, state, idempotency_key, created_at, processed_at
         FROM events
         WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("event {id} not found")))?;

    Ok(event)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn mark_event_state(
    pool: &PgPool,
    id: Uuid,
    state: EventState,
    processed_at: Option<DateTime<Utc>>,
) -> Result<(), AppError> {
    let rows_affected = sqlx::query(
        "UPDATE events SET state = $1, processed_at = $2 WHERE id = $3",
    )
    .bind(state)
    .bind(processed_at)
    .bind(id)
    .execute(pool)
    .await?
    .rows_affected();

    if rows_affected == 0 {
        return Err(AppError::NotFound(format!("event {id} not found")));
    }

    Ok(())
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_pending_events(pool: &PgPool, limit: i32) -> Result<Vec<Event>, AppError> {
    let events: Vec<Event> = sqlx::query_as(
        "SELECT id, merchant_id, event_type, event_source, external_event_id,
                payload, state, idempotency_key, created_at, processed_at
         FROM events
         WHERE state = $1
         ORDER BY created_at ASC
         LIMIT $2",
    )
    .bind(EventState::Received)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(events)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn list_events(
    pool: &PgPool,
    merchant_id: Option<Uuid>,
    event_type: Option<&str>,
    state: Option<EventState>,
    limit: i32,
    offset: i32,
) -> Result<Vec<Event>, AppError> {
    let events: Vec<Event> = sqlx::query_as(
        "SELECT id, merchant_id, event_type, event_source, external_event_id,
                payload, state, idempotency_key, created_at, processed_at
         FROM events
         WHERE ($1::uuid IS NULL OR merchant_id = $1)
           AND ($2::text IS NULL OR event_type = $2)
           AND ($3::event_state IS NULL OR state = $3)
         ORDER BY created_at DESC
         LIMIT $4 OFFSET $5",
    )
    .bind(merchant_id)
    .bind(event_type)
    .bind(state)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(events)
}
