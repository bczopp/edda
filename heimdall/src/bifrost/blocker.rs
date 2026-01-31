//! Connection blocking: immediate block on security threats, token revocation, temporary/permanent, unblock.

use crate::bifrost::{ConnectionMonitor, ConnectionStatus};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use thiserror::Error;

fn connection_key(source: &str, target: &str) -> String {
    format!("{}:{}", source, target)
}

#[derive(Debug, Error)]
pub enum ConnectionBlockerError {
    #[error("Token revocation failed: {0}")]
    Revocation(#[from] crate::token::TokenRevocationError),
}

/// Blocks connections: immediate block, optional token revocation, temporary or permanent, unblock.
pub struct ConnectionBlocker {
    monitor: Arc<ConnectionMonitor>,
    revocation: Option<Arc<crate::token::TokenRevocationManager>>,
    block_until: Arc<RwLock<HashMap<String, Option<i64>>>>,
}

impl ConnectionBlocker {
    pub fn new(
        monitor: Arc<ConnectionMonitor>,
        revocation: Option<Arc<crate::token::TokenRevocationManager>>,
    ) -> Self {
        Self {
            monitor,
            revocation,
            block_until: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Block connection immediately; optionally revoke token and set duration (None = permanent).
    pub async fn block_connection(
        &self,
        source_device_id: &str,
        target_device_id: &str,
        token_id_to_revoke: Option<&str>,
        duration_secs: Option<u64>,
    ) -> Result<(), ConnectionBlockerError> {
        self.monitor.register_connection(source_device_id, target_device_id).await;
        if let (Some(rev), Some(token_id)) = (&self.revocation, token_id_to_revoke) {
            rev.revoke(token_id).await?;
        }
        self.monitor
            .set_status(source_device_id, target_device_id, ConnectionStatus::Blocked)
            .await;
        let key = connection_key(source_device_id, target_device_id);
        let unblock_at = duration_secs
            .map(|d| Utc::now().timestamp() + d as i64);
        let mut until = self.block_until.write().await;
        until.insert(key, unblock_at);
        Ok(())
    }

    /// Unblock connection (manual or after temporary expiry).
    pub async fn unblock_connection(
        &self,
        source_device_id: &str,
        target_device_id: &str,
    ) -> Result<(), ConnectionBlockerError> {
        self.monitor
            .set_status(source_device_id, target_device_id, ConnectionStatus::Active)
            .await;
        let key = connection_key(source_device_id, target_device_id);
        let mut until = self.block_until.write().await;
        until.remove(&key);
        Ok(())
    }

    /// True if connection is currently blocked. Call apply_auto_unblock to clear expired temporary blocks.
    pub async fn is_blocked(&self, source_device_id: &str, target_device_id: &str) -> bool {
        let key = connection_key(source_device_id, target_device_id);
        let until = self.block_until.read().await;
        if until.contains_key(&key) {
            if let Some(Some(unblock_at)) = until.get(&key) {
                if Utc::now().timestamp() > *unblock_at {
                    drop(until);
                    let _ = self.unblock_connection(source_device_id, target_device_id).await;
                    return false;
                }
            }
            return true;
        }
        self.monitor
            .get_status(source_device_id, target_device_id)
            .await
            .map(|s| s == ConnectionStatus::Blocked)
            .unwrap_or(false)
    }

    /// Unblock all connections whose temporary block has expired.
    pub async fn apply_auto_unblock(&self) {
        let now = Utc::now().timestamp();
        let mut until = self.block_until.write().await;
        let keys_to_remove: Vec<String> = until
            .iter()
            .filter(|(_, v)| v.map(|t| now > t).unwrap_or(false))
            .map(|(k, _)| k.clone())
            .collect();
        for key in keys_to_remove {
            if let Some((source, target)) = key.split_once(':') {
                self.monitor
                    .set_status(source, target, ConnectionStatus::Active)
                    .await;
            }
            until.remove(&key);
        }
    }
}
