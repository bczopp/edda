//! Conditional permission evaluation: time, context, IP conditions.

use std::collections::HashMap;
use chrono::Timelike;

/// Conditions that must all hold for a permission to be granted (in addition to base permission).
#[derive(Debug, Clone, Default)]
pub struct PermissionConditions {
    /// (start_hour_utc, end_hour_utc) inclusive; evaluated only if set.
    pub time_window: Option<(u8, u8)>,
    /// Required key-value pairs in context; all must match.
    pub required_context: Option<HashMap<String, String>>,
    /// Allowed client IPs; if set, client_ip must be in list.
    pub allowed_ips: Option<Vec<String>>,
}

/// Context for evaluating conditions (current time, context map, client IP).
#[derive(Debug, Clone, Default)]
pub struct EvaluationContext {
    /// Current hour in UTC (0–23); if None, time condition is skipped when no time_window.
    pub current_hour_utc: Option<u8>,
    /// Key-value context (e.g. env, role).
    pub context: HashMap<String, String>,
    /// Client IP address for IP condition.
    pub client_ip: Option<String>,
}

impl EvaluationContext {
    /// Build context with current hour from system time.
    pub fn with_current_time(mut self) -> Self {
        self.current_hour_utc = Some(chrono::Utc::now().hour() as u8);
        self
    }
}

/// Evaluates conditional permissions: base permission plus time, context, IP conditions.
pub struct ConditionalPermissionEvaluator;

impl ConditionalPermissionEvaluator {
    /// Returns true only if base_allowed is true and all configured conditions pass.
    pub fn evaluate(
        &self,
        base_allowed: bool,
        conditions: &PermissionConditions,
        ctx: &EvaluationContext,
    ) -> bool {
        if !base_allowed {
            return false;
        }
        if let Some((start, end)) = conditions.time_window {
            let hour = ctx.current_hour_utc.unwrap_or(0);
            if hour < start || hour > end {
                return false;
            }
        }
        if let Some(ref required) = conditions.required_context {
            for (k, v) in required {
                if ctx.context.get(k) != Some(v) {
                    return false;
                }
            }
        }
        if let Some(ref allowed) = conditions.allowed_ips {
            match &ctx.client_ip {
                Some(ip) if allowed.contains(ip) => {}
                Some(_) => return false,
                None => return false,
            }
        }
        true
    }
}
