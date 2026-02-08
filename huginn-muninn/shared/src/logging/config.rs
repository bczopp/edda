//! Logging configuration for Huginn & Muninn

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Default log level (trace, debug, info, warn, error)
    pub level: String,
    /// Audio-specific log level
    pub audio_level: String,
    /// Enable JSON output
    pub json: bool,
    /// Log file path (optional)
    pub log_file: Option<PathBuf>,
    /// Enable log rotation
    pub rotation: bool,
    /// Max log file size in MB
    pub max_file_size_mb: u64,
    /// Max number of log files to keep
    pub max_files: usize,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            audio_level: "debug".to_string(),
            json: false,
            log_file: None,
            rotation: true,
            max_file_size_mb: 100,
            max_files: 5,
        }
    }
}

impl LoggingConfig {
    /// Create a new logging config
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set log level
    pub fn with_level(mut self, level: String) -> Self {
        self.level = level;
        self
    }
    
    /// Set audio log level
    pub fn with_audio_level(mut self, level: String) -> Self {
        self.audio_level = level;
        self
    }
    
    /// Enable JSON output
    pub fn with_json(mut self, json: bool) -> Self {
        self.json = json;
        self
    }
    
    /// Set log file path
    pub fn with_log_file(mut self, path: PathBuf) -> Self {
        self.log_file = Some(path);
        self
    }
    
    /// Enable/disable log rotation
    pub fn with_rotation(mut self, rotation: bool) -> Self {
        self.rotation = rotation;
        self
    }
}
