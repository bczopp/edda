//! Settings validator (Phase 1.3.2) – TDD.

use crate::utils::config::JotunheimSettings;

/// Validates Jotunheim settings.
pub struct SettingsValidator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    PortOutOfRange(u16),
    MaxRetriesZero,
    BackoffZero,
    MaxMemoryZero,
}

impl SettingsValidator {
    pub fn validate(settings: &JotunheimSettings) -> Result<(), ValidationError> {
        if settings.loki.port == 0 || settings.loki.port > 65535 {
            return Err(ValidationError::PortOutOfRange(settings.loki.port));
        }
        if settings.network_resilience_settings.max_retries == 0 {
            return Err(ValidationError::MaxRetriesZero);
        }
        if settings.network_resilience_settings.backoff_ms == 0 {
            return Err(ValidationError::BackoffZero);
        }
        if settings.resource_limits.max_memory_kb == 0 {
            return Err(ValidationError::MaxMemoryZero);
        }
        Ok(())
    }
}
