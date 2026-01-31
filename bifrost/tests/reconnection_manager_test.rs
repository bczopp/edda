//! Tests for Phase 7.2.1: ReconnectionManager (immediate first attempt, exponential backoff, max 60s, jitter, continuous).

use bifrost::websocket::reconnection::{ReconnectionConfig, ReconnectionManager};
use std::time::Duration;

fn default_config() -> ReconnectionConfig {
    ReconnectionConfig {
        base_delay: Duration::from_secs(1),
        max_delay: Duration::from_secs(60),
        jitter_ratio: 0.0,
    }
}

#[test]
fn first_delay_is_zero_immediate_reconnect() {
    let config = default_config();
    let mgr = ReconnectionManager::new(config);
    assert_eq!(mgr.next_delay(), Duration::ZERO);
}

#[test]
fn after_one_attempt_delay_is_base() {
    let config = default_config();
    let mut mgr = ReconnectionManager::new(config);
    mgr.record_attempt();
    assert_eq!(mgr.next_delay(), Duration::from_secs(1));
}

#[test]
fn exponential_backoff_doubles_each_attempt() {
    let config = default_config();
    let mut mgr = ReconnectionManager::new(config);
    mgr.record_attempt();
    assert_eq!(mgr.next_delay(), Duration::from_secs(1));
    mgr.record_attempt();
    assert_eq!(mgr.next_delay(), Duration::from_secs(2));
    mgr.record_attempt();
    assert_eq!(mgr.next_delay(), Duration::from_secs(4));
    mgr.record_attempt();
    assert_eq!(mgr.next_delay(), Duration::from_secs(8));
}

#[test]
fn delay_capped_at_max_60_seconds() {
    let config = ReconnectionConfig {
        base_delay: Duration::from_secs(2),
        max_delay: Duration::from_secs(60),
        jitter_ratio: 0.0,
    };
    let mut mgr = ReconnectionManager::new(config);
    for _ in 0..10 {
        mgr.record_attempt();
    }
    let delay = mgr.next_delay();
    assert!(delay <= Duration::from_secs(60), "delay {} should be <= 60s", delay.as_secs());
}

#[test]
fn reset_returns_to_immediate_first_delay() {
    let config = default_config();
    let mut mgr = ReconnectionManager::new(config);
    mgr.record_attempt();
    mgr.record_attempt();
    mgr.reset();
    assert_eq!(mgr.next_delay(), Duration::ZERO);
}

#[test]
fn continuous_attempts_next_delay_always_returns_value() {
    let config = default_config();
    let mut mgr = ReconnectionManager::new(config);
    for _ in 0..20 {
        mgr.record_attempt();
    }
    let delay = mgr.next_delay();
    assert_eq!(delay, Duration::from_secs(60), "capped at max after many attempts");
}

#[test]
fn default_config_has_60s_max() {
    let config = ReconnectionConfig::default();
    assert_eq!(config.max_delay, Duration::from_secs(60));
}

#[test]
fn jitter_zero_produces_deterministic_delay() {
    let config = ReconnectionConfig {
        base_delay: Duration::from_secs(1),
        max_delay: Duration::from_secs(60),
        jitter_ratio: 0.0,
    };
    let mut mgr = ReconnectionManager::new(config);
    mgr.record_attempt();
    let d1 = mgr.next_delay();
    let d2 = mgr.next_delay();
    assert_eq!(d1, d2);
    assert_eq!(d1, Duration::from_secs(1));
}
