//! Remote control – receive commands from Midgard/Asgard/controller (Phase 5).

pub mod command_handler;
pub mod error;
pub mod executor;

pub use command_handler::RemoteCommandHandler;
pub use error::RemoteCommandError;
pub use executor::ScriptExecutor;

/// Legacy placeholder alias.
pub struct RemoteControl;

impl RemoteControl {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RemoteControl {
    fn default() -> Self {
        Self::new()
    }
}
