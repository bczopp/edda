//! Logging setup for Huginn & Muninn

use super::LoggingConfig;
use tracing_subscriber::{
    fmt,
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
    Registry,
};
use tracing_appender::{non_blocking, rolling};
use std::path::PathBuf;

/// Setup structured logging
pub fn setup_logging(config: LoggingConfig) -> Result<(), Box<dyn std::error::Error>> {
    // Create environment filter
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            // Default filter with audio-specific levels
            EnvFilter::new(&format!(
                "{}={},huginn::audio={},muninn::audio={},shared::audio={}",
                config.level,
                config.level,
                config.audio_level,
                config.audio_level,
                config.audio_level
            ))
        });
    
    // Setup log file if specified
    if let Some(log_file) = config.log_file {
        let file_appender = if config.rotation {
            rolling::daily(log_file.parent().unwrap_or(std::path::Path::new(".")), "huginn-muninn.log")
        } else {
            rolling::never(log_file.parent().unwrap_or(std::path::Path::new(".")), "huginn-muninn.log")
        };
        
        let (non_blocking_appender, _guard) = non_blocking(file_appender);
        
        if config.json {
            Registry::default()
                .with(filter)
                .with(fmt::Layer::default().json().with_writer(non_blocking_appender))
                .with(fmt::Layer::default().json().with_writer(std::io::stdout))
                .init();
        } else {
            Registry::default()
                .with(filter)
                .with(fmt::Layer::default().with_writer(non_blocking_appender))
                .with(fmt::Layer::default().with_writer(std::io::stdout))
                .init();
        }
        
        // Keep guard alive (would need to return it in production)
        std::mem::forget(_guard);
    } else {
        // Console-only logging
        if config.json {
            tracing_subscriber::fmt()
                .json()
                .with_env_filter(filter)
                .init();
        } else {
            tracing_subscriber::fmt()
                .with_env_filter(filter)
                .init();
        }
    }
    
    Ok(())
}

/// Setup default logging (console only, info level)
pub fn setup_default_logging() -> Result<(), Box<dyn std::error::Error>> {
    setup_logging(LoggingConfig::default())
}
