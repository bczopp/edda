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

## Phase 1: Projekt-Setup & Grundstruktur

### 1.1 Projekt-Initialisierung

**Abhängigkeiten**: Keine
**Erforderliche USER-Eingaben**: Protobuf-Rust-Tool, Primary-Script-Engine

#### 1.1.1 Cargo-Workspace erstellen
- [ ] `Cargo.toml` mit Workspace erstellen
  - `loki/` (Main Coordinating Service)
  - `fenrir/` (Hardware-Control Service)
  - `jormungandr/` (Network/Communication Service)
  - `hel/` (Data/Storage Service)
  - `shared/` (Shared Code zwischen den Services)
- [ ] Basis-Dependencies definieren
  - Async Runtime (tokio-minimal)
  - gRPC Server (tonic-light, prost-lite)
  - Script Engine (mlua oder rlua)
  - Serialization (serde-minimal)
  - Logging (defmt oder log-minimal)
  - Error-Handling (anyhow-minimal)
- [ ] `.gitignore` erstellen

#### 1.1.2 Verzeichnisstruktur erstellen
- [ ] `loki/src/main.rs` erstellen
- [ ] `loki/src/lib.rs` erstellen
- [ ] `loki/src/grpc/` für gRPC-Server erstellen
- [ ] `loki/src/script/` für Script-Management erstellen
- [ ] `loki/src/coordinator/` für Koordination der 3 Services erstellen
- [ ] `loki/src/config/` für Tool-Config-Loading erstellen
- [ ] `loki/src/resource/` für Resource-Management erstellen
- [ ] `loki/src/utils/` für Utilities erstellen
- [ ] `shared/src/lib.rs` erstellen
- [ ] `shared/src/error.rs` für gemeinsame Error-Typen erstellen
- [ ] `shared/src/models.rs` für gemeinsame Script-Models erstellen

#### 1.1.3 Build-System einrichten
- [ ] Build-Scripts für Protobuf-Code-Generierung erstellen
- [ ] Code-Generierungs-Pipeline einrichten (Protobuf → Rust)
- [ ] Cargo-Features definieren (z.B. `esp32`, `desktop`, `lua`, `python`)

### 1.2 Test-Infrastruktur

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.2.1 Container-Setup für Tests
- [ ] `Dockerfile` für Test-Umgebung erstellen
- [ ] Docker Compose für Test-Services konfigurieren
  - Mock-Platform-Client (Jotunheim/Midgard)
  - Test-Filesystem für Scripts
  - Mock-Hardware für Fenrir-Tests
- [ ] Test-Container-Startup-Scripts erstellen
- [ ] **WICHTIG**: Alle Tests müssen in Containern laufen - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren

#### 1.2.2 Test-Framework konfigurieren
- [ ] Test-Dependencies hinzufügen
- [ ] Test-Utilities und Helpers erstellen
- [ ] Mock-Setup für Services
- [ ] Test-Script-Generators erstellen

#### 1.2.3 CI/CD-Pipeline
- [ ] GitHub Actions / GitLab CI Workflow erstellen
- [ ] Automatische Test-Ausführung bei Commits konfigurieren
- [ ] Code-Coverage-Reporting einrichten
- [ ] Linting und Formatting (cargo clippy, cargo fmt)

### 1.3 Settings-System

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.3.1 Settings-Schema definieren
- [ ] Settings-Struktur entwerfen (JSON)
  - resource_limits
  - script_storage_path
  - children_config (Fenrir, Jörmungandr, Hel)

#### 1.3.2 Settings-Validierung
- [ ] Tests für Settings-Validierung schreiben
- [ ] Rust-Structs für Settings definieren
- [ ] Settings-Validator implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 1.3.3 Settings-Loader & Hot-Reload
- [ ] Tests für Settings-Loader schreiben
- [ ] Settings-Loader implementieren (TDD)
  - File-Watcher für Hot-Reload
  - Runtime-Settings-Reload
- [ ] Tests ausführen und bestehen

---

## Phase 2: Tool-Konfigurationsdatei

### 2.1 Config-Schema

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)
**Erforderliche USER-Eingaben**: Config-File-Format

#### 2.1.1 Tool-Config-Schema definieren
- [ ] Tests für Config-Schema schreiben
- [ ] Tool-Config-Schema entwerfen (JSON/YAML/TOML)
  - Tool-Name
  - Tool-Beschreibung
  - Tool-Parameter (Name, Type, Required, Description)
  - Tool-Return-Type
  - Script-Path oder Inline-Script
- [ ] Tests ausführen und bestehen

### 2.2 Config-Loader

**Abhängigkeiten**: 2.1 (Config-Schema)
**Erforderliche USER-Eingaben**: Script-Storage

#### 2.2.1 Config-Loader-Implementation
- [ ] Tests für Config-Loader schreiben
- [ ] `ToolConfigLoader` implementieren (TDD)
  - Konfigurationsdatei laden (JSON/YAML/TOML)
  - Schema-Validierung
  - Script-Path-Resolution
- [ ] Tests ausführen und bestehen

### 2.3 Config-Validation

**Abhängigkeiten**: 2.2 (Config-Loader)

#### 2.3.1 Config-Validator
- [ ] Tests für Config-Validator schreiben
- [ ] `ToolConfigValidator` implementieren (TDD)
  - Tool-Definitionen validieren
  - Parameter-Validierung
  - Return-Type-Validierung
  - Script-Existenz-Checks
- [ ] Tests ausführen und bestehen

### 2.4 Config-Hot-Reload

**Abhängigkeiten**: 2.2 (Config-Loader)

#### 2.4.1 Config-File-Watcher
- [ ] Tests für File-Watcher schreiben
- [ ] `ConfigFileWatcher` implementieren (TDD)
  - Überwacht Konfigurationsdatei auf Änderungen
  - Hot-Reload bei Änderungen
  - gRPC-Funktionen aktualisieren
- [ ] Tests ausführen und bestehen

---

## Phase 3: Protobuf & gRPC Services

### 3.1 Shared Protobuf Definitions

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 3.1.1 Shared Protobuf-Projekt verwenden
❓ **HINWEIS**: Protobuf-Definitions sollten in einem separaten Projekt sein
- [ ] Loki als Dependency zu Protobuf-Projekt hinzufügen
- [ ] Protobuf-Definitions importieren (minimal subset für IoT)

#### 3.1.2 Loki Service Protocol definieren
- [ ] `LokiService.proto` definieren
  - `GetCapabilities()` RPC
  - `GetChildrenStatus()` RPC
  - `ListScripts()` RPC
  - `RegisterScript()` RPC
  - Dynamische Script-Funktionen (zur Laufzeit generiert)
  - `ScriptInput` / `ScriptOutput` Messages
  - `ScriptChunk` / `ScriptResult` Messages (für Streaming)
- [ ] Code-Generierung konfigurieren (Protobuf-Lite)

### 3.2 gRPC Server Implementation

**Abhängigkeiten**: 3.1 (Shared Protobuf Definitions), 2.2 (Config-Loader)

#### 3.2.1 Main Loki gRPC Server
- [ ] Tests für Main gRPC Server schreiben
- [ ] `LokiGrpcServer` implementieren (TDD)
  - Statische Methoden implementieren
  - Dynamische Script-Funktionen zur Laufzeit generieren
  - Script-Execution routing (zu Fenrir/Jörmungandr/Hel)
- [ ] Tests ausführen und bestehen

### 3.3 Dynamic gRPC Function Generation

**Abhängigkeiten**: 3.2 (Main gRPC Server)

#### 3.3.1 Function-Generator
- [ ] Tests für Function-Generator schreiben
- [ ] `DynamicFunctionGenerator` implementieren (TDD)
  - Jedes Script wird zu `Script_<script_name>()` RPC
  - Streaming-Support: `StreamScript_<script_name>()` RPC
  - Funktion-Registration bei gRPC-Server
- [ ] Tests ausführen und bestehen

---

## Phase 4: Script-Execution Engine

### 4.1 Script-Engine-Abstraction

**Abhängigkeiten**: 2.2 (Config-Loader)
**Erforderliche USER-Eingaben**: Primary-Script-Engine

#### 4.1.1 Script-Engine-Trait
- [ ] Tests für Script-Engine-Trait schreiben
- [ ] `ScriptEngine` Trait definieren
  - `execute_script()` Methode
  - `load_script()` Methode
  - `validate_script()` Methode
- [ ] Tests ausführen und bestehen

### 4.2 Lua-Engine-Implementation

**Abhängigkeiten**: 4.1 (Script-Engine-Abstraction)

#### 4.2.1 Lua-Script-Engine
- [ ] Tests für Lua-Engine schreiben
- [ ] `LuaScriptEngine` implementieren (TDD)
  - mlua oder rlua Integration
  - Script-Loading von Filesystem oder Inline
  - Script-Execution mit Parameters
  - Return-Value-Handling
- [ ] Tests ausführen und bestehen

### 4.3 Script-Manager

**Abhängigkeiten**: 4.2 (Lua-Engine-Implementation)

#### 4.3.1 Script-Registry & Manager
- [ ] Tests für Script-Manager schreiben
- [ ] `ScriptManager` implementieren (TDD)
  - Script-Loading
  - Script-Caching (für Performance)
  - Script-Validation
  - Script-Execution mit Resource-Limits
- [ ] Tests ausführen und bestehen

---

## Phase 5: Resource-Management

### 5.1 Resource-Limits

**Abhängigkeiten**: 1.3 (Settings-System)

#### 5.1.1 Resource-Limits-Configuration
- [ ] Tests für Resource-Limits schreiben
- [ ] `ResourceLimits` implementieren (TDD)
  - CPU-Limits
  - Memory-Limits
  - Execution-Time-Limits
  - Disk-Limits
- [ ] Tests ausführen und bestehen

### 5.2 Resource-Monitoring

**Abhängigkeiten**: 5.1 (Resource-Limits)

#### 5.2.1 Resource-Monitor
- [ ] Tests für Resource-Monitor schreiben
- [ ] `ResourceMonitor` implementieren (TDD)
  - RAM-Usage überwachen
  - CPU-Usage überwachen
  - Execution-Time überwachen
  - Resource-Exhaustion erkennen
- [ ] Tests ausführen und bestehen

### 5.3 Resource-Enforcement

**Abhängigkeiten**: 5.2 (Resource-Monitoring)

#### 5.3.1 Resource-Enforcer
- [ ] Tests für Resource-Enforcer schreiben
- [ ] `ResourceEnforcer` implementieren (TDD)
  - Script-Execution abbrechen bei Resource-Exhaustion
  - Resource-Limits durchsetzen
  - Script-Queue bei knappen Ressourcen
- [ ] Tests ausführen und bestehen

---

## Phase 6: Fenrir - Hardware-Control Service

### 6.1 Fenrir-Service-Setup

**Abhängigkeiten**: 4.3 (Script-Manager)

#### 6.1.1 Fenrir-Service-Initialisierung
- [ ] `fenrir/src/main.rs` erstellen
- [ ] `fenrir/src/lib.rs` erstellen
- [ ] `fenrir/src/hardware/` für Hardware-Access erstellen
- [ ] `fenrir/src/gpio/` für GPIO-Control erstellen
- [ ] `fenrir/src/sensors/` für Sensor-Reading erstellen
- [ ] `fenrir/src/actuators/` für Actuator-Control erstellen

### 6.2 Hardware-Access-Abstraction

**Abhängigkeiten**: 6.1 (Fenrir-Service-Setup)

#### 6.2.1 Hardware-Access-Trait
- [ ] Tests für Hardware-Access-Trait schreiben
- [ ] `HardwareAccess` Trait definieren
  - `read_gpio()` Methode
  - `write_gpio()` Methode
  - `read_sensor()` Methode
  - `control_actuator()` Methode
- [ ] Tests ausführen und bestehen

### 6.3 GPIO-Control

**Abhängigkeiten**: 6.2 (Hardware-Access-Abstraction)

#### 6.3.1 GPIO-Controller
- [ ] Tests für GPIO-Controller schreiben
- [ ] `GPIOController` implementieren (TDD)
  - GPIO-Read
  - GPIO-Write
  - GPIO-PWM (optional)
- [ ] Tests ausführen und bestehen

### 6.4 Sensor-Reading

**Abhängigkeiten**: 6.2 (Hardware-Access-Abstraction)

#### 6.4.1 Sensor-Reader
- [ ] Tests für Sensor-Reader schreiben
- [ ] `SensorReader` implementieren (TDD)
  - Temperatur-Sensor
  - Feuchtigkeits-Sensor
  - Bewegungs-Sensor (optional)
- [ ] Tests ausführen und bestehen

### 6.5 Actuator-Control

**Abhängigkeiten**: 6.2 (Hardware-Access-Abstraction)

#### 6.5.1 Actuator-Controller
- [ ] Tests für Actuator-Controller schreiben
- [ ] `ActuatorController` implementieren (TDD)
  - LED-Control
  - Motor-Control (optional)
  - Relay-Control (optional)
- [ ] Tests ausführen und bestehen

### 6.6 Fenrir-Script-Integration

**Abhängigkeiten**: 6.3-6.5 (GPIO, Sensor, Actuator)

#### 6.6.1 Fenrir-Script-API
- [ ] Tests für Fenrir-Script-API schreiben
- [ ] `FenrirScriptAPI` implementieren (TDD)
  - Lua-Bindings für Hardware-Access
  - Script-Funktionen für GPIO, Sensors, Actuators
- [ ] Tests ausführen und bestehen

---

## Phase 7: Jörmungandr - Network/Communication Service

### 7.1 Jörmungandr-Service-Setup

**Abhängigkeiten**: 4.3 (Script-Manager)

#### 7.1.1 Jörmungandr-Service-Initialisierung
- [ ] `jormungandr/src/main.rs` erstellen
- [ ] `jormungandr/src/lib.rs` erstellen
- [ ] `jormungandr/src/http/` für HTTP-Requests erstellen
- [ ] `jormungandr/src/websocket/` für WebSocket-Verbindungen erstellen
- [ ] `jormungandr/src/mqtt/` für MQTT-Communication erstellen

### 7.2 HTTP-Client

**Abhängigkeiten**: 7.1 (Jörmungandr-Service-Setup)

#### 7.2.1 HTTP-Request-Handler
- [ ] Tests für HTTP-Handler schreiben
- [ ] `HTTPRequestHandler` implementieren (TDD)
  - GET-Requests
  - POST-Requests
  - PUT/DELETE-Requests (optional)
- [ ] Tests ausführen und bestehen

### 7.3 WebSocket-Client

**Abhängigkeiten**: 7.1 (Jörmungandr-Service-Setup)

#### 7.3.1 WebSocket-Handler
- [ ] Tests für WebSocket-Handler schreiben
- [ ] `WebSocketHandler` implementieren (TDD)
  - WebSocket-Connection aufbauen
  - WebSocket-Send/Receive
  - WebSocket-Reconnection
- [ ] Tests ausführen und bestehen

### 7.4 MQTT-Client

**Abhängigkeiten**: 7.1 (Jörmungandr-Service-Setup)

#### 7.4.1 MQTT-Handler
- [ ] Tests für MQTT-Handler schreiben
- [ ] `MQTTHandler` implementieren (TDD)
  - MQTT-Connection aufbauen
  - MQTT-Publish
  - MQTT-Subscribe
- [ ] Tests ausführen und bestehen

### 7.5 Jörmungandr-Script-Integration

**Abhängigkeiten**: 7.2-7.4 (HTTP, WebSocket, MQTT)

#### 7.5.1 Jörmungandr-Script-API
- [ ] Tests für Jörmungandr-Script-API schreiben
- [ ] `JormungandrScriptAPI` implementieren (TDD)
  - Lua-Bindings für HTTP, WebSocket, MQTT
  - Script-Funktionen für Netzwerk-Operationen
- [ ] Tests ausführen und bestehen

---

## Phase 8: Hel - Data/Storage Service

### 8.1 Hel-Service-Setup

**Abhängigkeiten**: 4.3 (Script-Manager)

#### 8.1.1 Hel-Service-Initialisierung
- [ ] `hel/src/main.rs` erstellen
- [ ] `hel/src/lib.rs` erstellen
- [ ] `hel/src/filesystem/` für File-System-Operationen erstellen
- [ ] `hel/src/storage/` für Daten-Speicherung erstellen
- [ ] `hel/src/cache/` für Cache-Management erstellen

### 8.2 Filesystem-Access

**Abhängigkeiten**: 8.1 (Hel-Service-Setup)

#### 8.2.1 Filesystem-Handler
- [ ] Tests für Filesystem-Handler schreiben
- [ ] `FilesystemHandler` implementieren (TDD)
  - File-Read
  - File-Write
  - File-Delete
  - Directory-Operations
- [ ] Tests ausführen und bestehen

### 8.3 Data-Storage

**Abhängigkeiten**: 8.1 (Hel-Service-Setup)

#### 8.3.1 Storage-Manager
- [ ] Tests für Storage-Manager schreiben
- [ ] `StorageManager` implementieren (TDD)
  - Key-Value-Storage (einfach, für IoT)
  - Data-Serialization/Deserialization
- [ ] Tests ausführen und bestehen

### 8.4 Cache-Management

**Abhängigkeiten**: 8.1 (Hel-Service-Setup)

#### 8.4.1 Cache-Manager
- [ ] Tests für Cache-Manager schreiben
- [ ] `CacheManager` implementieren (TDD)
  - In-Memory-Cache
  - TTL-basierte Cache-Expiration
  - Cache-Invalidierung
- [ ] Tests ausführen und bestehen

### 8.5 Hel-Script-Integration

**Abhängigkeiten**: 8.2-8.4 (Filesystem, Storage, Cache)

#### 8.5.1 Hel-Script-API
- [ ] Tests für Hel-Script-API schreiben
- [ ] `HelScriptAPI` implementieren (TDD)
  - Lua-Bindings für Filesystem, Storage, Cache
  - Script-Funktionen für Daten-Operationen
- [ ] Tests ausführen und bestehen

---

## Phase 9: Service Coordination (Main Loki)

### 9.1 Service Coordinator

**Abhängigkeiten**: 6.6, 7.5, 8.5 (Fenrir, Jörmungandr, Hel Script-APIs)

#### 9.1.1 Coordinator-Implementation
- [ ] Tests für Coordinator schreiben
- [ ] `ServiceCoordinator` implementieren (TDD)
  - Lifecycle-Management für alle 3 Services
  - Script-Routing (zu Fenrir/Jörmungandr/Hel basierend auf Script-Typ)
  - Health-Checks für alle Services
- [ ] Tests ausführen und bestehen

### 9.2 Script-Routing

**Abhängigkeiten**: 9.1 (Service Coordinator)

#### 9.2.1 Script-Router
- [ ] Tests für Script-Router schreiben
- [ ] `ScriptRouter` implementieren (TDD)
  - Automatisches Routing basierend auf Script-Funktionen
  - Fenrir für Hardware-Scripts
  - Jörmungandr für Network-Scripts
  - Hel für Storage-Scripts
- [ ] Tests ausführen und bestehen

---

## Phase 10: Error Handling & Resilience

### 10.1 Error-Handler

**Abhängigkeiten**: 3.2 (Main gRPC Server)

#### 10.1.1 Error-Handler-Implementation
- [ ] Tests für Error-Handler schreiben
- [ ] `ErrorHandler` implementieren (TDD)
  - gRPC-Error-Mapping
  - Script-Execution-Fehler
  - Resource-Exhaustion-Fehler
  - Network-Fehler
- [ ] Tests ausführen und bestehen

### 10.2 Connection-Resilience

**Abhängigkeiten**: 3.2 (Main gRPC Server)

#### 10.2.1 Connection-Resilience-Manager
- [ ] Tests für Connection-Resilience schreiben
- [ ] `ConnectionResilienceManager` implementieren (TDD)
  - Automatische Reconnection
  - Exponential-Backoff-Retry
  - Connection-Monitoring
- [ ] Tests ausführen und bestehen

---

## Phase 11: Performance Optimization

### 11.1 Script-Caching

**Abhängigkeiten**: 4.3 (Script-Manager)

#### 11.1.1 Script-Cache
- [ ] Tests für Script-Cache schreiben
- [ ] Script-Caching implementieren (TDD)
  - Compiled-Script-Caching (für Lua)
  - Cache-Invalidierung bei Script-Updates
- [ ] Tests ausführen und bestehen

### 11.2 Memory-Optimization

**Abhängigkeiten**: 5.2 (Resource-Monitoring)

#### 11.2.1 Memory-Profiling & Optimization
- [ ] Memory-Profiling-Tests schreiben
- [ ] Memory-Usage optimieren
  - Stack-Usage minimieren
  - Heap-Allocations minimieren
- [ ] Memory-Tests ausführen und Benchmarks erreichen

---

## Phase 12: Monitoring & Logging

### 12.1 Minimal-Logging

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 12.1.1 Logging Setup (Minimal)
- [ ] Minimal-Logging konfigurieren (defmt oder log-minimal)
- [ ] Nur kritische Log-Levels (ERROR, WARN)
- [ ] Structured-Logging (optional)

### 12.2 Performance-Monitoring

**Abhängigkeiten**: 5.2 (Resource-Monitoring)

#### 12.2.1 Performance-Monitor
- [ ] Tests für Performance-Monitor schreiben
- [ ] `PerformanceMonitor` implementieren (TDD)
  - Script-Execution-Zeiten tracken
  - Resource-Usage tracken
  - Performance-Metriken sammeln
- [ ] Tests ausführen und bestehen

---

## Phase 13: Documentation

### 13.1 Service Documentation

**Abhängigkeiten**: Alle vorherigen Phasen

#### 13.1.1 Service-Dokumentation
- [ ] Service-Overview dokumentieren
- [ ] Tool-Config-Format dokumentieren
- [ ] Script-API dokumentieren (Fenrir, Jörmungandr, Hel)
- [ ] Integration-Guides erstellen (Jotunheim, Midgard, etc.)

### 13.2 Examples

**Abhängigkeiten**: Alle vorherigen Phasen

#### 13.2.1 Example-Scripts
- [ ] Hardware-Script-Beispiele erstellen (Fenrir)
  - LED-Control-Beispiel
  - Sensor-Reading-Beispiel
- [ ] Network-Script-Beispiele erstellen (Jörmungandr)
  - HTTP-Request-Beispiel
  - MQTT-Beispiel
- [ ] Storage-Script-Beispiele erstellen (Hel)
  - File-Write-Beispiel
  - Cache-Beispiel

---

## Phase 14: Testing & Quality Assurance

### 14.1 Integration Testing

**Abhängigkeiten**: Alle vorherigen Phasen

#### 14.1.1 End-to-End Tests
- [ ] E2E-Tests für Script-Execution schreiben
  - Tool-Config laden → Script ausführen → Result zurückgeben
  - Hardware-Script-Execution (Fenrir)
  - Network-Script-Execution (Jörmungandr)
  - Storage-Script-Execution (Hel)
- [ ] E2E-Tests ausführen und bestehen

### 14.2 Performance Testing

**Abhängigkeiten**: 11.1 (Script-Caching)

#### 14.2.1 Performance Test Suite
- [ ] Performance-Tests ausführen
  - Script-Execution-Performance-Tests
  - Memory-Usage-Tests
  - Resource-Constraint-Tests
- [ ] Performance-Tests bestehen

### 14.3 Security Testing

**Abhängigkeiten**: 5.3 (Resource-Enforcement)

#### 14.3.1 Security Test Suite
- [ ] Security-Tests ausführen
  - Resource-Limit-Enforcement-Tests
  - Input-Validation-Tests
  - Script-Isolation-Tests
- [ ] Security-Tests bestehen

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
