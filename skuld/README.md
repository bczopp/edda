# Skuld - LLM-Selection Service

## Übersicht

Skuld ist der LLM-Selection Service, der auf allen Devices installiert werden muss. Odin benötigt diesen Service für die Auswahl des optimalen LLM/Device für einen Request.

**Mythologische Bedeutung**: Skuld ist eine der drei Nornen (Schicksalsgöttinnen) und steht für die Zukunft. Sie entscheidet über das Schicksal (welches LLM/Device für einen Request verwendet wird).

**Programmiersprache**: Rust

**Wichtig**: Skuld ist ein separater Service, der auf allen Devices (Midgard, Alfheim, Asgard, Ragnarok) installiert werden muss. Er ist auch bei Yggdrasil verfügbar für globale LLM-Auswahl.

**Tests ausführen:** Von `skuld/`: `docker compose -f docker-compose.test.yml run --rm skuld-test` oder `./scripts/run-tests.sh` / `.\scripts\run-tests.ps1`. Von Repo-Root: `skuld/scripts/run-tests.sh` bzw. `.\skuld\scripts\run-tests.ps1`. Die Test-Umgebung stellt Postgres und `DATABASE_URL` bereit; Migrationen liegen unter `migrations/` und werden beim Start bzw. in den Integrationstests ausgeführt. **CI:** Bei Push/PR auf `skuld/**` läuft die Pipeline [.github/workflows/skuld.yml](../.github/workflows/skuld.yml) (Test im Container, Lint).

### Implementierungsstand (Für Entwickler)

Aktuell umgesetzt (Details siehe [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md)):

- **Model Registry** (`src/registry/`): PostgreSQL-basierte Registrierung und Abfrage von Modellen (`list_models`, `register_model`, `get_model_info`); Schema in `migrations/`.
- **Multi-Factor-Evaluation** (`src/evaluation/`): `ModelEvaluator` mit Performance-, Reliability- und Efficiency-Scores; gewichteter Gesamtscore.
- **Model Selection** (`src/selection/selector.rs`): `ModelSelector` wählt das beste Modell anhand der Evaluation; parallele Evaluation aller Modelle (Query-Optimization).
- **Model-Selection-Cache** (`src/selection/cache.rs`): `ModelSelectionCache` mit `select_best_model_cached(requirements)`; optional TTL, `invalidate_all()`; Key über `ModelRequirements::cache_key()`.
- **Load-Balancing** (`src/load_balancer/`): Weighted Round-Robin für Modell-Verteilung.
- **Eikthyrnir-Integration** (`src/eikthyrnir_client/`): gRPC-Client für Quality-Metriken (GetQualityMetrics).
- **Geri-Integration** (`src/geri_client/`): gRPC-Client für Model-Registry (ListModels, GetModelInfo).

✅ **Alle Phasen 1–9 komplett**. Skuld ist produktionsbereit.

## Verantwortlichkeiten

### 1. LLM-Selection
- **Netzwerkplan-Analyse**: 
  - **Analyse-Algorithmen**: Graph-basierte Analyse-Algorithmen für Netzwerkplan
  - **Plan-Updates**: Reagiert auf Netzwerkplan-Updates (Event-basiert oder Polling)
  - **Analyse-Caching**: Analyse-Ergebnisse werden gecacht für bessere Performance
- **Effektivster Weg**: 
  - **Routing-Algorithmen**: Dijkstra, A*, etc. für Routing-Optimierung
  - **Latency-Minimierung**: Findet Weg mit minimaler Latency
  - **Routing-Fehler**: Bei Routing-Fehlern wird automatisch auf alternative Route ausgewichen
- **Effektivstes Model**: 
  - **Model-Bewertungs-Algorithmen**: Multi-Faktor-Bewertung für Model-Quality und Performance
  - **Model-Ausfälle**: Bei Model-Ausfall wird automatisch auf alternatives Model ausgewichen
- **User-Vorgaben**: 
  - **Vorgaben-Prioritäten**: User-Vorgaben haben Priorität über automatische Auswahl
  - **Vorgaben-Konflikte**: Bei Vorgaben-Konflikten wird User um Klärung gebeten oder bestes Match gewählt
- **Provider-Integration**: Bezieht Provider mit ein (wenn Marketplace aktiv)

### 2. Netzwerkplan-Verarbeitung
- **Plan-Analyse**: Analysiert Netzwerkplan (von Odin oder Yggdrasil erstellt)
- **Cache-Verwaltung**: Verwaltet Cache für Netzwerkplan-Analysen
- **Plan-Updates**: Reagiert auf Netzwerkplan-Updates

### 3. Entscheidungslogik
- **Multi-Faktor-Bewertung**: Bewertet Devices/Models basierend auf mehreren Faktoren
- **Scoring-Algorithmus**: Berechnet Score für jedes Device/Model
- **Optimale Auswahl**: Wählt optimales Device/Model basierend auf Score

## Service-Interfaces

### Inputs
- `NetworkPlan` (von Odin oder Yggdrasil) - Netzwerkplan mit verfügbaren Devices/Models
  - Verfügbare Devices/Models
  - Provider (wenn vorhanden)
  - Capabilities
  - Quality-Metriken
  - Latency-Info
  - User-Preferences

- `SelectionRequest` (von Odin) - Request für LLM-Auswahl
  - Request-Requirements
  - User-Preferences
  - Cost-Constraints
  - Latency-Requirements

### Outputs
- `SelectionResponse` (an Odin) - Empfehlung für LLM/Device
  - Gewähltes Device/Model
  - Begründung
  - Alternative Optionen
  - Geschätzte Kosten/Latency

## Workflow

### LLM-Auswahl

1. **Odin erstellt Netzwerkplan**
   - Odin erstellt Netzwerkplan (on the fly, mit Cache)
   - Enthält: Verfügbare Devices, Models, Provider, Capabilities, Quality-Metriken, Latency-Info

2. **Odin fragt Skuld**
   - Odin sendet `SelectionRequest` mit Netzwerkplan an Skuld
   - Skuld analysiert Netzwerkplan

3. **Skuld analysiert und entscheidet**
   - Skuld analysiert alle verfügbaren Optionen
   - Berechnet Score für jedes Device/Model:
     - Effektivster Weg (Routing, Latency)
     - Effektivstes Model (Quality, Performance)
     - Entspricht User-Vorgaben (Requirements, Preferences)
   - Wählt optimales Device/Model

4. **Response an Odin**
   - Skuld sendet `SelectionResponse` mit Empfehlung
   - Odin nutzt diese Information für Request-Routing

### Netzwerkplan-Cache

- **Cache-Strategie**: Netzwerkplan wird gecacht, um wiederholte Analysen zu vermeiden
- **Cache-Invalidation**: Cache wird invalidiert bei:
  - Netzwerkplan-Updates
  - Device-Status-Änderungen
  - Quality-Metrik-Updates
  - Timeout (z.B. 5 Minuten)

## Scoring-Algorithmus

### Faktoren

1. **Effektivster Weg (30% Gewichtung)**
   - Routing-Optimierung
   - Latency-Minimierung
   - Network-Hops

2. **Effektivstes Model (40% Gewichtung)**
   - Quality-Metriken
   - Performance-Metriken
   - Verfügbarkeit

3. **User-Vorgaben (30% Gewichtung)**
   - Requirements-Erfüllung
   - Preferences-Berücksichtigung
   - Cost-Constraints

### Provider-Integration

Wenn Marketplace aktiv:
- **Provider-Quality**: Berücksichtigt Provider-Quality-Metriken
- **Provider-Pricing**: Berücksichtigt Provider-Pricing
- **Provider-Availability**: Berücksichtigt Provider-Verfügbarkeit
- **Fair Distribution**: Berücksichtigt Fair-Distribution-Score

## Netzwerkplan-Format

```rust
struct NetworkPlan {
    devices: Vec<DeviceInfo>,
    models: Vec<ModelInfo>,
    providers: Option<Vec<ProviderInfo>>,
    capabilities: HashMap<String, Capability>,
    quality_metrics: HashMap<String, QualityMetric>,
    latency_info: HashMap<String, LatencyInfo>,
    user_preferences: UserPreferences,
    timestamp: DateTime,
}
```

### DeviceInfo
- Device-ID
- Device-Type
- Capabilities
- Available Models
- Status
- Latency-Info

### ModelInfo
- Model-ID
- Model-Type
- Quality-Metrics
- Performance-Metrics
- Availability
- Cost-Info

### ProviderInfo (optional)
- Provider-ID
- Provider-Quality
- Provider-Pricing
- Provider-Availability
- Fair-Distribution-Score

## gRPC Communication

**gRPC Service Communication:**
- **Odin ↔ Skuld**: gRPC für LLM-Selection
- **Type-Safe**: Protobuf garantiert korrekte Service-Interfaces
- **Streaming**: Built-in Streaming für große Responses

**gRPC Connection-Management:**
- **Connection-Pooling**: Wiederverwendung von Verbindungen für bessere Performance
- **Connection Reuse**: Connections werden effizient wiederverwendet
- **Automatische Reconnection**: Kombination aus sofortigem Versuch + Exponential Backoff
  - Sofortiger Reconnect-Versuch bei Verbindungsabbruch
  - Nach erstem Fehler beginnt Exponential Backoff
  - Maximale Wartezeit (z.B. 60 Sekunden)
  - Kontinuierliche Versuche zur Wiederherstellung
- **Connection Monitoring**: Verbindungsstatus wird überwacht

**gRPC Error-Handling:**
- **gRPC Status-Codes**: gRPC-Fehler werden über Status-Codes behandelt
- **Retry-Mechanismen**: Automatischer Retry mit Exponential Backoff (siehe gemeinsame Klärungspunkte)
- **Timeout-Konfiguration**: Adaptive Timeouts mit Minimum/Maximum
- **Fallback**: Bei Fehler Fallback zu alternativen Routen

## Monitoring & Logging

### Strukturiertes Logging

**Strukturiertes Logging:**
- Structured Logging mit strukturierten Daten
- Log Levels: Verschiedene Log-Level (DEBUG, INFO, WARN, ERROR, etc.)
- Context Tracking: Context wird mitgeloggt
- Log Rotation: Automatische Log-Rotation
- Umfassendes Logging für Debugging und Monitoring

### Performance-Monitoring

**Performance-Monitoring:**
- Performance-Metriken: Response-Zeiten, Durchsatz, Resource-Usage
- Performance-Tracking für alle LLM-Selection-Requests
- Kontinuierliche Überwachung und Performance-Optimierung
- Alerts bei Performance-Problemen

## Abhängigkeiten

### Keine Core Library

- **WICHTIG**: Es gibt keine Edda Core Library
- **Separate Projekte**: Wenn gemeinsame Komponenten benötigt werden (DTOs, Protocols, Utils), sollte ein separates Projekt erstellt werden
- **Selektive Nutzung**: Dies hält Apps klein, da genau gewählt werden kann, was benötigt wird
- **Keine Abhängigkeit**: Skuld sollte nicht auf Dateien/Protocols/Utils aus dem `edda` Verzeichnis verweisen (KEIN PROJEKT - nur Metadaten-Sammlung)

### Technische Abhängigkeiten

- **Network-Stack**: Für Latency-Messungen (optional)
- **Caching-Library**: Für Netzwerkplan-Cache

## Settings und Konfiguration

### Allgemeine Settings-Prinzipien

**Wichtig**: Diese Prinzipien gelten für alle Services und Platformen im Edda-System.

#### Settings-Format
- **Format**: Vermutlich JSON-Format (es sei denn im Rust-Kontext gibt es ein besseres Format, das ebenso einfach für Menschen zu verstehen ist)
- **Menschlich lesbar**: Settings-Dateien müssen für Menschen einfach zu verstehen und zu bearbeiten sein
- **Validierung**: Settings werden beim Laden validiert (Schema-Validierung)

#### Platform-Integration
- **Settings-Sammlung**: Platformen müssen alle Settings/Konfigurationsdateien sammeln, die auf dem Device bzw. auf der Platform aktuell verfügbar und aktiv sind
- **Frontend-Konfiguration**: Settings müssen über Settings im Frontend konfigurierbar gemacht werden
- **Zentrale Verwaltung**: Platform stellt zentrale Settings-Verwaltung zur Verfügung

#### Hot-Reload
- **Keine Neukompilierung**: Änderungen an den Settings sollen nicht dazu führen, dass das Projekt/der Service neu kompiliert werden muss
- **Runtime-Reload**: Die neuen Werte können einfach zur Laufzeit neu geladen werden
- **Service-Funktionen**: Services müssen entsprechende Funktionen zur Verfügung stellen (Hot-Reload, Settings-API, etc.)

#### Service-spezifische Settings
- **Projekt-spezifisch**: Was genau in einer Settings/Konfigurationsdatei steht, hängt sehr stark vom Service oder der Platform ab
- **Dokumentation**: Service-spezifische Settings müssen in der jeweiligen README dokumentiert werden
- **Beispiele**: Service-spezifische Settings-Beispiele sollten in der README enthalten sein

### Skuld-spezifische Settings

**Settings-Inhalt (wird während Implementierung definiert)**
- LLM-Selection-Algorithmus-Einstellungen
- Netzwerkplan-Cache-Einstellungen
- Provider-Prioritäten

## Integration

- **Odin**: Nutzt Skuld für LLM-Auswahl
- **Yggdrasil**: Skuld ist auch bei Yggdrasil verfügbar für globale LLM-Auswahl
- **Lokale Devices**: Skuld muss auf allen Devices installiert sein (Midgard, Alfheim, Asgard, Ragnarok)

## Performance

### Performance-Optimierungen
- **Cache-Strategie**: Netzwerkplan-Cache verhindert wiederholte Analysen
- **Effiziente Algorithmen**: Optimierte Scoring-Algorithmen
- **Async Processing**: Asynchrone Verarbeitung von Selection-Requests
- **Parallel-Analyse**: Parallele Analyse mehrerer Optionen

### Performance-Metriken
- Schnelle Selection (< 50ms für Standard-Selection)
- Effiziente Netzwerkplan-Analyse (optimierte Algorithmen)
- Niedrige CPU-Nutzung (durch Caching)

## Implementierungs-Notizen

- **Programmiersprache**: Rust
- **Muss auf allen Devices installiert werden**: Midgard, Alfheim, Asgard, Ragnarok
- **Auch bei Yggdrasil verfügbar**: Für globale LLM-Auswahl
- **Cache-Strategie**: Netzwerkplan-Cache mit Timeout-basierter Invalidation
- **Scoring-Algorithmus**: Multi-Faktor-Bewertung mit konfigurierbaren Gewichtungen
- **Provider-Integration**: Unterstützung für Marketplace-Provider
- **Performance**: Optimiert für schnelle Selection-Entscheidungen

