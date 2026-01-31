# IMPLEMENTATION_PLAN - Gladsheim (Service Manager & Runtime Manager)

## Übersicht

Dieser Plan beschreibt die kleinstmöglichen Schritte zur Implementierung von Gladsheim - dem Service-Manager und Runtime-Manager für alle Plattformen (Midgard, Alfheim, Asgard, Ragnarok). Gladsheim verwaltet Service-Lifecycle, Ressourcen und Health-Status mit vier mythologischen Dienern als Sub-Komponenten.

**Mythologische Bedeutung**: Gladsheim (Gladsheimr) - "Die goldene Halle der Freude" - Ort des Rates der Götter, repräsentiert den RAM.

**Programmiersprache**: Rust (100%)

**Komponenten**: Thjalfi (Service Loader), Byggvir (Resource Manager), Roskva (Health Monitor), Skirnir (Service Registry)

## Offene Fragen (USER-INPUT ERFORDERLICH)

### gRPC-Framework
❓ **FRAGE AN USER**: Welches gRPC-Framework soll verwendet werden?
- Option A: `tonic` mit `prost` (moderne, idiomatische Rust-Implementierung, empfohlen)
- Option B: `grpc-rs` (klassische gRPC-Bindings)

**Auswirkung**: Beeinflusst die gRPC-Implementierung und Code-Generierung.

### Health-Check-Strategie
❓ **FRAGE AN USER**: Welche Health-Check-Strategie bevorzugen?
- Option A: HTTP Health Endpoints (GET /health)
- Option B: gRPC Health Check Protocol (grpc.health.v1.Health)
- Option C: Beide Strategien unterstützen

**Auswirkung**: Beeinflusst die Health-Check-Implementierung in Roskva.

### Process-Management
❓ **FRAGE AN USER**: Welches Process-Management bevorzugen?
- Option A: `tokio::process` (async, gut integriert mit tokio)
- Option B: `std::process` (sync, einfacher)
- Option C: `nix` crate (Unix-spezifisch, mehr Kontrolle)

**Auswirkung**: Beeinflusst die Process-Management-Implementierung in Thjalfi.

---

## Phase 1: Projekt-Setup & Grundstruktur

### 1.1 Projekt-Initialisierung

**Abhängigkeiten**: Keine
**Erforderliche USER-Eingaben**: gRPC-Framework

#### 1.1.1 Cargo-Projekt erstellen
- [ ] `Cargo.toml` für Gladsheim erstellen
- [ ] Basis-Dependencies definieren
  - Async Runtime (tokio mit full features)
  - gRPC (tonic, prost, prost-types)
  - Process Management (tokio::process)
  - Resource Monitoring (sysinfo)
  - Serialization (serde, serde_json)
  - Logging (tracing, tracing-subscriber)
  - Error-Handling (anyhow, thiserror)
  - HTTP Client für Health Checks (reqwest)
- [ ] `.gitignore` erstellen
- [ ] `README.md`, `AGENTS.md`, `IMPLEMENTATION_PLAN.md` erstellen

#### 1.1.2 Verzeichnisstruktur erstellen
- [ ] `src/lib.rs` erstellen
- [ ] `src/gladsheim.rs` für Haupt-Gladsheim-Struct erstellen
- [ ] `src/thjalfi/` für Service Loader erstellen
  - `mod.rs`, `loader.rs`, `process.rs`
- [ ] `src/byggvir/` für Resource Manager erstellen
  - `mod.rs`, `resources.rs`, `limits.rs`
- [ ] `src/roskva/` für Health Monitor erstellen
  - `mod.rs`, `health.rs`, `monitoring.rs`
- [ ] `src/skirnir/` für Service Registry erstellen
  - `mod.rs`, `registry.rs`, `discovery.rs`
- [ ] `src/proto/` für Proto Definitions erstellen
- [ ] `src/grpc/` für gRPC Server Implementation erstellen
  - `mod.rs`, `server.rs`
- [ ] `src/utils/` für Utilities erstellen
  - `config.rs`, `errors.rs`
- [ ] `tests/` für Tests erstellen
  - `integration/`, `unit/`

#### 1.1.3 Build-System einrichten
- [ ] Build-Scripts in `Cargo.toml` definieren
- [ ] Protobuf-Code-Generierung konfigurieren (tonic-build)
- [ ] Proto-Build-Script erstellen (`build.rs`)
- [ ] Cargo-Features definieren (z.B. `http-health`, `grpc-health`)

### 1.2 Test-Infrastruktur

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.2.1 Container-Setup für Tests
- [ ] `Dockerfile` für Test-Umgebung erstellen
  - Rust-Toolchain
  - System-Dependencies (für sysinfo)
  - Test-Services (Mock-Services)
- [ ] Docker Compose für Test-Services konfigurieren
  - Mock-Service-Container (simulieren Thor, Freki, etc.)
  - Mock-Heimdall-Service (für Authorization-Tests)
- [ ] Test-Container-Startup-Scripts erstellen
  - Container-Startup
  - Health-Check-Wait
  - Test-Ausführung
- [ ] **WICHTIG**: Alle Tests müssen in Containern laufen - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren

#### 1.2.2 Test-Framework konfigurieren
- [ ] Test-Dependencies hinzufügen (tokio-test, mockall, wiremock)
- [ ] Test-Utilities und Helpers erstellen
  - Mock-Service-Helper (startet Mock-Services)
  - Assertions für Service-Status
  - Assertions für Resource-Usage
- [ ] Mock-Setup für Services (Heimdall, Thor, etc.)
  - Mock-gRPC-Services
  - Mock-Health-Endpoints

#### 1.2.3 CI/CD-Pipeline
- [ ] GitHub Actions / GitLab CI Workflow erstellen
- [ ] Automatische Test-Ausführung bei Commits konfigurieren
- [ ] Code-Coverage-Reporting einrichten (cargo-tarpaulin oder cargo-llvm-cov)
- [ ] Linting und Formatting (cargo clippy, cargo fmt)
- [ ] Container-Build in CI/CD integrieren

### 1.3 Projekt-Konfiguration

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.3.1 Settings-System Design
- [ ] Settings-Schema definieren (JSON)
- [ ] Settings-Struktur entwerfen
  - gRPC-Konfiguration (host, port)
  - Resource-Limits (max_services, default_memory_mb, default_cpu_percent)
  - Health-Monitoring (check_interval_ms, auto_restart, max_restart_attempts)
  - Service-Loader (startup_timeout_ms, shutdown_timeout_ms)
- [ ] Platform-spezifische Settings dokumentieren
  - Midgard, Alfheim, Asgard, Ragnarok

#### 1.3.2 Settings-Validierung
- [ ] Rust-Structs für Settings definieren
  - `GladsheimConfig`
  - `ResourceLimitsConfig`
  - `HealthMonitoringConfig`
  - `ServiceLoaderConfig`
- [ ] Tests für Settings-Validierung schreiben
- [ ] Settings-Validator implementieren (TDD)
  - Schema-Validierung
  - Range-Checks (ports, timeouts, percentages)
  - Platform-spezifische Validierung
- [ ] Tests ausführen und bestehen

#### 1.3.3 Settings-Loader
- [ ] Tests für Settings-Loader schreiben
- [ ] Settings-Loader implementieren (TDD)
  - JSON-Parsing (serde_json)
  - Environment-Variable-Override
  - Default-Settings für Platforms
- [ ] Platform-Detection implementieren (detect Midgard/Alfheim/Asgard/Ragnarok)
- [ ] Hot-Reload-Mechanismus implementieren (TDD)
  - File-Watcher für Settings-Datei (notify crate)
  - Settings-Reload ohne Restart
- [ ] Tests ausführen und bestehen

---

## Phase 2: Proto-Definitionen & gRPC-Setup

### 2.1 Protobuf-Definitionen

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 2.1.1 Proto-File erstellen
- [ ] `gladsheim.proto` erstellen
- [ ] Package und Imports definieren
- [ ] Service-Definition erstellen (`GladsheimService`)
  - `StartService`
  - `StopService`
  - `RestartService`
  - `GetServiceStatus`
  - `ListServices`
  - `GetServiceHealth`
  - `SubscribeServiceHealth` (streaming)
  - `GetResourceUsage`
  - `SetResourceLimits`
  - `GetResourceLimits`

#### 2.1.2 Message-Definitionen erstellen
- [ ] Request-Messages definieren
  - `StartServiceRequest`
  - `StopServiceRequest`
  - `RestartServiceRequest`
  - `ServiceStatusRequest`
  - `ListServicesRequest`
  - `ServiceHealthRequest`
  - `HealthSubscribeRequest`
  - `ResourceUsageRequest`
  - `ResourceLimitsRequest`
  - `ServiceRequest`
- [ ] Response-Messages definieren
  - `ServiceStatus`
  - `ServiceList`
  - `ServiceHealth`
  - `HealthUpdate`
  - `ResourceUsage`
  - `ResourceLimits`
- [ ] Enum-Definitionen erstellen
  - `ServiceState` (UNKNOWN, STARTING, RUNNING, STOPPING, STOPPED, CRASHED)
  - `HealthStatus` (HEALTHY, UNHEALTHY, UNKNOWN_HEALTH)

#### 2.1.3 Proto-Build testen
- [ ] `build.rs` für Proto-Compilation erstellen
- [ ] Proto-Compilation testen (cargo build)
- [ ] Generated Code überprüfen (target/debug/build/...)
- [ ] Proto-File dokumentieren (Kommentare)

### 2.2 gRPC-Server-Setup

**Abhängigkeiten**: 2.1 (Proto-Definitionen)

#### 2.2.1 gRPC-Server-Grundstruktur
- [ ] Tests für gRPC-Server-Startup schreiben
- [ ] `GladsheimServer` struct erstellen
- [ ] gRPC-Server-Implementation-Skeleton erstellen
  - Trait-Implementation für `GladsheimService`
  - Alle RPC-Methods als Stubs
- [ ] Server-Startup-Logik implementieren (TDD)
  - Server-Bind zu localhost:port
  - Server-Lifecycle-Management
  - Graceful-Shutdown
- [ ] Tests ausführen und bestehen

#### 2.2.2 gRPC-Client für Tests
- [ ] Test-gRPC-Client erstellen
- [ ] Client-Verbindung testen
- [ ] Stub-Calls testen (alle RPC-Methods)

---

## Phase 3: Thjalfi (Service Loader) Implementierung

### 3.1 Process-Management

**Abhängigkeiten**: 2.2 (gRPC-Server-Setup)

#### 3.1.1 Process-Wrapper
- [ ] Tests für Process-Wrapper schreiben
  - Process-Start
  - Process-Stop (graceful)
  - Process-Force-Kill
  - Process-Status-Query
- [ ] `ServiceProcess` struct erstellen
  - Process-Handle (tokio::process::Child)
  - Process-ID
  - Start-Time
  - Status
- [ ] Process-Start implementieren (TDD)
  - `tokio::process::Command::spawn`
  - Environment-Variables
  - Working-Directory
  - Startup-Validation
- [ ] Process-Stop implementieren (TDD)
  - Graceful-Shutdown (SIGTERM)
  - Timeout-Handling
  - Force-Kill (SIGKILL) als Fallback
- [ ] Tests ausführen und bestehen

#### 3.1.2 Service-Loader
- [ ] Tests für Service-Loader schreiben
  - Service-Start mit verschiedenen Parametern
  - Service-Stop (graceful + force)
  - Multiple-Services parallel
  - Startup-Timeout
  - Shutdown-Timeout
- [ ] `ServiceLoader` (Thjalfi) struct erstellen
- [ ] Service-Start-Logik implementieren (TDD)
  - Service-Path-Resolution
  - Process-Spawn
  - Startup-Validation
  - Timeout-Enforcement
- [ ] Service-Stop-Logik implementieren (TDD)
  - Graceful-Shutdown-Request
  - Timeout-Wait
  - Force-Kill bei Timeout
- [ ] Service-Restart implementieren (Stop + Start)
- [ ] Tests ausführen und bestehen

#### 3.1.3 Heimdall-Integration
- [ ] Tests für Heimdall-Authorization schreiben
  - Authorized-Service-Start
  - Unauthorized-Service-Start (rejected)
  - Token-Validation
- [ ] Heimdall-Client-Integration implementieren (TDD)
  - gRPC-Call zu Heimdall
  - Token-Validation
  - Service-Whitelist-Check
- [ ] Authorization-Check in Service-Start integrieren
- [ ] Tests ausführen und bestehen

### 3.2 gRPC-Endpoint-Implementation (Thjalfi)

**Abhängigkeiten**: 3.1 (Process-Management)

#### 3.2.1 StartService RPC
- [ ] Tests für StartService schreiben
- [ ] StartService-Handler implementieren (TDD)
  - Request-Validation
  - Heimdall-Authorization
  - Thjalfi.start_service()
  - Response-Building
- [ ] Error-Handling implementieren
- [ ] Tests ausführen und bestehen

#### 3.2.2 StopService RPC
- [ ] Tests für StopService schreiben
- [ ] StopService-Handler implementieren (TDD)
  - Request-Validation
  - Thjalfi.stop_service()
  - Response-Building
- [ ] Error-Handling implementieren
- [ ] Tests ausführen und bestehen

#### 3.2.3 RestartService RPC
- [ ] Tests für RestartService schreiben
- [ ] RestartService-Handler implementieren (TDD)
  - Request-Validation
  - Thjalfi.restart_service()
  - Response-Building
- [ ] Error-Handling implementieren
- [ ] Tests ausführen und bestehen

---

## Phase 4: Byggvir (Resource Manager) Implementierung

### 4.1 Resource-Monitoring

**Abhängigkeiten**: 3.1 (Process-Management)

#### 4.1.1 System-Resource-Monitor
- [ ] Tests für System-Resource-Monitor schreiben
  - Gesamt-RAM-Usage
  - Gesamt-CPU-Usage
  - Process-RAM-Usage
  - Process-CPU-Usage
- [ ] `SystemResourceMonitor` struct erstellen
  - sysinfo::System-Integration
  - Refresh-Strategie
- [ ] System-Monitoring implementieren (TDD)
  - RAM-Usage-Query
  - CPU-Usage-Query
  - Process-Monitoring
- [ ] Tests ausführen und bestehen

#### 4.1.2 Service-Resource-Tracker
- [ ] Tests für Service-Resource-Tracker schreiben
  - Resource-Tracking pro Service
  - Resource-History
  - Resource-Aggregation
- [ ] `ServiceResourceTracker` struct erstellen
- [ ] Per-Service-Resource-Tracking implementieren (TDD)
  - RAM-Usage pro Service
  - CPU-Usage pro Service
  - Resource-History (letzte N Werte)
- [ ] Tests ausführen und bestehen

### 4.2 Resource-Limit-Enforcement

**Abhängigkeiten**: 4.1 (Resource-Monitoring)

#### 4.2.1 Limit-Checker
- [ ] Tests für Limit-Checker schreiben
  - RAM-Limit-Check
  - CPU-Limit-Check
  - Multiple-Services
  - Platform-spezifische Limits
- [ ] `ResourceLimitChecker` struct erstellen
- [ ] Limit-Check-Logik implementieren (TDD)
  - RAM-Limit-Überprüfung
  - CPU-Limit-Überprüfung
  - Warning-Thresholds
- [ ] Platform-spezifische Limits implementieren
  - Midgard: moderate limits
  - Alfheim: strict limits, battery-aware
  - Asgard: relaxed limits
  - Ragnarok: minimal limits
- [ ] Tests ausführen und bestehen

#### 4.2.2 Limit-Enforcement-Actions
- [ ] Tests für Enforcement-Actions schreiben
  - Service-Warning bei Limit-Annäherung
  - Service-Throttling bei Limit-Überschreitung
  - Service-Stop bei kritischer Überschreitung
- [ ] `ResourceEnforcer` struct erstellen
- [ ] Enforcement-Actions implementieren (TDD)
  - Warning-Logging
  - Alert zu Odin (gRPC-Call)
  - Service-Stop bei kritischen Limits
- [ ] Tests ausführen und bestehen

### 4.3 Byggvir-Integration

**Abhängigkeiten**: 4.2 (Resource-Limit-Enforcement)

#### 4.3.1 Resource-Manager (Byggvir)
- [ ] Tests für Resource-Manager schreiben
- [ ] `ResourceManager` (Byggvir) struct erstellen
  - System-Monitor
  - Service-Tracker
  - Limit-Checker
  - Enforcer
- [ ] Resource-Manager-Logik implementieren (TDD)
  - Kontinuierliches Monitoring
  - Limit-Checks
  - Enforcement-Actions
- [ ] Tests ausführen und bestehen

#### 4.3.2 gRPC-Endpoints (Byggvir)
- [ ] Tests für GetResourceUsage schreiben
- [ ] GetResourceUsage-Handler implementieren (TDD)
- [ ] Tests für SetResourceLimits schreiben
- [ ] SetResourceLimits-Handler implementieren (TDD)
- [ ] Tests für GetResourceLimits schreiben
- [ ] GetResourceLimits-Handler implementieren (TDD)
- [ ] Tests ausführen und bestehen

---

## Phase 5: Roskva (Health Monitor) Implementierung

### 5.1 Health-Check-Strategies

**Abhängigkeiten**: 3.1 (Process-Management)

#### 5.1.1 HTTP-Health-Checker
- [ ] Tests für HTTP-Health-Checker schreiben
  - HTTP GET /health
  - Status-Code-Validierung (200 = healthy)
  - Timeout-Handling
  - Connection-Errors
- [ ] `HttpHealthChecker` struct erstellen
- [ ] HTTP-Health-Check implementieren (TDD)
  - reqwest-Client
  - GET-Request zu /health
  - Response-Parsing
  - Timeout-Enforcement
- [ ] Tests ausführen und bestehen

#### 5.1.2 gRPC-Health-Checker
- [ ] Tests für gRPC-Health-Checker schreiben
  - grpc.health.v1.Health/Check
  - Response-Validation
  - Timeout-Handling
- [ ] `GrpcHealthChecker` struct erstellen
- [ ] gRPC-Health-Check implementieren (TDD)
  - gRPC-Health-Client
  - Health-Check-Request
  - Response-Parsing
  - Timeout-Enforcement
- [ ] Tests ausführen und bestehen

#### 5.1.3 Process-Health-Checker
- [ ] Tests für Process-Health-Checker schreiben
  - Process-Alive-Check
  - Process-Zombie-Detection
- [ ] `ProcessHealthChecker` struct erstellen
- [ ] Process-Health-Check implementieren (TDD)
  - Process-Status-Query
  - Zombie-Detection
- [ ] Tests ausführen und bestehen

### 5.2 Health-Monitoring-Loop

**Abhängigkeiten**: 5.1 (Health-Check-Strategies)

#### 5.2.1 Health-Monitor
- [ ] Tests für Health-Monitor schreiben
  - Periodisches Health-Checking
  - Multiple-Services parallel
  - Health-Status-Updates
  - Check-Interval-Configuration
- [ ] `HealthMonitor` (Roskva) struct erstellen
  - Health-Checker (HTTP, gRPC, Process)
  - Health-Status-Cache
  - Monitoring-Loop
- [ ] Health-Monitoring-Loop implementieren (TDD)
  - Periodische Health-Checks
  - Parallel-Checks für multiple Services
  - Health-Status-Tracking
- [ ] Tests ausführen und bestehen

#### 5.2.2 Crash-Detection
- [ ] Tests für Crash-Detection schreiben
  - Service-Crash-Erkennung
  - Crash-Reporting
  - Crash-History
- [ ] Crash-Detection implementieren (TDD)
  - Process-Exit-Monitoring
  - Health-Check-Failures
  - Crash-Logging
- [ ] Tests ausführen und bestehen

### 5.3 Auto-Restart-Mechanismus

**Abhängigkeiten**: 5.2 (Health-Monitoring-Loop)

#### 5.3.1 Restart-Policy
- [ ] Tests für Restart-Policy schreiben
  - Auto-Restart enabled/disabled
  - Max-Restart-Attempts
  - Restart-Backoff
  - No-Restart für bestimmte Services
- [ ] `RestartPolicy` struct erstellen
- [ ] Restart-Policy-Logik implementieren (TDD)
  - Policy-Evaluation
  - Attempt-Tracking
  - Backoff-Calculation (exponential)
- [ ] Tests ausführen und bestehen

#### 5.3.2 Auto-Restart-Implementation
- [ ] Tests für Auto-Restart schreiben
  - Restart bei Crash
  - Max-Attempts-Enforcement
  - Backoff-Delays
  - Restart-Failure-Handling
- [ ] Auto-Restart implementieren (TDD)
  - Crash-Event-Handling
  - Thjalfi.restart_service() aufrufen
  - Attempt-Tracking
  - Backoff-Delays
- [ ] Tests ausführen und bestehen

### 5.4 Roskva-Integration

**Abhängigkeiten**: 5.3 (Auto-Restart-Mechanismus)

#### 5.4.1 gRPC-Endpoints (Roskva)
- [ ] Tests für GetServiceHealth schreiben
- [ ] GetServiceHealth-Handler implementieren (TDD)
- [ ] Tests für SubscribeServiceHealth schreiben (streaming)
- [ ] SubscribeServiceHealth-Handler implementieren (TDD)
  - Server-Streaming
  - Health-Updates pushen
  - Client-Disconnect-Handling
- [ ] Tests ausführen und bestehen

---

## Phase 6: Skirnir (Service Registry) Implementierung

### 6.1 Service-Registry

**Abhängigkeiten**: 3.1 (Process-Management)

#### 6.1.1 Registry-Datenstruktur
- [ ] Tests für Registry-Datenstruktur schreiben
  - Service-Registration
  - Service-Lookup (by name)
  - Service-List
  - Service-Unregistration
  - Thread-Safety
- [ ] `ServiceRegistry` (Skirnir) struct erstellen
  - HashMap<ServiceName, ServiceMetadata>
  - Arc<RwLock<>> für Thread-Safety
- [ ] Registry-Operations implementieren (TDD)
  - Register-Service
  - Unregister-Service
  - Get-Service
  - List-Services
- [ ] Tests ausführen und bestehen

#### 6.1.2 Service-Metadata
- [ ] Tests für Service-Metadata schreiben
- [ ] `ServiceMetadata` struct erstellen
  - service_name
  - process_id
  - start_time
  - state (ServiceState enum)
  - resource_usage
  - health_status
- [ ] Metadata-Update-Logik implementieren (TDD)
  - Status-Updates
  - Resource-Usage-Updates
  - Health-Status-Updates
- [ ] Tests ausführen und bestehen

### 6.2 Status-Tracking

**Abhängigkeiten**: 6.1 (Service-Registry)

#### 6.2.1 State-Transitions
- [ ] Tests für State-Transitions schreiben
  - STARTING → RUNNING
  - RUNNING → STOPPING
  - STOPPING → STOPPED
  - RUNNING → CRASHED
  - STOPPED → STARTING (restart)
  - Invalid-Transitions (should fail)
- [ ] State-Transition-Logik implementieren (TDD)
  - Valid-Transition-Check
  - State-Update
  - Transition-Logging
- [ ] Tests ausführen und bestehen

#### 6.2.2 Status-Query-Optimization
- [ ] Tests für Status-Queries schreiben
  - GetServiceStatus (< 10ms)
  - ListServices (< 20ms)
  - Concurrent-Queries
- [ ] Query-Optimization implementieren
  - In-Memory-Cache (Arc<RwLock<HashMap>>)
  - Read-Optimized (RwLock für mehr read-throughput)
- [ ] Performance-Tests ausführen und bestehen

### 6.3 Skirnir-Integration

**Abhängigkeiten**: 6.2 (Status-Tracking)

#### 6.3.1 Registry-Integration mit anderen Servants
- [ ] Tests für Registry-Updates schreiben
  - Thjalfi → Skirnir (Service-Start/Stop)
  - Byggvir → Skirnir (Resource-Updates)
  - Roskva → Skirnir (Health-Updates)
- [ ] Registry-Update-Integration implementieren (TDD)
  - Event-Callbacks von anderen Servants
  - Async-Updates
  - Thread-Safe-Updates
- [ ] Tests ausführen und bestehen

#### 6.3.2 gRPC-Endpoints (Skirnir)
- [ ] Tests für GetServiceStatus schreiben
- [ ] GetServiceStatus-Handler implementieren (TDD)
- [ ] Tests für ListServices schreiben
- [ ] ListServices-Handler implementieren (TDD)
- [ ] Tests ausführen und bestehen

---

## Phase 7: Gladsheim-Integration & Orchestration

### 7.1 Hauptkomponente Gladsheim

**Abhängigkeiten**: 3.2, 4.3, 5.4, 6.3 (alle Servants implementiert)

#### 7.1.1 Gladsheim-Struct
- [ ] Tests für Gladsheim-Struct schreiben
  - Initialization
  - Servant-Coordination
  - Shutdown
- [ ] `Gladsheim` struct erstellen
  - thjalfi: ServiceLoader
  - byggvir: ResourceManager
  - roskva: HealthMonitor
  - skirnir: ServiceRegistry
  - config: GladsheimConfig
- [ ] Gladsheim-Initialization implementieren (TDD)
  - Config-Loading
  - Servant-Initialization
  - gRPC-Server-Start
- [ ] Tests ausführen und bestehen

#### 7.1.2 Servant-Coordination
- [ ] Tests für Servant-Coordination schreiben
  - Service-Start-Workflow (alle Servants)
  - Service-Stop-Workflow (alle Servants)
  - Concurrent-Operations
- [ ] Coordination-Logik implementieren (TDD)
  - Thjalfi ↔ Skirnir (Registry-Updates)
  - Byggvir ↔ Skirnir (Resource-Updates)
  - Roskva ↔ Skirnir (Health-Updates)
  - Roskva ↔ Thjalfi (Auto-Restart)
- [ ] Tests ausführen und bestehen

### 7.2 End-to-End-Tests

**Abhängigkeiten**: 7.1 (Gladsheim-Integration)

#### 7.2.1 Complete-Workflow-Tests
- [ ] Test: Service-Start → Health-Check → Resource-Monitor → Stop
- [ ] Test: Service-Crash → Crash-Detection → Auto-Restart
- [ ] Test: Resource-Limit-Überschreitung → Warning → Enforcement
- [ ] Test: Multiple-Services parallel
- [ ] Test: Service-Lifecycle mit Odin-Integration
- [ ] Alle E2E-Tests ausführen und bestehen

#### 7.2.2 Performance-Tests
- [ ] Test: Service-Start-Latency (< 500ms)
- [ ] Test: Status-Query-Latency (< 10ms)
- [ ] Test: Health-Check-Overhead (< 1% CPU)
- [ ] Test: Memory-Overhead (< 50MB)
- [ ] Test: Concurrent-Operations (100+ requests)
- [ ] Performance-Tests ausführen und bestehen

#### 7.2.3 Failure-Scenario-Tests
- [ ] Test: Service-Startup-Failure
- [ ] Test: Service-Crash (unexpected)
- [ ] Test: Service-Timeout (startup/shutdown)
- [ ] Test: Resource-Exhaustion
- [ ] Test: Health-Check-Failures
- [ ] Test: Cascading-Failures (multiple services)
- [ ] Failure-Tests ausführen und bestehen

---

## Phase 8: Platform-Integration & Migration

### 8.1 Platform-Integration-Planung

**Abhängigkeiten**: 7.2 (End-to-End-Tests)

#### 8.1.1 Migration-Strategie dokumentieren
- [ ] Migration-Guide für Midgard schreiben
  - Service-Lifecycle-Manager → Gladsheim
  - Code-Änderungen
  - Testing-Strategie
- [ ] Migration-Guide für Alfheim schreiben
- [ ] Migration-Guide für Asgard schreiben
- [ ] Migration-Guide für Ragnarok schreiben
- [ ] Migration-Impact auf Odin dokumentieren

#### 8.1.2 Platform-spezifische Konfigurationen
- [ ] Midgard-Gladsheim-Config erstellen
- [ ] Alfheim-Gladsheim-Config erstellen (battery-aware)
- [ ] Asgard-Gladsheim-Config erstellen (high-capacity)
- [ ] Ragnarok-Gladsheim-Config erstellen (minimal)

### 8.2 Integration-Library

**Abhängigkeiten**: 8.1 (Platform-Integration-Planung)

#### 8.2.1 Gladsheim-Client-Library
- [ ] Tests für Gladsheim-Client schreiben
- [ ] `GladsheimClient` erstellen (wrapper um gRPC-Client)
- [ ] Client-API implementieren (TDD)
  - start_service()
  - stop_service()
  - restart_service()
  - get_service_status()
  - list_services()
  - get_service_health()
  - subscribe_service_health()
- [ ] Client-Error-Handling implementieren
- [ ] Tests ausführen und bestehen

#### 8.2.2 Platform-Integration-Helpers
- [ ] Tests für Platform-Integration-Helpers schreiben
- [ ] Helper-Functions erstellen
  - gladsheim_init() - für Platform-Startup
  - gladsheim_shutdown() - für Platform-Shutdown
  - platform_detect() - erkennt Platform (Midgard/Alfheim/Asgard/Ragnarok)
- [ ] Tests ausführen und bestehen

### 8.3 Odin-Integration

**Abhängigkeiten**: 8.2 (Integration-Library)

#### 8.3.1 Odin-Gladsheim-Client
- [ ] Odin-spezifische Client-Wrapper erstellen
- [ ] Service-Management-Integration in Odin
  - Odin nutzt GladsheimClient
  - Service-Start-Requests
  - Service-Stop-Requests
- [ ] Integration-Tests (Odin ↔ Gladsheim)

#### 8.3.2 Service-Discovery-Integration
- [ ] Einherjar-Protocol ↔ Skirnir Integration
  - Skirnir kann Einherjar-Daten cachen
  - Einherjar-Capabilities in Registry
- [ ] Integration-Tests

---

## Phase 9: Documentation & Testing

### 9.1 API-Documentation

**Abhängigkeiten**: 7.1 (Gladsheim-Integration)

#### 9.1.1 gRPC-API-Documentation
- [ ] Proto-File vollständig dokumentieren (Kommentare)
- [ ] API-Usage-Examples erstellen
  - Start-Service-Example
  - Stop-Service-Example
  - Health-Monitoring-Example
  - Resource-Management-Example
- [ ] Error-Codes dokumentieren

#### 9.1.2 Rust-API-Documentation
- [ ] Rustdoc-Kommentare für alle public APIs
- [ ] Module-Level-Documentation
- [ ] Struct-Documentation
- [ ] Function-Documentation mit Examples
- [ ] cargo doc generieren und prüfen

### 9.2 User-Documentation

**Abhängigkeiten**: 8.3 (Odin-Integration)

#### 9.2.1 Getting-Started-Guide
- [ ] Installation-Guide schreiben
- [ ] Configuration-Guide schreiben
  - Platform-spezifische Configs
  - Config-File-Format
  - Environment-Variables
- [ ] Quick-Start-Guide schreiben

#### 9.2.2 Integration-Guide
- [ ] Platform-Integration-Guide schreiben
  - Midgard-Integration
  - Alfheim-Integration
  - Asgard-Integration
  - Ragnarok-Integration
- [ ] Odin-Integration-Guide schreiben
- [ ] Service-Health-Check-Guide schreiben
  - HTTP-Health-Endpoint-Implementation
  - gRPC-Health-Protocol-Implementation

### 9.3 Final-Testing

**Abhängigkeiten**: 9.2 (User-Documentation)

#### 9.3.1 Complete-Test-Suite
- [ ] Alle Unit-Tests ausführen (100% pass)
- [ ] Alle Integration-Tests ausführen (100% pass)
- [ ] Alle E2E-Tests ausführen (100% pass)
- [ ] Performance-Tests ausführen (alle bestehen)
- [ ] Security-Tests ausführen (alle bestehen)

#### 9.3.2 Code-Quality-Checks
- [ ] cargo clippy (keine Warnings)
- [ ] cargo fmt --check (korrekt formatiert)
- [ ] cargo audit (keine bekannten Vulnerabilities)
- [ ] Code-Coverage prüfen (≥ 80%)

#### 9.3.3 Release-Preparation
- [ ] CHANGELOG.md erstellen
- [ ] Version in Cargo.toml setzen
- [ ] Release-Notes schreiben
- [ ] Migration-Checkliste erstellen

---

## Phase 10: Platform-Migration (nach Gladsheim-Release)

### 10.1 Midgard-Migration

**Abhängigkeiten**: 9.3 (Final-Testing)

#### 10.1.1 Midgard-Anpassungen
- [ ] Gladsheim als Dependency hinzufügen
- [ ] Service-Lifecycle-Manager entfernen
- [ ] Gladsheim-Integration implementieren
  - Platform-Startup: Gladsheim initialisieren
  - Platform-Shutdown: Gladsheim stoppen
- [ ] Midgard-Tests anpassen
- [ ] Midgard README.md aktualisieren
- [ ] Midgard IMPLEMENTATION_PLAN.md aktualisieren

### 10.2 Alfheim-Migration

**Abhängigkeiten**: 10.1 (Midgard-Migration)

#### 10.2.1 Alfheim-Anpassungen
- [ ] Gladsheim als Dependency hinzufügen
- [ ] Battery-aware-Config für Gladsheim
- [ ] Service-Lifecycle-Manager entfernen
- [ ] Gladsheim-Integration implementieren
- [ ] Alfheim-Tests anpassen
- [ ] Alfheim README.md aktualisieren
- [ ] Alfheim IMPLEMENTATION_PLAN.md aktualisieren

### 10.3 Asgard-Migration

**Abhängigkeiten**: 10.2 (Alfheim-Migration)

#### 10.3.1 Asgard-Anpassungen
- [ ] Gladsheim als Dependency hinzufügen
- [ ] High-Capacity-Config für Gladsheim
- [ ] Service-Lifecycle-Manager entfernen
- [ ] Gladsheim-Integration implementieren
- [ ] Asgard-Tests anpassen
- [ ] Asgard README.md aktualisieren
- [ ] Asgard IMPLEMENTATION_PLAN.md aktualisieren

### 10.4 Ragnarok-Migration

**Abhängigkeiten**: 10.3 (Asgard-Migration)

#### 10.4.1 Ragnarok-Anpassungen
- [ ] Gladsheim als Dependency hinzufügen
- [ ] Minimal-Config für Gladsheim
- [ ] Service-Lifecycle-Manager entfernen
- [ ] Gladsheim-Integration implementieren
- [ ] Ragnarok-Tests anpassen
- [ ] Ragnarok README.md aktualisieren
- [ ] Ragnarok IMPLEMENTATION_PLAN.md aktualisieren

### 10.5 Odin-Migration

**Abhängigkeiten**: 10.4 (Ragnarok-Migration)

#### 10.5.1 Odin-Anpassungen
- [ ] Gladsheim-Client als Dependency hinzufügen
- [ ] Service-Start-Logik → Gladsheim-Client
- [ ] Service-Stop-Logik → Gladsheim-Client
- [ ] Service-Discovery anpassen (Skirnir-Integration)
- [ ] Odin-Tests anpassen
- [ ] Odin README.md aktualisieren
- [ ] Odin IMPLEMENTATION_PLAN.md aktualisieren

### 10.6 Root-AGENTS.md-Update

**Abhängigkeiten**: 10.5 (Odin-Migration)

#### 10.6.1 AGENTS.md aktualisieren
- [ ] Infrastructure-Sektion: Gladsheim hinzufügen
- [ ] Platform-Konzept: Gladsheim erwähnen
- [ ] Architecture-Overview: Gladsheim dokumentieren
- [ ] Services-Liste: Gladsheim ergänzen

---

## Dependencies-Übersicht

```
Phase 1: Projekt-Setup & Grundstruktur
  ↓
Phase 2: Proto-Definitionen & gRPC-Setup
  ↓
Phase 3: Thjalfi (Service Loader)
  ↓
Phase 4: Byggvir (Resource Manager)
  ↓
Phase 5: Roskva (Health Monitor)
  ↓
Phase 6: Skirnir (Service Registry)
  ↓
Phase 7: Gladsheim-Integration & Orchestration
  ↓
Phase 8: Platform-Integration & Migration-Planning
  ↓
Phase 9: Documentation & Testing
  ↓
Phase 10: Platform-Migration (nach Release)
```

## Geschätzte Komplexität

- **Phase 1-2**: Basis-Setup (10-15 Schritte, ~3-5 Stunden)
- **Phase 3**: Thjalfi (20-25 Schritte, ~8-12 Stunden)
- **Phase 4**: Byggvir (15-20 Schritte, ~6-10 Stunden)
- **Phase 5**: Roskva (20-25 Schritte, ~8-12 Stunden)
- **Phase 6**: Skirnir (15-20 Schritte, ~6-10 Stunden)
- **Phase 7**: Integration (15-20 Schritte, ~6-10 Stunden)
- **Phase 8**: Platform-Integration (10-15 Schritte, ~4-6 Stunden)
- **Phase 9**: Documentation (10-15 Schritte, ~4-6 Stunden)
- **Phase 10**: Migration (20-30 Schritte, ~10-15 Stunden)

**Gesamt**: ~140-200 Schritte, ~55-86 Stunden

## Wichtige Hinweise

- **TDD ist MANDATORY**: Alle Tests müssen vor Implementation geschrieben werden
- **Container-basierte Tests**: Alle Tests müssen in Containern laufen
- **Security-First**: Heimdall-Integration für Authorization ist kritisch
- **Performance**: Alle Performance-Requirements müssen erfüllt werden
- **Platform-spezifisch**: Config und Implementierung müssen platform-aware sein
- **Migration**: Phase 10 erfolgt erst NACH Gladsheim-Release und -Testing
