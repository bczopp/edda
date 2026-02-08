use serde::{Serialize, Deserialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use std::collections::HashMap;

/// Request metrics for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time_ms: f64,
    pub requests_per_second: f64,
}

/// Provider-specific metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderMetrics {
    pub provider_id: String,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time_ms: f64,
    pub last_used: Option<SystemTime>,
}

/// System metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub uptime_seconds: u64,
    pub total_requests: u64,
    pub active_providers: usize,
    pub cache_hit_rate: f64,
    pub cache_size: usize,
    pub cache_max_size: usize,
}

/// Comprehensive metrics for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringMetrics {
    pub system: SystemMetrics,
    pub requests: RequestMetrics,
    pub providers: Vec<ProviderMetrics>,
    pub timestamp: SystemTime,
}

impl Default for RequestMetrics {
    fn default() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time_ms: 0.0,
            requests_per_second: 0.0,
        }
    }
}

impl Default for ProviderMetrics {
    fn default() -> Self {
        Self {
            provider_id: String::new(),
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time_ms: 0.0,
            last_used: None,
        }
    }
}
