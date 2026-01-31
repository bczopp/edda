# IMPLEMENTATION_PLAN - Nornen (Decision Service)

## Übersicht

Nornen ist der Decision Service bei Yggdrasil - er koordiniert Urd (Past: Provider-Registry), Verdandi (Present: Request-Routing), und Skuld (Future: Decision-Making für LLM-Selection).

**Programmiersprache**: Rust

## Entschiedene Konfiguration
✅ **Protobuf-Rust-Tool**: prost + tonic
✅ **Database**: PostgreSQL

---

## Phase 1: Projekt-Setup (10 Schritte)
- [x] Cargo-Workspace / Cargo-Projekt (`nornen/`, `urd/`, `verdandi/` als Module)
- [x] Dependencies (tokio, tonic, sqlx, serde, tracing, anyhow)
- [x] Verzeichnisstruktur
- [x] Test-Infrastruktur
  - [x] Container-Setup für Tests (Dockerfile.test, docker-compose.test.yml)
  - [ ] Database-Container (PostgreSQL)
  - [ ] Mock-Services in Containern
  - [ ] **WICHTIG**: Alle Tests müssen in Containern laufen - keine lokalen Dependencies auf der Entwicklungsmaschine
- [x] Settings-System

## Phase 2: Protobuf & gRPC (10 Schritte)
- [x] Protobuf-Definitions (`nornen.proto`)
- [x] gRPC-Server (Main Nornen + Urd + Verdandi)

## Phase 3: Urd - Provider-Registry (15 Schritte)
- [ ] Database-Schema (Providers, Capabilities, Status)
- [ ] Tests für Provider-Registry schreiben
- [ ] `ProviderRegistry` (TDD, Register-Provider, Update-Provider, Query-Providers, List-Providers)

## Phase 4: Verdandi - Request-Routing (15 Schritte)
- [ ] Tests für Request-Router schreiben
- [ ] `RequestRouter` (TDD, Route-Requests, Provider-Selection, Load-Balancing, Fallback-Routing)

## Phase 5: Nornen-Coordination (10 Schritte)
- [ ] Tests für Nornen-Coordinator schreiben
- [ ] `NornenCoordinator` (TDD, Coordinate Urd/Verdandi, Service-Lifecycle-Management)

## Phase 6: Performance-Optimization (8 Schritte)
- [ ] Caching (Provider-Cache)
- [ ] Query-Optimization
- [ ] Connection-Pooling

## Phase 7: Security & Monitoring (6 Schritte)
- [ ] Access-Control
- [ ] Audit-Logging
- [ ] Monitoring

## Phase 8: Documentation & Testing (6 Schritte)
- [ ] Dokumentation
- [ ] E2E-Tests (Provider-Registration → Request-Routing)
- [ ] Performance-Tests

---

**Schritte gesamt**: ~80
**Phasen**: 8
