//! RollbackHandler (Phase 7.3.1, TDD).

use std::sync::Mutex;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RollbackError {
    #[error("No previous version to rollback to")]
    NoPreviousVersion,
}

/// Restores previous version on failure or manual request.
pub struct RollbackHandler {
    current: Mutex<String>,
    previous: Mutex<Option<String>>,
}

impl RollbackHandler {
    pub fn new(current_version: String) -> Self {
        Self {
            current: Mutex::new(current_version),
            previous: Mutex::new(None),
        }
    }

    pub fn with_previous_version(mut self, prev: String) -> Self {
        *self.previous.lock().unwrap() = Some(prev);
        self
    }

    pub fn current_version(&self) -> String {
        self.current.lock().unwrap().clone()
    }

    pub fn can_rollback(&self) -> bool {
        self.previous.lock().unwrap().is_some()
    }

    pub fn rollback(&self) -> Result<(), RollbackError> {
        let mut prev = self.previous.lock().unwrap();
        let Some(p) = prev.take() else {
            return Err(RollbackError::NoPreviousVersion);
        };
        let mut cur = self.current.lock().unwrap();
        let old = std::mem::replace(&mut *cur, p);
        *prev = Some(old);
        Ok(())
    }
}
