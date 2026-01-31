# IMPLEMENTATION_PLAN - Skuld (LLM Selection Service)

## Übersicht

Skuld ist der LLM Selection Service - er entscheidet basierend auf Multi-Factor-Evaluation, welches LLM/Provider optimal für eine Request ist.

**Programmiersprache**: Rust

## Entschiedene Konfiguration
✅ **Protobuf-Rust-Tool**: prost + tonic
✅ **Database für Model-Registry**: PostgreSQL

---

## Phase 1: Projekt-Setup (8 Schritte)
- [x] Cargo-Projekt
- [x] Dependencies (tokio, tonic, serde, tracing, anyhow)
- [x] Verzeichnisstruktur (`src/selection/`, `src/evaluation/`, `src/registry/`)
- [x] Test-Infrastruktur
  - [x] Container-Setup für Tests (Dockerfile.test, docker-compose.test.yml)
  - [ ] Database-Container (PostgreSQL)
  - [x] Mock-Services in Containern (Eikthyrnir, Geri, Odin)
  - [ ] **WICHTIG**: Alle Tests müssen in Containern laufen - keine lokalen Dependencies auf der Entwicklungsmaschine
- [x] Settings-System

## Phase 2: Protobuf & gRPC (10 Schritte)
- [x] Protobuf-Definitions (`skuld.proto`)
- [x] gRPC-Server (Skuld)
- [ ] gRPC-Clients:
  - Eikthyrnir (Quality-Daten)
  - Geri (Model-Registry, Model-Info)
- [ ] gRPC-Server für Odin (Model-Selection-Requests)

## Phase 3: Model-Registry (10 Schritte)
- [ ] Tests für Model-Registry schreiben
- [ ] `ModelRegistry` (TDD, Register-Model, List-Models, Get-Model-Info)
- [ ] Database-Schema oder In-Memory-Storage

## Phase 4: Multi-Factor-Evaluation-Engine (15 Schritte)
- [ ] Tests für Evaluation-Engine schreiben
- [ ] `EvaluationEngine` (TDD)
- [ ] Evaluation-Factors (Size, Hardware, Reliability, Latency, Distance, Cost)
- [ ] `Evaluator` pro Faktor (SizeEvaluator, HardwareEvaluator, etc.)
- [ ] Weighted-Scoring

## Phase 5: Model-Selection (10 Schritte)
- [ ] Tests für Model-Selector schreiben
- [ ] `ModelSelector` (TDD, Multi-Factor-Evaluation, Best-Model-Selection)

## Phase 6: Load-Balancing (8 Schritte)
- [ ] Tests für Load-Balancer schreiben
- [ ] `LoadBalancer` (TDD, Weighted-Round-Robin)

## Phase 7: Integration mit Eikthyrnir (8 Schritte)
- [ ] Tests für Eikthyrnir-Client schreiben
- [ ] `EikthyrnirClient` (TDD, Quality-Daten abrufen)

## Phase 8: Performance-Optimization (6 Schritte)
- [ ] Caching (Model-Selection-Cache)
- [ ] Query-Optimization

## Phase 9: Documentation & Testing (6 Schritte)
- [ ] Dokumentation
- [ ] E2E-Tests (Request → Evaluation → Selection → Response)
- [ ] Performance-Tests

---

**Schritte gesamt**: ~81
**Phasen**: 9

**Hinweise**:
- Alle Schritte folgen TDD (Tests zuerst, dann Implementierung)
- **ALLE Tests müssen in Containern laufen** - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren
- Container-Setup muss vollständig isolierte Test-Umgebung bereitstellen
