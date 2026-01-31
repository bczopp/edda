//! Connection Error Handler (Phase 10.2.1): categorize errors, suggest Retry vs Fallback vs NotifyUser.

use std::error::Error;

/// Category of a connection error for deciding recovery action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionErrorCategory {
    /// Transient (e.g. temporary network glitch) – retry.
    Transient,
    /// Permanent (e.g. target not found) – fallback route.
    Permanent,
    /// Authentication/authorization failure – notify user.
    Auth,
    /// Network-level (refused, reset, unreachable) – retry.
    Network,
    /// Timeout – retry.
    Timeout,
    /// Critical/unexpected – notify user.
    Critical,
}

/// Suggested action after categorizing an error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionErrorAction {
    Retry,
    Fallback,
    NotifyUser,
    LogOnly,
}

/// Categorizes connection errors and suggests Retry vs Fallback vs NotifyUser.
#[derive(Debug, Default)]
pub struct ConnectionErrorHandler;

impl ConnectionErrorHandler {
    /// Categorize an error (e.g. from handle_connection or routing).
    pub fn categorize(&self, err: &dyn Error) -> ConnectionErrorCategory {
        let msg = err.to_string().to_lowercase();
        if msg.contains("timeout") || msg.contains("timed out") {
            return ConnectionErrorCategory::Timeout;
        }
        if msg.contains("not connected") || msg.contains("not found") {
            return ConnectionErrorCategory::Permanent;
        }
        if msg.contains("auth") || msg.contains("permission") || msg.contains("denied") {
            return ConnectionErrorCategory::Auth;
        }
        if msg.contains("refused") || msg.contains("connection reset") || msg.contains("unreachable") {
            return ConnectionErrorCategory::Network;
        }
        if msg.contains("interrupted") {
            return ConnectionErrorCategory::Transient;
        }
        ConnectionErrorCategory::Transient
    }

    /// Suggest action from category (Retry vs Fallback vs NotifyUser).
    pub fn suggest_action(&self, category: ConnectionErrorCategory) -> ConnectionErrorAction {
        match category {
            ConnectionErrorCategory::Transient | ConnectionErrorCategory::Network | ConnectionErrorCategory::Timeout => {
                ConnectionErrorAction::Retry
            }
            ConnectionErrorCategory::Permanent => ConnectionErrorAction::Fallback,
            ConnectionErrorCategory::Auth | ConnectionErrorCategory::Critical => {
                ConnectionErrorAction::NotifyUser
            }
        }
    }
}
