# IMPLEMENTATION_PLAN - Yggdrasil (Cloud Server)

## Übersicht

Yggdrasil ist der Elixir-basierte Cloud Server - er koordiniert alle Rust-Microservices (Nidhöggr, Nornen, Mimir, Heidrun, Eikthyrnir, Læraðr, Hirtir, Njörðr).

**Programmiersprache**: Elixir

## Entschiedene Konfiguration
✅ **Elixir-Framework**: Phoenix
✅ **Database**: PostgreSQL
✅ **gRPC-Client-Library**: grpc (Elixir)

---

## Phase 1: Projekt-Setup (10 Schritte)
- [ ] Mix-Projekt erstellen (`mix new yggdrasil --sup`)
- [ ] Dependencies (phoenix, grpc, postgrex, jason)
- [ ] Verzeichnisstruktur (`lib/yggdrasil/`, `lib/yggdrasil_web/`)
- [ ] Test-Infrastruktur
  - [ ] Container-Setup für Tests (Dockerfile, Docker Compose)
  - [ ] Database-Container (PostgreSQL)
  - [ ] Mock-Rust-Microservices in Containern
  - [ ] **WICHTIG**: Alle Tests müssen in Containern laufen - keine lokalen Dependencies auf der Entwicklungsmaschine
- [ ] Configuration-System

## Phase 2: Service-Communication (20 Schritte)
- [ ] Protobuf-Definitions importieren
- [ ] gRPC-Clients für Rust-Microservices (Nidhöggr, Nornen, Mimir, Heidrun, Eikthyrnir, Læraðr, Hirtir, Njörðr)
- [ ] Connection-Pooling für gRPC-Clients
- [ ] Ratatoskr Protocol (WebSocket) für Business-Logic (Marketplace, Payments)
- [ ] Bifrost Protocol (WebSocket) für Device-Relay
- [ ] gRPC-Server für Yggdrasil API (Device-Registry, User-Management)

## Phase 3: Service-Coordination (15 Schritte)
- [ ] Tests für Service-Coordinator schreiben
- [ ] `ServiceCoordinator` (TDD, Coordinate-Services, Service-Lifecycle-Management)
- [ ] Health-Checks für alle Services

## Phase 4: Request-Routing (15 Schritte)
- [ ] Tests für Request-Router schreiben
- [ ] `RequestRouter` (TDD, Route-Requests-to-Services, Load-Balancing)

## Phase 5: Phoenix-Web-Server (Optional) (10 Schritte)
- [ ] Phoenix-Setup (optional Web-Interface)
- [ ] REST-API (optional)
- [ ] WebSocket-Support (optional)

## Phase 6: Database-Setup (10 Schritte)
- [ ] PostgreSQL-Schema
- [ ] Ecto-Models
- [ ] Migrations

## Phase 7: Marketplace & Business-Logic (15 Schritte)
- [ ] Tests für Marketplace-Logic schreiben
- [ ] `MarketplaceManager` (TDD, Provider-Listings, User-Requests)
- [ ] Integration mit Nornen (Urd/Verdandi)

## Phase 8: Payment-Processing (10 Schritte)
- [ ] Tests für Payment-Manager schreiben
- [ ] `PaymentManager` (TDD, Payment-Requests-to-Heidrun)

## Phase 9: User-Management (10 Schritte)
- [ ] Tests für User-Manager schreiben
- [ ] `UserManager` (TDD, User-Registration, User-Authentication)
- [ ] Integration mit Mimir (Privacy-Data)

## Phase 10: Performance-Optimization (8 Schritte)
- [ ] Elixir-Concurrency-Optimization
- [ ] GenServer-Optimization
- [ ] Caching (ETS)

## Phase 11: Monitoring & Logging (6 Schritte)
- [ ] Structured-Logging (Logger)
- [ ] Telemetry
- [ ] Performance-Monitoring

## Phase 12: Documentation & Testing (6 Schritte)
- [ ] Dokumentation
- [ ] E2E-Tests (Request → Service-Routing → Response)
- [ ] Performance-Tests

---

**Schritte gesamt**: ~135
**Phasen**: 12

**Hinweise**:
- Alle Schritte folgen TDD (Tests zuerst, dann Implementierung)
- **ALLE Tests müssen in Containern laufen** - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren
- Container-Setup muss vollständig isolierte Test-Umgebung bereitstellen
