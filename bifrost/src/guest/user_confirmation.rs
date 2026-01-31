//! User Confirmation Manager (Phase 12.2.2). Notify user, receive Allow/Deny, 2-3 confirmations with 5s interval.

use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::RwLock;
use std::time::{Duration, Instant};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ConfirmationError {
    #[error("request not found")]
    RequestNotFound,
    #[error("confirmation too soon; wait min interval")]
    TooSoon,
}

/// Request for user confirmation (data transfer from guest device).
#[derive(Debug, Clone)]
pub struct UserConfirmationRequest {
    pub request_id: String,
    pub guest_device_id: String,
    pub target_user_id: String,
    pub mesh_id: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmationChoice {
    Allow,
    Deny,
}

#[derive(Debug, PartialEq)]
pub enum ConfirmationOutcome {
    Pending(u32),
    Allowed,
    Denied,
}

struct PendingState {
    #[allow(dead_code)]
    request: UserConfirmationRequest,
    confirmations: Vec<(Instant, ConfirmationChoice)>,
}

/// Notifies user via frontend (channel); receives 2-3 confirmations with min 5s interval; security warning is frontend responsibility.
pub struct UserConfirmationManager {
    pending: RwLock<HashMap<String, PendingState>>,
    required_confirmations: u32,
    min_interval: Duration,
}

impl UserConfirmationManager {
    pub fn new(required_confirmations: u32, min_interval: Duration) -> Self {
        Self {
            pending: RwLock::new(HashMap::new()),
            required_confirmations: required_confirmations.max(1),
            min_interval,
        }
    }

    /// Starts a confirmation flow: stores request and notifies via channel. Returns request_id for add_confirmation.
    pub fn start_request(
        &self,
        request: UserConfirmationRequest,
        notify_tx: mpsc::Sender<UserConfirmationRequest>,
    ) -> Result<String, mpsc::SendError<UserConfirmationRequest>> {
        let id = request.request_id.clone();
        notify_tx.send(request.clone())?;
        self.pending.write().unwrap().insert(
            id.clone(),
            PendingState {
                request,
                confirmations: Vec::new(),
            },
        );
        Ok(id)
    }

    /// Records one user confirmation. Returns Pending(remaining), Allowed, or Denied.
    pub fn add_confirmation(
        &self,
        request_id: &str,
        choice: ConfirmationChoice,
    ) -> Result<ConfirmationOutcome, ConfirmationError> {
        if choice == ConfirmationChoice::Deny {
            self.pending.write().unwrap().remove(request_id);
            return Ok(ConfirmationOutcome::Denied);
        }
        let now = Instant::now();
        let mut map = self.pending.write().unwrap();
        let state = map.get_mut(request_id).ok_or(ConfirmationError::RequestNotFound)?;
        if !state.confirmations.is_empty() {
            let last = state.confirmations.last().unwrap().0;
            if now.duration_since(last) < self.min_interval {
                return Err(ConfirmationError::TooSoon);
            }
        }
        state.confirmations.push((now, ConfirmationChoice::Allow));
        let count = state.confirmations.len() as u32;
        if count >= self.required_confirmations {
            map.remove(request_id);
            Ok(ConfirmationOutcome::Allowed)
        } else {
            Ok(ConfirmationOutcome::Pending(
                self.required_confirmations - count,
            ))
        }
    }
}
