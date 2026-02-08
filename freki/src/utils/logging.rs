//! Structured Logging (Phase 15.1.1): tracing-Setup, Log-Levels, Context-Tracking, Log-Rotation.
//!
//! Log-Levels: trace, debug, info, warn, error (via RUST_LOG, z.B. `RUST_LOG=freki=debug`).
//! Context-Tracking: request_id in Spans (gRPC-Handler); alle Logs innerhalb eines Spans erben die Felder.
//! Log-Rotation: optional via FREKI_LOG_FILE (z.B. `/var/log/freki/freki.log`) mit täglicher Rotation.

use tracing_subscriber::{
    fmt,
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};
use std::path::Path;

/// Initialisiert strukturiertes Logging.
///
/// - **RUST_LOG**: Filter (Default: `info`). Beispiele: `debug`, `freki=debug`, `freki=info,warn`.
/// - **FREKI_LOG_JSON**: `1` = JSON-Format (für zentrale Log-Aggregation).
/// - **FREKI_LOG_FILE**: Pfad für Log-Datei → tägliche Rotation (z.B. `freki.log` → `freki.log.2025-01-31`).
pub fn init_logging() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let json = std::env::var("FREKI_LOG_JSON").map(|v| v == "1" || v.eq_ignore_ascii_case("true")).unwrap_or(false);
    let log_file = std::env::var("FREKI_LOG_FILE").ok();

    let base = tracing_subscriber::registry().with(env_filter);

    if let Some(ref path) = log_file {
        let directory = Path::new(path).parent().unwrap_or(Path::new("."));
        let file_name = Path::new(path).file_name().and_then(|p| p.to_str()).unwrap_or("freki.log");
        let file_appender = tracing_appender::rolling::Builder::new()
            .rotation(tracing_appender::rolling::Rotation::DAILY)
            .filename_prefix(file_name)
            .build(directory)
            .map_err(|e| format!("Failed to create log file appender: {}", e))?;
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
        std::mem::forget(_guard);
        let file_layer = fmt::layer()
            .with_target(true)
            .with_ansi(false)
            .with_writer(non_blocking);
        if json {
            let stdout_layer = fmt::layer()
                .with_target(true)
                .with_thread_ids(false)
                .with_file(false)
                .with_line_number(false)
                .json()
                .with_writer(std::io::stdout);
            base.with(stdout_layer).with(file_layer).init();
        } else {
            let stdout_layer = fmt::layer()
                .with_target(true)
                .with_thread_ids(false)
                .with_file(false)
                .with_line_number(false)
                .with_writer(std::io::stdout);
            base.with(stdout_layer).with(file_layer).init();
        }
    } else if json {
        let layer = fmt::layer()
            .with_target(true)
            .with_thread_ids(false)
            .with_file(false)
            .with_line_number(false)
            .json()
            .with_writer(std::io::stdout);
        base.with(layer).init();
    } else {
        let layer = fmt::layer()
            .with_target(true)
            .with_thread_ids(false)
            .with_file(false)
            .with_line_number(false)
            .with_writer(std::io::stdout);
        base.with(layer).init();
    }

    Ok(())
}
