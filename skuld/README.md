# Skuld - LLM-Selection Service

## Übersicht

Skuld ist der LLM-Selection Service, der auf allen Devices installiert werden muss. Odin benötigt diesen Service für die Auswahl des optimalen LLM/Device für einen Request.

**Mythologische Bedeutung**: Skuld ist eine der drei Nornen (Schicksalsgöttinnen) und steht für die Zukunft. Sie entscheidet über das Schicksal (welches LLM/Device für einen Request verwendet wird).

**Programmiersprache**: Rust

**Wichtig**: Skuld ist ein separater Service, der auf allen Devices (Midgard, Alfheim, Asgard, Ragnarok) installiert werden muss. Er ist auch bei Yggdrasil verfügbar für globale LLM-Auswahl.

## Verantwortlichkeiten

### 1. LLM-Selection
- **Netzwerkplan-Analyse**: Analysiert Netzwerkplan mit verfügbaren Devices/Models
- **Effektivster Weg**: Findet effektivsten Weg (Routing, Latency)
- **Effektivstes Model**: Findet effektivstes Model (Quality, Performance)
- **User-Vorgaben**: Berücksichtigt User-Vorgaben (Requirements, Preferences)
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

## Abhängigkeiten

- **Edda Core Library**: DTOs, Protocols, Utils
- **Network-Stack**: Für Latency-Messungen (optional)
- **Caching-Library**: Für Netzwerkplan-Cache

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

