//! Bootstrap: register Frigg/Valkyries as remote plugins when enabled and URL set.

use std::sync::Arc;
use tracing::info;

/// Registers Frigg and Valkyries as remote plugins when enabled and URL is set.
/// Call after `discover_all_capabilities()`; uses capability cache and `PluginManager::register_remote_plugin`.
pub async fn bootstrap_frigg_valkyries_plugins(
    plugin_manager: &crate::plugins::PluginManager,
    protocol_manager: &crate::protocols::manager::ProtocolManager,
    settings: &Arc<tokio::sync::RwLock<crate::utils::config::OdinSettings>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let set = settings.read().await;
    let entries: [(&str, Option<&String>, bool); 2] = [
        ("frigg", set.service_urls.frigg.as_ref(), set.plugins.frigg.enabled),
        ("valkyries", set.service_urls.valkyries.as_ref(), set.plugins.valkyries.enabled),
    ];
    drop(set);
    let cache = protocol_manager.get_cache();
    for (name, url_opt, enabled) in entries {
        if !enabled {
            continue;
        }
        if url_opt.is_none() {
            continue;
        }
        let Some(c) = cache.get(name).await else {
            tracing::debug!("Plugin {} not in capability cache, skip registration", name);
            continue;
        };
        let mut caps = vec![c.capability.god_name.clone()];
        caps.extend(c.capability.responsibility_domains.clone());
        caps.extend(c.capability.responsibility_keywords.clone());
        if let Err(e) = plugin_manager.register_remote_plugin(name.to_string(), &c.service_url, caps).await {
            tracing::warn!("Failed to register plugin {} at {}: {}", name, c.service_url, e);
        } else {
            info!("Registered plugin {} at {}", name, c.service_url);
        }
    }
    Ok(())
}
