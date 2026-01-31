use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HuginnSettings {
    pub grpc_port: u16,
}

impl Default for HuginnSettings {
    fn default() -> Self {
        Self {
            grpc_port: 50057,
        }
    }
}
