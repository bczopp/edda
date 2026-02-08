//! Settings schema, validation, loader and hot-reload for Huginn & Muninn

pub mod error;
pub mod schema;
pub mod validator;
pub mod loader;
pub mod hot_reload;

pub use error::SettingsError;
pub use schema::{
    AudioDeviceConfig, HuginnSettings, LanguageSettings, MuninnSettings,
    SttQualitySettings, TtsCacheSettings, TtsQualitySettings, VoiceSettings,
    WakeWordSettings,
};
pub use validator::SettingsValidator;
pub use loader::{load_huginn_settings, load_muninn_settings};
pub use hot_reload::{HotReloadHandle, start_huginn_hot_reload, start_muninn_hot_reload};
