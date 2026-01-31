use crate::services::ServiceRegistry;
use std::sync::Arc;
use std::time::Duration;
use tracing::info;
use tokio::net::TcpStream;
use tokio::time::timeout;

/// Parse "http(s)://host:port" to (host, port). Minimal, for gRPC-style URLs.
fn parse_host_port(url: &str) -> Option<(String, u16)> {
    let after = url.split("://").nth(1)?;
    let authority = after.split('/').next().unwrap_or(after);
    let (host, port_str) = authority.rsplit_once(':')?;
    let port: u16 = port_str.parse().ok()?;
    Some((host.to_string(), port))
}

/// Phase 3 Lifecycle: echte Prozess-Start/Stop – injizierbar für Tests und echte Implementierung.
pub trait ProcessRunner: Send + Sync {
    fn start(&self, name: &str, service_url: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    fn stop(&self, name: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

pub struct ServiceLifecycleManager {
    registry: Arc<ServiceRegistry>,
    process_runner: Option<Arc<dyn ProcessRunner>>,
}

impl ServiceLifecycleManager {
    pub fn new(registry: Arc<ServiceRegistry>) -> Self {
        Self {
            registry,
            process_runner: None,
        }
    }

    /// Phase 3 Lifecycle: echte Prozess-Start/Stop – wenn gesetzt, rufen start_service/stop_service den Runner auf.
    pub fn with_process_runner(mut self, runner: Arc<dyn ProcessRunner>) -> Self {
        self.process_runner = Some(runner);
        self
    }

    pub async fn start_service(&self, service_name: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let info = self
            .registry
            .get(service_name)
            .await
            .ok_or_else(|| format!("Service '{}' not in registry", service_name))?;
        if let Some(ref r) = self.process_runner {
            r.start(service_name, &info.service_url)?;
        } else {
            info!("Starting service: {}", service_name);
        }
        Ok(())
    }

    pub async fn stop_service(&self, service_name: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if self.registry.get(service_name).await.is_none() {
            return Err(format!("Service '{}' not in registry", service_name).into());
        }
        if let Some(ref r) = self.process_runner {
            r.stop(service_name)?;
        } else {
            info!("Stopping service: {}", service_name);
        }
        Ok(())
    }

    /// Liefert true, wenn der Service in der Registry steht; false sonst.
    /// Keine echte Netzwerk-Prüfung – nur Registry-Consistency.
    pub async fn health_check(&self, service_name: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self.registry.get(service_name).await.is_some())
    }

    /// Phase 3 Lifecycle: Health-Check via Netzwerk – TCP connect to service_url. Returns true if reachable.
    pub async fn health_check_reachable(&self, service_name: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let info = self
            .registry
            .get(service_name)
            .await
            .ok_or_else(|| format!("Service '{}' not in registry", service_name))?;
        let (host, port) = parse_host_port(&info.service_url)
            .ok_or_else(|| format!("Invalid URL for health check: {}", info.service_url))?;
        let addr = format!("{}:{}", host, port);
        match timeout(Duration::from_secs(2), TcpStream::connect(&addr)).await {
            Ok(Ok(_)) => Ok(true),
            _ => Ok(false),
        }
    }
}
