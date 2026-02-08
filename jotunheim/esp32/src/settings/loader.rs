//! Settings loader (Phase 1.3.3) – load from JSON/TOML file.

use crate::utils::config::JotunheimSettings;
use std::path::Path;

/// Loads settings from file (JSON or TOML).
pub struct SettingsLoader;

impl SettingsLoader {
    /// Load settings from path; infers format from extension (.json / .toml).
    pub fn load(path: &Path) -> Result<JotunheimSettings, Box<dyn std::error::Error + Send + Sync>> {
        let s = std::fs::read_to_string(path)?;
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let settings: JotunheimSettings = match ext.to_lowercase().as_str() {
            "toml" => toml::from_str(&s)?,
            _ => serde_json::from_str(&s)?,
        };
        Ok(settings)
    }
}
