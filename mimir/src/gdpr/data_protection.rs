use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use tracing::{warn, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataMinimizationPolicy {
    /// Maximum data size in bytes
    pub max_data_size: Option<u64>,
    /// Maximum number of data entries per user
    pub max_entries_per_user: Option<u32>,
    /// Fields that should not be stored (blacklist)
    pub forbidden_fields: Vec<String>,
    /// Enable strict mode (reject if policy violated)
    pub strict_mode: bool,
}

impl Default for DataMinimizationPolicy {
    fn default() -> Self {
        Self {
            max_data_size: Some(10 * 1024 * 1024), // 10 MB default
            max_entries_per_user: Some(1000),
            forbidden_fields: vec![],
            strict_mode: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurposeLimitationPolicy {
    /// Allowed purposes for data storage
    pub allowed_purposes: Vec<String>,
    /// Require purpose specification
    pub require_purpose: bool,
}

impl Default for PurposeLimitationPolicy {
    fn default() -> Self {
        Self {
            allowed_purposes: vec![
                "service_operation".to_string(),
                "user_request".to_string(),
                "legal_requirement".to_string(),
            ],
            require_purpose: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageLimitationPolicy {
    /// Default retention period in days
    pub default_retention_days: u32,
    /// Enable automatic deletion
    pub enable_auto_deletion: bool,
    /// Anonymize instead of delete
    pub anonymize_on_deletion: bool,
}

impl Default for StorageLimitationPolicy {
    fn default() -> Self {
        Self {
            default_retention_days: 365,
            enable_auto_deletion: false,
            anonymize_on_deletion: true,
        }
    }
}

#[derive(Debug, Error)]
pub enum DataProtectionError {
    #[error("Data minimization violation: {0}")]
    DataMinimizationViolation(String),
    #[error("Purpose limitation violation: {0}")]
    PurposeLimitationViolation(String),
    #[error("Storage limitation violation: {0}")]
    StorageLimitationViolation(String),
}

pub struct DataProtectionManager {
    minimization_policy: Arc<DataMinimizationPolicy>,
    purpose_policy: Arc<PurposeLimitationPolicy>,
    storage_policy: Arc<StorageLimitationPolicy>,
}

impl DataProtectionManager {
    pub fn new(
        minimization_policy: DataMinimizationPolicy,
        purpose_policy: PurposeLimitationPolicy,
        storage_policy: StorageLimitationPolicy,
    ) -> Self {
        Self {
            minimization_policy: Arc::new(minimization_policy),
            purpose_policy: Arc::new(purpose_policy),
            storage_policy: Arc::new(storage_policy),
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(
            DataMinimizationPolicy::default(),
            PurposeLimitationPolicy::default(),
            StorageLimitationPolicy::default(),
        )
    }

    /// Check data minimization policy before storing data
    pub async fn check_data_minimization(
        &self,
        data_size: usize,
        user_entry_count: u32,
        data_content: Option<&[u8]>,
    ) -> Result<(), DataProtectionError> {
        // Check maximum data size
        if let Some(max_size) = self.minimization_policy.max_data_size {
            if data_size as u64 > max_size {
                let msg = format!(
                    "Data size {} exceeds maximum allowed size {}",
                    data_size, max_size
                );
                if self.minimization_policy.strict_mode {
                    return Err(DataProtectionError::DataMinimizationViolation(msg));
                } else {
                    warn!("{}", msg);
                }
            }
        }

        // Check maximum entries per user
        if let Some(max_entries) = self.minimization_policy.max_entries_per_user {
            if user_entry_count >= max_entries {
                let msg = format!(
                    "User has {} entries, exceeds maximum allowed {}",
                    user_entry_count, max_entries
                );
                if self.minimization_policy.strict_mode {
                    return Err(DataProtectionError::DataMinimizationViolation(msg));
                } else {
                    warn!("{}", msg);
                }
            }
        }

        // Check for forbidden fields (if data is JSON)
        if let Some(content) = data_content {
            if let Ok(json_value) = serde_json::from_slice::<serde_json::Value>(content) {
                if let Some(obj) = json_value.as_object() {
                    for field in &self.minimization_policy.forbidden_fields {
                        if obj.contains_key(field) {
                            let msg = format!("Data contains forbidden field: {}", field);
                            if self.minimization_policy.strict_mode {
                                return Err(DataProtectionError::DataMinimizationViolation(msg));
                            } else {
                                warn!("{}", msg);
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Check purpose limitation policy
    pub fn check_purpose_limitation(
        &self,
        purpose: Option<&str>,
    ) -> Result<(), DataProtectionError> {
        if self.purpose_policy.require_purpose && purpose.is_none() {
            return Err(DataProtectionError::PurposeLimitationViolation(
                "Purpose is required but not provided".to_string(),
            ));
        }

        if let Some(purpose_str) = purpose {
            if !self.purpose_policy.allowed_purposes.contains(&purpose_str.to_string()) {
                return Err(DataProtectionError::PurposeLimitationViolation(format!(
                    "Purpose '{}' is not in allowed purposes: {:?}",
                    purpose_str, self.purpose_policy.allowed_purposes
                )));
            }
        }

        Ok(())
    }

    /// Get storage limitation policy
    pub fn get_storage_policy(&self) -> &StorageLimitationPolicy {
        &self.storage_policy
    }

    /// Check if data should be deleted based on retention policy
    pub fn should_delete_data(&self, days_since_creation: u32) -> bool {
        if !self.storage_policy.enable_auto_deletion {
            return false;
        }

        days_since_creation >= self.storage_policy.default_retention_days
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_minimization_policy_default() {
        let policy = DataMinimizationPolicy::default();
        assert_eq!(policy.max_data_size, Some(10 * 1024 * 1024));
        assert_eq!(policy.max_entries_per_user, Some(1000));
        assert_eq!(policy.strict_mode, false);
    }

    #[tokio::test]
    async fn test_check_data_minimization_size_ok() {
        let manager = DataProtectionManager::with_defaults();
        let result = manager.check_data_minimization(1024, 10, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_check_data_minimization_size_exceeded() {
        let policy = DataMinimizationPolicy {
            max_data_size: Some(1000),
            max_entries_per_user: None,
            forbidden_fields: vec![],
            strict_mode: true,
        };
        let manager = DataProtectionManager::new(
            policy,
            PurposeLimitationPolicy::default(),
            StorageLimitationPolicy::default(),
        );
        let result = manager.check_data_minimization(2000, 10, None).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_check_purpose_limitation_ok() {
        let manager = DataProtectionManager::with_defaults();
        let result = manager.check_purpose_limitation(Some("service_operation"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_purpose_limitation_invalid() {
        let manager = DataProtectionManager::with_defaults();
        let result = manager.check_purpose_limitation(Some("invalid_purpose"));
        assert!(result.is_err());
    }

    #[test]
    fn test_check_purpose_limitation_required() {
        let policy = PurposeLimitationPolicy {
            allowed_purposes: vec!["purpose1".to_string()],
            require_purpose: true,
        };
        let manager = DataProtectionManager::new(
            DataMinimizationPolicy::default(),
            policy,
            StorageLimitationPolicy::default(),
        );
        let result = manager.check_purpose_limitation(None);
        assert!(result.is_err());
    }

    #[test]
    fn test_should_delete_data() {
        let policy = StorageLimitationPolicy {
            default_retention_days: 365,
            enable_auto_deletion: true,
            anonymize_on_deletion: false,
        };
        let manager = DataProtectionManager::new(
            DataMinimizationPolicy::default(),
            PurposeLimitationPolicy::default(),
            policy,
        );

        assert!(!manager.should_delete_data(100));
        assert!(manager.should_delete_data(365));
        assert!(manager.should_delete_data(400));
    }
}
