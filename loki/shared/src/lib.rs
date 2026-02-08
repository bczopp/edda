//! Shared code for Loki services

pub mod error;
pub mod models;

pub use error::{LokiError, Result};
pub use models::{ScriptDefinition, ScriptLanguage, ResourceLimits};
