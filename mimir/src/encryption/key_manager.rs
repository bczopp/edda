//! Key Manager for encryption key management and rotation

use ring::rand::{SecureRandom, SystemRandom};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::Duration;
use tokio::task::JoinHandle;
use tracing::{info, warn};

const DEFAULT_MAX_HISTORICAL_KEYS: usize = 3;

#[derive(Debug, Clone)]
pub struct KeyVersion {
    pub version: u32,
    pub key: Vec<u8>,
}

pub struct KeyManager {
    current_key: Arc<RwLock<KeyVersion>>,
    historical_keys: Arc<RwLock<Vec<KeyVersion>>>,
    max_historical_keys: usize,
    rng: SystemRandom,
    rotation_task: Arc<RwLock<Option<JoinHandle<()>>>>,
}

impl KeyManager {
    /// Create a new KeyManager with a random 32-byte key
    pub fn new_with_random_key() -> Self {
        let rng = SystemRandom::new();
        let mut key_bytes = vec![0u8; 32];
        rng.fill(&mut key_bytes).expect("Failed to generate random key");
        
        Self {
            current_key: Arc::new(RwLock::new(KeyVersion {
                version: 1,
                key: key_bytes,
            })),
            historical_keys: Arc::new(RwLock::new(Vec::new())),
            max_historical_keys: DEFAULT_MAX_HISTORICAL_KEYS,
            rng,
            rotation_task: Arc::new(RwLock::new(None)),
        }
    }

    /// Create a new KeyManager with a specific key
    pub fn new_with_key(key_bytes: Vec<u8>) -> Result<Self, String> {
        if key_bytes.len() != 32 {
            return Err("Key must be 32 bytes for AES-256".to_string());
        }
        
        Ok(Self {
            current_key: Arc::new(RwLock::new(KeyVersion {
                version: 1,
                key: key_bytes,
            })),
            historical_keys: Arc::new(RwLock::new(Vec::new())),
            max_historical_keys: DEFAULT_MAX_HISTORICAL_KEYS,
            rng: SystemRandom::new(),
            rotation_task: Arc::new(RwLock::new(None)),
        })
    }

    /// Get the current encryption key
    pub async fn get_current_key(&self) -> Vec<u8> {
        self.current_key.read().await.key.clone()
    }

    /// Get the current key version
    pub async fn get_current_version(&self) -> u32 {
        self.current_key.read().await.version
    }

    /// Get all historical keys (excluding current)
    pub async fn get_historical_keys(&self) -> Vec<Vec<u8>> {
        self.historical_keys.read().await
            .iter()
            .map(|kv| kv.key.clone())
            .collect()
    }

    /// Get a key by its version
    pub async fn get_key_by_version(&self, version: u32) -> Option<Vec<u8>> {
        // Check current key
        {
            let current = self.current_key.read().await;
            if current.version == version {
                return Some(current.key.clone());
            }
        }
        
        // Check historical keys
        let historical = self.historical_keys.read().await;
        historical.iter()
            .find(|kv| kv.version == version)
            .map(|kv| kv.key.clone())
    }

    /// Rotate the encryption key (generates a new key and moves current to historical)
    pub async fn rotate_key(&mut self) -> Result<(), String> {
        info!("Rotating encryption key");
        
        // Generate new key
        let mut new_key_bytes = vec![0u8; 32];
        self.rng.fill(&mut new_key_bytes)
            .map_err(|e| format!("Failed to generate new key: {}", e))?;
        
        // Get current key and version
        let old_key_version = {
            let current = self.current_key.read().await;
            current.clone()
        };
        
        // Update current key
        {
            let mut current = self.current_key.write().await;
            current.version = old_key_version.version + 1;
            current.key = new_key_bytes;
        }
        
        // Move old key to historical
        {
            let mut historical = self.historical_keys.write().await;
            historical.push(old_key_version);
            
            // Keep only max_historical_keys
            if historical.len() > self.max_historical_keys {
                historical.remove(0);
            }
        }
        
        info!("Key rotation complete. New version: {}", self.get_current_version().await);
        Ok(())
    }

    /// Enable automatic key rotation at specified interval
    pub fn enable_automatic_rotation(&mut self, interval: Duration) {
        let current_key = Arc::clone(&self.current_key);
        let historical_keys = Arc::clone(&self.historical_keys);
        let max_historical_keys = self.max_historical_keys;
        let rotation_task = Arc::clone(&self.rotation_task);
        
        let handle = tokio::spawn(async move {
            let rng = SystemRandom::new();
            let mut ticker = tokio::time::interval(interval);
            
            loop {
                ticker.tick().await;
                
                // Generate new key
                let mut new_key_bytes = vec![0u8; 32];
                if let Err(e) = rng.fill(&mut new_key_bytes) {
                    warn!("Failed to generate new key during automatic rotation: {}", e);
                    continue;
                }
                
                // Get current key
                let old_key_version = {
                    let current = current_key.read().await;
                    current.clone()
                };
                
                // Update current key
                {
                    let mut current = current_key.write().await;
                    current.version = old_key_version.version + 1;
                    current.key = new_key_bytes;
                }
                
                // Move old key to historical
                {
                    let mut historical = historical_keys.write().await;
                    historical.push(old_key_version);
                    
                    // Keep only max_historical_keys
                    if historical.len() > max_historical_keys {
                        historical.remove(0);
                    }
                }
                
                info!("Automatic key rotation complete");
            }
        });
        
        // Store handle
        let task_clone = Arc::clone(&rotation_task);
        tokio::spawn(async move {
            *task_clone.write().await = Some(handle);
        });
    }

    /// Disable automatic key rotation
    pub fn disable_automatic_rotation(&self) {
        let rotation_task = Arc::clone(&self.rotation_task);
        tokio::spawn(async move {
            if let Some(handle) = rotation_task.write().await.take() {
                handle.abort();
                info!("Automatic key rotation disabled");
            }
        });
    }

    /// Set maximum number of historical keys to retain
    pub fn set_max_historical_keys(&mut self, max: usize) {
        self.max_historical_keys = max;
    }
}

impl Drop for KeyManager {
    fn drop(&mut self) {
        // Abort rotation task if still running
        let rotation_task = Arc::clone(&self.rotation_task);
        tokio::spawn(async move {
            if let Some(handle) = rotation_task.write().await.take() {
                handle.abort();
            }
        });
    }
}
