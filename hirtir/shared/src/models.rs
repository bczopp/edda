use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderData {
    pub provider_id: String,
    pub data: Vec<u8>,
    pub metadata: std::collections::HashMap<String, String>,
}
