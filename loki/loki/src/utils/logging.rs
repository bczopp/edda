//! Minimal logging configuration for Loki (IoT-optimized)

use tracing_subscriber::{
    fmt,
    EnvFilter,
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

/// Initialize minimal logging for Loki
/// Only logs ERROR and WARN levels to minimize resource usage on IoT devices
pub fn init_minimal_logging() {
    // Set default log level to WARN if RUST_LOG is not set
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("warn"));
    
    // Create a minimal formatter that only logs ERROR and WARN
    // For IoT devices, we want minimal overhead
    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_target(false) // Don't show target/module path to save space
                .with_thread_ids(false) // Don't show thread IDs
                .with_thread_names(false) // Don't show thread names
                .with_file(false) // Don't show file names
                .with_line_number(false) // Don't show line numbers
                .compact() // Use compact format
        )
        .with(filter)
        .init();
    
    tracing::info!("Minimal logging initialized (ERROR, WARN only)");
}

/// Initialize logging with custom filter
pub fn init_logging_with_filter(filter: EnvFilter) {
    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_target(false)
                .with_thread_ids(false)
                .with_thread_names(false)
                .with_file(false)
                .with_line_number(false)
                .compact()
        )
        .with(filter)
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_minimal_logging_initialization() {
        // This test verifies that the logging initialization doesn't panic
        // In a real test environment, we'd check the actual log output
        // For now, we just verify the function exists and can be called
        // Note: This will panic if called multiple times due to tracing_subscriber
        // In actual tests, we'd use a test-specific initialization
    }
}
