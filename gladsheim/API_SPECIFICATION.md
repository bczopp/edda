# Gladsheim API Specification

## Übersicht

Diese Spezifikation beschreibt die detaillierte API für alle vier Diener (Servants) von Gladsheim:
- **Thjalfi (Thjalfi)**: Service Loader
- **Byggvir**: Resource Manager
- **Roskva (Roskva)**: Health Monitor
- **Skirnir (Skirnir)**: Service Registry

## Thjalfi (Service Loader) API

### Verantwortlichkeiten

Thjalfi ist zuständig für:
- Laden von Services in den RAM
- Starten von Service-Prozessen
- Stoppen von Services (graceful + force)
- Process-Management und -Überwachung

### Public API (Rust)

```rust
pub struct ServiceLoader {
    // Internal state
}

impl ServiceLoader {
    /// Erstellt einen neuen ServiceLoader
    pub fn new(config: ServiceLoaderConfig) -> Self;
    
    /// Startet einen Service
    ///
    /// # Arguments
    /// * `service_name` - Name des Services (z.B. "thor", "freki")
    /// * `options` - Start-Optionen (env vars, working dir, args)
    ///
    /// # Returns
    /// * `Ok(ServiceProcess)` - Service-Process-Handle bei Erfolg
    /// * `Err(ServiceLoaderError)` - Fehler bei Startup
    ///
    /// # Workflow
    /// 1. Service-Path auflösen
    /// 2. Heimdall-Authorization prüfen
    /// 3. Process spawnen (tokio::process::Command)
    /// 4. Startup-Validation (Process läuft?)
    /// 5. Timeout-Enforcement
    pub async fn start_service(
        &mut self,
        service_name: &str,
        options: StartOptions,
    ) -> Result<ServiceProcess, ServiceLoaderError>;
    
    /// Stoppt einen Service gracefully
    ///
    /// # Arguments
    /// * `service_name` - Name des Services
    /// * `timeout` - Timeout für graceful shutdown
    ///
    /// # Returns
    /// * `Ok(())` - Service erfolgreich gestoppt
    /// * `Err(ServiceLoaderError)` - Fehler beim Stoppen
    ///
    /// # Workflow
    /// 1. SIGTERM an Process senden
    /// 2. Auf Process-Exit warten (mit Timeout)
    /// 3. Bei Timeout: SIGKILL senden (force-kill)
    pub async fn stop_service(
        &mut self,
        service_name: &str,
        timeout: Duration,
    ) -> Result<(), ServiceLoaderError>;
    
    /// Stoppt einen Service sofort (force-kill)
    ///
    /// # Arguments
    /// * `service_name` - Name des Services
    ///
    /// # Returns
    /// * `Ok(())` - Service erfolgreich gestoppt
    /// * `Err(ServiceLoaderError)` - Fehler beim Force-Kill
    ///
    /// # Workflow
    /// 1. SIGKILL an Process senden
    pub async fn force_kill_service(
        &mut self,
        service_name: &str,
    ) -> Result<(), ServiceLoaderError>;
    
    /// Startet einen Service neu (Stop + Start)
    ///
    /// # Arguments
    /// * `service_name` - Name des Services
    /// * `options` - Start-Optionen für Restart
    ///
    /// # Returns
    /// * `Ok(ServiceProcess)` - Service-Process-Handle nach Restart
    /// * `Err(ServiceLoaderError)` - Fehler beim Restart
    pub async fn restart_service(
        &mut self,
        service_name: &str,
        options: StartOptions,
    ) -> Result<ServiceProcess, ServiceLoaderError>;
    
    /// Gibt Process-Handle für Service zurück
    ///
    /// # Returns
    /// * `Some(&ServiceProcess)` - Process-Handle wenn Service läuft
    /// * `None` - Service läuft nicht
    pub fn get_process(&self, service_name: &str) -> Option<&ServiceProcess>;
    
    /// Prüft ob Service läuft
    pub fn is_running(&self, service_name: &str) -> bool;
}

/// Start-Optionen für Service
pub struct StartOptions {
    /// Environment-Variables
    pub env_vars: HashMap<String, String>,
    
    /// Working-Directory
    pub working_dir: Option<PathBuf>,
    
    /// Command-Line-Arguments
    pub args: Vec<String>,
    
    /// Resource-Limits
    pub resource_limits: Option<ResourceLimits>,
    
    /// Startup-Timeout
    pub startup_timeout: Duration,
}

/// Service-Process-Handle
pub struct ServiceProcess {
    /// Service-Name
    pub service_name: String,
    
    /// Process-ID
    pub pid: u32,
    
    /// Process-Handle (tokio::process::Child)
    process: Child,
    
    /// Start-Zeit
    pub start_time: SystemTime,
}

impl ServiceProcess {
    /// Wartet auf Process-Exit
    pub async fn wait(&mut self) -> Result<ExitStatus, std::io::Error>;
    
    /// Sendet Signal an Process
    pub fn send_signal(&mut self, signal: Signal) -> Result<(), std::io::Error>;
    
    /// Prüft ob Process noch läuft
    pub fn is_alive(&mut self) -> bool;
}

/// Service-Loader-Errors
#[derive(Debug, thiserror::Error)]
pub enum ServiceLoaderError {
    #[error("Service not found: {0}")]
    ServiceNotFound(String),
    
    #[error("Service already running: {0}")]
    AlreadyRunning(String),
    
    #[error("Unauthorized to start service: {0}")]
    Unauthorized(String),
    
    #[error("Startup timeout: {0}")]
    StartupTimeout(String),
    
    #[error("Process spawn failed: {0}")]
    SpawnFailed(String),
    
    #[error("Service not running: {0}")]
    NotRunning(String),
    
    #[error("Shutdown timeout: {0}")]
    ShutdownTimeout(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

### Configuration

```rust
pub struct ServiceLoaderConfig {
    /// Startup-Timeout (default: 30s)
    pub startup_timeout: Duration,
    
    /// Shutdown-Timeout (default: 10s)
    pub shutdown_timeout: Duration,
    
    /// Force-Kill nach Shutdown-Timeout (default: true)
    pub force_kill_after_timeout: bool,
    
    /// Service-Binary-Paths (Mapping von Service-Name zu Binary-Path)
    pub service_paths: HashMap<String, PathBuf>,
}
```

---

## Byggvir (Resource Manager) API

### Verantwortlichkeiten

Byggvir ist zuständig für:
- Resource-Monitoring (RAM, CPU)
- Resource-Limit-Enforcement
- Platform-spezifische Resource-Limits
- Battery-aware Resource-Management (Alfheim)

### Public API (Rust)

```rust
pub struct ResourceManager {
    // Internal state
}

impl ResourceManager {
    /// Erstellt einen neuen ResourceManager
    pub fn new(config: ResourceManagerConfig) -> Self;
    
    /// Startet kontinuierliches Resource-Monitoring
    ///
    /// Startet Background-Task der periodisch Resources überwacht
    pub async fn start_monitoring(&mut self);
    
    /// Stoppt Resource-Monitoring
    pub async fn stop_monitoring(&mut self);
    
    /// Gibt aktuelle Resource-Usage für Service zurück
    ///
    /// # Returns
    /// * `Ok(ResourceUsage)` - Aktuelle Resource-Usage
    /// * `Err(ResourceManagerError)` - Service nicht gefunden oder Fehler
    pub fn get_resource_usage(
        &self,
        service_name: &str,
    ) -> Result<ResourceUsage, ResourceManagerError>;
    
    /// Gibt Gesamt-System-Resource-Usage zurück
    pub fn get_system_usage(&self) -> SystemResourceUsage;
    
    /// Setzt Resource-Limits für Service
    ///
    /// # Arguments
    /// * `service_name` - Name des Services
    /// * `limits` - Neue Resource-Limits
    ///
    /// # Returns
    /// * `Ok(())` - Limits erfolgreich gesetzt
    /// * `Err(ResourceManagerError)` - Fehler beim Setzen
    pub fn set_resource_limits(
        &mut self,
        service_name: &str,
        limits: ResourceLimits,
    ) -> Result<(), ResourceManagerError>;
    
    /// Gibt aktuelle Resource-Limits für Service zurück
    pub fn get_resource_limits(
        &self,
        service_name: &str,
    ) -> Result<ResourceLimits, ResourceManagerError>;
    
    /// Prüft ob Service Resource-Limits überschreitet
    ///
    /// # Returns
    /// * `Ok(None)` - Innerhalb der Limits
    /// * `Ok(Some(LimitViolation))` - Limit überschritten
    /// * `Err(ResourceManagerError)` - Fehler
    pub fn check_limits(
        &self,
        service_name: &str,
    ) -> Result<Option<LimitViolation>, ResourceManagerError>;
    
    /// Registriert Callback für Limit-Violations
    ///
    /// Callback wird aufgerufen wenn Service Limits überschreitet
    pub fn on_limit_violation<F>(&mut self, callback: F)
    where
        F: Fn(LimitViolation) + Send + Sync + 'static;
}

/// Resource-Usage
#[derive(Debug, Clone)]
pub struct ResourceUsage {
    /// Memory-Usage in Bytes
    pub memory_bytes: u64,
    
    /// CPU-Usage in Prozent (0.0 - 100.0)
    pub cpu_percent: f32,
    
    /// Measurement-Timestamp
    pub measured_at: SystemTime,
}

impl ResourceUsage {
    /// Memory in MB
    pub fn memory_mb(&self) -> f32 {
        self.memory_bytes as f32 / 1024.0 / 1024.0
    }
    
    /// Memory in GB
    pub fn memory_gb(&self) -> f32 {
        self.memory_bytes as f32 / 1024.0 / 1024.0 / 1024.0
    }
}

/// System-Resource-Usage
#[derive(Debug, Clone)]
pub struct SystemResourceUsage {
    /// Gesamt-RAM in Bytes
    pub total_memory: u64,
    
    /// Genutzter RAM in Bytes
    pub used_memory: u64,
    
    /// Verfügbarer RAM in Bytes
    pub available_memory: u64,
    
    /// Gesamt-CPU-Usage (0.0 - 100.0 * num_cores)
    pub cpu_usage: f32,
    
    /// Anzahl CPU-Cores
    pub num_cores: usize,
}

/// Resource-Limits
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    /// Max Memory in Bytes (0 = unlimited)
    pub max_memory_bytes: u64,
    
    /// Max CPU in Prozent (0.0 = unlimited)
    pub max_cpu_percent: f32,
    
    /// Warning-Threshold für Memory (Prozent von max_memory)
    pub memory_warning_percent: f32,
    
    /// Warning-Threshold für CPU (Prozent von max_cpu)
    pub cpu_warning_percent: f32,
}

impl ResourceLimits {
    /// Default-Limits für Platform
    pub fn for_platform(platform: Platform) -> Self;
    
    /// Max Memory in MB
    pub fn max_memory_mb(&self) -> f32;
    
    /// Setzt Max Memory in MB
    pub fn set_max_memory_mb(&mut self, mb: f32);
}

/// Limit-Violation
#[derive(Debug, Clone)]
pub struct LimitViolation {
    /// Service-Name
    pub service_name: String,
    
    /// Violation-Type
    pub violation_type: ViolationType,
    
    /// Aktuelle Usage
    pub current_usage: ResourceUsage,
    
    /// Limits
    pub limits: ResourceLimits,
    
    /// Timestamp
    pub timestamp: SystemTime,
}

/// Violation-Type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViolationType {
    /// Memory-Limit überschritten
    MemoryExceeded,
    
    /// CPU-Limit überschritten
    CpuExceeded,
    
    /// Memory-Warning-Threshold überschritten
    MemoryWarning,
    
    /// CPU-Warning-Threshold überschritten
    CpuWarning,
}

/// Resource-Manager-Errors
#[derive(Debug, thiserror::Error)]
pub enum ResourceManagerError {
    #[error("Service not found: {0}")]
    ServiceNotFound(String),
    
    #[error("Monitoring not started")]
    MonitoringNotStarted,
    
    #[error("Invalid resource limits: {0}")]
    InvalidLimits(String),
    
    #[error("System monitoring error: {0}")]
    SystemError(String),
}
```

### Configuration

```rust
pub struct ResourceManagerConfig {
    /// Monitoring-Interval (default: 1s)
    pub monitoring_interval: Duration,
    
    /// Platform-Type (für platform-spezifische Limits)
    pub platform: Platform,
    
    /// Default-Resource-Limits
    pub default_limits: ResourceLimits,
    
    /// Battery-aware (nur für Alfheim)
    pub battery_aware: bool,
    
    /// Battery-Threshold für Service-Stop (nur Alfheim, default: 15%)
    pub stop_services_below_battery_percent: u8,
}

/// Platform-Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Midgard,  // Desktop
    Alfheim,  // Mobile
    Asgard,   // Homeserver
    Ragnarok, // Terminal
}
```

---

## Roskva (Health Monitor) API

### Verantwortlichkeiten

Roskva ist zuständig für:
- Periodische Health-Checks
- Crash-Detection
- Auto-Restart bei Failures
- Health-Status-Tracking

### Public API (Rust)

```rust
pub struct HealthMonitor {
    // Internal state
}

impl HealthMonitor {
    /// Erstellt einen neuen HealthMonitor
    pub fn new(config: HealthMonitorConfig) -> Self;
    
    /// Startet Health-Monitoring für einen Service
    ///
    /// # Arguments
    /// * `service_name` - Name des Services
    /// * `strategy` - Health-Check-Strategy (HTTP, gRPC, Process)
    pub async fn start_monitoring(
        &mut self,
        service_name: &str,
        strategy: HealthCheckStrategy,
    ) -> Result<(), HealthMonitorError>;
    
    /// Stoppt Health-Monitoring für einen Service
    pub async fn stop_monitoring(&mut self, service_name: &str);
    
    /// Gibt aktuellen Health-Status für Service zurück
    ///
    /// # Returns
    /// * `Ok(ServiceHealth)` - Aktueller Health-Status
    /// * `Err(HealthMonitorError)` - Service nicht gefunden oder Fehler
    pub fn get_health(
        &self,
        service_name: &str,
    ) -> Result<ServiceHealth, HealthMonitorError>;
    
    /// Führt manuellen Health-Check für Service durch
    ///
    /// Führt sofort einen Health-Check durch (außerhalb des regulären Intervalls)
    pub async fn check_health_now(
        &mut self,
        service_name: &str,
    ) -> Result<ServiceHealth, HealthMonitorError>;
    
    /// Subscribt zu Health-Updates für Service
    ///
    /// Returns Stream von Health-Updates
    pub fn subscribe_health_updates(
        &self,
        service_name: &str,
    ) -> Result<HealthUpdateStream, HealthMonitorError>;
    
    /// Registriert Callback für Health-Status-Änderungen
    ///
    /// Callback wird aufgerufen wenn sich Health-Status ändert
    pub fn on_health_change<F>(&mut self, callback: F)
    where
        F: Fn(HealthStatusChange) + Send + Sync + 'static;
    
    /// Registriert Callback für Service-Crashes
    ///
    /// Callback wird aufgerufen wenn Service crashed
    pub fn on_service_crash<F>(&mut self, callback: F)
    where
        F: Fn(ServiceCrash) + Send + Sync + 'static;
}

/// Service-Health-Status
#[derive(Debug, Clone)]
pub struct ServiceHealth {
    /// Service-Name
    pub service_name: String,
    
    /// Health-Status
    pub status: HealthStatus,
    
    /// Health-Message (Details, z.B. Error-Message)
    pub message: String,
    
    /// Letzte Check-Zeit
    pub last_check: SystemTime,
    
    /// Nächste Check-Zeit
    pub next_check: SystemTime,
    
    /// Aufeinanderfolgende Failures
    pub consecutive_failures: u32,
    
    /// Health-Check-Strategy
    pub check_strategy: HealthCheckStrategy,
}

/// Health-Status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    /// Health-Status unbekannt (noch kein Check)
    Unknown,
    
    /// Service ist healthy
    Healthy,
    
    /// Service ist unhealthy
    Unhealthy,
    
    /// Health-Check läuft gerade
    Checking,
    
    /// Health-Check-Timeout
    Timeout,
}

/// Health-Check-Strategy
#[derive(Debug, Clone)]
pub enum HealthCheckStrategy {
    /// HTTP Health-Check (GET /health)
    Http {
        /// Health-Endpoint-URL
        url: String,
        
        /// Timeout für HTTP-Request
        timeout: Duration,
    },
    
    /// gRPC Health-Check (grpc.health.v1.Health)
    Grpc {
        /// gRPC-Server-Address
        address: String,
        
        /// Service-Name für gRPC Health-Check
        service_name: String,
        
        /// Timeout für gRPC-Call
        timeout: Duration,
    },
    
    /// Process-Monitoring (prüft nur ob Process läuft)
    Process,
}

/// Health-Status-Change
#[derive(Debug, Clone)]
pub struct HealthStatusChange {
    /// Service-Name
    pub service_name: String,
    
    /// Alter Status
    pub old_status: HealthStatus,
    
    /// Neuer Status
    pub new_status: HealthStatus,
    
    /// Timestamp
    pub timestamp: SystemTime,
}

/// Service-Crash
#[derive(Debug, Clone)]
pub struct ServiceCrash {
    /// Service-Name
    pub service_name: String,
    
    /// Exit-Code (falls verfügbar)
    pub exit_code: Option<i32>,
    
    /// Crash-Timestamp
    pub timestamp: SystemTime,
    
    /// Restart-Attempt-Count
    pub restart_attempt: u32,
}

/// Health-Update-Stream
pub type HealthUpdateStream = tokio::sync::broadcast::Receiver<ServiceHealth>;

/// Health-Monitor-Errors
#[derive(Debug, thiserror::Error)]
pub enum HealthMonitorError {
    #[error("Service not found: {0}")]
    ServiceNotFound(String),
    
    #[error("Health check failed: {0}")]
    HealthCheckFailed(String),
    
    #[error("Monitoring already started for service: {0}")]
    AlreadyMonitoring(String),
    
    #[error("Health check timeout: {0}")]
    Timeout(String),
    
    #[error("Invalid health check strategy: {0}")]
    InvalidStrategy(String),
}
```

### Configuration

```rust
pub struct HealthMonitorConfig {
    /// Check-Interval (default: 5s)
    pub check_interval: Duration,
    
    /// Auto-Restart enabled (default: true)
    pub auto_restart: bool,
    
    /// Max-Restart-Attempts (default: 3)
    pub max_restart_attempts: u32,
    
    /// Restart-Backoff (exponential, default: 1s)
    pub restart_backoff: Duration,
    
    /// Health-Check-Timeout (default: 5s)
    pub health_check_timeout: Duration,
    
    /// Consecutive-Failures vor Unhealthy (default: 3)
    pub consecutive_failures_threshold: u32,
}
```

---

## Skirnir (Service Registry) API

### Verantwortlichkeiten

Skirnir ist zuständig für:
- Lokale Service-Registry (alle laufenden Services)
- Service-Discovery
- Status-Tracking (State-Transitions)
- Metadata-Verwaltung

### Public API (Rust)

```rust
pub struct ServiceRegistry {
    // Internal state
}

impl ServiceRegistry {
    /// Erstellt eine neue ServiceRegistry
    pub fn new() -> Self;
    
    /// Registriert einen Service
    ///
    /// # Arguments
    /// * `service_name` - Name des Services
    /// * `metadata` - Service-Metadata
    ///
    /// # Returns
    /// * `Ok(())` - Service erfolgreich registriert
    /// * `Err(ServiceRegistryError)` - Service bereits registriert oder Fehler
    pub fn register_service(
        &mut self,
        service_name: String,
        metadata: ServiceMetadata,
    ) -> Result<(), ServiceRegistryError>;
    
    /// Deregistriert einen Service
    pub fn unregister_service(&mut self, service_name: &str) -> Result<(), ServiceRegistryError>;
    
    /// Gibt Service-Metadata zurück
    ///
    /// # Returns
    /// * `Some(&ServiceMetadata)` - Service gefunden
    /// * `None` - Service nicht gefunden
    pub fn get_service(&self, service_name: &str) -> Option<&ServiceMetadata>;
    
    /// Gibt alle Services zurück
    pub fn list_services(&self) -> Vec<&ServiceMetadata>;
    
    /// Gibt Services gefiltert nach State zurück
    pub fn list_services_by_state(&self, state: ServiceState) -> Vec<&ServiceMetadata>;
    
    /// Prüft ob Service registriert ist
    pub fn is_registered(&self, service_name: &str) -> bool;
    
    /// Aktualisiert Service-State
    ///
    /// # Arguments
    /// * `service_name` - Name des Services
    /// * `new_state` - Neuer State
    ///
    /// # Returns
    /// * `Ok(())` - State erfolgreich aktualisiert
    /// * `Err(ServiceRegistryError)` - Service nicht gefunden oder invalid transition
    pub fn update_state(
        &mut self,
        service_name: &str,
        new_state: ServiceState,
    ) -> Result<(), ServiceRegistryError>;
    
    /// Aktualisiert Resource-Usage für Service
    pub fn update_resource_usage(
        &mut self,
        service_name: &str,
        usage: ResourceUsage,
    ) -> Result<(), ServiceRegistryError>;
    
    /// Aktualisiert Health-Status für Service
    pub fn update_health_status(
        &mut self,
        service_name: &str,
        health: ServiceHealth,
    ) -> Result<(), ServiceRegistryError>;
    
    /// Registriert Callback für State-Changes
    pub fn on_state_change<F>(&mut self, callback: F)
    where
        F: Fn(StateChange) + Send + Sync + 'static;
    
    /// Gibt Service-Count-Statistics zurück
    pub fn get_statistics(&self) -> ServiceStatistics;
}

/// Service-Metadata
#[derive(Debug, Clone)]
pub struct ServiceMetadata {
    /// Service-Name
    pub service_name: String,
    
    /// Process-ID (0 wenn nicht running)
    pub process_id: u32,
    
    /// Start-Zeit
    pub start_time: Option<SystemTime>,
    
    /// Stop-Zeit
    pub stop_time: Option<SystemTime>,
    
    /// Aktueller State
    pub state: ServiceState,
    
    /// Resource-Usage (optional)
    pub resource_usage: Option<ResourceUsage>,
    
    /// Health-Status (optional)
    pub health_status: Option<ServiceHealth>,
    
    /// Error-Message (bei Crash)
    pub error_message: Option<String>,
    
    /// Restart-Count
    pub restart_count: u32,
    
    /// Custom-Metadata (für Platform-spezifische Daten)
    pub custom: HashMap<String, String>,
}

/// Service-State
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServiceState {
    /// Unknown State
    Unknown,
    
    /// Service startet
    Starting,
    
    /// Service läuft
    Running,
    
    /// Service stoppt
    Stopping,
    
    /// Service gestoppt
    Stopped,
    
    /// Service crashed
    Crashed,
    
    /// Service wird neu gestartet
    Restarting,
}

impl ServiceState {
    /// Prüft ob Transition valid ist
    pub fn is_valid_transition(&self, new_state: ServiceState) -> bool;
    
    /// Gibt alle möglichen Next-States zurück
    pub fn possible_next_states(&self) -> Vec<ServiceState>;
}

/// State-Change
#[derive(Debug, Clone)]
pub struct StateChange {
    /// Service-Name
    pub service_name: String,
    
    /// Alter State
    pub old_state: ServiceState,
    
    /// Neuer State
    pub new_state: ServiceState,
    
    /// Timestamp
    pub timestamp: SystemTime,
}

/// Service-Statistics
#[derive(Debug, Clone)]
pub struct ServiceStatistics {
    /// Gesamt-Anzahl Services
    pub total_count: usize,
    
    /// Anzahl running Services
    pub running_count: usize,
    
    /// Anzahl stopped Services
    pub stopped_count: usize,
    
    /// Anzahl crashed Services
    pub crashed_count: usize,
    
    /// Anzahl starting Services
    pub starting_count: usize,
    
    /// Anzahl stopping Services
    pub stopping_count: usize,
}

/// Service-Registry-Errors
#[derive(Debug, thiserror::Error)]
pub enum ServiceRegistryError {
    #[error("Service not found: {0}")]
    ServiceNotFound(String),
    
    #[error("Service already registered: {0}")]
    AlreadyRegistered(String),
    
    #[error("Invalid state transition from {0:?} to {1:?}")]
    InvalidTransition(ServiceState, ServiceState),
    
    #[error("Registry is locked")]
    RegistryLocked,
}
```

---

## Integration zwischen Servants

### Servant-Coordination

Die vier Servants arbeiten zusammen, um Service-Management zu ermöglichen:

```rust
/// Gladsheim koordiniert alle Servants
pub struct Gladsheim {
    thjalfi: ServiceLoader,
    byggvir: ResourceManager,
    roskva: HealthMonitor,
    skirnir: ServiceRegistry,
    config: GladsheimConfig,
}

impl Gladsheim {
    /// Erstellt neue Gladsheim-Instanz mit allen Servants
    pub fn new(config: GladsheimConfig) -> Self;
    
    /// Startet Gladsheim (startet alle Monitoring-Loops)
    pub async fn start(&mut self) -> Result<(), GladsheimError>;
    
    /// Stoppt Gladsheim gracefully
    pub async fn shutdown(&mut self) -> Result<(), GladsheimError>;
}

/// Beispiel: Service-Start-Workflow
async fn start_service_workflow(
    gladsheim: &mut Gladsheim,
    service_name: &str,
    options: StartOptions,
) -> Result<ServiceStatus, GladsheimError> {
    // 1. Skirnir: Prüfen ob Service bereits läuft
    if gladsheim.skirnir.is_registered(service_name) {
        let metadata = gladsheim.skirnir.get_service(service_name).unwrap();
        if metadata.state == ServiceState::Running {
            return Err(GladsheimError::AlreadyRunning(service_name.to_string()));
        }
    }
    
    // 2. Thjalfi: Service starten
    let process = gladsheim.thjalfi.start_service(service_name, options).await?;
    
    // 3. Skirnir: Service registrieren
    gladsheim.skirnir.register_service(
        service_name.to_string(),
        ServiceMetadata {
            service_name: service_name.to_string(),
            process_id: process.pid,
            start_time: Some(process.start_time),
            state: ServiceState::Running,
            ..Default::default()
        },
    )?;
    
    // 4. Byggvir: Resource-Monitoring starten (implizit)
    // Byggvir monitort automatisch alle registrierten Services
    
    // 5. Roskva: Health-Monitoring starten
    gladsheim.roskva.start_monitoring(
        service_name,
        HealthCheckStrategy::Http {
            url: format!("http://localhost:{}/health", get_service_port(service_name)),
            timeout: Duration::from_secs(5),
        },
    ).await?;
    
    // 6. ServiceStatus zurückgeben
    Ok(ServiceStatus {
        service_name: service_name.to_string(),
        state: ServiceState::Running,
        process_id: process.pid as i32,
        start_time_unix: process.start_time
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
        ..Default::default()
    })
}
```

### Event-Flow zwischen Servants

```
Service-Start:
  Thjalfi → Skirnir: register_service()
  Thjalfi → Byggvir: (automatisch via Registry)
  Thjalfi → Roskva: start_monitoring()

Resource-Monitoring:
  Byggvir → Skirnir: update_resource_usage()
  
Health-Monitoring:
  Roskva → Skirnir: update_health_status()
  Roskva → Thjalfi: restart_service() (bei Auto-Restart)

Service-Crash:
  Roskva: detects crash
  Roskva → Skirnir: update_state(Crashed)
  Roskva → Thjalfi: restart_service() (falls auto-restart enabled)
```

---

## Performance-Anforderungen

### Latency-Requirements

| Operation | Target | Max |
|-----------|--------|-----|
| start_service() | < 500ms | 1s |
| stop_service() | < 100ms | 500ms |
| get_service_status() | < 10ms | 20ms |
| list_services() | < 20ms | 50ms |
| get_health() | < 10ms | 20ms |
| get_resource_usage() | < 10ms | 20ms |

### Memory-Requirements

| Component | Target | Max |
|-----------|--------|-----|
| Gladsheim (gesamt) | < 50MB | 100MB |
| Thjalfi | < 10MB | 20MB |
| Byggvir | < 15MB | 30MB |
| Roskva | < 15MB | 30MB |
| Skirnir | < 10MB | 20MB |

### CPU-Requirements

| Component | Target | Max |
|-----------|--------|-----|
| Gladsheim (idle) | < 1% | 2% |
| Health-Monitoring | < 1% | 2% |
| Resource-Monitoring | < 1% | 2% |

---

## Error-Handling

Alle Servants verwenden Result-Types für Fehlerbehandlung:

```rust
pub type Result<T> = std::result::Result<T, GladsheimError>;

#[derive(Debug, thiserror::Error)]
pub enum GladsheimError {
    #[error("Service loader error: {0}")]
    ServiceLoader(#[from] ServiceLoaderError),
    
    #[error("Resource manager error: {0}")]
    ResourceManager(#[from] ResourceManagerError),
    
    #[error("Health monitor error: {0}")]
    HealthMonitor(#[from] HealthMonitorError),
    
    #[error("Service registry error: {0}")]
    ServiceRegistry(#[from] ServiceRegistryError),
    
    #[error("Service already running: {0}")]
    AlreadyRunning(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("gRPC error: {0}")]
    Grpc(#[from] tonic::Status),
}
```

---

## Thread-Safety

Alle Public APIs sind thread-safe und können von mehreren Threads gleichzeitig aufgerufen werden:

```rust
// Alle Servants verwenden Arc<RwLock<>> oder Arc<Mutex<>> intern
pub struct Gladsheim {
    thjalfi: Arc<RwLock<ServiceLoader>>,
    byggvir: Arc<RwLock<ResourceManager>>,
    roskva: Arc<RwLock<HealthMonitor>>,
    skirnir: Arc<RwLock<ServiceRegistry>>,
    config: Arc<GladsheimConfig>,
}
```

Alle Public-Methods sind `Send + Sync`.
