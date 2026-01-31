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

## Phase 2: Protobuf & gRPC (8 Schritte)
- [x] Protobuf-Definitions (`thor.proto`, ThorAction, ThorResult)
- [x] gRPC-Server (Thor)
- [ ] gRPC-Client (Heimdall für Permissions)

## Phase 3: Action-Framework (10 Schritte)
- [ ] Tests für Action-Framework schreiben
- [ ] `ActionExecutor` Trait (TDD)
- [ ] `ActionRegistry` (TDD, Register-Actions)
- [ ] `ActionDispatcher` (TDD, Dispatch-Actions)

## Phase 4: File-Operations (10 Schritte)
- [ ] Tests für File-Actions schreiben
- [ ] `FileActions` (TDD, Read, Write, Delete, Move, Copy)

## Phase 5: Application-Control (8 Schritte)
- [ ] Tests für App-Control schreiben
- [ ] `AppControl` (TDD, Start-App, Stop-App, Query-Status)

## Phase 6: System-Commands (8 Schritte)
- [ ] Tests für System-Commands schreiben
- [ ] `SystemCommands` (TDD, Execute-Command, Get-System-Info)

## Phase 7: Network-Operations (8 Schritte)
- [ ] Tests für Network-Actions schreiben
- [ ] `NetworkActions` (TDD, HTTP-Request, DNS-Query)

## Phase 8: Permission-System & Sandboxing (10 Schritte)
- [ ] Tests für Permission-Checker schreiben
- [ ] `PermissionChecker` (TDD, Check-Permissions-via-Heimdall)
- [ ] Sandboxing (optional, TDD)

## Phase 9: Cross-Device-Actions (8 Schritte)
- [ ] Tests für Cross-Device-Actions schreiben
- [ ] `CrossDeviceActionHandler` (TDD, Receive-Actions-via-gRPC, Execute, Send-Results)

## Phase 10: Performance & Security (6 Schritte)
- [ ] Async-Processing
- [ ] Audit-Logging
- [ ] Error-Handling

## Phase 11: Terminal-Operations (12 Schritte)

**Abhängigkeiten**: Phase 3 (Action-Framework)

### 11.1 PTY-Management
- [ ] Tests für PTY-Wrapper schreiben
  - PTY-Setup
  - PTY-Resize
  - PTY-Close
  - Input/Output-Streaming
- [ ] `PtyWrapper` struct erstellen
- [ ] PTY-Setup implementieren (TDD)
  - `portable-pty` oder `pty` crate Integration
  - Terminal-Size-Configuration (Rows, Columns)
  - Environment-Variables
- [ ] Tests ausführen und bestehen

### 11.2 Interactive-Program-Execution
- [ ] Tests für Interactive-Programs schreiben
  - vim, nano, htop als Test-Cases
  - Input-Streaming (User → Program)
  - Output-Streaming (Program → User)
  - Timeout-Handling
- [ ] `InteractiveExecutor` implementieren (TDD)
  - PTY-basierte Execution
  - Async Input/Output-Streaming
  - Terminal-Control-Codes-Handling
- [ ] Tests ausführen und bestehen

### 11.3 Terminal-Action-Handler
- [ ] Tests für TERMINAL_OPERATION schreiben
- [ ] `TerminalActionHandler` implementieren (TDD)
  - Action-Parsing
  - Interactive vs. Non-Interactive Detection
  - PTY-Execution für Interactive
  - Standard-Execution für Non-Interactive
- [ ] In ActionRegistry registrieren
- [ ] Tests ausführen und bestehen

---

## Phase 12: UI-Automation (15 Schritte)

**Abhängigkeiten**: Phase 3 (Action-Framework)

### 12.1 Platform-Detection
- [ ] Tests für Platform-Detection schreiben
- [ ] Platform-Detection implementieren (TDD)
  - Windows, macOS, Linux Detection
  - Platform-spezifische Capabilities
- [ ] Tests ausführen und bestehen

### 12.2 Windows UI-Automation
- [ ] Tests für Windows UI-Automation schreiben
  - Click, Type, Move-Cursor
  - Element-Finding
  - Accessibility-API-Integration
- [ ] `WindowsUIAutomation` implementieren (TDD)
  - `windows-rs` crate Integration
  - UI Automation API Calls
  - Element-Locators (by name, by position)
- [ ] Tests ausführen und bestehen

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

## Phase 13: Scheduler-Operations (10 Schritte)

**Abhängigkeiten**: Phase 3 (Action-Framework)

### 13.1 Cron-Integration (Linux/macOS)
- [ ] Tests für Cron-Integration schreiben
  - Create/Delete/Update/List Cron-Jobs
  - Crontab-Parsing
  - Cron-Expression-Validation
- [ ] `CronScheduler` implementieren (TDD)
  - `cron_parser` crate Integration
  - Crontab-File-Manipulation
  - Cron-Expression-Building
- [ ] Tests ausführen und bestehen

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

### 13.5 Scheduler-Action-Handler
- [ ] Tests für SCHEDULER_OPERATION schreiben
- [ ] `SchedulerActionHandler` implementieren (TDD)
  - Platform-Dispatch (Cron/TaskScheduler/launchd/GoogleCalendar)
  - Action-Type-Handling (Create/Delete/Update/List)
  - Heimdall-Permission-Checks
- [ ] In ActionRegistry registrieren
- [ ] Tests ausführen und bestehen

---

## Phase 14: Jotunheim-Integration (8 Schritte)

**Abhängigkeiten**: Phase 3 (Action-Framework)

### 14.1 Jotunheim-gRPC-Client
- [ ] Tests für Jotunheim-Client schreiben
  - Connect to Jotunheim
  - Send Device-Commands
  - Receive Device-Status
- [ ] `JotunheimClient` implementieren (TDD)
  - gRPC-Client-Setup
  - Device-Command-Sending
  - Status-Queries
- [ ] Tests ausführen und bestehen

### 14.2 Device-Registry-Integration
- [ ] Tests für Device-Registry schreiben
  - List available Devices
  - Get Device-Capabilities
  - Device-Status-Query
- [ ] Device-Registry-Integration implementieren (TDD)
  - Heimdall-Device-Registry-Client
  - Device-Caching
- [ ] Tests ausführen und bestehen

### 14.3 Jotunheim-Action-Handler
- [ ] Tests für JOTUNHEIM_OPERATION schreiben
  - Device-Command-Execution
  - Device-Status-Query
  - Tool-Calling via Einherjar Protocol
- [ ] `JotunheimActionHandler` implementieren (TDD)
  - Device-Command-Execution via Jotunheim
  - Device-Status-Query
  - Generisches Tool-Calling (RegisterScript, Script_*, etc.)
  - Heimdall-Device-Permission-Checks
  - User-Confirmation für kritische Actions
- [ ] In ActionRegistry registrieren
- [ ] Tests ausführen und bestehen

---

## Phase 15: Documentation & Testing (6 Schritte)

**Abhängigkeiten**: Phase 14 (alle Action-Types implementiert)

- [ ] Dokumentation für alle neuen Action-Types
- [ ] E2E-Tests (Action-Request → Execute → Result)
- [ ] Security-Tests (Permission-Checks für alle Action-Types)
- [ ] Performance-Tests (Latency, Throughput)
- [ ] Integration-Tests (Odin → Thor → Services)

---

**Schritte gesamt**: ~135
**Phasen**: 15

**Hinweise**:
- Alle Schritte folgen TDD (Tests zuerst, dann Implementierung)
- **ALLE Tests müssen in Containern laufen** - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren
- Container-Setup muss vollständig isolierte Test-Umgebung bereitstellen
