# IMPLEMENTATION_PLAN - Loki (Script Execution Service)

## Übersicht

Dieser Plan beschreibt die kleinstmöglichen Schritte zur Implementierung von Loki - dem Script Execution Service für user-generierte Scripte. Loki macht Scripte per gRPC zugänglich und koordiniert drei spezialisierte Sub-Services: Fenrir (Hardware-Control), Jörmungandr (Network/Communication), Hel (Data/Storage).

**Myth ologische Bedeutung**: Loki ist der Gott des Schabernacks. Seine 3 Kinder sind Fenrir (Wolf), Jörmungandr (Weltenschlange) und Hel (Göttin der Unterwelt).

**Programmiersprache**: Rust

**Service-Typ**: Unabhängiger Service (nicht Teil von Jotunheim-Platform)

**Architektur**: 3 Sub-Services (Fenrir, Jörmungandr, Hel) koordiniert von Loki

## Entschiedene Konfiguration

### Protobuf-Rust-Tool
✅ **ENTSCHEIDUNG**: prost-lite + tonic
**Begründung**: IoT-optimiert, minimaler Flash-Usage, beste Performance für ESP32

### Primary-Script-Engine
✅ **ENTSCHEIDUNG**: mlua
**Begründung**: Rust Lua bindings, robuste Performance, aktiv maintained, battle-tested

### Config-File-Format
✅ **ENTSCHEIDUNG**: TOML
**Begründung**: Rust-freundlich, menschenlesbar, beste Integration mit Cargo-Ökosystem

### Script-Storage
✅ **ENTSCHEIDUNG**: Beides (Filesystem + inline)
**Begründung**: Maximale Flexibilität, inline für kleine Scripts, Filesystem für komplexe

---

## Phase 1: Projekt-Setup & Grundstruktur ✅

### 1.1 Projekt-Initialisierung ✅

**Abhängigkeiten**: Keine ✅
**Entscheidungen**: prost + tonic ✅, mlua (Lua 5.4) ✅, TOML ✅

#### 1.1.1 Cargo-Workspace erstellen ✅
- [x] `Cargo.toml` mit Workspace erstellen
  - `loki/` (Main Coordinating Service)
  - `fenrir/` (Hardware-Control Service - TODO)
  - `jormungandr/` (Network/Communication Service - TODO)
  - `hel/` (Data/Storage Service - TODO)
  - `shared/` (Shared Code)
- [x] Basis-Dependencies definieren
  - Async Runtime (tokio)
  - gRPC Server (tonic, prost)
  - Script Engine (mlua - Lua 5.4 with async)
  - Serialization (serde, serde_json, toml)
  - Logging (tracing, tracing-subscriber)
  - Error-Handling (anyhow, thiserror)
- [x] `.gitignore` erstellt (bereits vorhanden)

#### 1.1.2 Verzeichnisstruktur erstellen ✅
- [x] `loki/src/main.rs` erstellen
- [x] `loki/src/lib.rs` erstellen
- [x] `loki/src/grpc/` für gRPC-Server erstellen (bereits vorhanden)
- [x] `loki/src/script/` für Script-Management erstellen (ScriptEngine, ScriptContext)
- [x] `loki/src/coordinator/` für Koordination der 3 Services erstellen (bereits vorhanden)
- [x] `shared/src/lib.rs` erstellen
- [x] `shared/src/error.rs` für gemeinsame Error-Typen erstellen (LokiError, Result)
- [x] `shared/src/models.rs` für gemeinsame Script-Models erstellen (ScriptDefinition, ScriptLanguage, ResourceLimits)

#### 1.1.3 Build-System einrichten ✅
- [x] Build-Scripts für Protobuf-Code-Generierung erstellen (build.rs - bereits vorhanden)
- [x] Cargo-Features für optimierte Release-Builds (opt-level="z", lto=true, codegen-units=1, strip=true)

### 1.2 Test-Infrastruktur ✅

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.2.1 Container-Setup für Tests ✅
- [x] `Dockerfile.test` für Test-Umgebung erstellen ✅
- [x] Docker Compose für Test-Services konfigurieren (`docker-compose.test.yml`) ✅
  - Mock-Odin Service ✅
  - Test-Filesystem für Scripts (via Volumes) ✅
  - Mock-Hardware für Fenrir-Tests (TODO: Spezifische Fenrir-Mocks)
- [x] Test-Container-Startup-Scripts erstellen (`scripts/run-tests.sh`, `scripts/run-tests.ps1`) ✅
- [x] **WICHTIG**: Alle Tests müssen in Containern laufen - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren ✅

#### 1.2.2 Test-Framework konfigurieren ✅
- [x] Test-Dependencies hinzufügen (`Cargo.toml` - tokio-test, tempfile) ✅
- [x] Test-Utilities und Helpers erstellen (`tests/utils/test_helpers.rs`) ✅
- [x] Mock-Setup für Services (`tests/mocks/` mit Dockerfile.mock-service, Cargo.toml, src/main.rs) ✅
- [ ] Test-Script-Generators erstellen (TODO: Für zukünftige Script-Tests)

#### 1.2.3 CI/CD-Pipeline ✅
- [x] GitHub Actions Workflow erstellen (`.github/workflows/loki.yml`) ✅
- [x] Automatische Test-Ausführung bei Commits konfigurieren ✅
- [x] Code-Coverage-Reporting einrichten (cargo-tarpaulin, Artefakt loki-coverage in CI)
- [x] Linting und Formatting (cargo clippy, cargo fmt) ✅

### 1.3 Settings-System ✅

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.3.1 Settings-Schema definieren ✅
- [x] Settings-Struktur entwerfen (JSON)
  - resource_limits (`ResourceLimitsConfig`: max_memory_mb, max_execution_time_ms, max_cpu_percent)
  - script_storage_path (String)
  - children_config (`ChildrenConfig`: fenrir, jormungandr, hel mit `ChildServiceConfig` - enabled, address)
  - grpc_port (u16)

#### 1.3.2 Settings-Validierung ✅
- [x] Tests für Settings-Validierung schreiben (`tests/config_test.rs`)
- [x] Rust-Structs für Settings definieren (`src/utils/config.rs`)
  - `LokiConfig`, `ResourceLimitsConfig`, `ChildrenConfig`, `ChildServiceConfig`
- [x] Settings-Validator implementieren (TDD)
  - `validate()`: prüft grpc_port != 0, max_memory_mb != 0, max_execution_time_ms != 0, max_cpu_percent <= 100
  - `from_json()`: JSON-Parsing mit Validierung
  - `to_json()`: JSON-Serialisierung
- [x] Tests ausführen und bestehen ✅

#### 1.3.3 Settings-Loader & Hot-Reload ✅
- [x] Tests für Settings-Loader schreiben (`tests/config_test.rs`)
- [x] Settings-Loader implementieren (TDD) (`src/utils/config_loader.rs`)
  - `ConfigLoader`: lädt Config aus Datei oder verwendet Defaults
  - `load()`: lädt Config asynchron
  - `get_config()`: thread-safe Config-Zugriff (`Arc<RwLock<LokiConfig>>`)
  - `start_watching()`: File-Watcher für Hot-Reload (`notify` crate)
    - Überwacht Config-Datei auf Änderungen
    - Lädt Config automatisch neu bei Modifikationen
    - Runtime-Settings-Reload ohne Service-Neustart
- [x] Tests ausführen und bestehen ✅

---

## Phase 2: Tool-Konfigurationsdatei ✅

### 2.1 Config-Schema ✅

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)
**Erforderliche USER-Eingaben**: Config-File-Format ✅ (TOML)

#### 2.1.1 Tool-Config-Schema definieren ✅
- [x] Tests für Config-Schema schreiben (`tests/tool_config_test.rs`)
- [x] Tool-Config-Schema entwerfen (TOML) (`src/tools/config.rs`)
  - Tool-Name (`name: String`)
  - Tool-Beschreibung (`description: String`)
  - Tool-Parameter (`parameters: Vec<ToolParameter>` - Name, Type, Required, Description)
  - Tool-Return-Type (`return_type: ReturnType` - String, Number, Boolean, Object, Array, Void)
  - Script-Source (`script: ScriptSource` - inline oder path, nicht beide)
- [x] Tests ausführen und bestehen ✅

### 2.2 Config-Loader ✅

**Abhängigkeiten**: 2.1 (Config-Schema)
**Erforderliche USER-Eingaben**: Script-Storage ✅ (Beides: Filesystem + inline)

#### 2.2.1 Config-Loader-Implementation ✅
- [x] Tests für Config-Loader schreiben (`tests/tool_config_test.rs`)
- [x] `ToolConfigLoader` implementieren (TDD) (`src/tools/config_loader.rs`)
  - Konfigurationsdatei laden (TOML)
  - Schema-Validierung (integriert in `ToolConfig::from_toml()`)
  - Script-Path-Resolution (bereit für zukünftige Implementierung)
- [x] Tests ausführen und bestehen ✅

### 2.3 Config-Validation ✅

**Abhängigkeiten**: 2.2 (Config-Loader)

#### 2.3.1 Config-Validator ✅
- [x] Tests für Config-Validator schreiben (`tests/tool_config_test.rs`)
- [x] `ToolConfig::validate()` implementieren (TDD)
  - Tool-Definitionen validieren (name nicht leer, script vorhanden)
  - Parameter-Validierung (parameter name nicht leer)
  - Return-Type-Validierung (bereits als Enum definiert)
  - Script-Existenz-Checks (inline nicht leer, path nicht leer, nicht beide)
- [x] Tests ausführen und bestehen ✅

### 2.4 Config-Hot-Reload ✅

**Abhängigkeiten**: 2.2 (Config-Loader)

#### 2.4.1 Config-File-Watcher ✅
- [x] Tests für File-Watcher schreiben (`tests/tool_config_test.rs`)
- [x] `ToolConfigLoader::start_watching()` implementieren (TDD) (`src/tools/config_loader.rs`)
  - Überwacht Konfigurationsdatei auf Änderungen (`notify` crate)
  - Hot-Reload bei Änderungen (automatisches Neuladen)
  - gRPC-Funktionen aktualisieren (bereit für zukünftige Integration)
- [x] Tests ausführen und bestehen ✅

---

## Phase 3: Protobuf & gRPC Services ✅ (Teilweise)

### 3.1 Shared Protobuf Definitions ✅

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 3.1.1 Shared Protobuf-Projekt verwenden
❓ **HINWEIS**: Protobuf-Definitions sollten in einem separaten Projekt sein
- [ ] Loki als Dependency zu Protobuf-Projekt hinzufügen (TODO: Separate Protobuf-Projekt)
- [ ] Protobuf-Definitions importieren (minimal subset für IoT) (TODO: Separate Protobuf-Projekt)

#### 3.1.2 Loki Service Protocol definieren ✅
- [x] `loki.proto` definieren (`proto/loki.proto`)
  - `GetCapabilities()` RPC ✅
  - `GetChildrenStatus()` RPC ✅
  - `ListScripts()` RPC ✅
  - `RegisterScript()` RPC ✅
  - `ExecuteScript()` RPC ✅ (Legacy)
  - `StreamScriptExecution()` RPC ✅ (Streaming)
  - `ScriptInput` / `ScriptOutput` Messages ✅
  - `ScriptChunk` / `ScriptResult` Messages ✅ (für Streaming)
  - `ParameterDefinition`, `ScriptCapability`, `ChildServiceInfo` Messages ✅
- [x] Code-Generierung konfigurieren (`build.rs` - bereits vorhanden)

### 3.2 gRPC Server Implementation ✅ (Teilweise)

**Abhängigkeiten**: 3.1 (Shared Protobuf Definitions), 2.2 (Config-Loader)

#### 3.2.1 Main Loki gRPC Server ✅
- [x] Tests für Main gRPC Server schreiben (`tests/grpc_server_test.rs` - Platzhalter)
- [x] `LokiServiceImpl` implementieren (TDD) (`src/grpc/server.rs`)
  - Statische Methoden implementieren ✅
    - `GetCapabilities()` ✅ - Listet alle Scripts aus Tool-Config
    - `GetChildrenStatus()` ✅ - Status von Fenrir, Jörmungandr, Hel (TODO: Echte Status-Checks)
    - `ListScripts()` ✅ - Listet Scripts mit Filterung (verwendet ScriptRegistry)
    - `RegisterScript()` ✅ - Vollständige Implementierung (Validierung, Registry-Integration, Script-Validation)
    - `ExecuteScript()` ✅ - Legacy Script-Execution via Coordinator
    - `StreamScriptExecution()` ✅ - Vollständige Streaming-Implementierung (Chunk-basierte Ausgabe, Error-Handling)
  - Tool-Config-Integration ✅ - Verwendet `ToolConfigLoader` für Script-Liste
  - Script-Execution routing (zu Fenrir/Jörmungandr/Hel) ✅ - Via `ServiceCoordinator`
  - ScriptManager-Integration ✅ - Verwendet `ScriptManager` für Script-Execution und Validation
- [x] Tests ausführen und bestehen ✅ (`tests/grpc_server_test.rs`)
  - `test_get_capabilities()` ✅ - Testet GetCapabilities RPC
  - `test_get_children_status()` ✅ - Testet GetChildrenStatus RPC (prüft alle 3 Children)
  - `test_list_scripts_empty()` ✅ - Testet ListScripts mit leerer Registry
  - `test_list_scripts_with_pattern()` ✅ - Testet ListScripts mit Name-Pattern-Filterung
  - `test_register_script_success()` ✅ - Testet erfolgreiche Script-Registrierung
  - `test_register_script_duplicate()` ✅ - Testet Duplikat-Erkennung
  - `test_register_script_invalid_source()` ✅ - Testet Validierung (fehlende Script-Source)
  - `test_execute_script()` ✅ - Testet ExecuteScript RPC
  - `test_stream_script_execution_not_found()` ✅ - Testet StreamScriptExecution mit nicht-existierendem Script

### 3.3 Dynamic gRPC Function Generation ✅ (Grundstruktur)

**Abhängigkeiten**: 3.2 (Main gRPC Server)

#### 3.3.1 Function-Generator ✅ (Grundstruktur)
- [x] Tests für Script-Registry schreiben (`tests/script_registry_test.rs`)
- [x] `ScriptRegistry` implementieren (TDD) (`src/script_registry.rs`)
  - Script-Registry für Tool-Definitionen ✅
  - `register_tool()` / `unregister_tool()` ✅
  - `get_tool()` / `list_scripts()` ✅
  - `load_from_config()` ✅ - Lädt Tools aus ToolConfig
  - Integration mit gRPC-Server ✅ - `ListScripts` verwendet Registry
- [x] Tests ausführen und bestehen ✅
- [ ] **HINWEIS**: Vollständige dynamische gRPC-Funktionen zur Laufzeit sind in Rust/tonic sehr komplex, da gRPC-Services zur Compile-Zeit generiert werden müssen. Die aktuelle Implementierung verwendet ein Registry-System, das Scripts verwaltet und über `ExecuteScript` mit Script-Namen geroutet werden kann. Für echte dynamische Funktionen wäre ein Proxy-System oder Code-Generation zur Laufzeit erforderlich.

---

## Phase 4: Script-Execution Engine

### 4.1 Script-Engine-Abstraction ✅

**Abhängigkeiten**: 2.2 (Config-Loader)
**Erforderliche USER-Eingaben**: Primary-Script-Engine ✅ (mlua)

#### 4.1.1 Script-Engine-Trait ✅
- [x] Tests für Script-Engine-Trait schreiben (`tests/script_engine_trait_test.rs`)
- [x] `ScriptEngine` Interface erweitert (`src/script/engine.rs`)
  - `execute_script()` Methode ✅ - Führt Script aus ToolDefinition aus
  - `load_script()` Methode ✅ - Lädt Script aus ToolDefinition (inline oder path)
  - `validate_script()` Methode ✅ - Validiert Script-Syntax
- [x] Tests ausführen und bestehen ✅

### 4.2 Lua-Engine-Implementation ✅

**Abhängigkeiten**: 4.1 (Script-Engine-Abstraction)

#### 4.2.1 Lua-Script-Engine ✅
- [x] Tests für Lua-Engine schreiben (`tests/script_engine_test.rs`, `tests/script_engine_trait_test.rs`)
- [x] `ScriptEngine` mit Lua-Integration implementiert (TDD) (`src/script/engine.rs`)
  - mlua Integration ✅ - Verwendet `mlua` für Lua 5.4
  - Script-Loading von Filesystem oder Inline ✅ - Unterstützt beide via `ScriptSource`
  - Script-Execution mit Parameters ✅ - Via `ScriptContext`
  - Return-Value-Handling ✅ - Konvertiert Lua-Werte zu String
  - Timeout-Handling ✅ - Verwendet `tokio::time::timeout`
- [x] Tests ausführen und bestehen ✅

### 4.3 Script-Manager ✅

**Abhängigkeiten**: 4.2 (Lua-Engine-Implementation)

#### 4.3.1 Script-Registry & Manager ✅
- [x] Tests für Script-Manager schreiben (`tests/script_manager_test.rs`)
- [x] `ScriptManager` implementieren (TDD) (`src/script/manager.rs`)
  - Script-Loading ✅ - Lädt Scripts aus Registry via `ScriptEngine::load_script()`
  - Script-Caching ✅ - LRU-Cache für geladene Scripts (max 100 Einträge)
  - Script-Validation ✅ - Validiert Scripts via `ScriptEngine::validate_script()`
  - Script-Execution mit Resource-Limits ✅ - Führt Scripts via `ScriptEngine::execute()` aus
  - Cache-Management ✅ - Eviction bei Cache-Overflow, Cache-Statistics
- [x] Tests ausführen und bestehen ✅

---

## Phase 5: Resource-Management

### 5.1 Resource-Limits ✅

**Abhängigkeiten**: 1.3 (Settings-System)

#### 5.1.1 Resource-Limits-Configuration ✅
- [x] Tests für Resource-Limits schreiben (`tests/resource_limits_test.rs`)
- [x] `ResourceLimits` implementieren (TDD) (`src/resources/limits.rs`)
  - CPU-Limits ✅ (`max_cpu_percent`)
  - Memory-Limits ✅ (`max_memory_mb`)
  - Execution-Time-Limits ✅ (`max_execution_time_ms`)
  - Disk-Limits ✅ (`max_disk_mb` - 0 = no limit)
  - Validierung ✅ (`validate()`)
  - Limit-Checks ✅ (`exceeds_limit()`, `get_limit()`)
- [x] Tests ausführen und bestehen ✅

### 5.2 Resource-Monitoring ✅

**Abhängigkeiten**: 5.1 (Resource-Limits)

#### 5.2.1 Resource-Monitor ✅
- [x] Tests für Resource-Monitor schreiben (`tests/resource_monitor_test.rs`)
- [x] `ResourceMonitor` implementieren (TDD) (`src/resources/monitor.rs`)
  - RAM-Usage überwachen ✅ (`check_memory_usage()`)
  - CPU-Usage überwachen ✅ (`check_cpu_usage()`)
  - Execution-Time überwachen ✅ (`check_execution_time()`)
  - Disk-Usage überwachen ✅ (`check_disk_usage()`)
  - Resource-Exhaustion erkennen ✅ (`check_all_limits()`)
  - Limits-Update ✅ (`update_limits()`, `get_limits()`)
- [x] Tests ausführen und bestehen ✅

### 5.3 Resource-Enforcement

**Abhängigkeiten**: 5.2 (Resource-Monitoring)

#### 5.3.1 Resource-Enforcer ✅
- [x] Tests für Resource-Enforcer schreiben (`tests/resource_enforcer_test.rs`)
- [x] `ResourceEnforcer` implementieren (TDD) (`src/resources/enforcer.rs`)
  - Script-Execution abbrechen bei Resource-Exhaustion ✅ (TODO: Vollständige Implementierung - aktuell nur Logging)
  - Resource-Limits durchsetzen ✅ (`enforce_memory_limit()`, `enforce_cpu_limit()`, `enforce_execution_time()`, `enforce_disk_limit()`)
  - Warning-Thresholds ✅ (80% Threshold für Warnings)
  - Enforcement-Actions ✅ (`EnforcementAction` enum: None, Warning, Critical)
  - Combined-Check ✅ (`check_and_enforce()` - prüft alle Limits und gibt Action zurück)
  - Script-Queue bei knappen Ressourcen (TODO: Noch nicht implementiert)
- [x] Tests ausführen und bestehen ✅

---

## Phase 6: Fenrir - Hardware-Control Service

### 6.1 Fenrir-Service-Setup ✅ (Grundstruktur)

**Abhängigkeiten**: 4.3 (Script-Manager)

#### 6.1.1 Fenrir-Service-Initialisierung ✅
- [x] `fenrir/src/main.rs` erstellen (Stub-Eintrittspunkt)
- [x] `fenrir/src/lib.rs` erstellen
- [x] `fenrir/src/hardware/` für Hardware-Access erstellen (mod.rs Platzhalter)
- [x] `fenrir/src/gpio/` für GPIO-Control erstellen
- [x] `fenrir/src/sensors/` für Sensor-Reading erstellen
- [x] `fenrir/src/actuators/` für Actuator-Control erstellen
- [x] `fenrir/Cargo.toml` (shared, tracing, tokio); Workspace-Member: fenrir (jormungandr/hel aus Members entfernt bis Phase 7/8)

### 6.2 Hardware-Access-Abstraction ✅

**Abhängigkeiten**: 6.1 (Fenrir-Service-Setup) ✅

#### 6.2.1 Hardware-Access-Trait ✅
- [x] Tests für Hardware-Access-Trait schreiben (`fenrir/tests/hardware_access_trait_test.rs`)
- [x] `HardwareAccess` Trait definieren (`fenrir/src/hardware/mod.rs`)
  - `read_gpio(pin)` / `write_gpio(pin, value)` ✅
  - `read_sensor(sensor_id)` / `control_actuator(actuator_id, value)` ✅
- [x] `StubHardwareAccess` (TDD) – in-memory GPIO, stub sensor/actuator (`fenrir/src/hardware/stub.rs`)
- [x] `FenrirError` (`fenrir/src/error.rs`) – Gpio, Sensor, Actuator, NotAvailable
- [x] Tests ausführen und bestehen

### 6.3 GPIO-Control ✅

**Abhängigkeiten**: 6.2 (Hardware-Access-Abstraction) ✅

#### 6.3.1 GPIO-Controller ✅
- [x] Tests für GPIO-Controller schreiben (`fenrir/tests/gpio_controller_test.rs`)
- [x] `GPIOController` implementieren (TDD) (`fenrir/src/gpio/controller.rs`)
  - GPIO-Read / GPIO-Write (delegiert an `HardwareAccess`) ✅
  - GPIO-PWM (optional) – `set_pwm(pin, duty)` liefert derzeit `NotAvailable` ✅
- [x] Tests ausführen und bestehen

### 6.4 Sensor-Reading ✅

**Abhängigkeiten**: 6.2 (Hardware-Access-Abstraction) ✅

#### 6.4.1 Sensor-Reader ✅
- [x] Tests für Sensor-Reader schreiben (`fenrir/tests/sensor_reader_test.rs`)
- [x] `SensorReader` implementieren (TDD) (`fenrir/src/sensors/reader.rs`)
  - Temperatur-Sensor (`read_temperature(sensor_id)` → °C) ✅
  - Feuchtigkeits-Sensor (`read_humidity(sensor_id)` → 0–100 %) ✅
  - Bewegungs-Sensor (`read_motion(sensor_id)` → bool, optional) ✅
- [x] Tests ausführen und bestehen

### 6.5 Actuator-Control ✅

**Abhängigkeiten**: 6.2 (Hardware-Access-Abstraction) ✅

#### 6.5.1 Actuator-Controller ✅
- [x] Tests für Actuator-Controller schreiben (`fenrir/tests/actuator_controller_test.rs`)
- [x] `ActuatorController` implementieren (TDD) (`fenrir/src/actuators/controller.rs`)
  - LED-Control (`set_led(actuator_id, brightness)` 0.0–1.0) ✅
  - Relay-Control (`set_relay(actuator_id, on)`) ✅
  - Motor-Control (`set_motor(actuator_id, value)`, optional) ✅
- [x] Tests ausführen und bestehen

### 6.6 Fenrir-Script-Integration ✅

**Abhängigkeiten**: 6.3-6.5 (GPIO, Sensor, Actuator) ✅

#### 6.6.1 Fenrir-Script-API ✅
- [x] Tests für Fenrir-Script-API schreiben (`fenrir/tests/fenrir_script_api_test.rs`)
- [x] `FenrirScriptAPI` implementieren (TDD) (`fenrir/src/script/api.rs`)
  - Lua-Bindings (mlua UserData): Global `fenrir` ✅
  - Script-Funktionen: `fenrir:gpio_read(pin)`, `fenrir:gpio_write(pin, value)`, `fenrir:sensor_read(id)`, `fenrir:actuator_set(id, value)` ✅
- [x] Tests ausführen und bestehen

---

## Phase 7: Jörmungandr - Network/Communication Service

### 7.1 Jörmungandr-Service-Setup ✅ (Grundstruktur)

**Abhängigkeiten**: 4.3 (Script-Manager)

#### 7.1.1 Jörmungandr-Service-Initialisierung ✅
- [x] `jormungandr/src/main.rs` erstellen (Stub-Eintrittspunkt)
- [x] `jormungandr/src/lib.rs` erstellen
- [x] `jormungandr/src/http/` für HTTP-Requests erstellen (mod.rs Platzhalter)
- [x] `jormungandr/src/websocket/` für WebSocket-Verbindungen erstellen
- [x] `jormungandr/src/mqtt/` für MQTT-Communication erstellen
- [x] `jormungandr/Cargo.toml` (shared, tracing, tokio); Workspace-Member: jormungandr

### 7.2 HTTP-Client ✅

**Abhängigkeiten**: 7.1 (Jörmungandr-Service-Setup) ✅

#### 7.2.1 HTTP-Request-Handler ✅
- [x] Tests für HTTP-Handler schreiben (`jormungandr/tests/http_handler_test.rs`, mockito)
- [x] `HTTPRequestHandler` implementieren (TDD) (`jormungandr/src/http/handler.rs`, reqwest)
  - GET-Requests ✅
  - POST-Requests ✅
  - PUT/DELETE-Requests (optional) ✅
- [x] Tests ausführen und bestehen

### 7.3 WebSocket-Client ✅

**Abhängigkeiten**: 7.1 (Jörmungandr-Service-Setup) ✅

#### 7.3.1 WebSocket-Handler ✅
- [x] Tests für WebSocket-Handler schreiben (`jormungandr/tests/websocket_handler_test.rs`, lokaler Echo-Server)
- [x] `WebSocketHandler` implementieren (TDD) (`jormungandr/src/websocket/handler.rs`, tokio-tungstenite)
  - WebSocket-Connection aufbauen (`connect()`) ✅
  - WebSocket-Send/Receive (`send()`, `receive()`) ✅
  - WebSocket-Reconnection (`reconnect()`) ✅
- [x] Tests ausführen und bestehen

### 7.4 MQTT-Client ✅

**Abhängigkeiten**: 7.1 (Jörmungandr-Service-Setup) ✅

#### 7.4.1 MQTT-Handler ✅
- [x] Tests für MQTT-Handler schreiben (`jormungandr/tests/mqtt_handler_test.rs`, ohne Broker)
- [x] `MQTTHandler` implementieren (TDD) (`jormungandr/src/mqtt/handler.rs`, rumqttc)
  - MQTT-Connection aufbauen (`connect(host, port)`, EventLoop im Hintergrund) ✅
  - MQTT-Publish (`publish(topic, payload)`) ✅
  - MQTT-Subscribe (`subscribe(topic)`) ✅
- [x] Tests ausführen und bestehen (publish/subscribe vor connect → NotConnected)

### 7.5 Jörmungandr-Script-Integration ✅

**Abhängigkeiten**: 7.2-7.4 (HTTP, WebSocket, MQTT)

#### 7.5.1 Jörmungandr-Script-API ✅
- [x] Tests für Jörmungandr-Script-API schreiben (`jormungandr/tests/jormungandr_script_api_test.rs`)
- [x] `JormungandrScriptAPI` implementieren (TDD) (`jormungandr/src/script/api.rs`)
  - Lua-Bindings für HTTP, WebSocket, MQTT ✅
  - Script-Funktionen: `jormungandr:http_get/post/put/delete`, `ws_connect/send/receive/reconnect`, `mqtt_connect/publish/subscribe` ✅
- [x] Tests ausführen und bestehen

---

## Phase 8: Hel - Data/Storage Service ✅

### 8.1 Hel-Service-Setup ✅

**Abhängigkeiten**: 4.3 (Script-Manager)

#### 8.1.1 Hel-Service-Initialisierung ✅
- [x] `hel/src/main.rs` erstellen
- [x] `hel/src/lib.rs` erstellen
- [x] `hel/src/filesystem/` für File-System-Operationen erstellen
- [x] `hel/src/storage/` für Daten-Speicherung erstellen
- [x] `hel/src/cache/` für Cache-Management erstellen
- [x] Hel zum Loki-Workspace und Dockerfile.test hinzugefügt

### 8.2 Filesystem-Access ✅

**Abhängigkeiten**: 8.1 (Hel-Service-Setup)

#### 8.2.1 Filesystem-Handler ✅
- [x] Tests für Filesystem-Handler schreiben (`hel/tests/filesystem_handler_test.rs`)
- [x] `FilesystemHandler` implementieren (TDD) (`hel/src/filesystem/handler.rs`)
  - File-Read, File-Write, File-Delete ✅
  - Directory-Operations (list_dir, create_dir) ✅
  - Path-Escape-Schutz (.., absolute paths) ✅
- [x] Tests ausführen und bestehen

### 8.3 Data-Storage ✅

**Abhängigkeiten**: 8.1 (Hel-Service-Setup)

#### 8.3.1 Storage-Manager ✅
- [x] Tests für Storage-Manager schreiben (`hel/tests/storage_manager_test.rs`)
- [x] `StorageManager` implementieren (TDD) (`hel/src/storage/manager.rs`)
  - Key-Value-Storage (in-memory + persistent JSON) ✅
  - get, set, remove, keys ✅
- [x] Tests ausführen und bestehen

### 8.4 Cache-Management ✅

**Abhängigkeiten**: 8.1 (Hel-Service-Setup)

#### 8.4.1 Cache-Manager ✅
- [x] Tests für Cache-Manager schreiben (`hel/tests/cache_manager_test.rs`)
- [x] `CacheManager` implementieren (TDD) (`hel/src/cache/manager.rs`)
  - In-Memory-Cache ✅
  - TTL-basierte Cache-Expiration (set_with_ttl) ✅
  - Cache-Invalidierung (invalidate, invalidate_all) ✅
- [x] Tests ausführen und bestehen

### 8.5 Hel-Script-Integration ✅

**Abhängigkeiten**: 8.2-8.4 (Filesystem, Storage, Cache)

#### 8.5.1 Hel-Script-API ✅
- [x] Tests für Hel-Script-API schreiben (`hel/tests/hel_script_api_test.rs`)
- [x] `HelScriptAPI` implementieren (TDD) (`hel/src/script/api.rs`)
  - Lua-Bindings: Global `hel` ✅
  - fs_read, fs_write, fs_delete, fs_list_dir, fs_create_dir ✅
  - storage_get, storage_set, storage_remove, storage_keys ✅
  - cache_get, cache_set, cache_invalidate, cache_invalidate_all ✅
- [x] Tests ausführen und bestehen

---

## Phase 9: Service Coordination (Main Loki) ✅

### 9.1 Service Coordinator ✅

**Abhängigkeiten**: 6.6, 7.5, 8.5 (Fenrir, Jörmungandr, Hel Script-APIs)

#### 9.1.1 Coordinator-Implementation ✅
- [x] Tests für Coordinator schreiben (`loki/tests/service_coordinator_test.rs`)
- [x] `ServiceCoordinator` implementieren (TDD) (`loki/src/coordination/coordinator.rs`)
  - In-process Script-APIs (Fenrir, Jörmungandr, Hel) mit Stub/Default-Config ✅
  - Script-Routing via `route(script)` vor Execution ✅
  - `execute_script(script)` registriert benötigte APIs in Lua ✅
  - Health-Check `health_check()` (Fenrir/Jörmungandr/Hel verfügbar) ✅
- [x] Tests ausführen und bestehen

### 9.2 Script-Routing ✅

**Abhängigkeiten**: 9.1 (Service Coordinator)

#### 9.2.1 Script-Router ✅
- [x] Tests für Script-Router schreiben (`coordination/router.rs` #[cfg(test)])
- [x] `route(script_source)` implementieren (TDD) (`loki/src/coordination/router.rs`)
  - Scan nach `fenrir:`, `jormungandr:`, `hel:` → ScriptRoute { fenrir, jormungandr, hel } ✅
  - Fenrir für Hardware-Scripts, Jörmungandr für Network-Scripts, Hel für Storage-Scripts ✅
- [x] Tests ausführen und bestehen

---

## Phase 10: Error Handling & Resilience ✅

### 10.1 Error-Handler ✅

**Abhängigkeiten**: 3.2 (Main gRPC Server)

#### 10.1.1 Error-Handler-Implementation ✅
- [x] Tests für Error-Handler schreiben (`loki/tests/error_handler_test.rs`)
- [x] `ErrorHandler` implementieren (TDD) (`loki/src/error_handler/handler.rs`)
  - gRPC-Error-Mapping: LokiError → tonic::Status (Internal, NotFound, ResourceExhausted, InvalidArgument, Unavailable, FailedPrecondition) ✅
  - Script-Execution-Fehler, Resource-Exhaustion-Fehler, Network/ServiceUnavailable ✅
  - `from_dyn_error` für generische Fehler ✅
- [x] gRPC-Server nutzt ErrorHandler für ExecuteScript-Fehler ✅
- [x] Tests ausführen und bestehen

### 10.2 Connection-Resilience ✅

**Abhängigkeiten**: 3.2 (Main gRPC Server)

#### 10.2.1 Connection-Resilience-Manager ✅
- [x] Tests für Connection-Resilience schreiben (`loki/tests/connection_resilience_test.rs`)
- [x] `ConnectionResilienceManager` implementieren (TDD) (`loki/src/resilience/manager.rs`)
  - Exponential-Backoff-Retry (`run_with_retry`) ✅
  - Retriable-Erkennung (ServiceUnavailable, IoError) ✅
- [x] Tests ausführen und bestehen

---

## Phase 11: Performance Optimization ✅

### 11.1 Script-Caching ✅

**Abhängigkeiten**: 4.3 (Script-Manager)

#### 11.1.1 Script-Cache ✅
- [x] Tests für Script-Cache schreiben (`loki/tests/script_cache_test.rs`)
- [x] Script-Caching (ScriptDefinition-Cache pro Tool-Name, LRU-Eviction bei max_cache_size) ✅
  - Cache-Invalidierung bei Script-Updates: `invalidate_script(name)` ✅
  - RegisterScript invalidiert Cache-Eintrag, nächste Execution lädt frisch ✅
- [x] Tests: Cache-Hit bei zweiter Execution, Invalidierung lädt neue Version, clear_cache ✅
- [x] Tests ausführen und bestehen

### 11.2 Memory-Optimization ✅

**Abhängigkeiten**: 5.2 (Resource-Monitoring)

#### 11.2.1 Memory-Profiling & Optimization ✅
- [x] Memory-Stabilitätstest schreiben (`loki/tests/memory_optimization_test.rs`)
  - 100 wiederholte Script-Execution ohne unbounded growth, Cache bleibt begrenzt ✅
- [x] Memory-Tests ausführen und bestehen

---

## Phase 12: Monitoring & Logging

### 12.1 Minimal-Logging

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 12.1.1 Logging Setup (Minimal) ✅
- [x] Minimal-Logging konfigurieren (`src/utils/logging.rs`) ✅ - Verwendet `tracing-subscriber` mit kompaktem Format
- [x] Nur kritische Log-Levels (ERROR, WARN) ✅ - Default Filter auf WARN, minimiert Overhead für IoT-Devices
- [x] Kompaktes Format ✅ - Keine Thread-IDs, File-Names, Line-Numbers für minimalen Speicherverbrauch
- [x] Integration in main.rs ✅ - `init_minimal_logging()` wird beim Start aufgerufen
- [x] Tests ✅ (`tests/logging_test.rs`)
- [ ] Structured-Logging (optional - kann später ergänzt werden)

### 12.2 Performance-Monitoring ✅

**Abhängigkeiten**: 5.2 (Resource-Monitoring)

#### 12.2.1 Performance-Monitor ✅
- [x] Tests für Performance-Monitor schreiben (`loki/tests/performance_monitor_test.rs`)
- [x] `PerformanceMonitor` implementieren (TDD) (`loki/src/resources/performance_monitor.rs`)
  - Script-Execution-Zeiten tracken (`record_execution`, `get_metrics`) ✅
  - Performance-Metriken sammeln (`ScriptMetrics`, `PerformanceMetrics`, `avg_duration_ms`) ✅
  - Execution-Limit-Check (`exceeds_execution_limit`) ✅
- [x] Tests ausführen und bestehen

---

## Phase 13: Documentation ✅

### 13.1 Service Documentation ✅

**Abhängigkeiten**: Alle vorherigen Phasen

#### 13.1.1 Service-Dokumentation ✅
- [x] Service-Overview dokumentieren (README Status ~74%, Phasen-Übersicht)
- [x] Tool-Config-Format dokumentieren (`docs/TOOL_CONFIG.md` – TOML-Schema)
- [x] Script-API dokumentieren (Fenrir, Jörmungandr, Hel) (`docs/SCRIPT_API.md`)
- [x] Integration: README „Integration“ (Jotunheim, Midgard, gRPC)

### 13.2 Examples ✅

**Abhängigkeiten**: Alle vorherigen Phasen

#### 13.2.1 Example-Scripts ✅
- [x] Hardware-Script-Beispiele (Fenrir): `examples/fenrir_led.lua`, `examples/fenrir_sensor.lua`
- [x] Network-Script-Beispiele (Jörmungandr): `examples/jormungandr_http.lua`, `examples/jormungandr_mqtt.lua`
- [x] Storage-Script-Beispiele (Hel): `examples/hel_file.lua`, `examples/hel_cache.lua`
- [x] `examples/README.md` mit Kurzbeschreibung

---

## Phase 14: Testing & Quality Assurance ✅

### 14.1 Integration Testing ✅

**Abhängigkeiten**: Alle vorherigen Phasen

#### 14.1.1 End-to-End Tests ✅
- [x] E2E-Tests für Script-Execution schreiben (`loki/tests/e2e_script_execution_test.rs`)
  - Tool-Registry laden → Script ausführen → Result (`e2e_tool_registry_load_execute_result`) ✅
  - Hardware-Script (Fenrir) (`e2e_coordinator_fenrir_hardware_script`) ✅
  - Network-Script (Jörmungandr HTTP mit mockito) (`e2e_coordinator_jormungandr_http_script`) ✅
  - Storage-Script (Hel) (`e2e_coordinator_hel_storage_script`) ✅
- [x] E2E-Tests ausführen und bestehen

### 14.2 Performance Testing ✅

**Abhängigkeiten**: 11.1 (Script-Caching)

#### 14.2.1 Performance Test Suite ✅
- [x] Performance-Tests schreiben (`loki/tests/performance_script_execution_test.rs`)
  - 50 Script-Executionen, alle erfolgreich, Gesamtzeit < 60s ✅
- [x] Memory-Usage: `memory_optimization_test.rs` (100 Executionen) ✅
- [x] Performance-Tests bestehen

### 14.3 Security Testing ✅

**Abhängigkeiten**: 5.3 (Resource-Enforcement)

#### 14.3.1 Security Test Suite ✅
- [x] Security-Tests schreiben (`loki/tests/security_script_execution_test.rs`)
  - Input-Validation: ungültiges Lua → Fehler ✅
  - Path-Escape: `hel:fs_read('..')` → Fehler ✅
- [x] Resource-Limit-Enforcement: `resource_enforcer_test.rs`, `resource_limits_test.rs` ✅
- [x] Security-Tests bestehen

---

## Verbleibende optionale Punkte (Übersicht)

- [ ] Phase 14 Performance-Benchmarks auf Ziel-Hardware (optional)
- [ ] Weitere Dokumentation (optional)

---

## Zusammenfassung

**Gesamtanzahl Phasen**: 14
**Gesamtanzahl Schritte**: ~250+

**Kritische Abhängigkeiten**:
1. Protobuf-Rust-Tool (prost-lite+tonic empfohlen)
2. Primary-Script-Engine (mlua empfohlen)
3. Config-File-Format (JSON empfohlen)
4. Script-Storage (Local Filesystem empfohlen)

**Offene Fragen für USER**:
1. Protobuf-Rust-Tool (prost-lite+tonic, prost+tonic)
2. Primary-Script-Engine (mlua, rlua, Eigene)
3. Config-File-Format (JSON, YAML, TOML)
4. Script-Storage (Local Filesystem, Inline, Beides)

**Hinweise**:
- Alle Schritte folgen TDD
- Alle Tests in Containern
- Extrem lightweight: minimaler Footprint für IoT-Devices
- 3 Sub-Services (Fenrir, Jörmungandr, Hel) koordiniert von Loki
- Dynamische gRPC-Funktionen für jedes User-Script
- Resource-Management mit strikten Limits
- Lua als primäre Script-Sprache (für IoT-Devices)
