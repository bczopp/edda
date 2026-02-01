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
  - [x] Database-Container (PostgreSQL in docker-compose.test.yml, migrations/)
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
- [x] Tests für Model-Registry schreiben (`tests/model_registry_test.rs`: register/list, idempotent, get_model_info)
- [x] `ModelRegistry` (TDD, Register-Model, List-Models, Get-Model-Info) – `src/registry/manager.rs`
- [x] Database-Schema (`migrations/001_model_registry.sql`), Postgres-Storage

## Phase 4: Multi-Factor-Evaluation-Engine (15 Schritte) ⚠️ TEILWEISE
- [x] Tests für Evaluation-Engine schreiben (`src/evaluation/engine.rs`: test_evaluate_returns_scores_in_range, test_evaluate_gpt4_scores_higher_than_default)
- [x] `ModelEvaluator` (TDD) – Performance/Reliability/Efficiency-Scores, Weighted total_score
- [x] Evaluation-Factors (Performance, Reliability, Efficiency) – evaluate_performance, evaluate_reliability, evaluate_efficiency
- [ ] Weitere Faktoren (Size, Hardware, Latency, Distance, Cost) – optional
- [ ] `Evaluator` pro Faktor (SizeEvaluator, etc.) – optional

## Phase 5: Model-Selection (10 Schritte)
- [x] Tests für Model-Selector schreiben (`tests/model_selector_test.rs`: select_best_model returns one of registered models, single model returns that model)
- [x] `ModelSelector` (TDD, Multi-Factor-Evaluation, Best-Model-Selection) – `src/selection/selector.rs`

## Phase 6: Load-Balancing (8 Schritte)
- [x] Tests für Load-Balancer (`src/load_balancer/balance.rs`: test_empty, test_single_model, test_weighted_round_robin)
- [x] `LoadBalancer` (Weighted-Round-Robin) – `src/load_balancer/balance.rs`

## Phase 7: Integration mit Eikthyrnir (8 Schritte)
- [x] Tests für Eikthyrnir-Client schreiben (`tests/eikthyrnir_client_test.rs`: QualityMetric, construction, get_quality_metrics result type)
- [x] `EikthyrnirClient` (TDD, Quality-Daten abrufen) – `src/eikthyrnir_client/client.rs` (GetQualityMetrics RPC)

## Phase 8: Performance-Optimization (6 Schritte)
- [x] Caching (Model-Selection-Cache) – `src/selection/cache.rs`: ModelSelectionCache, select_best_model_cached, TTL optional, invalidate_all; ModelRequirements::cache_key(); Tests: tests/model_selection_cache_test.rs
- [x] Query-Optimization – parallele Evaluation im ModelSelector (tokio::spawn pro Modell, join; weniger Latenz bei vielen Modellen)

## Phase 9: Documentation & Testing (6 Schritte)
- [x] Dokumentation – README „Implementierungsstand (Für Entwickler)“: Registry, Evaluator, Selector, Cache, Load-Balancer, Eikthyrnir; Verweis auf IMPLEMENTATION_PLAN
- [x] E2E-Tests (Request → Evaluation → Selection → Response) – `tests/e2e_selection_test.rs`: e2e_request_evaluation_selection_response, e2e_cached_selection_same_response
- [x] Performance-Tests – `tests/performance_selection_test.rs`: selection_latency_under_threshold (≤2s), cached_selection_faster_than_uncached

---

**Schritte gesamt**: ~81
**Phasen**: 9

**Hinweise**:
- Alle Schritte folgen TDD (Tests zuerst, dann Implementierung)
- **ALLE Tests müssen in Containern laufen** - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren
- Container-Setup muss vollständig isolierte Test-Umgebung bereitstellen
