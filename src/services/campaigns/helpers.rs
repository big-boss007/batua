use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::PgPool;

use crate::error::AppError;
use crate::services::rules::storage::create_campaign;
use crate::services::rules::types::{Campaign, CreateCampaignRequest};

use super::storage;
use super::types::{
    CampaignCalendarEntry, CampaignCalendarQuery, CreateFromTemplateRequest, FestiveTemplate,
};

pub fn get_festive_templates() -> Vec<FestiveTemplate> {
    vec![
        FestiveTemplate {
            name: "diwali".to_string(),
            display_name: "Diwali".to_string(),
            description: "Festival of lights — peak shopping season across India".to_string(),
            default_multiplier: 3.0,
            default_duration_days: 10,
            suggested_start: Some("Oct-Nov".to_string()),
            category: "religious".to_string(),
        },
        FestiveTemplate {
            name: "navratri".to_string(),
            display_name: "Navratri".to_string(),
            description: "Nine nights of celebration with high consumer spending".to_string(),
            default_multiplier: 2.0,
            default_duration_days: 9,
            suggested_start: Some("Sep-Oct".to_string()),
            category: "religious".to_string(),
        },
        FestiveTemplate {
            name: "holi".to_string(),
            display_name: "Holi".to_string(),
            description: "Festival of colors — spring shopping boost".to_string(),
            default_multiplier: 2.0,
            default_duration_days: 3,
            suggested_start: Some("Mar".to_string()),
            category: "religious".to_string(),
        },
        FestiveTemplate {
            name: "republic_day".to_string(),
            display_name: "Republic Day".to_string(),
            description: "National holiday with patriotic sale events".to_string(),
            default_multiplier: 1.5,
            default_duration_days: 3,
            suggested_start: Some("Jan 26".to_string()),
            category: "national".to_string(),
        },
        FestiveTemplate {
            name: "independence_day".to_string(),
            display_name: "Independence Day".to_string(),
            description: "National holiday driving seasonal promotions".to_string(),
            default_multiplier: 1.5,
            default_duration_days: 3,
            suggested_start: Some("Aug 15".to_string()),
            category: "national".to_string(),
        },
        FestiveTemplate {
            name: "eid".to_string(),
            display_name: "Eid".to_string(),
            description: "Celebration marking end of Ramadan with gift-giving traditions".to_string(),
            default_multiplier: 2.0,
            default_duration_days: 3,
            suggested_start: None,
            category: "religious".to_string(),
        },
        FestiveTemplate {
            name: "christmas".to_string(),
            display_name: "Christmas".to_string(),
            description: "Year-end holiday shopping and gift-giving season".to_string(),
            default_multiplier: 2.0,
            default_duration_days: 7,
            suggested_start: Some("Dec 25".to_string()),
            category: "international".to_string(),
        },
        FestiveTemplate {
            name: "new_year".to_string(),
            display_name: "New Year".to_string(),
            description: "New year celebrations with fresh-start promotions".to_string(),
            default_multiplier: 2.0,
            default_duration_days: 5,
            suggested_start: Some("Jan 1".to_string()),
            category: "international".to_string(),
        },
        FestiveTemplate {
            name: "raksha_bandhan".to_string(),
            display_name: "Raksha Bandhan".to_string(),
            description: "Sibling bond celebration driving gifting purchases".to_string(),
            default_multiplier: 1.5,
            default_duration_days: 3,
            suggested_start: Some("Aug".to_string()),
            category: "religious".to_string(),
        },
        FestiveTemplate {
            name: "valentines_day".to_string(),
            display_name: "Valentine's Day".to_string(),
            description: "Romance-driven gifting and lifestyle purchases".to_string(),
            default_multiplier: 1.5,
            default_duration_days: 3,
            suggested_start: Some("Feb 14".to_string()),
            category: "international".to_string(),
        },
    ]
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_from_template(
    pool: &PgPool,
    req: &CreateFromTemplateRequest,
) -> Result<Campaign, AppError> {
    let templates = get_festive_templates();
    let template = templates
        .iter()
        .find(|t| t.name == req.template_name)
        .ok_or_else(|| {
            AppError::BadRequest(format!("unknown template: {}", req.template_name))
        })?;

    let multiplier = req.multiplier.unwrap_or(template.default_multiplier);

    let name = req
        .custom_name
        .clone()
        .unwrap_or_else(|| format!("{} Campaign", template.display_name));

    let starts_at = parse_datetime(&req.starts_at)?;
    let ends_at = parse_datetime(&req.ends_at)?;

    if ends_at <= starts_at {
        return Err(AppError::BadRequest(
            "ends_at must be after starts_at".to_string(),
        ));
    }

    let config = serde_json::json!({
        "template": template.name,
        "category": template.category,
        "default_multiplier": template.default_multiplier,
        "default_duration_days": template.default_duration_days,
    });

    let create_req = CreateCampaignRequest {
        merchant_id: req.merchant_id,
        name,
        campaign_type: "festive".to_string(),
        config,
        base_rule_id: Some(req.base_rule_id),
        multiplier: Some(multiplier),
        starts_at,
        ends_at,
    };

    let campaign = create_campaign(pool, &create_req).await?;
    Ok(campaign)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_calendar(
    pool: &PgPool,
    query: &CampaignCalendarQuery,
) -> Result<Vec<CampaignCalendarEntry>, AppError> {
    let from = match &query.from {
        Some(s) => parse_datetime(s)?,
        None => Utc::now(),
    };

    let to = match &query.to {
        Some(s) => parse_datetime(s)?,
        None => from + chrono::Duration::days(90),
    };

    storage::get_campaign_calendar(pool, query.merchant_id, from, to).await
}

fn parse_datetime(s: &str) -> Result<DateTime<Utc>, AppError> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Ok(dt.with_timezone(&Utc));
    }

    if let Ok(ndt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S") {
        return Ok(ndt.and_utc());
    }

    if let Ok(nd) = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        let ndt = nd
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| AppError::BadRequest(format!("invalid date: {s}")))?;
        return Ok(ndt.and_utc());
    }

    Err(AppError::BadRequest(format!(
        "invalid datetime format: {s}, expected RFC3339, YYYY-MM-DDTHH:MM:SS, or YYYY-MM-DD"
    )))
}
