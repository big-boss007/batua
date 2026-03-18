use chrono::Utc;
use sqlx::PgPool;

use crate::error::AppError;

use super::storage;
use super::types::{
    Campaign, Condition, EvaluationContext, EvaluationResult, RewardAction, RewardRuleConfig,
};

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn evaluate_rules(
    pool: &PgPool,
    context: &EvaluationContext,
) -> Result<Vec<EvaluationResult>, AppError> {
    let rules = storage::get_active_rules(pool, context.merchant_id, "reward").await?;
    let now = Utc::now();
    let active_campaigns = storage::get_active_campaigns(pool, context.merchant_id, now).await?;

    let mut results = Vec::new();

    for rule in &rules {
        let rule_config: RewardRuleConfig = serde_json::from_value(rule.config.clone())
            .map_err(|e| AppError::Internal(format!("invalid rule config for {}: {e}", rule.id)))?;

        if rule_config.event_type != context.event_type {
            continue;
        }

        if !check_conditions(&rule_config.conditions, context) {
            continue;
        }

        let snapshot = storage::create_rule_snapshot(pool, rule).await?;

        let (earning_unit, currency_equivalent, conversion_rate) =
            calculate_reward(&rule_config.action, context);

        let mut result = EvaluationResult {
            matched: true,
            rule_snapshot_id: Some(snapshot.id),
            campaign_snapshot_id: None,
            earning_unit,
            currency_equivalent,
            conversion_rate,
            bucket_type: rule_config.action.bucket_type.clone(),
            expiry_days: rule_config.action.expiry_days,
            constraints: serde_json::json!({}),
        };

        let matching_campaign = active_campaigns
            .iter()
            .find(|c| c.base_rule_id == Some(rule.id));

        if let Some(campaign) = matching_campaign {
            let campaign_snapshot = storage::create_campaign_snapshot(pool, campaign).await?;
            result.campaign_snapshot_id = Some(campaign_snapshot.id);
            result = apply_campaign_multiplier(result, campaign);
        }

        results.push(result);
    }

    Ok(results)
}

#[tracing::instrument]
pub fn check_conditions(conditions: &[Condition], context: &EvaluationContext) -> bool {
    conditions.iter().all(|c| check_condition(c, context))
}

#[tracing::instrument]
pub fn check_condition(condition: &Condition, context: &EvaluationContext) -> bool {
    let field_value = extract_field_value(&condition.field, context);

    match condition.operator.as_str() {
        "eq" => values_equal(&field_value, &condition.value),
        "neq" => !values_equal(&field_value, &condition.value),
        "gt" => compare_numeric(&field_value, &condition.value, |a, b| a > b),
        "gte" => compare_numeric(&field_value, &condition.value, |a, b| a >= b),
        "lt" => compare_numeric(&field_value, &condition.value, |a, b| a < b),
        "lte" => compare_numeric(&field_value, &condition.value, |a, b| a <= b),
        "in" => value_in_list(&field_value, &condition.value),
        "not_in" => !value_in_list(&field_value, &condition.value),
        _ => false,
    }
}

#[tracing::instrument]
pub fn calculate_reward(
    action: &RewardAction,
    context: &EvaluationContext,
) -> (f64, f64, f64) {
    let conversion_rate = action.conversion_rate.unwrap_or(1.0);

    match action.calculation.as_str() {
        "percentage" => {
            let order_amount = context.order_amount.unwrap_or(0.0);
            let mut earning_unit = order_amount * action.value / 100.0;

            if let Some(max) = action.max_amount {
                if earning_unit > max {
                    earning_unit = max;
                }
            }

            let currency_equivalent = earning_unit * conversion_rate;
            (earning_unit, currency_equivalent, conversion_rate)
        }
        "fixed" => {
            let earning_unit = action.value;
            let currency_equivalent = earning_unit * conversion_rate;
            (earning_unit, currency_equivalent, conversion_rate)
        }
        _ => (0.0, 0.0, conversion_rate),
    }
}

#[tracing::instrument]
pub fn apply_campaign_multiplier(
    mut result: EvaluationResult,
    campaign: &Campaign,
) -> EvaluationResult {
    let multiplier = campaign.multiplier.unwrap_or(1.0);
    result.earning_unit *= multiplier;
    result.currency_equivalent *= multiplier;
    result
}

fn extract_field_value(field: &str, context: &EvaluationContext) -> serde_json::Value {
    match field {
        "payment_method" => context
            .payment_method
            .as_ref()
            .map(|v| serde_json::Value::String(v.clone()))
            .unwrap_or(serde_json::Value::Null),
        "order_amount" => context
            .order_amount
            .map(|v| serde_json::json!(v))
            .unwrap_or(serde_json::Value::Null),
        "is_cod" => serde_json::Value::Bool(context.is_cod),
        "is_first_order" => serde_json::Value::Bool(context.is_first_order),
        "collection" | "collections" => serde_json::json!(context.collections),
        "customer_tag" | "customer_tags" => serde_json::json!(context.customer_tags),
        "event_type" => serde_json::Value::String(context.event_type.clone()),
        _ => context
            .event_payload
            .get(field)
            .cloned()
            .unwrap_or(serde_json::Value::Null),
    }
}

fn values_equal(a: &serde_json::Value, b: &serde_json::Value) -> bool {
    match (a, b) {
        (serde_json::Value::String(a_str), serde_json::Value::String(b_str)) => a_str == b_str,
        (serde_json::Value::Number(a_num), serde_json::Value::Number(b_num)) => {
            a_num.as_f64() == b_num.as_f64()
        }
        (serde_json::Value::Bool(a_bool), serde_json::Value::Bool(b_bool)) => a_bool == b_bool,
        _ => a == b,
    }
}

fn compare_numeric(
    a: &serde_json::Value,
    b: &serde_json::Value,
    cmp: fn(f64, f64) -> bool,
) -> bool {
    let a_num = a.as_f64();
    let b_num = b.as_f64();
    match (a_num, b_num) {
        (Some(a_val), Some(b_val)) => cmp(a_val, b_val),
        _ => false,
    }
}

fn value_in_list(field_value: &serde_json::Value, list_value: &serde_json::Value) -> bool {
    let Some(list) = list_value.as_array() else {
        return false;
    };

    match field_value {
        serde_json::Value::Array(field_arr) => {
            field_arr.iter().any(|item| list.contains(item))
        }
        other => list.contains(other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use uuid::Uuid;

    fn make_context() -> EvaluationContext {
        EvaluationContext {
            merchant_id: Uuid::new_v4(),
            event_type: "order.completed".to_string(),
            event_payload: json!({}),
            order_amount: Some(1000.0),
            payment_method: Some("upi".to_string()),
            is_cod: false,
            collections: vec!["electronics".to_string()],
            customer_tags: vec!["vip".to_string()],
            is_first_order: false,
        }
    }

    #[test]
    fn condition_eq_matches() {
        let ctx = make_context();
        let cond = Condition {
            field: "payment_method".to_string(),
            operator: "eq".to_string(),
            value: json!("upi"),
        };
        assert!(check_condition(&cond, &ctx));
    }

    #[test]
    fn condition_eq_no_match() {
        let ctx = make_context();
        let cond = Condition {
            field: "payment_method".to_string(),
            operator: "eq".to_string(),
            value: json!("card"),
        };
        assert!(!check_condition(&cond, &ctx));
    }

    #[test]
    fn condition_neq() {
        let ctx = make_context();
        let cond = Condition {
            field: "payment_method".to_string(),
            operator: "neq".to_string(),
            value: json!("card"),
        };
        assert!(check_condition(&cond, &ctx));
    }

    #[test]
    fn condition_gt() {
        let ctx = make_context();
        let cond = Condition {
            field: "order_amount".to_string(),
            operator: "gt".to_string(),
            value: json!(500.0),
        };
        assert!(check_condition(&cond, &ctx));
    }

    #[test]
    fn condition_gte_equal() {
        let ctx = make_context();
        let cond = Condition {
            field: "order_amount".to_string(),
            operator: "gte".to_string(),
            value: json!(1000.0),
        };
        assert!(check_condition(&cond, &ctx));
    }

    #[test]
    fn condition_lt() {
        let ctx = make_context();
        let cond = Condition {
            field: "order_amount".to_string(),
            operator: "lt".to_string(),
            value: json!(500.0),
        };
        assert!(!check_condition(&cond, &ctx));
    }

    #[test]
    fn condition_lte() {
        let ctx = make_context();
        let cond = Condition {
            field: "order_amount".to_string(),
            operator: "lte".to_string(),
            value: json!(1000.0),
        };
        assert!(check_condition(&cond, &ctx));
    }

    #[test]
    fn condition_in_scalar() {
        let ctx = make_context();
        let cond = Condition {
            field: "payment_method".to_string(),
            operator: "in".to_string(),
            value: json!(["upi", "card", "netbanking"]),
        };
        assert!(check_condition(&cond, &ctx));
    }

    #[test]
    fn condition_in_array_overlap() {
        let ctx = make_context();
        let cond = Condition {
            field: "collections".to_string(),
            operator: "in".to_string(),
            value: json!(["electronics", "fashion"]),
        };
        assert!(check_condition(&cond, &ctx));
    }

    #[test]
    fn condition_not_in() {
        let ctx = make_context();
        let cond = Condition {
            field: "payment_method".to_string(),
            operator: "not_in".to_string(),
            value: json!(["card", "netbanking"]),
        };
        assert!(check_condition(&cond, &ctx));
    }

    #[test]
    fn condition_bool_eq() {
        let ctx = make_context();
        let cond = Condition {
            field: "is_cod".to_string(),
            operator: "eq".to_string(),
            value: json!(false),
        };
        assert!(check_condition(&cond, &ctx));
    }

    #[test]
    fn condition_unknown_operator_returns_false() {
        let ctx = make_context();
        let cond = Condition {
            field: "order_amount".to_string(),
            operator: "regex".to_string(),
            value: json!(".*"),
        };
        assert!(!check_condition(&cond, &ctx));
    }

    #[test]
    fn condition_event_payload_fallback() {
        let ctx = EvaluationContext {
            event_payload: json!({"custom_field": "hello"}),
            ..make_context()
        };
        let cond = Condition {
            field: "custom_field".to_string(),
            operator: "eq".to_string(),
            value: json!("hello"),
        };
        assert!(check_condition(&cond, &ctx));
    }

    #[test]
    fn all_conditions_must_match() {
        let ctx = make_context();
        let conditions = vec![
            Condition {
                field: "payment_method".to_string(),
                operator: "eq".to_string(),
                value: json!("upi"),
            },
            Condition {
                field: "order_amount".to_string(),
                operator: "gt".to_string(),
                value: json!(500.0),
            },
        ];
        assert!(check_conditions(&conditions, &ctx));
    }

    #[test]
    fn one_failing_condition_rejects() {
        let ctx = make_context();
        let conditions = vec![
            Condition {
                field: "payment_method".to_string(),
                operator: "eq".to_string(),
                value: json!("upi"),
            },
            Condition {
                field: "order_amount".to_string(),
                operator: "gt".to_string(),
                value: json!(2000.0),
            },
        ];
        assert!(!check_conditions(&conditions, &ctx));
    }

    #[test]
    fn calculate_percentage_reward() {
        let ctx = make_context();
        let action = RewardAction {
            bucket_type: "earned_credit".to_string(),
            calculation: "percentage".to_string(),
            value: 5.0,
            max_amount: None,
            conversion_rate: Some(1.0),
            expiry_days: Some(90),
        };
        let (earning, currency, rate) = calculate_reward(&action, &ctx);
        assert!((earning - 50.0).abs() < f64::EPSILON);
        assert!((currency - 50.0).abs() < f64::EPSILON);
        assert!((rate - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn calculate_percentage_with_max_cap() {
        let ctx = make_context();
        let action = RewardAction {
            bucket_type: "earned_credit".to_string(),
            calculation: "percentage".to_string(),
            value: 10.0,
            max_amount: Some(50.0),
            conversion_rate: Some(1.0),
            expiry_days: None,
        };
        let (earning, _, _) = calculate_reward(&action, &ctx);
        assert!((earning - 50.0).abs() < f64::EPSILON);
    }

    #[test]
    fn calculate_fixed_reward() {
        let ctx = make_context();
        let action = RewardAction {
            bucket_type: "earned_credit".to_string(),
            calculation: "fixed".to_string(),
            value: 25.0,
            max_amount: None,
            conversion_rate: Some(2.0),
            expiry_days: None,
        };
        let (earning, currency, rate) = calculate_reward(&action, &ctx);
        assert!((earning - 25.0).abs() < f64::EPSILON);
        assert!((currency - 50.0).abs() < f64::EPSILON);
        assert!((rate - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn calculate_unknown_type_returns_zero() {
        let ctx = make_context();
        let action = RewardAction {
            bucket_type: "earned_credit".to_string(),
            calculation: "tiered".to_string(),
            value: 10.0,
            max_amount: None,
            conversion_rate: Some(1.0),
            expiry_days: None,
        };
        let (earning, currency, _) = calculate_reward(&action, &ctx);
        assert!((earning - 0.0).abs() < f64::EPSILON);
        assert!((currency - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn campaign_multiplier_applied() {
        let result = EvaluationResult {
            matched: true,
            rule_snapshot_id: Some(Uuid::new_v4()),
            campaign_snapshot_id: None,
            earning_unit: 50.0,
            currency_equivalent: 50.0,
            conversion_rate: 1.0,
            bucket_type: "earned_credit".to_string(),
            expiry_days: Some(90),
            constraints: json!({}),
        };
        let campaign = Campaign {
            id: Uuid::new_v4(),
            merchant_id: Uuid::new_v4(),
            name: "2x Diwali".to_string(),
            campaign_type: "multiplier".to_string(),
            config: json!({}),
            base_rule_id: None,
            multiplier: Some(2.0),
            starts_at: Utc::now(),
            ends_at: Utc::now(),
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let boosted = apply_campaign_multiplier(result, &campaign);
        assert!((boosted.earning_unit - 100.0).abs() < f64::EPSILON);
        assert!((boosted.currency_equivalent - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn campaign_multiplier_none_defaults_to_one() {
        let result = EvaluationResult {
            matched: true,
            rule_snapshot_id: Some(Uuid::new_v4()),
            campaign_snapshot_id: None,
            earning_unit: 50.0,
            currency_equivalent: 50.0,
            conversion_rate: 1.0,
            bucket_type: "earned_credit".to_string(),
            expiry_days: None,
            constraints: json!({}),
        };
        let campaign = Campaign {
            id: Uuid::new_v4(),
            merchant_id: Uuid::new_v4(),
            name: "No multiplier".to_string(),
            campaign_type: "bonus".to_string(),
            config: json!({}),
            base_rule_id: None,
            multiplier: None,
            starts_at: Utc::now(),
            ends_at: Utc::now(),
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let same = apply_campaign_multiplier(result, &campaign);
        assert!((same.earning_unit - 50.0).abs() < f64::EPSILON);
    }
}
