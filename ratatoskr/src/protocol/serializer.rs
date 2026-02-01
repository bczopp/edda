use crate::proto::ratatoskr::*;
use prost::Message;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SerializationError {
    #[error("Failed to encode message: {0}")]
    EncodeError(#[from] prost::EncodeError),
    #[error("Failed to decode message: {0}")]
    DecodeError(#[from] prost::DecodeError),
}

/// MessageSerializer handles serialization and deserialization of Ratatoskr messages
#[derive(Clone)]
pub struct MessageSerializer;

impl MessageSerializer {
    /// Create a new MessageSerializer
    pub fn new() -> Self {
        Self
    }

    /// Serialize a RatatoskrRequest to bytes
    pub fn serialize_request(&self, request: &RatatoskrRequest) -> Result<Vec<u8>, SerializationError> {
        let mut buf = Vec::new();
        request.encode(&mut buf)?;
        Ok(buf)
    }

    /// Deserialize bytes to a RatatoskrRequest
    pub fn deserialize_request(&self, data: &[u8]) -> Result<RatatoskrRequest, SerializationError> {
        RatatoskrRequest::decode(data).map_err(SerializationError::DecodeError)
    }

    /// Serialize a RatatoskrResponse to bytes
    pub fn serialize_response(&self, response: &RatatoskrResponse) -> Result<Vec<u8>, SerializationError> {
        let mut buf = Vec::new();
        response.encode(&mut buf)?;
        Ok(buf)
    }

    /// Deserialize bytes to a RatatoskrResponse
    pub fn deserialize_response(&self, data: &[u8]) -> Result<RatatoskrResponse, SerializationError> {
        RatatoskrResponse::decode(data).map_err(SerializationError::DecodeError)
    }
}

impl Default for MessageSerializer {
    fn default() -> Self {
        Self::new()
    }
}
