# IMPLEMENTATION_PLAN - Thor (Action Executor)

## Übersicht

Thor ist der Action Executor - er führt Actions aus (File-Operations, Application-Control, System-Commands, Network-Operations, etc.).

**Programmiersprache**: Rust

## Entschiedene Konfiguration
✅ **Protobuf-Rust-Tool**: prost + tonic
✅ **Sandboxing-Library**: bubblewrap (robustes Linux-Sandboxing)

---

## Phase 1: Projekt-Setup (8 Schritte)
- [x] Cargo-Projekt
- [x] Dependencies (tokio, tonic, serde, tracing, anyhow, system-APIs)
- [x] Verzeichnisstruktur (`src/actions/`, `src/file/`, `src/app/`, `src/system/`, `src/network/`)
- [x] Test-Infrastruktur
  - [x] Container-Setup für Tests (Dockerfile.test, docker-compose.test.yml)
  - [x] Mock-Services in Containern (Heimdall, Jotunheim)
  - [ ] **WICHTIG**: Alle Tests müssen in Containern laufen - keine lokalen Dependencies auf der Entwicklungsmaschine
- [x] Settings-System

## Phase 2: Protobuf & gRPC (8 Schritte) ✅
- [x] Protobuf-Definitions (`thor.proto`, ThorAction, ThorResult)
- [x] gRPC-Server (Thor)
- [x] gRPC-Client (Heimdall für Permissions) – `proto/heimdall_authorization.proto`, `src/heimdall_authz.rs`, `PermissionChecker::check_permission()` ruft Heimdall `AuthorizationService.CheckPermission` auf; `new_allow_on_connection_error()` für Tests ohne Heimdall

## Phase 3: Action-Framework (10 Schritte) ✅
- [x] Tests für Action-Framework schreiben (`tests/unit/cross_device_test.rs`, `tests/integration/action_execution_test.rs`)
- [x] `ActionExecutor` Trait (TDD) – `src/actions/executor.rs`
- [x] `ActionRegistry` (TDD, Register-Actions) – `src/actions/registry.rs`
- [x] `ActionDispatcher` (TDD, Dispatch-Actions) – `src/actions/dispatcher.rs`

## Phase 4: File-Operations (10 Schritte) ✅
- [x] Tests für File-Actions (integration/action_execution_test, file actions)
- [x] `FileActionExecutor` (TDD, Read, Write, Delete, Move, Copy) – `src/file/actions.rs`

## Phase 5: Application-Control (8 Schritte) ✅
- [x] Tests für App-Control (integration)
- [x] `AppControlExecutor` (TDD, Start-App, Stop-App, Query-Status) – `src/app/actions.rs`

## Phase 6: System-Commands (8 Schritte) ✅
- [x] Tests für System-Commands (integration)
- [x] `SystemCommandExecutor` (TDD, Execute-Command, Get-System-Info) – `src/system/actions.rs`

## Phase 7: Network-Operations (8 Schritte) ✅
- [x] Tests für Network-Actions (integration)
- [x] `NetworkActionExecutor` (TDD, HTTP-Request, DNS-Query) – `src/network/actions.rs`

## Phase 8: Permission-System & Sandboxing (10 Schritte) ✅ (ohne Sandboxing)
- [x] Tests für Permission-Checker (`tests/unit/permission_checker_test.rs`)
- [x] `PermissionChecker` – echte Heimdall-gRPC-Anfrage (`AuthorizationService.CheckPermission`), Fallback `new_allow_on_connection_error()` für Tests ohne Heimdall – `src/permissions/checker.rs`
- [ ] **TODO**: Sandboxing (optional, TDD)

## Phase 9: Cross-Device-Actions (8 Schritte) ✅
- [x] Tests für Cross-Device-Actions schreiben (`tests/unit/cross_device_test.rs`, integration)
- [x] `CrossDeviceActionHandler` (TDD, Receive-Actions-via-gRPC, Execute, Send-Results) – `src/cross_device/handler.rs`

## Phase 10: Performance & Security (6 Schritte) ⚠️ TEILWEISE
- [ ] Async-Processing (bereits async/await in Dispatcher und Executors)
- [x] Audit-Logging – `src/audit.rs` (Trait `AuditLogger`, `TracingAuditLogger`), integriert in `ActionDispatcher::new_with_audit`, konfigurierbar via `enable_audit_logging` in Settings
- [x] Error-Handling (bereits `ActionError`, `PermissionError`, Fehlerbehandlung in Dispatcher und gRPC-Server)

## Phase 11: Terminal-Operations (12 Schritte) ✅

**Abhängigkeiten**: Phase 3 (Action-Framework)

### 11.1 PTY-Management ✅
- [x] Tests für PTY-Wrapper (`tests/unit/terminal_test.rs`: PTY-Setup, Resize, Read-Output)
- [x] `PtyWrapper` – `src/terminal/pty.rs` (portable-pty, Rows/Columns)
- [x] PTY-Setup, Resize, Wait, Read-Output
- [x] Tests ausführen und bestehen

### 11.2 Interactive-Program-Execution ✅
- [x] `InteractiveExecutor` – `src/terminal/executor.rs` (PTY-basierte Execution)
- [x] Async Execution, PTY-Wrapper-Integration
- [x] Tests in terminal_test.rs

### 11.3 Terminal-Action-Handler ✅
- [x] Tests für TERMINAL_OPERATION (`tests/unit/terminal_test.rs`)
- [x] `TerminalActionHandler` – `src/terminal/handler.rs` (Action-Parsing, Interactive vs. Non-Interactive, PTY/Standard-Execution)
- [x] In ActionRegistry registriert (`main.rs`)
- [x] Tests ausführen und bestehen

---

## Phase 12: UI-Automation (15 Schritte) ⚠️ TEILWEISE

**Abhängigkeiten**: Phase 3 (Action-Framework)

### 12.1 Platform-Detection ✅
- [x] Platform-Detection – `src/ui_automation/platform/mod.rs` (OperatingSystem, OperatingSystemDetector, Windows/macOS/Linux)
- [x] `UIAutomationHandler` nutzt Platform für Routing
- [x] In ActionRegistry registriert

### 12.2 Windows UI-Automation ⚠️
- [x] Struktur: `src/ui_automation/platform/windows.rs`, Handler-Routing nach OS
- [ ] **TODO**: Echte Windows-UI-Automation (Click, Type, Move-Cursor) – aktuell Stub "not yet fully implemented"

### 12.3 macOS Accessibility-API
- [ ] Tests für macOS Accessibility schreiben
- [ ] `MacOSAccessibility` implementieren (TDD)
  - `cocoa` crate Integration
  - Accessibility API Calls
- [ ] Tests ausführen und bestehen

### 12.4 Linux AT-SPI
- [ ] Tests für Linux AT-SPI schreiben
- [ ] `LinuxATSPI` implementieren (TDD)
  - `atspi` crate Integration
  - AT-SPI API Calls
- [ ] Tests ausführen und bestehen

### 12.5 UI-Action-Handler
- [ ] Tests für UI_AUTOMATION schreiben
- [ ] `UIAutomationHandler` implementieren (TDD)
  - Platform-Dispatch (Windows/macOS/Linux)
  - Action-Type-Handling (Click, Type, Move, etc.)
  - Heimdall-Permission-Checks
  - User-Confirmation für kritische Actions
- [ ] In ActionRegistry registrieren
- [ ] Tests ausführen und bestehen

---

## Phase 13: Scheduler-Operations (10 Schritte) ⚠️ TEILWEISE

**Abhängigkeiten**: Phase 3 (Action-Framework)

### 13.1 Cron-Integration (Linux/macOS) ✅
- [x] Tests für Cron-Integration schreiben
  - Create/Delete/Update/List Cron-Jobs (mit InMemoryCrontabStore, container-tauglich)
  - Crontab-Parsing (`parse_crontab`, `format_crontab`, Thor-Format "# Thor job: name=<name>")
  - Cron-Expression-Validation (`cron-parser` crate)
- [x] `CronScheduler` implementieren (TDD)
  - `cron-parser` crate Integration für Expression-Validation
  - `CrontabStore`-Trait + `InMemoryCrontabStore` für tests ohne lokale crontab
  - create_job/delete_job/list_jobs/update_job mit optionalem Store
- [x] Tests ausführen und bestehen

### 13.2 Windows Task-Scheduler
- [ ] Tests für Task-Scheduler schreiben
- [ ] `WindowsTaskScheduler` implementieren (TDD)
  - `windows-service` crate Integration
  - Task-Scheduler-API-Calls
- [ ] Tests ausführen und bestehen

### 13.3 macOS launchd
- [ ] Tests für launchd schreiben
- [ ] `LaunchdScheduler` implementieren (TDD)
  - launchctl-Integration
  - plist-File-Generation
- [ ] Tests ausführen und bestehen

### 13.4 Google-Calendar (optional)
- [ ] Tests für Google-Calendar schreiben
- [ ] `GoogleCalendarScheduler` implementieren (TDD)
  - `google-calendar3` crate Integration
  - OAuth-Flow
  - Event-Creation/Deletion
- [ ] Tests ausführen und bestehen

### 13.5 Scheduler-Action-Handler ✅
- [x] Tests für SCHEDULER_OPERATION schreiben (`tests/unit/scheduler_test.rs`)
- [x] `SchedulerActionHandler` implementieren (TDD)
  - Platform-Dispatch (Cron/TaskScheduler/launchd) – Cron mit optionalem Store, Windows/macOS stubbed
  - Action-Type-Handling (Create/Delete/Update/List)
  - Parameter-Parsing (operation, job_name, schedule, command, operating_system)
- [x] In ActionRegistry registrieren (`main.rs`)
- [x] Tests ausführen und bestehen
- **Hinweis**: Echte Crontab-Persistenz (z. B. `crontab -l`/`crontab -`) kann über eine spätere `CrontabStore`-Implementierung ergänzt werden; Tests laufen mit `InMemoryCrontabStore` ohne lokale crontab.

---

## Phase 14: Jotunheim-Integration (8 Schritte) ✅ (ohne Device-Registry)

**Abhängigkeiten**: Phase 3 (Action-Framework)

### 14.1 Jotunheim-gRPC-Client ✅
- [x] Tests für Jotunheim-Client schreiben (`tests/unit/jotunheim_test.rs`)
  - Connect: unreachable URL → Err (kein laufender Service nötig)
  - Handler execute mit unreachable URL → Err
- [x] `JotunheimClient` implementieren – `src/jotunheim/client.rs`
  - gRPC-Client (proto `jotunheim.proto`: SendDeviceCommand, GetDeviceStatus, CallTool)
  - Device-Command-Sending, Status-Queries, Tool-Call
- [x] Tests ausführen und bestehen (container-tauglich)

### 14.2 Device-Registry-Integration (optional / TODO)
- [ ] Tests für Device-Registry schreiben (List Devices, Capabilities, Status)
- [ ] Device-Registry-Integration (Heimdall-Device-Registry-Client, Device-Caching)
- **Hinweis**: Kann nachgelagert ergänzt werden; Handler arbeitet mit direkter device_id.

### 14.3 Jotunheim-Action-Handler ✅
- [x] Tests für JOTUNHEIM_OPERATION schreiben
  - device_command, device_status, tool_call (mit unreachable URL → Err)
  - invalid/missing operation → Err
- [x] `JotunheimActionHandler` implementieren – `src/jotunheim/handler.rs`
  - Device-Command-Execution, Device-Status-Query, Tool-Calling (operation + device_id/tool_name/parameters)
  - Parameter-Parsing (JSON), gRPC-Requests an JotunheimClient
- [x] In ActionRegistry registrieren (`main.rs`, wenn `jotunheim_url` gesetzt)
- [x] Tests ausführen und bestehen

---

## Phase 15: Documentation & Testing (6 Schritte) ⚠️ TEILWEISE

**Abhängigkeiten**: Phase 14 (alle Action-Types implementiert)

- [x] Dokumentation für alle neuen Action-Types (README: TERMINAL_OPERATION, UI_AUTOMATION, SCHEDULER_OPERATION, JOTUNHEIM_OPERATION)
- [x] E2E-/Integration-Tests (Action-Request → Dispatch → Result in `tests/integration/action_execution_test.rs`)
- [x] Security-Test: Permission-Deny (PermissionChecker::new_deny_all, dispatch → Err mit "permission")
- [ ] Performance-Tests (Latency, Throughput) – optional
- [ ] Integration-Tests (Odin → Thor → Services) – optional

---

**Schritte gesamt**: ~135
**Phasen**: 15

**Hinweise**:
- Alle Schritte folgen TDD (Tests zuerst, dann Implementierung)
- **ALLE Tests müssen in Containern laufen** - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren
- Container-Setup muss vollständig isolierte Test-Umgebung bereitstellen
