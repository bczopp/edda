//! Logging setup (Phase 14.1.1). Structured logging with tracing, levels, context (spans).

use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initializes structured logging (tracing) with env filter and levels (trace, debug, info, warn, error).
/// Context tracking: use `tracing::info_span!` / `span.enter()` in code for request/connection context.
/// Log rotation: not built-in; use external tools (e.g. logrotate) or tracing-appender for file output.
pub fn init_logging() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_target(true))
        .init();
}
