# IMPLEMENTATION_PLAN - Heidrun (Token & Pricing Service)

## Übersicht

Dieser Plan beschreibt die kleinstmöglichen Schritte zur Implementierung von Heidrun - dem Token & Pricing Service für Yggdrasil. Heidrun führt Token-Berechnungen, Pricing, Settlement und Pre-Authorization für den Marketplace durch.

**Mythologische Bedeutung**: Heidrun ist die Ziege, die Met produziert (Wert/Flüssigkeit).

**Programmiersprache**: Rust

**Service-Typ**: Yggdrasil Microservice (Rust-Microservice für Elixir-Server)

## Entschiedene Konfiguration

### Protobuf-Rust-Tool
✅ **ENTSCHEIDUNG**: prost + tonic
**Begründung**: Async-native, beste Performance, idiomatisches Rust

### Database für Token-Tracking
✅ **ENTSCHEIDUNG**: PostgreSQL
**Begründung**: Persistent, robust, ACID-compliant, beste Verlässlichkeit für Payment-kritische Daten

### Commission-Rate
✅ **ENTSCHEIDUNG**: 15% (konfigurierbar)
**Begründung**: Faire Business-Provision, konfigurierbar für Flexibilität

### Pricing-Storage
✅ **ENTSCHEIDUNG**: Database
**Begründung**: Strukturiert, dynamisch, zentral verwaltbar, beste Performance für Queries

---

## Phase 1: Projekt-Setup & Grundstruktur

### 1.1 Projekt-Initialisierung

**Abhängigkeiten**: Keine
**Erforderliche USER-Eingaben**: Protobuf-Rust-Tool

#### 1.1.1 Cargo-Projekt erstellen
- [ ] `Cargo.toml` erstellen
- [ ] Basis-Dependencies definieren
  - Async Runtime (tokio)
  - gRPC (tonic, prost) - oder rust-protobuf
  - Serialization (serde, serde_json)
  - Logging (tracing, tracing-subscriber)
  - Error-Handling (anyhow, thiserror)
  - Math-Library (num-bigint für präzise Berechnungen)
- [ ] `.gitignore` erstellen

#### 1.1.2 Verzeichnisstruktur erstellen
- [ ] `heidrun/src/main.rs` erstellen
- [ ] `heidrun/src/lib.rs` erstellen
- [ ] `heidrun/src/token/` für Token-Counting erstellen
- [ ] `heidrun/src/pricing/` für Pricing-Berechnungen erstellen
- [ ] `heidrun/src/settlement/` für Settlement erstellen
- [ ] `heidrun/src/preauth/` für Pre-Authorization erstellen
- [ ] `heidrun/src/grpc/` für gRPC-Service erstellen
- [ ] `heidrun/src/utils/` für Utilities erstellen
- [ ] `heidrun/config/` für Konfigurationsdateien erstellen
- [ ] `heidrun/tests/` für Tests erstellen

#### 1.1.3 Build-System einrichten
- [ ] Build-Scripts in `Cargo.toml` definieren
- [ ] Code-Generierungs-Pipeline einrichten (Protobuf → Rust)
- [ ] Cargo-Features definieren

### 1.2 Test-Infrastruktur

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.2.1 Container-Setup für Tests
- [ ] `Dockerfile` für Test-Umgebung erstellen
- [ ] Docker Compose für Test-Services konfigurieren
  - Mock-Yggdrasil-Service
  - Optional: Database-Container (falls Database gewählt)
- [ ] Test-Container-Startup-Scripts erstellen
- [ ] **WICHTIG**: Alle Tests müssen in Containern laufen - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren

#### 1.2.2 Test-Framework konfigurieren
- [ ] Test-Dependencies hinzufügen (tokio-test, mockall, etc.)
- [ ] Test-Utilities und Helpers erstellen
- [ ] Mock-Setup für Yggdrasil-gRPC-Client
- [ ] Test-Data-Generators für Token/Pricing-Data erstellen

#### 1.2.3 CI/CD-Pipeline
- [ ] GitHub Actions / GitLab CI Workflow erstellen
- [ ] Automatische Test-Ausführung bei Commits konfigurieren
- [ ] Code-Coverage-Reporting einrichten (cargo-tarpaulin)
- [ ] Linting und Formatting (cargo clippy, cargo fmt)

### 1.3 Settings-System

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)
**Erforderliche USER-Eingaben**: Commission-Rate, Pricing-Storage

#### 1.3.1 Settings-Schema definieren
- [ ] Settings-Struktur entwerfen (JSON-Format)
  - commission_rate (10-15%)
  - pricing_configurations (pro Provider)
  - token_counting_settings
  - settlement_settings
  - preauth_settings

#### 1.3.2 Settings-Validierung
- [ ] Tests für Settings-Validierung schreiben
- [ ] Rust-Structs für Settings definieren
- [ ] Settings-Validator implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 1.3.3 Settings-Loader & Hot-Reload
- [ ] Tests für Settings-Loader schreiben
- [ ] Settings-Loader implementieren (TDD)
- [ ] Hot-Reload-Mechanismus implementieren (TDD)
- [ ] Tests ausführen und bestehen

---

## Phase 2: Protobuf & gRPC Setup

### 2.1 Protobuf Definitions

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 2.1.1 Shared Protobuf-Projekt verwenden
❓ **HINWEIS**: Protobuf-Definitions sollten in einem separaten Projekt sein
- [ ] Heidrun als Dependency zu Protobuf-Projekt hinzufügen
- [ ] Protobuf-Definitions importieren

#### 2.1.2 Token-Service Protocol
- [ ] `TokenService.proto` definieren
  - `CountTokensRequest` Message (requestId, inputTokens, outputTokens, providerId)
  - `CountTokensResponse` Message (requestId, totalTokens, inputTokens, outputTokens)
  - `TrackTokensRequest` Message
  - `TrackTokensResponse` Message
  - `AggregateTokensRequest` Message
  - `AggregateTokensResponse` Message
- [ ] Code-Generierung konfigurieren

#### 2.1.3 Pricing-Service Protocol
- [ ] `PricingService.proto` definieren
  - `CalculatePricingRequest` Message (requestId, tokens, providerId)
  - `CalculatePricingResponse` Message (requestId, totalCost, pricePerToken)
  - `GetPricingConfigRequest` Message
  - `GetPricingConfigResponse` Message
- [ ] Code-Generierung konfigurieren

#### 2.1.4 Settlement-Service Protocol
- [ ] `SettlementService.proto` definieren
  - `CalculateSettlementRequest` Message (requestId, totalCost)
  - `CalculateSettlementResponse` Message (requestId, providerEarnings, companyFee, commissionRate)
  - `GetSettlementHistoryRequest` Message
  - `GetSettlementHistoryResponse` Message
- [ ] Code-Generierung konfigurieren

#### 2.1.5 Pre-Authorization Protocol
- [ ] `PreAuthService.proto` definieren
  - `PreAuthorizeRequest` Message (requestId, estimatedTokens, providerId)
  - `PreAuthorizeResponse` Message (requestId, estimatedCost, authorized)
  - `CapturePreAuthRequest` Message
  - `CapturePreAuthResponse` Message
  - `CancelPreAuthRequest` Message
  - `CancelPreAuthResponse` Message
- [ ] Code-Generierung konfigurieren

### 2.2 gRPC Server Implementation

**Abhängigkeiten**: 2.1 (Protobuf Definitions)

#### 2.2.1 gRPC Server Setup
- [ ] Tests für gRPC-Server-Setup schreiben
- [ ] gRPC-Server-Setup implementieren (TDD)
  - tonic-Server konfigurieren
  - Health-Check-Service
- [ ] Tests ausführen und bestehen

#### 2.2.2 Token Service
- [ ] Tests für Token-Service schreiben
- [ ] `TokenServiceImpl` implementieren (TDD)
  - `CountTokens()` RPC
  - `TrackTokens()` RPC
  - `AggregateTokens()` RPC
- [ ] Tests ausführen und bestehen

#### 2.2.3 Pricing Service
- [ ] Tests für Pricing-Service schreiben
- [ ] `PricingServiceImpl` implementieren (TDD)
  - `CalculatePricing()` RPC
  - `GetPricingConfig()` RPC
- [ ] Tests ausführen und bestehen

#### 2.2.4 Settlement Service
- [ ] Tests für Settlement-Service schreiben
- [ ] `SettlementServiceImpl` implementieren (TDD)
  - `CalculateSettlement()` RPC
  - `GetSettlementHistory()` RPC (optional)
- [ ] Tests ausführen und bestehen

#### 2.2.5 Pre-Authorization Service
- [ ] Tests für Pre-Auth-Service schreiben
- [ ] `PreAuthServiceImpl` implementieren (TDD)
  - `PreAuthorize()` RPC
  - `CapturePreAuth()` RPC
  - `CancelPreAuth()` RPC
- [ ] Tests ausführen und bestehen

---

## Phase 3: Token Counting Engine

### 3.1 Token Counter

**Abhängigkeiten**: 2.1 (Protobuf Definitions)

#### 3.1.1 Token-Counter
- [ ] Tests für Token-Counter schreiben
- [ ] `TokenCounter` implementieren (TDD)
  - Token-Counting für Input-Tokens
  - Token-Counting für Output-Tokens
  - Total-Token-Calculation
- [ ] Tests ausführen und bestehen

### 3.2 Provider-Specific Token-Counting

**Abhängigkeiten**: 3.1 (Token Counter)

#### 3.2.1 Provider-Token-Counter-Trait
- [ ] Tests für Provider-Token-Counter-Trait schreiben
- [ ] `ProviderTokenCounter` Trait definieren
  - `count_tokens()` Methode
- [ ] Tests ausführen und bestehen

#### 3.2.2 OpenAI-Token-Counter
- [ ] Tests für OpenAI-Token-Counter schreiben
- [ ] `OpenAITokenCounter` implementieren (TDD)
  - tiktoken-basiertes Token-Counting
- [ ] Tests ausführen und bestehen

#### 3.2.3 Anthropic-Token-Counter
- [ ] Tests für Anthropic-Token-Counter schreiben
- [ ] `AnthropicTokenCounter` implementieren (TDD)
  - Anthropic-spezifisches Token-Counting
- [ ] Tests ausführen und bestehen

#### 3.2.4 Generic-Token-Counter (Fallback)
- [ ] Tests für Generic-Token-Counter schreiben
- [ ] `GenericTokenCounter` implementieren (TDD)
  - Fallback Token-Counting (einfache Zeichenzahl-Schätzung)
- [ ] Tests ausführen und bestehen

### 3.3 Token-Tracking

**Abhängigkeiten**: 3.1 (Token Counter)
**Erforderliche USER-Eingaben**: Database für Token-Tracking

#### 3.3.1 Token-Tracker
- [ ] Tests für Token-Tracker schreiben
- [ ] `TokenTracker` implementieren (TDD)
  - Token-Verbrauch pro Request tracken
  - Request-ID, Token-Count, Timestamp
  - Falls Database: Persistierung in Database
  - Falls In-Memory: In-Memory-Storage
- [ ] Tests ausführen und bestehen

### 3.4 Token-Aggregation

**Abhängigkeiten**: 3.3 (Token-Tracking)

#### 3.4.1 Token-Aggregator
- [ ] Tests für Token-Aggregator schreiben
- [ ] `TokenAggregator` implementieren (TDD)
  - Token-Statistiken aggregieren (Summe, Durchschnitt, etc.)
  - Pro Provider, pro User, pro Zeitraum
- [ ] Tests ausführen und bestehen

---

## Phase 4: Pricing Engine

### 4.1 Pricing Configuration

**Abhängigkeiten**: 1.3 (Settings-System)
**Erforderliche USER-Eingaben**: Pricing-Storage

#### 4.1.1 Pricing-Config-Manager
- [ ] Tests für Pricing-Config-Manager schreiben
- [ ] `PricingConfigManager` implementieren (TDD)
  - Pricing-Konfigurationen laden (Settings/Database/Yggdrasil)
  - Pricing pro Provider verwalten
  - Hot-Reload von Pricing-Konfigurationen
- [ ] Tests ausführen und bestehen

### 4.2 Pricing Calculator

**Abhängigkeiten**: 4.1 (Pricing Configuration), 3.1 (Token Counter)

#### 4.2.1 Pricing-Calculator
- [ ] Tests für Pricing-Calculator schreiben
- [ ] `PricingCalculator` implementieren (TDD)
  - Formel: `(tokens / 1000) * pricePerToken` (aufgerundet)
  - Integer-Arithmetik (Cent-Berechnung)
  - Keine Rundungsfehler
- [ ] Tests ausführen und bestehen

### 4.3 Cost-Calculation-Workflow

**Abhängigkeiten**: 4.2 (Pricing Calculator), 3.1 (Token Counter)

#### 4.3.1 Cost-Calculation-Manager
- [ ] Tests für Cost-Calculation-Manager schreiben
- [ ] `CostCalculationManager` implementieren (TDD)
  - Token-Counting + Pricing-Berechnung
  - Input-Tokens + Output-Tokens
  - Total-Cost berechnen
- [ ] Tests ausführen und bestehen

---

## Phase 5: Settlement Engine

### 5.1 Commission-Calculation

**Abhängigkeiten**: 4.2 (Pricing Calculator)
**Erforderliche USER-Eingaben**: Commission-Rate

#### 5.1.1 Commission-Calculator
- [ ] Tests für Commission-Calculator schreiben
- [ ] `CommissionCalculator` implementieren (TDD)
  - Formel: `companyFee = totalCost * commissionRate`
  - Commission-Rate aus Settings (10-15%)
  - Integer-Arithmetik
- [ ] Tests ausführen und bestehen

### 5.2 Provider-Earnings-Calculation

**Abhängigkeiten**: 5.1 (Commission-Calculation)

#### 5.2.1 Provider-Earnings-Calculator
- [ ] Tests für Provider-Earnings-Calculator schreiben
- [ ] `ProviderEarningsCalculator` implementieren (TDD)
  - Formel: `providerEarnings = totalCost - companyFee`
  - Integer-Arithmetik
- [ ] Tests ausführen und bestehen

### 5.3 Settlement-Manager

**Abhängigkeiten**: 5.1 (Commission-Calculation), 5.2 (Provider-Earnings-Calculation)

#### 5.3.1 Settlement-Manager
- [ ] Tests für Settlement-Manager schreiben
- [ ] `SettlementManager` implementieren (TDD)
  - Settlement berechnen (Provider-Earnings + Company-Fee)
  - Settlement-History tracken (optional)
- [ ] Tests ausführen und bestehen

---

## Phase 6: Pre-Authorization Engine

### 6.1 Cost-Estimation

**Abhängigkeiten**: 4.2 (Pricing Calculator)

#### 6.1.1 Cost-Estimator
- [ ] Tests für Cost-Estimator schreiben
- [ ] `CostEstimator` implementieren (TDD)
  - Formel: `estimatedCost = (estimatedTokens / 1000) * pricePerToken` (aufgerundet)
  - Basierend auf Request-Parametern (z.B. Prompt-Länge)
- [ ] Tests ausführen und bestehen

### 6.2 Pre-Authorization-Manager

**Abhängigkeiten**: 6.1 (Cost-Estimation)

#### 6.2.1 Pre-Auth-Manager
- [ ] Tests für Pre-Auth-Manager schreiben
- [ ] `PreAuthManager` implementieren (TDD)
  - Pre-Authorization erstellen
  - Pre-Authorization-Status verwalten (pending, captured, cancelled)
  - Authorization-Expiration-Handling
- [ ] Tests ausführen und bestehen

---

## Phase 7: Audit Logging

### 7.1 Audit-Logger

**Abhängigkeiten**: 3.1 (Token Counter), 4.2 (Pricing Calculator), 5.3 (Settlement-Manager)

#### 7.1.1 Audit-Logger
- [ ] Tests für Audit-Logger schreiben
- [ ] `AuditLogger` implementieren (TDD)
  - Alle Token-Counting-Events loggen
  - Alle Pricing-Berechnungen loggen
  - Alle Settlement-Berechnungen loggen
  - Strukturiertes Logging mit Request-IDs
- [ ] Tests ausführen und bestehen

---

## Phase 8: Caching System

### 8.1 Pricing-Config-Cache

**Abhängigkeiten**: 4.1 (Pricing Configuration)

#### 8.1.1 Pricing-Cache-Manager
- [ ] Tests für Pricing-Cache schreiben
- [ ] `PricingCacheManager` implementieren (TDD)
  - Pricing-Konfigurationen cachen
  - Cache-Invalidation bei Updates
  - TTL-basierte Expiration
- [ ] Tests ausführen und bestehen

---

## Phase 9: Error Handling & Validation

### 9.1 Input-Validation

**Abhängigkeiten**: 2.2 (gRPC Server Implementation)

#### 9.1.1 Input-Validator
- [ ] Tests für Input-Validation schreiben
- [ ] `InputValidator` implementieren (TDD)
  - Token-Count-Validation (> 0, < MAX)
  - Provider-ID-Validation
  - Request-ID-Validation
- [ ] Tests ausführen und bestehen

### 9.2 Error-Handler

**Abhängigkeiten**: 2.2 (gRPC Server Implementation)

#### 9.2.1 Error-Handler
- [ ] Tests für Error-Handler schreiben
- [ ] `ErrorHandler` implementieren (TDD)
  - gRPC-Status-Codes
  - Fehler-Logging
  - User-freundliche Fehlermeldungen
- [ ] Tests ausführen und bestehen

---

## Phase 10: Performance Optimization

### 10.1 Batch-Processing

**Abhängigkeiten**: 5.3 (Settlement-Manager)

#### 10.1.1 Batch-Processor
- [ ] Tests für Batch-Processing schreiben
- [ ] `BatchProcessor` implementieren (TDD)
  - Batch-Processing für Settlement-Berechnungen
  - Optimierte Batch-Operationen
- [ ] Tests ausführen und bestehen

### 10.2 Calculation-Optimization

**Abhängigkeiten**: 4.2 (Pricing Calculator), 5.1 (Commission-Calculation)

#### 10.2.1 Calculation-Optimizer
- [ ] Performance-Tests für Berechnungen schreiben
- [ ] Berechnungen optimieren
  - Integer-Arithmetik (keine Floating-Point)
  - Lookup-Tables für häufige Berechnungen (optional)
- [ ] Performance-Tests ausführen und Benchmarks erreichen

---

## Phase 11: Monitoring & Metrics

### 11.1 Metrics Collector

**Abhängigkeiten**: 2.2 (gRPC Server Implementation)

#### 11.1.1 Metrics-Collector
- [ ] Tests für Metrics-Collector schreiben
- [ ] `MetricsCollector` implementieren (TDD)
  - Token-Counting-Metriken
  - Pricing-Berechnungs-Metriken
  - Settlement-Metriken
  - Request-Throughput
- [ ] Tests ausführen und bestehen

### 11.2 Performance-Monitoring

**Abhängigkeiten**: 11.1 (Metrics Collector)

#### 11.2.1 Performance-Monitor
- [ ] Tests für Performance-Monitoring schreiben
- [ ] `PerformanceMonitor` implementieren (TDD)
  - Calculation-Performance tracken
  - Response-Zeiten tracken
  - Resource-Usage tracken
- [ ] Tests ausführen und bestehen

---

## Phase 12: Monitoring & Logging

### 12.1 Structured Logging

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 12.1.1 Logging Setup
- [ ] Structured-Logging konfigurieren (tracing)
- [ ] Calculation-specific Log-Levels
- [ ] Log-Rotation konfigurieren

---

## Phase 13: Documentation

### 13.1 API Documentation

**Abhängigkeiten**: 2.2 (gRPC Server Implementation)

#### 13.1.1 gRPC Service Documentation
- [ ] gRPC-Service-Documentation erstellen
- [ ] Token-Service-API dokumentieren
- [ ] Pricing-Service-API dokumentieren
- [ ] Settlement-Service-API dokumentieren
- [ ] Pre-Auth-Service-API dokumentieren

### 13.2 Calculation-Documentation

**Abhängigkeiten**: 4.2 (Pricing Calculator), 5.3 (Settlement-Manager)

#### 13.2.1 Formula-Documentation
- [ ] Berechnungsformeln dokumentieren
- [ ] Beispiele für Berechnungen erstellen
- [ ] Edge-Cases dokumentieren

---

## Phase 14: Testing & Quality Assurance

### 14.1 Integration Testing

**Abhängigkeiten**: Alle vorherigen Phasen

#### 14.1.1 End-to-End Tests
- [ ] E2E-Tests für Token-Workflows schreiben
  - Token-Counting → Pricing → Settlement
  - Pre-Authorization → Token-Counting → Capture
- [ ] E2E-Tests ausführen und bestehen

### 14.2 Accuracy Testing

**Abhängigkeiten**: 4.2 (Pricing Calculator), 5.1 (Commission-Calculation)

#### 14.2.1 Accuracy Test Suite
- [ ] Accuracy-Tests ausführen
  - Keine Rundungsfehler
  - Integer-Arithmetik-Tests
  - Edge-Case-Tests (0 Tokens, sehr große Token-Zahlen)
- [ ] Accuracy-Tests bestehen

### 14.3 Performance Testing

**Abhängigkeiten**: 11.1 (Metrics Collector)

#### 14.3.1 Performance Test Suite
- [ ] Performance-Tests ausführen
  - High-Throughput-Tests (tausende Requests pro Sekunde)
  - Calculation-Performance-Tests (< 1ms pro Berechnung)
  - Batch-Processing-Performance-Tests
- [ ] Performance-Tests bestehen

---

## Zusammenfassung

**Gesamtanzahl Phasen**: 14
**Gesamtanzahl Schritte**: ~160+

**Kritische Abhängigkeiten**:
1. Protobuf-Rust-Tool (prost + tonic empfohlen)
2. Database für Token-Tracking (PostgreSQL, In-Memory, Keine)
3. Commission-Rate (10%, 15%, Konfigurierbar)
4. Pricing-Storage (Settings, Database, Yggdrasil)

**Offene Fragen für USER**:
1. Protobuf-Rust-Tool (prost + tonic, rust-protobuf)
2. Database für Token-Tracking (PostgreSQL, In-Memory, Keine)
3. Commission-Rate (10%, 15%, Konfigurierbar)
4. Pricing-Storage (Settings, Database, Yggdrasil)

**Hinweise**:
- Alle Schritte folgen TDD
- Alle Tests in Containern
- Integer-Arithmetik für Pricing (keine Rundungsfehler)
- Audit-Logging für alle Berechnungen
- Performance-Optimierung für hohe Request-Volumes
- Cent-Berechnung pro 1000 Tokens
