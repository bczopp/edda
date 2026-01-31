use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRecord {
    pub data_id: String,
    pub data: Vec<u8>,
    pub metadata: std::collections::HashMap<String, String>,
}
