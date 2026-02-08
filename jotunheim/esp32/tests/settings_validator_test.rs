//! Tests for SettingsValidator (Phase 1.3.2 – TDD).

use jotunheim_esp32::settings::SettingsValidator;
use jotunheim_esp32::utils::config::{
    JotunheimSettings, LokiConfig, NetworkConfig, NetworkResilienceSettings, ResourceLimits,
};

fn default_settings() -> JotunheimSettings {
    JotunheimSettings::default()
}

#[test]
fn validator_accepts_default_settings() {
    let s = default_settings();
    assert!(SettingsValidator::validate(&s).is_ok());
}

#[test]
fn validator_rejects_port_zero() {
    let mut s = default_settings();
    s.loki.port = 0;
    let r = SettingsValidator::validate(&s);
    assert!(r.is_err());
}

#[test]
fn validator_rejects_max_retries_zero() {
    let mut s = default_settings();
    s.network_resilience_settings.max_retries = 0;
    let r = SettingsValidator::validate(&s);
    assert!(r.is_err());
}

#[test]
fn validator_rejects_backoff_zero() {
    let mut s = default_settings();
    s.network_resilience_settings.backoff_ms = 0;
    let r = SettingsValidator::validate(&s);
    assert!(r.is_err());
}

#[test]
fn validator_rejects_max_memory_zero() {
    let mut s = default_settings();
    s.resource_limits.max_memory_kb = 0;
    let r = SettingsValidator::validate(&s);
    assert!(r.is_err());
}
