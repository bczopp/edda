# IMPLEMENTATION_PLAN - Gladsheim (Service Manager & Runtime Manager)

## Übersicht

Dieser Plan beschreibt die kleinstmöglichen Schritte zur Implementierung von Gladsheim - dem Service-Manager und Runtime-Manager für alle Plattformen (Midgard, Alfheim, Asgard, Ragnarok). Gladsheim verwaltet Service-Lifecycle, Ressourcen und Health-Status mit vier mythologischen Dienern als Sub-Komponenten. **Gladsheim ist verpflichtende Infrastruktur:** Jede Platform-Installation (außer Jotunheim) MUSS eine lokale Gladsheim-Instanz enthalten, die Odin, Thor, Loki und weitere Services (je nach Konfiguration) verwaltet und ausführt. Platforms bleiben „thin“ und delegieren sämtliches Service-Lifecycle-/Process-Management an Gladsheim.

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

#### 1.1.1 Cargo-Projekt erstellen ✅
- [x] `Cargo.toml` für Gladsheim erstellen
- [x] Basis-Dependencies definieren
  - Async Runtime (tokio mit full features) ✅
  - gRPC (tonic, prost, prost-types) ✅
  - Process Management (tokio::process) ✅
  - Resource Monitoring (sysinfo) ✅
  - Serialization (serde, serde_json) ✅
  - Logging (tracing, tracing-subscriber) ✅
  - Error-Handling (anyhow, thiserror) ✅
  - HTTP Client für Health Checks (reqwest) ✅
- [x] `.gitignore` erstellt (bereits vorhanden)
- [x] `README.md`, `AGENTS.md`, `IMPLEMENTATION_PLAN.md` erstellt (bereits vorhanden)

#### 1.1.2 Verzeichnisstruktur erstellen ✅
- [x] `src/lib.rs` erstellen
- [x] `src/gladsheim.rs` für Haupt-Gladsheim-Struct erstellen
- [x] `src/thjalfi/` für Service Loader erstellen
  - `mod.rs`, `loader.rs`, `process.rs` ✅
- [x] `src/byggvir/` für Resource Manager erstellen
  - `mod.rs`, `resources.rs`, `limits.rs` ✅
- [x] `src/roskva/` für Health Monitor erstellen
  - `mod.rs`, `health.rs`, `monitoring.rs` ✅
- [x] `src/skirnir/` für Service Registry erstellen
  - `mod.rs`, `registry.rs`, `discovery.rs` ✅
- [x] `src/proto/` für Proto Definitions erstellen
- [x] `src/grpc/` für gRPC Server Implementation erstellen
  - `mod.rs`, `server.rs` (bereits vorhanden)
- [x] `src/utils/` für Utilities erstellen
  - `config.rs` (bereits vorhanden), `errors.rs` ✅
- [x] `tests/` für Tests erstellen
  - `integration/`, `unit/` (bereits vorhanden)

#### 1.1.3 Build-System einrichten ✅
- [x] Build-Scripts in `Cargo.toml` definieren ✅ (`[build-dependencies]` mit `tonic-build`)
- [x] Protobuf-Code-Generierung konfigurieren (tonic-build) ✅ (`build.rs`)
- [x] Proto-Build-Script erstellen (`build.rs`) ✅
- [x] Cargo-Features definieren (`http-health`, `grpc-health`) ✅ (`[features]` in `Cargo.toml`)

### 1.2 Test-Infrastruktur

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.2.1 Container-Setup für Tests ✅
- [x] `Dockerfile.test` für Test-Umgebung erstellen
  - Rust-Toolchain (rust:latest)
  - System-Dependencies (pkg-config, libssl-dev, build-essential, protobuf-compiler)
  - Lockfile im Container: `cargo generate-lockfile`
- [x] Docker Compose für Tests konfigurieren (`docker-compose.test.yml`)
  - Service `gladsheim-test`, Build aus Dockerfile.test, Volumes für target/tests/src
  - Mock-Services bei Bedarf später ergänzbar (Thor, Freki, Heimdall)
- [x] Test-Container-Startup-Scripts erstellen
  - `scripts/run-tests.sh`, `scripts/run-tests.ps1` – `docker compose -f docker-compose.test.yml run --rm gladsheim-test`
- **WICHTIG**: Alle Tests müssen in Containern laufen – keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren

#### 1.2.2 Test-Framework konfigurieren ✅
- [x] Test-Dependencies hinzufügen (tokio-test, mockall, tempfile, wiremock)
- [x] Test-Utilities und Helpers erstellen (`tests/utils/test_helpers.rs`)
  - `wait_for_service`, `get_service_url` (bereits vorhanden)
  - `assert_service_listens(addr, timeout)` – Service erreichbar
  - `assert_resource_within_bounds(memory_mb, max_mb, cpu_percent, max_cpu)` – Resource-Assertions
  - `test_grpc_addr()` – Test-gRPC-Adresse (env `GLADSHEIM_TEST_GRPC_ADDR` oder Default)
- [x] Mock-Setup vorbereitet
  - wiremock für HTTP-Mock (z. B. Health-Endpoints) in Integrationstests nutzbar
  - Mock-gRPC-Services bei Bedarf in `tests/mocks/` ergänzbar (wie in Thor/Odin)

#### 1.2.3 CI/CD-Pipeline ✅
- [x] GitHub Actions Workflow (`.github/workflows/gladsheim.yml`)
- [x] Automatische Test-Ausführung bei Commits (push/PR auf `gladsheim/**`)
- [x] Code-Coverage-Reporting (cargo-tarpaulin, Artefakt `gladsheim-coverage`)
- [x] Linting und Formatting (cargo fmt --check, cargo clippy)
- [x] Container-Build in CI (Test-Job: `docker compose -f docker-compose.test.yml run --rm gladsheim-test`)

### 1.3 Projekt-Konfiguration

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.3.1 Settings-System Design ✅
- [x] Settings-Schema definieren (JSON) - `GladsheimConfig` struct
- [x] Settings-Struktur entwerfen
  - gRPC-Konfiguration (host, port) ✅
  - Resource-Limits (max_services, default_memory_mb, default_cpu_percent) ✅
  - Health-Monitoring (check_interval_ms, auto_restart, max_restart_attempts) ✅
  - Service-Loader (startup_timeout_ms, shutdown_timeout_ms) ✅
- [x] Platform-spezifische Settings dokumentieren
  - Midgard (15 services, 512MB), Alfheim (5 services, 256MB), Asgard (25 services, 2048MB), Ragnarok (8 services, 512MB) ✅

#### 1.3.2 Settings-Validierung ✅
- [x] Rust-Structs für Settings definieren
  - `GladsheimConfig` ✅
  - `ResourceLimitsConfig` ✅
  - `HealthMonitoringConfig` ✅
  - `ServiceLoaderConfig` ✅
- [x] Tests für Settings-Validierung schreiben (`tests/unit/config_test.rs`)
- [x] Settings-Validator implementieren (TDD) - `src/utils/config.rs`
  - Schema-Validierung ✅
  - Range-Checks (ports, timeouts, percentages) ✅
  - Platform-spezifische Validierung ✅
- [x] Tests ausführen und bestehen ✅

#### 1.3.3 Settings-Loader ✅
- [x] Tests für Settings-Loader schreiben (`src/utils/config_loader.rs` tests)
- [x] Settings-Loader implementieren (TDD) - `src/utils/config_loader.rs`
  - JSON-Parsing (serde_json) ✅
  - Default-Settings für Platforms ✅
- [x] Hot-Reload-Mechanismus implementieren (TDD)
  - File-Watcher für Settings-Datei (notify crate) ✅
  - Settings-Reload ohne Restart ✅
- [x] Tests ausführen und bestehen ✅

---

## Phase 1.5: Platform-Integration Requirements

### 1.5.1 Platform-Integration-Spezifikation

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.5.1.1 Platform-Integrationsanforderungen
- [ ] Anforderungen für Platform-Integration dokumentieren
  - Gladsheim MUSS bei jeder Platform-Installation (Midgard, Alfheim, Asgard, Ragnarok) mitgeliefert und lokal gestartet werden
  - Platforms nutzen Gladsheim für Service-Lifecycle-Management (Start/Stop/Restart) statt eigener Process-Manager
  - Standard-Services unter Gladsheim: Odin, Thor, Loki (weitere Services konfigurierbar pro Platform)
  - Jotunheim bleibt explizit ohne Gladsheim (siehe Jotunheim-Plan)
- [ ] Platform-spezifische Default-Konfigurationen definieren (Settings)
  - Midgard: 15 Services, Desktop-Profile (Odin, Thor, Loki, Geri, Freki, Huginn-Muninn, weitere)
  - Alfheim: 5 Services, Battery-aware Limits (Odin, Thor, Loki, Geri, Freki, Huginn-Muninn)
  - Asgard: 25+ Services, High-Capacity-Config (Odin, Thor, Loki, Bifrost, Heimdall, weitere)
  - Ragnarok: 8 Services, Minimal-Config (Odin, Thor, Loki, Geri, Freki, Huginn-Muninn)

---

## Phase 2: Proto-Definitionen & gRPC-Setup

### 2.1 Protobuf-Definitionen ✅

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 2.1.1 Proto-File erstellen ✅
- [x] `proto/gladsheim.proto` erstellen
- [x] Package `gladsheim.v1` und Imports definieren
- [x] Service-Definition `GladsheimService`
  - StartService, StopService, RestartService, GetServiceStatus, ListServices
  - GetServiceHealth, SubscribeServiceHealth (streaming), GetResourceUsage
  - SetResourceLimits, GetResourceLimits

#### 2.1.2 Message-Definitionen erstellen ✅
- [x] Request-Messages (StartServiceRequest, StopServiceRequest, …)
- [x] Response-Messages (ServiceStatus, ServiceList, ServiceHealth, HealthUpdate, ResourceUsage, ResourceLimits)
- [x] Enum-Definitionen: ServiceState (UNKNOWN, STARTING, RUNNING, STOPPING, STOPPED, CRASHED, RESTARTING), HealthStatus (UNKNOWN, HEALTHY, UNHEALTHY, CHECKING, TIMEOUT)
- [x] GladsheimError für Error-Responses

#### 2.1.3 Proto-Build testen ✅
- [x] `build.rs` für Proto-Compilation (tonic-build)
- [x] Proto-Compilation (cargo build)
- [x] Proto-File dokumentiert (Kommentare im proto)

### 2.2 gRPC-Server-Setup

**Abhängigkeiten**: 2.1 (Proto-Definitionen)

#### 2.2.1 gRPC-Server-Grundstruktur ✅
- [x] Tests für gRPC-Server schreiben (`tests/unit/grpc_server_test.rs`)
- [x] `GladsheimServiceImpl` struct erstellen (`src/grpc/server.rs`)
- [x] gRPC-Server-Implementation-Skeleton erstellt
  - Trait-Implementation für `GladsheimService` ✅
  - Alle RPC-Methods implementiert (teilweise als stubs) ✅
- [x] Service-Integration mit Thjalfi, Byggvir, Roskva, Skirnir ✅
- [x] Tests ausführen und bestehen ✅

**Implementierte RPCs:**
- ✅ `GetServiceStatus` - vollständig implementiert (nutzt Skirnir)
- ✅ `ListServices` - vollständig implementiert (mit Filtering, nutzt Skirnir)
- ✅ `GetResourceUsage` - vollständig implementiert (nutzt Byggvir)
- ✅ `StartService` - vollständig implementiert (nutzt Thjalfi, Skirnir) - Phase 3.1 ✅
- ✅ `StopService` - vollständig implementiert (nutzt Thjalfi, Skirnir) - Phase 3.1 ✅
- ✅ `RestartService` - vollständig implementiert (Stop + Start) - Phase 3.1 ✅
- ✅ `GetServiceHealth` - vollständig implementiert (nutzt Roskva) - Phase 5.1 ✅
- ✅ `SubscribeServiceHealth` - vollständig implementiert (Server-Side-Streaming, nutzt Roskva) - Phase 5.1 ✅
- ✅ `SetResourceLimits` - implementiert (nutzt Skirnir für Service-Check) - Phase 4.2 ✅
- ✅ `GetResourceLimits` - implementiert (nutzt Skirnir, gibt Default-Limits zurück) - Phase 4.2 ✅

#### 2.2.2 gRPC-Client für Tests ✅
- [x] Test-gRPC-Client in Tests integriert (tonic::Request)
- [x] Client-Verbindung getestet
- [x] Stub-Calls getestet (alle RPC-Methods)

---

## Phase 3: Thjalfi (Service Loader) Implementierung

### 3.1 Process-Management

**Abhängigkeiten**: 2.2 (gRPC-Server-Setup)

#### 3.1.1 Process-Wrapper ✅
- [x] Tests für Process-Wrapper schreiben (`tests/unit/thjalfi_test.rs`)
  - Process-Start ✅
  - Process-Stop (graceful) ✅
  - Process-Force-Kill ✅
  - Process-Status-Query ✅
- [x] `ServiceProcess` struct erstellen (`src/thjalfi/service_process.rs`)
  - Process-Handle (tokio::process::Child) ✅
  - Process-ID ✅
  - Start-Time ✅
  - Status (Starting, Running, Stopping, Finished, Killed) ✅
- [x] Process-Start implementieren (TDD)
  - `tokio::process::Command::spawn` ✅
  - Environment-Variables ✅
  - Working-Directory ✅
  - Startup-Validation ✅
- [x] Process-Stop implementieren (TDD)
  - Graceful-Shutdown (SIGTERM) ✅
  - Timeout-Handling ✅
  - Force-Kill (SIGKILL) als Fallback ✅
- [x] Tests ausführen und bestehen ✅

#### 3.1.2 Service-Loader ✅
- [x] Tests für Service-Loader schreiben (`tests/unit/thjalfi_test.rs`)
  - Service-Start mit verschiedenen Parametern ✅
  - Service-Stop (graceful + force) ✅
  - Startup-Timeout ✅
  - Shutdown-Timeout ✅
- [x] `Thjalfi` (Service Loader) struct erweitert (`src/thjalfi/loader.rs`)
- [x] Service-Start-Logik implementiert (TDD)
  - Service-Config (ServiceConfig) ✅
  - Process-Spawn ✅
  - Startup-Validation ✅
  - Timeout-Enforcement ✅
  - Running-Services-Tracking ✅
- [x] Service-Stop-Logik implementiert (TDD)
  - Graceful-Stop mit Timeout ✅
  - Force-Stop ✅
  - Service-Removal aus Registry ✅
  - Graceful-Shutdown-Request
  - Timeout-Wait
  - Force-Kill bei Timeout
- [x] Service-Restart implementieren (Stop + Start) – `Thjalfi::restart_service(service_name, config, startup_timeout, shutdown_timeout)` ruft `stop_service` dann `start_service` auf
- [x] Test für Restart hinzugefügt (`tests/unit/thjalfi_test.rs`: `test_restart_service`)
- [x] Build- und Enforcer-Test behoben (grpc/server, Critical-vor-Warning-Prüfung in `enforcer.rs`); Tests im Container ausführbar

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
- [x] Tests für StopService schreiben
- [x] StopService-Handler implementieren (TDD)
  - Request-Validation
  - Thjalfi.stop_service()
  - Response-Building
- [x] Error-Handling implementieren
- [x] Tests ausführen und bestehen

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
- [x] Tests für System-Resource-Monitor schreiben
  - Gesamt-RAM-Usage
  - Gesamt-CPU-Usage
  - Process-RAM-Usage
  - Process-CPU-Usage
- [x] `SystemResourceMonitor` struct erstellen
  - sysinfo::System-Integration
  - Refresh-Strategie
- [x] System-Monitoring implementieren (TDD)
  - RAM-Usage-Query
  - CPU-Usage-Query
  - Process-Monitoring
- [x] Tests ausführen und bestehen

#### 4.1.2 Service-Resource-Tracker
- [x] Tests für Service-Resource-Tracker schreiben
  - Resource-Tracking pro Service
  - Resource-History
  - Resource-Aggregation
- [x] `ServiceResourceTracker` struct erstellen
- [x] Per-Service-Resource-Tracking implementieren (TDD)
  - RAM-Usage pro Service
  - CPU-Usage pro Service
  - Resource-History (letzte N Werte)
- [x] Tests ausführen und bestehen

### 4.2 Resource-Limit-Enforcement

**Abhängigkeiten**: 4.1 (Resource-Monitoring)

#### 4.2.1 Limit-Checker ✅
- [x] Tests für Limit-Checker schreiben (`tests/unit/byggvir_test.rs`)
  - RAM-Limit-Check ✅
  - CPU-Limit-Check ✅
  - Platform-spezifische Limits ✅
- [x] `ResourceLimitChecker` struct erstellt (`src/byggvir/limit_checker.rs`)
- [x] Limit-Check-Logik implementiert (TDD)
  - RAM-Limit-Überprüfung ✅
  - CPU-Limit-Überprüfung ✅
  - Warning-Thresholds (80% default) ✅
- [x] Platform-spezifische Limits implementiert (bereits in ResourceLimits)
  - Midgard: moderate limits (512MB, 25%) ✅
  - Alfheim: strict limits (256MB, 25%) ✅
  - Asgard: relaxed limits (2048MB, 75%) ✅
  - Ragnarok: minimal limits (512MB, 30%) ✅
- [x] Tests ausführen und bestehen ✅

#### 4.2.2 Limit-Enforcement-Actions ✅
- [x] Tests für Enforcement-Actions schreiben (`tests/unit/byggvir_test.rs`)
  - Service-Warning bei Limit-Annäherung ✅
  - Service-Stop bei kritischer Überschreitung ✅
- [x] `ResourceEnforcer` struct erstellt (`src/byggvir/enforcer.rs`)
- [x] Enforcement-Actions implementiert (TDD)
  - Warning-Logging ✅
  - EnforcementAction enum (Ok, Warning, Critical) ✅
  - Service-Stop-Flag bei kritischen Limits ✅
  - Prüfreihenfolge: zuerst Critical (über Limit), dann Warning (z. B. 80 %) ✅
- [x] Tests ausführen und bestehen ✅
- ⚠️ TODO: Alert zu Odin (gRPC-Call) - Phase 6
- ⚠️ TODO: Service-Throttling - optional

### 4.3 Byggvir-Integration

**Abhängigkeiten**: 4.2 (Resource-Limit-Enforcement)

#### 4.3.1 Resource-Manager (Byggvir)
- [x] Tests für Resource-Manager schreiben
- [x] `ResourceManager` (Byggvir) struct erstellen
  - System-Monitor
  - Service-Tracker
  - Limit-Checker
  - Enforcer
- [x] Resource-Manager-Logik implementieren (TDD)
  - Kontinuierliches Monitoring
  - Limit-Checks
  - Enforcement-Actions
- [x] Tests ausführen und bestehen

#### 4.3.2 gRPC-Endpoints (Byggvir)
- [x] Tests für GetResourceUsage schreiben
- [x] GetResourceUsage-Handler implementieren (TDD)
- [x] Tests für SetResourceLimits schreiben
- [x] SetResourceLimits-Handler implementieren (TDD)
- [x] Tests für GetResourceLimits schreiben
- [x] GetResourceLimits-Handler implementieren (TDD)
- [x] Tests ausführen und bestehen

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

#### 5.2.1 Health-Monitor ✅
- [x] Tests für Health-Monitor schreiben (`tests/unit/roskva_test.rs`)
  - Health-Status-Updates ✅
  - Check-Interval-Configuration ✅
- [x] `ServiceHealthTracker` struct erstellt (`src/roskva/monitoring.rs`)
  - Health-Checker (HTTP, gRPC, Process) ✅
  - Health-Status-Cache (HashMap) ✅
  - Service-Registration ✅
- [x] Health-Status-Tracking implementiert (TDD)
  - Service-Registration mit Strategy ✅
  - Health-Updates ✅
  - Consecutive-Failures-Tracking ✅
  - Last-Check-Timestamp ✅
- [x] Tests ausführen und bestehen ✅
- [x] Monitoring-Loop (periodische Checks) – Phase 5.2.2 ✅
  - [x] `Roskva::start_monitoring_loop(Arc<Self>, Duration)` – Hintergrund-Task mit konfigurierbarem Intervall
  - [x] `MonitoringLoopHandle` – Drop stoppt den Loop (watch::Sender)
  - [x] Pro Tick: alle registrierten Services per Strategy prüfen, `update_health` aufrufen
  - [x] Tests: `test_monitoring_loop_updates_health_after_tick`, `test_monitoring_loop_stops_when_handle_dropped`

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

#### 5.3.1 Restart-Policy ✅
- [x] Tests für Restart-Policy schreiben (`src/roskva/restart_policy.rs`)
  - Auto-Restart enabled/disabled ✅
  - Max-Restart-Attempts ✅
  - Restart-Backoff (exponential, capped) ✅
  - No-Restart für bestimmte Services ✅
- [x] `RestartPolicy` struct erstellen – `src/roskva/restart_policy.rs`
- [x] Restart-Policy-Logik implementieren (TDD)
  - Policy-Evaluation (`should_allow_restart(service_name, current_attempts)`) ✅
  - Backoff-Calculation (`backoff_duration(attempt)` exponential, max_backoff) ✅
  - `add_no_restart` / `remove_no_restart` ✅
- [x] Tests ausführen und bestehen ✅

#### 5.3.2 Auto-Restart-Implementation ⚠️ TEILWEISE
- [x] Tests für Attempt-Tracking und Restart-Evaluation schreiben
  - RestartAttemptTracker (get/increment/reset) ✅
  - evaluate_restart (Policy + Attempts) ✅
  - Monitoring-Loop mit optionalem Attempt-Tracker (Reset bei healthy) ✅
- [x] Attempt-Tracking – `RestartAttemptTracker` in `restart_policy.rs` (get, increment, reset)
- [x] `Roskva::evaluate_restart(service_name, policy, attempt_tracker) -> Option<Duration>` – Aufrufer ruft Thjalfi.restart_service() und dann attempt_tracker.increment() auf
- [x] Monitoring-Loop: optionaler `attempt_tracker`; bei Update auf healthy wird reset(service) aufgerufen
- [x] Tests ausführen und bestehen (43 Tests) ✅
- [ ] Crash-Event-Handling / Aufruf Thjalfi.restart_service() – bleibt an Integration (gRPC-Handler oder Supervisor mit Config-Provider)

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

## Phase 8: Platform-Integration & Migration (REQUIRED)

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
 - [ ] **Gladsheim als Pflichtkomponente klarstellen:** Dokumentieren, dass jede Platform-Installation (Midgard, Alfheim, Asgard, Ragnarok) Gladsheim zwingend voraussetzt und eigenes Service-Lifecycle-/Process-Management der Platforms entfernt bzw. deaktiviert wird.

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
