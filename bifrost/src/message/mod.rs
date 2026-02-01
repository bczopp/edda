//! Message-Format und -Validierung (Phase 2.1).
//!
//! Enthält [`BifrostMessage`], [`MessageType`], [`MessageHandler`] (parse/serialize)
//! und [`MessageValidator`] / [`ValidationError`].

pub mod handler;
pub mod validator;

pub use handler::*;
pub use validator::{MessageValidator, ValidationError};
