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

## Phase 2: Protobuf & gRPC-Clients (8 Schritte)
- [ ] Protobuf-Definitions importieren
- [ ] gRPC-Clients (Odin, Huginn/Muninn, Thor, Freki, Geri)

## Phase 3: CLI-Interface (15 Schritte)
- [ ] Tests für CLI-Parser schreiben
- [ ] `CLIParser` (TDD, Command-Parsing, Argument-Validation)
- [ ] Commands (chat, action, settings, status)

## Phase 4: Service-Integration (15 Schritte)
- [ ] Tests für Service-Integrations schreiben
- [ ] Odin-Client-Integration (TDD)
- [ ] Thor-Client-Integration (TDD)
- [ ] Freki/Geri-Client-Integration (TDD)

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

## Phase 8: Documentation & Testing (6 Schritte)
- [ ] Dokumentation (README, Usage-Examples)
- [ ] E2E-Tests (CLI-Command → Service-Call → Response)

---

**Schritte gesamt**: ~78
**Phasen**: 8

**Hinweise**:
- Alle Schritte folgen TDD (Tests zuerst, dann Implementierung)
- **ALLE Tests müssen in Containern laufen** - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren
- Container-Setup muss vollständig isolierte Test-Umgebung bereitstellen
