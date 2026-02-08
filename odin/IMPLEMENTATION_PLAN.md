# IMPLEMENTATION_PLAN - Odin (Main Orchestrator)

## Übersicht

Odin ist der Main Orchestrator - er koordiniert alle Services (Thor, Freki, Geri, Huginn/Muninn, Loki, etc.), verarbeitet User-Requests, und orchestriert Actions.

**Programmiersprache**: Rust

## Entschiedene Konfiguration
✅ **Protobuf-Rust-Tool**: prost + tonic
✅ **Plugin-System**: Compile-Time (für bessere Performance und Sicherheit)

---

## Phase 1: Projekt-Setup (10 Schritte)
- [x] Cargo-Projekt
- [x] Dependencies (tokio, tonic, serde, tracing, anyhow, config, notify)
- [x] Verzeichnisstruktur (`src/orchestration/`, `src/services/`, `src/plugins/`, `src/grpc/`)
- [x] Build-System (build.rs für Protobuf)
- [x] Test-Infrastruktur
  - [x] Container-Setup für Tests (Dockerfile.test, docker-compose.test.yml)
  - [x] Mock-Services (Thor, Freki, Geri, Huginn/Muninn, Loki, Heimdall, Skuld) – **ein Container pro Projekt**: Mocks laufen im selben Test-Container als Hintergrundprozesse (tests/scripts/run-tests-with-mocks.sh)
  - [x] Test-Utilities für Container-basierte Tests
  - [x] **WICHTIG**: Alle Tests müssen in Containern laufen – canonical run: `docker compose -f docker-compose.test.yml run --rm odin-test` bzw. `./scripts/run-tests-in-container.sh`; README § Testing
- [x] Settings-System (vollständig implementiert mit Hot-Reload)

## Phase 2: Protobuf & gRPC (10 Schritte)
- [x] Protobuf-Definitions (`proto/odin.proto`)
- [x] gRPC-Server (Odin) - Basis-Implementation vorhanden
- [x] gRPC-Clients für Services (Thor, Freki, Geri, Huginn/Muninn, Loki, Heimdall)

## Phase 3: Service-Discovery & Lifecycle (15 Schritte)
- [x] Tests für Service-Discovery schreiben (tests/services/registry_test.rs, lifecycle_test.rs; LifecycleManager nutzt Registry)
- [x] `ServiceRegistry` - Basis-Implementation vorhanden (ohne Einherjar-Protocol, ohne Capability-Aggregation)
- [x] `ServiceLifecycleManager` - Basis-Implementation vorhanden; nutzt Registry für start/stop/health_check
- [x] Einherjar-Protocol Integration (ServiceRegistry::register_from_capability(service_url, CapabilityResponse); Test: registry_register_from_capability_registers_service_with_domains_and_keywords)
- [x] Capability-Aggregation (CapabilityCache::get_aggregated → AggregatedCapabilities by_domain/by_keyword; services_for_domain/services_for_keyword; Test: capability_cache_get_aggregated_returns_services_by_domain_and_keyword)
- [x] Health-Checks via Netzwerk (ServiceLifecycleManager::health_check_reachable, TCP connect + Timeout; Tests: lifecycle_health_check_reachable_false_when_port_unreachable, lifecycle_health_check_reachable_true_when_port_listening)
- [x] Vollständige Service-Lifecycle-Implementierung: echte Prozess-Start/Stop (ProcessRunner-Trait, ServiceLifecycleManager::with_process_runner; start_service/stop_service rufen Runner auf; Tests: lifecycle_start_service_calls_process_runner_when_set, lifecycle_stop_service_calls_process_runner_when_set)

## Phase 4: Request-Processing (15 Schritte)
- [x] Tests für Request-Processor schreiben (processor_test: Fallback audio/image/video/text/unknown, mit/ohne Responsibility)
- [x] `RequestProcessor` - Basis-Struktur vorhanden (nur Placeholder-Implementation, TODO-Comments)
- [x] Fallback nutzt ActionOrchestrator für action-ähnlichen Text (new_with_action_fallback, plan_actions → „Planned N action(s)“)
- [x] Vollständige Request-Processing-Logik (mit ResponsibilityManager: Parse=determine_responsibility → Route=route_request → Coordinate=execute_service_request; Fallback: type/keyword + optional plan_actions)

## Phase 5: Action-Orchestration (15 Schritte)
- [x] Tests für Action-Orchestrator schreiben (action_test: plan file/terminal/app/fallback, execute ohne Client/non-Thor)
- [x] `ActionOrchestrator` - vollständige Logik (plan_actions, execute_actions via Thor, Handle-Results)
- [x] Vollständige Action-Orchestration-Logik (Plan-Actions, Execute-Actions-via-Thor, Handle-Results)

## Phase 6: Plugin-System (OdinPlugin-Interface) (15 Schritte)
- [x] Tests für Plugin-System schreiben (tests/plugin_test.rs: register, get, list, process_request, overwrite)
- [x] `PluginManager` - Basis-Implementation vorhanden
- [x] `OdinPlugin` Trait - definiert
- [x] Plugin-Kommunikation via gRPC (Odin ↔ Plugins): GrpcPluginProxy, ProcessClient, OdinGrpcProcessClient; Tests: grpc_plugin_proxy_*, plugin_manager_register_grpc_proxy_then_get_and_process
- [x] Einherjar Protocol Integration (gRPC für Capability-Exposure): PluginManager::register_remote_plugin(name, base_url, capabilities); Capabilities typisch aus EinherjarClient.get_capabilities(); Test: plugin_manager_register_remote_plugin_unreachable_url_returns_err
- [x] Responsibility Service Integration (gRPC für Responsibility-Management): ProtocolManager::resolve_service_url nutzt Capability-Cache für Plugins; take/return/reject_responsibility rufen ResponsibilityClient an Plugin-URL
- [x] Frigg/Valkyries-Integration: ServiceUrls.frigg/valkyries, discover_all_capabilities inkl. Plugins, resolve_service_url für frigg/valkyries, bootstrap::bootstrap_frigg_valkyries_plugins in main; Test: bootstrap_frigg_valkyries_both_disabled_leaves_plugin_list_empty

## Phase 7: Responsibility-Service (10 Schritte)
- [x] Tests für Responsibility-Manager schreiben (tests/orchestration/responsibility_test.rs)
- [x] `ResponsibilityManager` - implementiert (src/orchestration/responsibility.rs)
- [x] Determine-Responsibility-Logik
- [x] Route-based-on-Responsibility

## Phase 8: Performance-Optimization (8 Schritte)
- [x] Request-Queuing (RequestQueue in utils; RequestProcessor::process_one_from_queue; Test: process_one_from_queue_processes_queued_request)
- [x] Parallel-Processing (ParallelProcessor in utils; RequestProcessor::process_parallel(self: Arc<Self>, requests); Test: process_parallel_processes_multiple_requests)
- [x] Caching (ResponseCache in utils; RequestProcessor::with_response_cache, cache by request_id; Test: processor_with_cache_returns_cached_response_on_duplicate_request)

## Phase 9: Monitoring & Security (6 Schritte)
- [x] Basic Logging (tracing vorhanden)
- [x] Monitoring (MonitoringService aus utils; RequestProcessor::with_monitoring, active_requests 1/0 in process(); Test: processor_with_monitoring_updates_active_requests)
- [x] Audit-Logging (AuditEvent, AuditLogger in src/orchestration/audit.rs; RequestProcessor::with_audit_logger; Test: processor_audit_logs_request_received_when_logger_set)
- [x] Error-Handling (OrchestrationError in src/orchestration/error.rs, ResponsibilityManager nutzt strukturierte Fehler; Test: processor_with_responsibility_empty_cache_returns_orchestration_error)

## Phase 10: Documentation & Testing (6 Schritte)
- [x] Dokumentation (Rustdoc: orchestration, plugins, OrchestrationError; Modul- und API-Kommentare)
- [x] E2E-Tests (processor_e2e_flow_with_responsibility_and_capability; voller Container-E2E: tests/e2e_container_test.rs, e2e_container_request_flow_uses_mock_urls – nutzt Env-URLs, Request-Flow in Container; Mocks aktuell TCP-only → Ok oder Err je nach Mock-Verfügbarkeit)
- [x] Performance-Tests (plan_actions_performance_reasonable_latency, processor_performance_action_fallback_reasonable_latency; Latenz-Grenzen für rein lokale Pfade)

---

## Phase 11: Device-Scheduler & Device-Loop (Asgard/Midgard/Devices)

**Ziel:** Odin erhält einen vom User explizit aktivierbaren Hintergrund-Scheduler/Loop, der – abhängig von Settings – regelmäßig den Status und die Fähigkeiten der angebundenen Services/Devices aktualisiert und damit die Grundlage für Haus-/Fahrzeug-/Roboter-Steuerung über Asgard/Midgard legt.

- [x] **Device-Scheduler-Design (High-Level)**
  - Definition der Rolle des Schedulers im Kontext von Odin (nur Orchestrierung, keine direkte Hardware-Ansteuerung)
  - Klärung der Schnittstellen Asgard/Midgard ↔ Odin ↔ Thor/Bifrost/Jotunheim
  - Dokumentation im README (`Device Scheduler & Device-Loop`-Abschnitt)
- [x] **Scheduler-Modulstruktur**
  - Neues Modul `src/scheduler/` mit `DeviceScheduler` als zentraler Einstieg
  - Klare Abgrenzung zu `services::*` (Service-Lifecycle) und `protocols::*` (Einherjar/Responsibility)
  - Trait-basierte Abstraktion (`CapabilityDiscoverer`) für Capability-Refresh, um Tests zu erleichtern
- [x] **Konfigurierbares Polling / Intervall**
  - Nutzung von `state_sync.sync_interval_ms` aus `OdinSettings` als zentrales Intervall
  - Fallback auf sinnvollen Default (z.B. 1000ms) bei fehlender Konfiguration
  - Validierung im Settings-System (kein `0`, sinnvolle Grenzen)
- [x] **Explizite Aktivierung durch User**
  - Neues Settings-Objekt `scheduler` mit mindestens `enabled` und `capability_refresh_enabled`
  - Default: `scheduler.enabled = false` (kein Hintergrund-Loop aktiv ohne User-Opt-in)
  - README-Abschnitt beschreibt, wie der User den Scheduler aktiviert und welche Teilfunktionen (z.B. Capability-Refresh) er ein-/ausschalten kann
- [x] **Capability-Refresh-Loop**
  - Hintergrund-Task, der periodisch `discover_all_capabilities()` aufruft (nur wenn `scheduler.enabled` und `scheduler.capability_refresh_enabled`)
  - Robustes Error-Handling (Logging, kein Crash bei Netzwerkfehlern)
  - Tests: Einzelner Poll-Durchlauf (`poll_once`) darf nie paniken, auch wenn keine Services erreichbar sind
- [ ] **Device-Loop-Erweiterung (schrittweise)**
  - [x] **DeviceRegistry (logische Devices)**
    - Typen: `LogicalDevice` (id, kind: House/Vehicle/Robot, platform_id, name), `DeviceRegistry` (register, get, list)
    - Zuordnung zu Platform (Asgard/Midgard) über `platform_id`; Odin hält nur die Registry, keine Hardware-Logik
    - Tests: register/list/get, leere Registry (tests/scheduler/device_registry_test.rs)
  - [ ] **Integration Scheduler ↔ DeviceRegistry (später)**
    - Scheduler-Loop kann bei aktiviertem Device-Polling die Registry mit Status-Updates füttern (Quelle: Asgard/Midgard-API)
    - Konfiguration: z.B. `scheduler.device_polling_enabled` (zukünftig)
  - [ ] **Design Sensor-/Aktor-Polling**
    - Odin orchestriert nur: Anforderung an Asgard/Midgard (gRPC/Bifrost), Ausführung in Platform/Thor/Bifrost/Jotunheim
    - Dokumentation im README (Ablauf, Verantwortungsgrenzen)
  - [ ] **Event-getriebene Signale (Vorbereitung)**
    - Vorbereitung auf Bifrost-Events, StateSync, Responsibility/Einherjar für Device-Events (später)

*Hinweis:* Phase 11 baut direkt auf den bestehenden Service-Discovery-/Capability-Mechanismen (Phase 3) und den State-Sync-/Network-Konzepten aus dem README auf. Die konkrete Hardware-/Haus-/Fahrzeug-Logik bleibt in den Platformen (Midgard, Asgard, Jotunheim); Odin koordiniert nur.

---

**Schritte gesamt**: ~110
**Phasen**: 11

**Hinweise**:
- Alle Schritte folgen TDD (Tests zuerst, dann Implementierung)
- **ALLE Tests müssen in Containern laufen** - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren
- Container-Setup muss vollständig isolierte Test-Umgebung bereitstellen
