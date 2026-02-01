# IMPLEMENTATION_PLAN - Nidhöggr (Connection Endpoint & Message Receiver)

## Übersicht

Nidhöggr ist der Server-Side Connection Endpoint bei Yggdrasil. Er empfängt Verbindungen von Vedrfolnir-Clients über das Ratatoskr-Protocol und leitet Nachrichten an entsprechende Services weiter.

**Programmiersprache**: Rust

## Entschiedene Konfiguration

✅ **Protobuf-Rust-Tool**: prost + tonic
✅ **WebSocket-Library**: tokio-tungstenite
✅ **TLS-Library**: rustls

**Begründung**: Beste Performance, async-native, pure Rust, robuste Security

---

## Phase 1: Projekt-Setup ✅ ABGESCHLOSSEN
- [x] Cargo-Projekt erstellen
- [x] Dependencies (tokio, tonic, tokio-tungstenite, rustls, serde, tracing, anyhow)
- [x] Verzeichnisstruktur (`src/connection/`, `src/routing/`, `src/validation/`)
- [x] Test-Infrastruktur
  - [x] Container-Setup für Tests (Dockerfile, Docker Compose)
  - [x] Mock-Services in Containern (Nornen, Heidrun, Mimir)
  - [x] **WICHTIG**: Alle Tests müssen in Containern laufen - keine lokalen Dependencies auf der Entwicklungsmaschine
- [x] CI/CD-Pipeline (`.github/workflows/nidhoggr.yml`)
- [x] Settings-System (JSON, Hot-Reload)

## Phase 2: Protobuf & Ratatoskr Protocol ✅ ABGESCHLOSSEN
- [x] Shared Protobuf-Projekt verwenden
- [x] `RatatoskrProtocol.proto` importieren (RatatoskrRequest, RatatoskrResponse)
- [x] gRPC-Clients für Services (Nornen, Heidrun, Mimir)

## Phase 3: Connection Management ✅ ABGESCHLOSSEN
- [x] Tests für Connection-Manager schreiben
- [x] `ConnectionManager` (TDD, Connection-Empfang, Validation, Monitoring, Termination)
- [x] Connection-Pooling
- [x] Heartbeat-System

## Phase 4: WebSocket-Server ✅ ABGESCHLOSSEN
- [x] Tests für WebSocket-Server schreiben
- [x] `WebSocketServer` (TDD, TLS 1.3)
- [x] Connection-Establishment-Flow
- [x] Connection-Termination-Flow

## Phase 5: Message-Receiving ✅ ABGESCHLOSSEN
- [x] Tests für Message-Receiver schreiben
- [x] `MessageReceiver` (TDD, Message-Validation, Signature-Check, Nonce-Validation, Rate-Limiting, Audit-Logging)

## Phase 6: Message-Routing ✅ ABGESCHLOSSEN
- [x] Tests für Message-Router schreiben
- [x] `MessageRouter` (TDD, Service-Discovery, Message-Type → Service Mapping, Load-Balancing)

## Phase 7: Rate-Limiting & Security ✅ ABGESCHLOSSEN
- [x] Tests für Rate-Limiter schreiben
- [x] `RateLimiter` (TDD, Per-Device/User Limits)
- [x] Security-Monitoring
- [x] Audit-Logger

## Phase 8: Performance-Optimization ⚠️ TEILWEISE
- [x] Connection-Pooling optimieren
- [x] Message-Batching
- [x] Async-Processing
- [ ] Benchmarks (< 100ms Connection-Latenz, < 10ms Message-Routing, 1000+ Messages/s)

## Phase 9: Monitoring & Logging ✅ ABGESCHLOSSEN
- [x] Structured-Logging (tracing)
- [x] Performance-Monitor

## Phase 10: Documentation & Testing ⚠️ TEILWEISE
- [x] Service-Dokumentation (README.md)
- [x] E2E-Tests (Connection → Message → Routing → Response)
- [ ] Performance-Tests
- [x] Security-Tests

---

**Schritte gesamt**: ~100
**Phasen**: 10
