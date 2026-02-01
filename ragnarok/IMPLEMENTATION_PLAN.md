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
  - [ ] **WICHTIG**: Alle Tests müssen in Containern laufen - keine lokalen Dependencies auf der Entwicklungsmaschine
- [x] Settings-System

## Phase 2: Protobuf & gRPC-Clients (8 Schritte) ⚠️ TEILWEISE
- [x] Protobuf-Definitions für Odin (`proto/odin.proto`: ProcessRequest, ProcessResponse, OdinService)
- [x] gRPC-Client Odin – `src/grpc_client/odin_client.rs` (OdinClient::new, process_request)
- [x] Test: OdinClient connect unreachable → Err (`tests/odin_client_test.rs`, container-tauglich)
- [ ] gRPC-Clients Huginn/Muninn, Thor, Freki, Geri (optional / bei Bedarf)

## Phase 3: CLI-Interface (15 Schritte) ⚠️ TEILWEISE
- [x] Tests für CLI-Parser schreiben (`tests/cli_parser_test.rs`: chat, action, status, settings, no-subcommand)
- [x] CLI mit clap (`src/cli/parser.rs`: Cli, Commands, parse_args) – Command-Parsing, Argument-Validation
- [x] Commands (chat, action, status, settings) – in `main.rs` integriert; ProcessRequest/ProcessResponse an Proto angepasst

## Phase 4: Service-Integration (15 Schritte) ⚠️ TEILWEISE
- [x] Tests für Service-Integrations schreiben (`tests/service_integration_test.rs`: Odin new unreachable → Err)
- [x] Odin-Client-Integration (TDD) – `src/services/odin_integration.rs` (OdinServiceIntegration::new, send_chat), in main.rs genutzt
- [ ] Thor-Client-Integration (TDD) – optional / bei Bedarf
- [ ] Freki/Geri-Client-Integration (TDD) – optional / bei Bedarf

## Phase 5: Optional TUI (Terminal-UI) (10 Schritte)
- [ ] Tests für TUI schreiben (optional)
- [ ] `TUIManager` (TDD, ratatui, Status-Dashboard, Chat-Interface)

## Phase 6: Service-Discovery & Lifecycle (10 Schritte)
- [ ] Tests für Service-Discovery schreiben
- [ ] `ServiceRegistry` (TDD, Einherjar-Protocol)
- [ ] `ServiceLifecycleManager` (TDD)

## Phase 7: Performance & Security (6 Schritte)
- [ ] Async-Processing
- [ ] Heimdall-Integration (Auth)
- [ ] Audit-Logging

## Phase 8: Documentation & Testing (6 Schritte) ⚠️ TEILWEISE
- [x] Dokumentation (README: Usage-Examples, Config-Beispiel, CLI-Beispiele, Projektstruktur aktuell)
- [ ] E2E-Tests (CLI-Command → Service-Call → Response) – optional / bei Bedarf

---

**Schritte gesamt**: ~78
**Phasen**: 8

**Hinweise**:
- Alle Schritte folgen TDD (Tests zuerst, dann Implementierung)
- **ALLE Tests müssen in Containern laufen** - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren
- Container-Setup muss vollständig isolierte Test-Umgebung bereitstellen
