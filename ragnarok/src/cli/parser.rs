use clap::{Parser, Subcommand};
use thiserror::Error;

#[derive(Parser)]
#[command(name = "ragnarok")]
#[command(about = "Terminal Platform for Edda")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Chat with the assistant (via Odin)
    Chat {
        /// Message to send (empty = interactive mode)
        message: Option<String>,
    },
    /// Execute an action (via Thor when configured)
    Action {
        /// Action to execute
        action: String,
    },
    /// Send a prompt directly to Geri (when configured)
    Prompt {
        /// Prompt text
        prompt: String,
    },
    /// List models from Geri (when configured)
    Models,
    /// Retrieve RAG context from Freki (when configured)
    Retrieve {
        /// Query (note: Freki API uses embeddings; query may be used for future text-based API)
        query: String,
    },
    /// Transcribe audio file via Huginn STT (when configured)
    Transcribe {
        /// Path to audio file (wav, mp3, opus, etc.)
        file: std::path::PathBuf,
    },
    /// Generate speech from text via Muninn TTS (when configured)
    Speak {
        /// Text to speak
        text: String,
    },
    /// Show status
    Status,
    /// Show or manage settings
    Settings,
    /// Start TUI (Status-Dashboard + Chat-Interface)
    Tui,
}

#[derive(Debug, Error)]
pub enum CliError {
    #[error("CLI parsing error: {0}")]
    ParsingError(String),
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
