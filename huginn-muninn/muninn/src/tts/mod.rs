//! TTS modules for Muninn

pub mod engine;
pub mod service;

pub use engine::{TtsEngine, TtsConfig, TtsVoice};
pub use service::{TTSService, TtsResult, TtsStreamChunk};
