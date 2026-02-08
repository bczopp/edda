//! Logging-Setup (Phase 18.1.1): Structured-Logging (tracing), Default-Log-Level.

use std::io;
use tracing_subscriber::EnvFilter;

/// Initialisiert strukturiertes Logging (tracing).
/// Nutzt `RUST_LOG`; falls nicht gesetzt, wird `default_filter` oder `info` verwendet (LLM-Service-weit).
pub fn init_logging(default_filter: Option<&str>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(default_filter.unwrap_or("info")))?;
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(io::stderr)
        .try_init()?;
    Ok(())
}
