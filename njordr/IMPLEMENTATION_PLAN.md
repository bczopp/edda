# IMPLEMENTATION_PLAN - Njörðr (Maritime Trade & Provider Settlements)

## Übersicht

Njörðr ist der Service für Maritime Trade & Provider Settlements bei Yggdrasil - er verwaltet Provider-Earnings, Settlement-Processing und Trade-Management.

**Programmiersprache**: Rust

## Entschiedene Konfiguration
✅ **Protobuf-Rust-Tool**: prost + tonic
✅ **Database**: PostgreSQL
✅ **Payment-Gateway-Integration**: Stripe + PayPal

**Begründung**: Beste Performance, robuste Database, maximale Payment-Flexibilität

---

## Phase 1: Projekt-Setup (8 Schritte)
- [ ] Cargo-Projekt, Dependencies (tokio, tonic, sqlx, serde, tracing, anyhow)
- [ ] Verzeichnisstruktur (`src/settlements/`, `src/earnings/`, `src/trade/`)
- [ ] Test-Infrastruktur
  - [ ] Container-Setup für Tests (Dockerfile, Docker Compose)
  - [ ] Database-Container (PostgreSQL)
  - [ ] Mock-Services in Containern (Heidrun, Nornen)
  - [ ] **WICHTIG**: Alle Tests müssen in Containern laufen - keine lokalen Dependencies auf der Entwicklungsmaschine
- [ ] Settings-System

## Phase 2: Protobuf & gRPC (8 Schritte)
- [ ] Protobuf-Definitions (`NjordrService.proto`)
- [ ] gRPC-Server

## Phase 3: Database (10 Schritte)
- [ ] Database-Schema (Provider-Earnings, Settlements, Trades)
- [ ] Connection-Pooling
- [ ] Migrations

## Phase 4: Provider-Earnings-Management (10 Schritte)
- [ ] Tests für Earnings-Manager schreiben
- [ ] `EarningsManager` (TDD, Calculate-Earnings, Track-Earnings, Aggregate-Earnings)

## Phase 5: Settlement-Processing (10 Schritte)
- [ ] Tests für Settlement-Processor schreiben
- [ ] `SettlementProcessor` (TDD, Generate-Settlements, Execute-Settlements, Track-Status)

## Phase 6: Trade-Management (8 Schritte)
- [ ] Tests für Trade-Manager schreiben
- [ ] `TradeManager` (TDD, Trade-Tracking, Trade-History)

## Phase 7: Payment-Gateway-Integration (8 Schritte)
- [ ] Tests für Payment-Gateway schreiben
- [ ] `PaymentGatewayClient` (TDD, Stripe/PayPal Integration)

## Phase 8: Performance & Security (6 Schritte)
- [ ] Caching (Earnings-Cache)
- [ ] Audit-Logging
- [ ] Input-Validation

## Phase 9: Documentation & Testing (6 Schritte)
- [ ] Dokumentation
- [ ] E2E-Tests (Earnings → Settlement → Payment)
- [ ] Performance-Tests

---

**Schritte gesamt**: ~74
**Phasen**: 9
