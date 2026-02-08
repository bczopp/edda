//! Prompt processing: formatting, system prompts, context window (Phase 1.1.2, 8.1.1, 8.2.1, 8.2.2, 8.3.1, 8.3.2).

mod formatter;
mod context_formatter;
mod context_integrator;
mod token_counter;
mod context_window_manager;
mod protocol;
pub use formatter::PromptFormatter;
pub use protocol::{XML_PROTOCOL_INSTRUCTIONS, inject_xml_protocol};
pub use context_formatter::{ContextDocument, ContextFormatter};
pub use context_integrator::ContextIntegrator;
pub use token_counter::TokenCounter;
pub use context_window_manager::ContextWindowManager;
