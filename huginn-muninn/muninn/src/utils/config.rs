use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuninnSettings {
    pub grpc_port: u16,
}

impl Default for MuninnSettings {
    fn default() -> Self {
        Self {
            grpc_port: 50058,
        }
    }
}
