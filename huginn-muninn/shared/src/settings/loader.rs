//! Settings loader (JSON from path) for Huginn & Muninn

use std::path::Path;

use crate::settings::error::SettingsError;
use crate::settings::schema::{HuginnSettings, MuninnSettings};
use crate::settings::validator::SettingsValidator;

/// Load Huginn settings from a JSON file and validate.
pub fn load_huginn_settings(path: impl AsRef<Path>) -> Result<HuginnSettings, SettingsError> {
    let path = path.as_ref();
    let contents = std::fs::read_to_string(path).map_err(|e| SettingsError::Load {
        path: path.display().to_string(),
        cause: e.to_string(),
    })?;
    let s: HuginnSettings = serde_json::from_str(&contents)?;
    SettingsValidator::validate_huginn(&s)?;
    Ok(s)
}

/// Load Muninn settings from a JSON file and validate.
pub fn load_muninn_settings(path: impl AsRef<Path>) -> Result<MuninnSettings, SettingsError> {
    let path = path.as_ref();
    let contents = std::fs::read_to_string(path).map_err(|e| SettingsError::Load {
        path: path.display().to_string(),
        cause: e.to_string(),
    })?;
    let s: MuninnSettings = serde_json::from_str(&contents)?;
    SettingsValidator::validate_muninn(&s)?;
    Ok(s)
}
