# IMPLEMENTATION_PLAN - Ragnarok (Terminal Platform)

## Übersicht

Ragnarok ist die Terminal Platform - ein CLI-Interface für Edda.

**Programmiersprache**: Rust

## Entschiedene Konfiguration
✅ **CLI-Framework**: clap
✅ **TUI-Framework**: ratatui (optional)

---

## Phase 1: Projekt-Setup (8 Schritte)
- [x] Cargo-Projekt
- [x] Dependencies (tokio, tonic, clap, serde, tracing, anyhow)
- [x] Verzeichnisstruktur (`src/cli/`, `src/services/`)
- [x] Test-Infrastruktur
  - [x] Container-Setup für Tests (Dockerfile.test, docker-compose.test.yml)
  - [x] Mock-Services in Containern (Odin, Huginn/Muninn, Thor, Freki, Geri)
  - [x] **WICHTIG**: Alle Tests müssen in Containern laufen – CI (ragnarok.yml) und scripts/run-tests.* führen Tests im Container aus; keine lokalen Dependencies nötig
- [x] Settings-System

## Phase 2: Protobuf & gRPC-Clients (8 Schritte) ⚠️ TEILWEISE
- [x] Protobuf-Definitions für Odin (`proto/odin.proto`: ProcessRequest, ProcessResponse, OdinService)
- [x] gRPC-Client Odin – `src/grpc_client/odin_client.rs` (OdinClient::new, process_request)
- [x] Test: OdinClient connect unreachable → Err (`tests/odin_client_test.rs`, container-tauglich)
- [x] gRPC-Client Thor (optional) – `proto/thor.proto`, `src/grpc_client/thor_client.rs`, `tests/thor_client_test.rs`
- [x] gRPC-Client Geri (optional) – `proto/geri.proto`, `src/grpc_client/geri_client.rs`, `tests/geri_client_test.rs`
- [x] gRPC-Client Freki (optional) – `proto/freki.proto`, `src/grpc_client/freki_client.rs` (retrieve_context), `tests/freki_client_test.rs`
- [x] gRPC-Clients Huginn/Muninn (optional) – `src/grpc_client/huginn_client.rs`, `muninn_client.rs`; Config `huginn`/`muninn`; CLI `transcribe`, `speak`; Tests `huginn_client_test.rs`, `muninn_client_test.rs`

## Phase 3: CLI-Interface (15 Schritte) ⚠️ TEILWEISE
- [x] Tests für CLI-Parser schreiben (`tests/cli_parser_test.rs`: chat, action, status, settings, no-subcommand)
- [x] CLI mit clap (`src/cli/parser.rs`: Cli, Commands, parse_args) – Command-Parsing, Argument-Validation
- [x] Commands (chat, action, status, settings) – in `main.rs` integriert; ProcessRequest/ProcessResponse an Proto angepasst

## Phase 4: Service-Integration (15 Schritte) ⚠️ TEILWEISE
- [x] Tests für Service-Integrations schreiben (`tests/service_integration_test.rs`: Odin new unreachable → Err)
- [x] Odin-Client-Integration (TDD) – `src/services/odin_integration.rs` (OdinServiceIntegration::new, send_chat), in main.rs genutzt
- [x] Thor-Client-Integration (optional) – ThorClient in grpc_client; Action-Command ruft Thor auf, wenn `settings.thor` (address, port) konfiguriert ist; sonst Hinweis "Thor not configured"
- [x] Geri-Client (optional) – GeriClient in grpc_client; in main nutzbar: `prompt` (ProcessPrompt), `models` (ListModels), wenn `settings.geri` konfiguriert
- [x] Freki-Client (optional) – FrekiClient in grpc_client (retrieve_context); in main nutzbar: `retrieve`-Command, wenn `settings.freki` konfiguriert

## Phase 5: Optional TUI (Terminal-UI) (10 Schritte) ✅
- [x] Tests für TUI schreiben (`tests/tui_manager_test.rs` – State/API, optional ohne Terminal)
- [x] `TUIManager` (TDD, ratatui + crossterm)
  - [x] Status-Dashboard (Status-Zeilen, Odin-Connection)
  - [x] Chat-Interface (Nachrichten-Liste, Input-Bereich)
  - [x] `TuiState` (status_lines, chat_messages, input_buffer, odin_connected)
  - [x] CLI-Command `tui` – startet TUI (q=quit)

## Phase 6: Service-Discovery & Lifecycle (10 Schritte) ✅
- [x] Tests für Service-Discovery schreiben (`tests/service_registry_test.rs`)
- [x] `ServiceRegistry` (TDD, Einherjar-Protocol) – `src/services/service_registry.rs`
  - Service registration/unregistration
  - Capability-based service discovery
  - Status tracking (Available/Unavailable/Degraded)
- [x] `ServiceLifecycleManager` (TDD) – `src/services/service_lifecycle.rs`
  - Health monitoring
  - Automatic status updates
  - Lifecycle management (register/unregister/list)
- [x] Tests für ServiceLifecycleManager (`tests/service_lifecycle_test.rs`)

## Phase 7: Performance & Security (6 Schritte) ✅
- [x] Async-Processing (bereits vorhanden via tokio)
- [x] Heimdall-Integration (Auth) – `src/services/heimdall_integration.rs`
  - Authentication (username/password → token)
  - Token validation
  - Token refresh
  - Tests: `tests/heimdall_integration_test.rs`
- [x] Audit-Logging – `src/services/audit_logger.rs`
  - Event logging (user_id, action, resource, timestamp, details)
  - User/action-based filtering
  - Tests: `tests/audit_logging_test.rs`

## Phase 8: Documentation & Testing (6 Schritte) ✅
- [x] Dokumentation (README: Usage-Examples, Config-Beispiel, CLI-Beispiele, Projektstruktur aktuell)
- [x] E2E-Tests (CLI → Odin Service → Response)
  - [x] Mock-Odin als gRPC OdinService (`tests/mocks`: proto, build.rs, Process RPC mit fester Response)
  - [x] `tests/e2e_workflow_test.rs`: e2e_chat_via_odin_returns_response (nutzt ODIN_URL, skip wenn Odin unreachable)
  - In Container (docker-compose) mit mock-odin ausführbar
- [x] Code-Coverage in CI (cargo-tarpaulin, Artefakt ragnarok-coverage in .github/workflows/ragnarok.yml)

---

## Verbleibende optionale Punkte (Übersicht)

- [ ] **Phase 2** gRPC-Clients Huginn/Muninn (optional / bei Bedarf)
- [x] **Phase 4** Thor-Client ✅, Geri-Client ✅, Freki-Client ✅ (alle in main integriert: action, prompt, models, retrieve; Config: thor, geri, freki)

*(Constraint: Alle Tests in Containern – keine lokalen Dependencies.)*

---

**Schritte gesamt**: ~78
**Phasen**: 8

**Hinweise**:
- Alle Schritte folgen TDD (Tests zuerst, dann Implementierung)
- **ALLE Tests müssen in Containern laufen** - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren
- Container-Setup muss vollständig isolierte Test-Umgebung bereitstellen
