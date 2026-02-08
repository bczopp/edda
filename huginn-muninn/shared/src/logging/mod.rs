//! Structured logging utilities for Huginn & Muninn

pub mod config;
pub mod setup;

pub use config::LoggingConfig;
pub use setup::setup_logging;
