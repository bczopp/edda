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
    /// Chat with the assistant
    Chat {
        /// Message to send
        message: String,
    },
    /// Execute an action
    Action {
        /// Action to execute
        action: String,
    },
    /// Show status
    Status,
    /// Show or manage settings
    Settings,
}

#[derive(Debug, Error)]
pub enum CliError {
    #[error("CLI parsing error: {0}")]
    ParsingError(String),
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
