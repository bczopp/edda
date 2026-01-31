//! Tests for Frigg/Valkyries bootstrap (Phase 6).

use odin::bootstrap::bootstrap_frigg_valkyries_plugins;
use odin::plugins::PluginManager;
use odin::protocols::manager::ProtocolManager;
use odin::utils::config::OdinSettings;
use std::sync::Arc;

#[tokio::test]
async fn bootstrap_frigg_valkyries_both_disabled_leaves_plugin_list_empty() {
    let mut settings = OdinSettings::default();
    settings.plugins.frigg.enabled = false;
    settings.plugins.valkyries.enabled = false;
    let settings_arc = Arc::new(tokio::sync::RwLock::new(settings));
    let protocol_manager = ProtocolManager::new(settings_arc.clone());
    let plugin_manager = PluginManager::new();

    bootstrap_frigg_valkyries_plugins(
        &plugin_manager,
        &protocol_manager,
        &settings_arc,
    )
    .await
    .unwrap();

    let names = plugin_manager.list().await;
    assert!(names.is_empty(), "both plugins disabled => list empty");
}
