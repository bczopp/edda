//! Tests for ConditionalPermissionEvaluator (Phase 7.3.1): time, context, IP conditions.

use heimdall::authz::{ConditionalPermissionEvaluator, EvaluationContext, PermissionConditions};
use std::collections::HashMap;

#[test]
fn test_evaluate_denies_when_base_denied() {
    let eval = ConditionalPermissionEvaluator;
    let conditions = PermissionConditions::default();
    let ctx = EvaluationContext::default();
    assert!(!eval.evaluate(false, &conditions, &ctx));
}

#[test]
fn test_evaluate_allows_when_base_allowed_and_no_conditions() {
    let eval = ConditionalPermissionEvaluator;
    let conditions = PermissionConditions::default();
    let ctx = EvaluationContext::default();
    assert!(eval.evaluate(true, &conditions, &ctx));
}

#[test]
fn test_time_condition_denies_outside_window() {
    let eval = ConditionalPermissionEvaluator;
    let conditions = PermissionConditions {
        time_window: Some((9, 17)),
        ..Default::default()
    };
    let mut ctx = EvaluationContext::default();
    ctx.current_hour_utc = Some(3);
    assert!(!eval.evaluate(true, &conditions, &ctx));
}

#[test]
fn test_time_condition_allows_inside_window() {
    let eval = ConditionalPermissionEvaluator;
    let conditions = PermissionConditions {
        time_window: Some((9, 17)),
        ..Default::default()
    };
    let mut ctx = EvaluationContext::default();
    ctx.current_hour_utc = Some(12);
    assert!(eval.evaluate(true, &conditions, &ctx));
}

#[test]
fn test_context_condition_denies_when_missing_key() {
    let eval = ConditionalPermissionEvaluator;
    let mut required = HashMap::new();
    required.insert("env".to_string(), "prod".to_string());
    let conditions = PermissionConditions {
        required_context: Some(required),
        ..Default::default()
    };
    let ctx = EvaluationContext::default();
    assert!(!eval.evaluate(true, &conditions, &ctx));
}

#[test]
fn test_context_condition_allows_when_context_matches() {
    let eval = ConditionalPermissionEvaluator;
    let mut required = HashMap::new();
    required.insert("env".to_string(), "prod".to_string());
    let conditions = PermissionConditions {
        required_context: Some(required),
        ..Default::default()
    };
    let mut ctx = EvaluationContext::default();
    ctx.context.insert("env".to_string(), "prod".to_string());
    assert!(eval.evaluate(true, &conditions, &ctx));
}

#[test]
fn test_ip_condition_denies_when_ip_not_allowed() {
    let eval = ConditionalPermissionEvaluator;
    let conditions = PermissionConditions {
        allowed_ips: Some(vec!["192.168.1.1".to_string()]),
        ..Default::default()
    };
    let mut ctx = EvaluationContext::default();
    ctx.client_ip = Some("10.0.0.1".to_string());
    assert!(!eval.evaluate(true, &conditions, &ctx));
}

#[test]
fn test_ip_condition_allows_when_ip_in_list() {
    let eval = ConditionalPermissionEvaluator;
    let conditions = PermissionConditions {
        allowed_ips: Some(vec!["192.168.1.1".to_string(), "10.0.0.1".to_string()]),
        ..Default::default()
    };
    let mut ctx = EvaluationContext::default();
    ctx.client_ip = Some("10.0.0.1".to_string());
    assert!(eval.evaluate(true, &conditions, &ctx));
}

#[test]
fn test_combined_conditions_all_must_pass() {
    let eval = ConditionalPermissionEvaluator;
    let mut required = HashMap::new();
    required.insert("role".to_string(), "admin".to_string());
    let conditions = PermissionConditions {
        time_window: Some((0, 23)),
        required_context: Some(required),
        allowed_ips: Some(vec!["127.0.0.1".to_string()]),
    };
    let mut ctx = EvaluationContext::default();
    ctx.current_hour_utc = Some(12);
    ctx.context.insert("role".to_string(), "admin".to_string());
    ctx.client_ip = Some("127.0.0.1".to_string());
    assert!(eval.evaluate(true, &conditions, &ctx));
}
