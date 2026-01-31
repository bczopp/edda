// Common test utilities and helpers

pub fn setup_logging() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();
}
