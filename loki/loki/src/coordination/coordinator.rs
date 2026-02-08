//! Service coordinator – lifecycle, script routing, health (Phase 9.1.1).

use crate::coordination::router::{route, ScriptRoute};
use crate::script::{ScriptContext, ScriptEngine};
use fenrir::{FenrirScriptAPI, GPIOController, HardwareAccess, SensorReader, ActuatorController, StubHardwareAccess};
use hel::{HelScriptAPI, FilesystemHandler, StorageManager, CacheManager};
use jormungandr::JormungandrScriptAPI;
use shared::{LokiError, Result, ScriptDefinition, ScriptLanguage};
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;

/// Coordinator for Loki's sub-services (Fenrir, Jörmungandr, Hel).
pub struct ServiceCoordinator {
    script_engine: ScriptEngine,
    fenrir_api: Arc<FenrirScriptAPI>,
    jormungandr_api: Arc<JormungandrScriptAPI>,
    hel_api: Arc<HelScriptAPI>,
}

impl ServiceCoordinator {
    /// Create coordinator with in-process script APIs (stub/default config).
    pub fn new() -> Result<Self> {
        let hw = Arc::new(StubHardwareAccess::new());
        let gpio = Arc::new(GPIOController::new(Arc::clone(&hw)));
        let sensors = Arc::new(SensorReader::new(Arc::clone(&hw)));
        let actuators = Arc::new(ActuatorController::new(hw));
        let fenrir_api = Arc::new(FenrirScriptAPI::new(gpio, sensors, actuators));

        let http = Arc::new(jormungandr::HttpHandler::new().map_err(|e| LokiError::ExecutionError(e.to_string()))?);
        let ws = Arc::new(TokioMutex::new(None::<jormungandr::WsHandler>));
        let mqtt = Arc::new(TokioMutex::new(jormungandr::MqttHandler::new()));
        let jormungandr_api = Arc::new(JormungandrScriptAPI::new(http, ws, mqtt));

        let base = std::env::temp_dir().join("loki_hel");
        let _ = std::fs::create_dir_all(&base);
        let fs = Arc::new(FilesystemHandler::new(&base).map_err(|e| LokiError::ExecutionError(e.to_string()))?);
        let storage = Arc::new(StorageManager::new_in_memory());
        let cache = Arc::new(CacheManager::new(60));
        let hel_api = Arc::new(HelScriptAPI::new(fs, storage, cache));

        Ok(Self {
            script_engine: ScriptEngine::new(),
            fenrir_api,
            jormungandr_api,
            hel_api,
        })
    }

    /// Execute script; registers fenrir/jormungandr/hel APIs as needed (from script scan).
    pub async fn execute_script(&self, script: &str) -> Result<String> {
        let route = route(script);
        let script_def = ScriptDefinition::new("inline".to_string(), ScriptLanguage::Lua, script.to_string());
        let context = ScriptContext::new();

        let fenrir = Arc::clone(&self.fenrir_api);
        let jormungandr = Arc::clone(&self.jormungandr_api);
        let hel = Arc::clone(&self.hel_api);

        let lua_setup = move |lua: &mlua::Lua| {
            if route.needs_fenrir() {
                fenrir.register_into(lua).map_err(|e| LokiError::ExecutionError(e.to_string()))?;
            }
            if route.needs_jormungandr() {
                jormungandr.register_into(lua).map_err(|e| LokiError::ExecutionError(e.to_string()))?;
            }
            if route.needs_hel() {
                hel.register_into(lua).map_err(|e| LokiError::ExecutionError(e.to_string()))?;
            }
            Ok(())
        };

        self.script_engine
            .execute_with_lua_setup(&script_def, context, Some(lua_setup))
            .await
    }

    /// Health: all three APIs are present (in-process, always true when constructed).
    pub fn health_check(&self) -> (bool, bool, bool) {
        (true, true, true)
    }
}
