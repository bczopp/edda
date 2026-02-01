# Gladsheim - Service Manager & Runtime Manager

## Übersicht

Gladsheim ist der **Service-Manager und Runtime-Manager** auf allen Plattformen (Midgard, Alfheim, Asgard, Ragnarok). Er verwaltet den Lifecycle aller Services (Götter), managed Ressourcen (RAM, CPU) und überwacht ihre Gesundheit. Gladsheim ist die "Halle", in der die aktiven Götter versammelt sind - eine Repräsentation dessen, was aktuell im RAM läuft.

**Mythologische Bedeutung**: Gladsheim (Gladsheimr) - "Die goldene Halle der Freude"
- Große Halle in Asgard mit 12 Hochsitzen der Götter
- Ort, wo der Rat der Götter zusammenkommt
- Repräsentiert den RAM - welche Götter sind aktuell geladen und aktiv

**Tests ausführen:** Von `gladsheim/`: `docker compose -f docker-compose.test.yml run --rm gladsheim-test` oder `./scripts/run-tests.sh` (bzw. `.\scripts\run-tests.ps1` unter Windows). **CI:** Bei Push/PR auf `gladsheim/**` läuft [.github/workflows/gladsheim.yml](../.github/workflows/gladsheim.yml) (Test im Container, Lint).

## Architektur

### Gladsheim als Plattform-Komponente

Gladsheim ist **Teil jeder Plattform** (nicht separater Service):
- **Midgard-Gladsheim**: Verwaltet Services auf Desktop
- **Alfheim-Gladsheim**: Verwaltet Services auf Mobile (battery-aware)
- **Asgard-Gladsheim**: Verwaltet Services auf Homeserver (höhere Kapazität)
- **Ragnarok-Gladsheim**: Verwaltet Services auf Terminal (minimal)
- **Jotunheim**: Hat KEIN Gladsheim (zu ressourcenlimitiert für IoT-Devices)

### Sub-Komponenten (Mythologische Diener)

Gladsheim besteht aus vier Dienern, die als Sub-Komponenten spezifische Aufgaben übernehmen:

#### 1. Thjalfi (Thjalfi) - Service Loader

**Mythologische Rolle**: Schneller Diener von Thor, bekannt für seine außerordentliche Geschwindigkeit

**Aufgaben**:
- Lädt Services schnell in den RAM
- Startet Service-Prozesse (std::process / tokio::process)
- Stoppt Services (graceful shutdown)
- Force-Kill bei Bedarf
- Process-Management

**Beispiel**:
```
Odin: "Schick mir Thor"
  ↓
Thjalfi lädt Thor in RAM und startet Prozess
```

#### 2. Byggvir - Resource Manager

**Mythologische Rolle**: Diener von Freyr, bekannt als Verwalter

**Aufgaben**:
- Verwaltet RAM-Allokation für Services
- Verwaltet CPU-Ressourcen
- Überwacht Ressourcen-Nutzung in Echtzeit
- Enforcement von Resource-Limits
- Warnt bei Ressourcen-Engpässen

**Features**:
- Platform-spezifische Resource-Limits (Desktop: hoch, Mobile: niedrig, Battery-aware)
- Dynamische Resource-Anpassung basierend auf Verfügbarkeit
- Resource-Monitoring mit `sysinfo` crate

#### 3. Roskva - Health Monitor

**Mythologische Rolle**: Schwester von Thjalfi, Dienerin

**Aufgaben**:
- Überwacht Service-Gesundheit (periodische Health Checks)
- Erkennt Service-Crashes automatisch
- Triggert Restarts bei Bedarf (konfigurierbar)
- Status-Reporting für alle Services
- Warnt Odin bei kritischen Problemen

**Health Check Strategien**:
- HTTP Health Endpoints (GET /health)
- gRPC Health Check Protocol
- Process-Monitoring (ist Prozess noch aktiv?)
- Configurable Check-Intervalle

#### 4. Skirnir - Service Registry

**Mythologische Rolle**: Bote von Freyr, bekannt als Kommunikator

**Aufgaben**:
- Lokale Registry aller Services auf der Plattform
- Service-Discovery (lokal)
- Service-Status-Verwaltung (running, stopped, crashed, starting)
- API für Odin (gRPC)
- Capability-Aggregation (arbeitet mit Einherjar Protocol)

**Registry-Daten**:
- Service-Name
- Service-Status
- Process-ID
- Start-Zeit
- Resource-Usage
- Health-Status

## Kommunikationsfluss

### Odin → Gladsheim (Service-Management)

```
Odin: "Starte Thor"
  ↓ (gRPC Call an Gladsheim)
Skirnir (Registry): Prüft ob Thor bereits läuft
  ↓ (wenn nicht)
Thjalfi (Loader): Lädt Thor in RAM, startet Prozess
  ↓
Byggvir (Resources): Alloziert RAM/CPU für Thor
  ↓
Roskva (Monitor): Startet Health-Monitoring für Thor
  ↓
Skirnir (Registry): Registriert Thor als "running"
  ↓
Odin erhält Bestätigung (ServiceStatus)
```

### Odin → Services (direkt via gRPC)

```
Odin → Thor (gRPC): ExecuteAction(...)
  ↓
Thor führt Action aus
  ↓
Thor → Odin (gRPC): ActionResult(...)
```

**WICHTIG**: Gladsheim ist NICHT in der Service-Kommunikation involviert! Odin kommuniziert direkt mit Services via gRPC.

## Features

### Service-Lifecycle-Management (Thjalfi)

- **Start Services**: Startet Service-Prozesse mit konfigurierbaren Parametern
- **Stop Services**: Graceful Shutdown mit Timeout, Force-Kill als Fallback
- **Restart Services**: Stop + Start in einem Schritt
- **Process Management**: Verwaltet alle laufenden Service-Prozesse
- **Startup-Validation**: Prüft ob Service erfolgreich gestartet wurde
- **Shutdown-Timeout**: Konfigurierbare Timeouts für graceful shutdown

### Resource Management (Byggvir)

- **RAM-Allokation**: Memory-Limits pro Service
- **CPU-Limits**: CPU-Prozent pro Service
- **Resource-Monitoring**: Echtzeit-Monitoring von RAM/CPU-Usage
- **Resource-Enforcement**: Automatische Actions bei Limit-Überschreitung
- **Platform-spezifisch**: Unterschiedliche Limits für Desktop/Mobile/Server
- **Battery-Aware** (Alfheim): Resource-Management basierend auf Battery-Status

### Health Monitoring (Roskva)

- **Periodische Health Checks**: Konfigurierbare Check-Intervalle (default: 5s)
- **Crash Detection**: Automatische Erkennung von Service-Crashes
- **Auto-Restart**: Konfigurierbar, mit Max-Restart-Attempts
- **Status-Reporting**: Health-Status-Berichte für Odin
- **Alerting**: Warnt Odin bei kritischen Problemen
- **Health History**: Speichert Health-Check-Historie

### Service Registry (Skirnir)

- **Lokale Service-Liste**: Liste aller Services auf der Plattform
- **Status-Tracking**: Aktueller Status jedes Services
- **Service-Discovery**: Lokale Discovery für Odin
- **API für Odin**: gRPC-API für Service-Management
- **Metadata-Verwaltung**: Service-Metadaten (Start-Zeit, Resource-Usage, etc.)

## API-Design (gRPC)

### GladsheimService

```protobuf
service GladsheimService {
  // Service Lifecycle (Thjalfi)
  rpc StartService(StartServiceRequest) returns (ServiceStatus);
  rpc StopService(StopServiceRequest) returns (ServiceStatus);
  rpc RestartService(RestartServiceRequest) returns (ServiceStatus);
  
  // Service Status (Skirnir)
  rpc GetServiceStatus(ServiceStatusRequest) returns (ServiceStatus);
  rpc ListServices(ListServicesRequest) returns (ServiceList);
  
  // Health Monitoring (Roskva)
  rpc GetServiceHealth(ServiceHealthRequest) returns (ServiceHealth);
  rpc SubscribeServiceHealth(HealthSubscribeRequest) returns (stream HealthUpdate);
  
  // Resource Management (Byggvir)
  rpc GetResourceUsage(ResourceUsageRequest) returns (ResourceUsage);
  rpc SetResourceLimits(ResourceLimitsRequest) returns (ResourceLimits);
  rpc GetResourceLimits(ServiceRequest) returns (ResourceLimits);
}

message StartServiceRequest {
  string service_name = 1;
  map<string, string> environment_vars = 2;
  ResourceLimits resource_limits = 3;
}

message ServiceStatus {
  string service_name = 1;
  ServiceState state = 2;
  int32 process_id = 3;
  int64 start_time_unix = 4;
  ResourceUsage resource_usage = 5;
}

enum ServiceState {
  UNKNOWN = 0;
  STARTING = 1;
  RUNNING = 2;
  STOPPING = 3;
  STOPPED = 4;
  CRASHED = 5;
}

message ServiceHealth {
  string service_name = 1;
  HealthStatus status = 2;
  string message = 3;
  int64 last_check_unix = 4;
}

enum HealthStatus {
  HEALTHY = 0;
  UNHEALTHY = 1;
  UNKNOWN_HEALTH = 2;
}

message ResourceUsage {
  uint64 memory_bytes = 1;
  float cpu_percent = 2;
}

message ResourceLimits {
  uint64 max_memory_bytes = 1;
  float max_cpu_percent = 2;
}
```

## Integration in Plattformen

### Midgard (Desktop)

**Platform-Implementierung**:
```rust
// src/platform/gladsheim/mod.rs
pub struct Gladsheim {
    thjalfi: ServiceLoader,
    byggvir: ResourceManager,
    roskva: HealthMonitor,
    skirnir: ServiceRegistry,
    config: GladsheimConfig,
}
```

**Konfiguration**:
```json
{
  "gladsheim": {
    "resource_limits": {
      "max_services": 15,
      "default_memory_mb": 1024,
      "default_cpu_percent": 50
    },
    "health_monitoring": {
      "check_interval_ms": 5000,
      "auto_restart": true,
      "max_restart_attempts": 3
    }
  }
}
```

### Alfheim (Mobile)

**Mobile-spezifische Anpassungen**:
- **Reduzierte Resource-Limits**: Weniger RAM/CPU
- **Battery-Aware**: Services stoppen bei niedrigem Battery
- **Background-Processing**: Berücksichtigt iOS/Android-Background-Limits
- **Network-Aware**: Services pausieren bei schlechter Netzverbindung

**Konfiguration**:
```json
{
  "gladsheim": {
    "resource_limits": {
      "max_services": 5,
      "default_memory_mb": 256,
      "default_cpu_percent": 25
    },
    "battery_management": {
      "stop_services_below_percent": 15,
      "reduce_monitoring_below_percent": 30
    }
  }
}
```

### Asgard (Homeserver)

**Server-spezifische Anpassungen**:
- **Höhere Resource-Limits**: Mehr RAM/CPU verfügbar
- **Mehr parallele Services**: Bis zu 20+ Services gleichzeitig
- **24/7-Betrieb**: Optimiert für Dauerbetrieb

**Konfiguration**:
```json
{
  "gladsheim": {
    "resource_limits": {
      "max_services": 25,
      "default_memory_mb": 2048,
      "default_cpu_percent": 75
    }
  }
}
```

### Ragnarok (Terminal)

**Terminal-spezifische Anpassungen**:
- **Minimale Resources**: Nur essenzielle Services
- **CLI-optimiert**: Text-basierte Status-Ausgabe
- **Keine GUI**: Kein Overhead für UI-Components

**Konfiguration**:
```json
{
  "gladsheim": {
    "resource_limits": {
      "max_services": 8,
      "default_memory_mb": 512,
      "default_cpu_percent": 30
    }
  }
}
```

### Jotunheim (IoT)

**KEIN Gladsheim**: Jotunheim-Devices haben kein Gladsheim aufgrund extremer Resource-Limitierungen (ESP32 mit ~520KB RAM). Services werden manuell gestartet und managed.

## Beziehung zu anderen Komponenten

### Zu Odin

**Odin orchestriert, Gladsheim managed**:
- **Odin entscheidet**: Welche Services benötigt werden
- **Gladsheim führt aus**: Start/Stop/Restart von Services
- **Kommunikation**: Odin → Gladsheim (gRPC) für Service-Management
- **KEINE Vermittlung**: Odin ↔ Services kommunizieren direkt (gRPC), nicht über Gladsheim

**Beispiel-Workflow**:
```
1. Odin: "Ich brauche Thor für diese Action"
2. Odin → Gladsheim.StartService("thor")
3. Gladsheim startet Thor
4. Odin → Thor.ExecuteAction(...) [DIREKT via gRPC]
5. Thor → Odin.ActionResult(...) [DIREKT via gRPC]
```

### Zu Einherjar Protocol

**Völlig getrennte Verantwortlichkeiten**:
- **Einherjar Protocol**: Beschreibt Funktionen (WAS kann ein Service)
- **Gladsheim**: Verwaltet Runtime (IST Service aktiv)
- **Keine Überschneidung**: Verschiedene Concerns
- **Zusammenarbeit**: Skirnir kann Einherjar-Daten cachen für schnellere Service-Discovery

### Zu Services (Thor, Freki, Geri, etc.)

**Gladsheim verwaltet, kommuniziert aber nicht**:
- **Gladsheim startet/stoppt**: Service-Prozesse
- **Gladsheim überwacht**: Health und Resources
- **Gladsheim registriert**: Laufende Services
- **KEINE Kommunikation**: Gladsheim vermittelt NICHT zwischen Odin und Services
- **Health-Check-Integration**: Services müssen Health-Endpoints bereitstellen

**Service-Requirements**:
- Services müssen Health-Endpoints implementieren (HTTP `/health` oder gRPC Health Protocol)
- Services müssen graceful shutdown unterstützen (SIGTERM-Handling)
- Services müssen Process-Monitoring tolerieren

### Zu Plattformen

**Gladsheim ist Teil der Plattform**:
- **Nicht separater Service**: Gladsheim ist Plattform-Komponente
- **Platform-spezifisch**: Jede Plattform hat eigene Gladsheim-Konfiguration
- **Platform-Integration**: Gladsheim nutzt Platform-spezifische APIs

## Betroffene Projekte durch Gladsheim-Einführung

### Plattformen (Midgard, Alfheim, Asgard, Ragnarok)

**MUSS angepasst werden**:
- **README.md**: Service-Lifecycle-Abschnitt → Verweis auf Gladsheim
- **IMPLEMENTATION_PLAN.md**: Phase "Service-Discovery & Lifecycle" → Gladsheim-Integration
- **Code**: `ServiceLifecycleManager` → Gladsheim-Integration

**Migration**:
- Bestehendes Service-Lifecycle-Management wird durch Gladsheim ersetzt
- Platform startet Gladsheim als erste Komponente
- Platform kommuniziert mit Gladsheim via gRPC

### Odin

**MUSS angepasst werden**:
- **README.md**: Service-Management-Abschnitt → Gladsheim-API-Nutzung
- **IMPLEMENTATION_PLAN.md**: Service-Lifecycle-Phase → Gladsheim-Client
- **Code**: Service-Start-Logik → Gladsheim-Client-Integration

**Migration**:
- Odin nutzt Gladsheim-API zum Starten/Stoppen von Services
- Direkte Service-Kommunikation bleibt unverändert (gRPC)

### Services (Thor, Freki, Geri, Skuld, etc.)

**SOLLTE angepasst werden**:
- **README.md**: Health-Check-Requirements dokumentieren
- **Code**: Health-Check-Endpoints implementieren
- **Testing**: Health-Check-Tests hinzufügen

**Migration** (optional, aber empfohlen):
- Services implementieren Health-Endpoints (HTTP `/health` oder gRPC Health Protocol)
- Services implementieren graceful shutdown (SIGTERM-Handling)

### Root AGENTS.md

**MUSS angepasst werden**:
- **Infrastructure**: Gladsheim als neues Infrastruktur-Konzept
- **Platform-Konzept**: Plattformen nutzen Gladsheim für Service-Management
- **Architecture Overview**: Gladsheim erwähnen

### Jotunheim

**KEINE Änderung**: Jotunheim hat kein Gladsheim (zu ressourcenlimitiert)

## Technologie-Stack

- **Programmiersprache**: Rust (100%)
- **Service-Prozesse**: `tokio::process` für async process management
- **Resource-Monitoring**: `sysinfo` crate für System-Monitoring
- **IPC**: Lokale gRPC-Kommunikation (Odin ↔ Gladsheim)
- **Health Checks**: 
  - HTTP health endpoints (via `reqwest`)
  - gRPC health check protocol
  - Process monitoring
- **Serialization**: `serde` für Konfiguration
- **Logging**: `tracing` für strukturiertes Logging
- **Error-Handling**: `anyhow`, `thiserror`

## Projektstruktur

```
gladsheim/
├── Cargo.toml
├── README.md
├── AGENTS.md
├── IMPLEMENTATION_PLAN.md
├── src/
│   ├── lib.rs
│   ├── gladsheim.rs          # Haupt-Gladsheim-Struct
│   ├── thjalfi/              # Service Loader
│   │   ├── mod.rs
│   │   ├── loader.rs         # Service-Start-Logik
│   │   └── process.rs        # Process-Management
│   ├── byggvir/              # Resource Manager
│   │   ├── mod.rs
│   │   ├── resources.rs      # Resource-Monitoring
│   │   └── limits.rs         # Limit-Enforcement
│   ├── roskva/               # Health Monitor
│   │   ├── mod.rs
│   │   ├── health.rs         # Health-Check-Logik
│   │   └── monitoring.rs     # Monitoring-Loop
│   ├── skirnir/              # Service Registry
│   │   ├── mod.rs
│   │   ├── registry.rs       # Service-Registry
│   │   └── discovery.rs      # Service-Discovery
│   ├── proto/                # gRPC Proto Definitions
│   │   └── gladsheim.proto
│   ├── grpc/                 # gRPC Server Implementation
│   │   ├── mod.rs
│   │   └── server.rs
│   └── utils/
│       ├── config.rs         # Konfiguration
│       └── errors.rs         # Error-Types
└── tests/
    ├── integration/
    │   ├── service_lifecycle_tests.rs
    │   ├── resource_management_tests.rs
    │   └── health_monitoring_tests.rs
    └── unit/
        ├── thjalfi_tests.rs
        ├── byggvir_tests.rs
        ├── roskva_tests.rs
        └── skirnir_tests.rs
```

## Konfiguration

### Gladsheim-Konfiguration (JSON)

```json
{
  "gladsheim": {
    "grpc": {
      "host": "127.0.0.1",
      "port": 50051
    },
    "resource_limits": {
      "max_services": 10,
      "default_memory_mb": 512,
      "default_cpu_percent": 25
    },
    "health_monitoring": {
      "check_interval_ms": 5000,
      "auto_restart": true,
      "max_restart_attempts": 3,
      "restart_backoff_ms": 1000
    },
    "service_loader": {
      "startup_timeout_ms": 30000,
      "shutdown_timeout_ms": 10000,
      "force_kill_after_timeout": true
    }
  }
}
```

### Platform-spezifische Konfiguration

Jede Plattform kann eigene Gladsheim-Konfiguration haben:
- **Midgard**: `~/.edda/midgard/gladsheim.json`
- **Alfheim**: `<app_data>/alfheim/gladsheim.json`
- **Asgard**: `/etc/edda/asgard/gladsheim.json`
- **Ragnarok**: `~/.edda/ragnarok/gladsheim.json`

## Sicherheitsaspekte

### Process-Isolation

- **Separate Prozesse**: Alle Services laufen in separaten Prozessen
- **Process-Boundaries**: Strikte Process-Isolation
- **No Shared Memory**: Kein Shared Memory zwischen Services
- **IPC via gRPC**: Alle Kommunikation via gRPC (type-safe, secure)

### Resource-Limits

- **Enforcement**: Strikte Enforcement von RAM/CPU-Limits
- **Graceful Degradation**: Bei Resource-Mangel Services geordnet stoppen
- **Priority-Based**: Wichtige Services (Odin, Heimdall) haben höhere Priorität
- **Platform-Aware**: Limits basierend auf Platform (Mobile: strenger, Server: lockerer)

### Security-Validierung

- **Heimdall-Integration**: Nur autorisierte Services dürfen gestartet werden
- **Token-Based**: Services authentifizieren sich via Heimdall-Tokens
- **Audit-Logging**: Alle Service-Starts/Stops werden geloggt
- **No Arbitrary Code**: Services müssen registriert sein (keine arbitrary executables)

### Graceful Failure

- **Cascading Failure Prevention**: Verhindert Cascading Failures
- **Service-Isolation**: Service-Failures isoliert behandeln
- **Auto-Recovery**: Automatische Recovery bei Failures
- **Fallback**: Fallback zu alternativen Services (wenn möglich)

## Performance-Anforderungen

### Latency-Anforderungen

- **Service-Start**: < 500ms für Service-Start (ohne Service-Startup-Zeit)
- **Health-Check-Overhead**: < 1% CPU für Health-Monitoring
- **Memory-Overhead**: < 50MB für Gladsheim selbst
- **Status-Query**: < 10ms für Status-Abfragen
- **gRPC-Calls**: < 5ms für lokale gRPC-Calls

### Throughput-Anforderungen

- **Concurrent-Requests**: Unterstützt 100+ concurrent gRPC-Requests
- **Service-Starts**: Kann 10+ Services parallel starten
- **Health-Checks**: Kann 100+ Services parallel monitoren

### Scalability

- **Max Services**: Bis zu 25 Services auf Asgard, 5 auf Alfheim
- **Resource-Monitoring**: Effizientes Monitoring auch mit vielen Services
- **Registry-Performance**: O(1) Lookup in Service-Registry

## Monitoring & Observability

### Strukturiertes Logging

- **Tracing**: Alle Gladsheim-Operationen werden geloggt
- **Correlation-IDs**: Requests haben Correlation-IDs
- **Log-Levels**: DEBUG, INFO, WARN, ERROR
- **Service-Context**: Logs enthalten Service-Kontext

### Metriken

- **Service-Count**: Anzahl laufender Services
- **Resource-Usage**: Gesamt-Resource-Usage aller Services
- **Health-Status**: Health-Status-Statistiken
- **Operation-Latency**: Latency von Start/Stop/Restart-Operationen

### Alerting

- **Service-Crashes**: Alert bei Service-Crash
- **Resource-Limits**: Alert bei Resource-Limit-Überschreitung
- **Health-Failures**: Alert bei anhaltenden Health-Failures
- **Platform-Integration**: Alerts werden an Odin weitergeleitet

## Implementierungs-Notizen

- **TDD**: Test-Driven Development ist MANDATORY
- **Container-basierte Tests**: Alle Tests müssen in Containern laufen
- **Security-First**: Security-Aspekte von Anfang an berücksichtigen
- **Performance-Optimierung**: Latency und Resource-Usage optimieren
- **Documentation**: Umfassende Code-Dokumentation
- **Error-Handling**: Robustes Error-Handling mit klaren Error-Messages
- **Graceful-Degradation**: System muss bei Failures graceful degradieren
- **Platform-Abstraction**: Code muss auf allen Plattformen laufen (Windows, macOS, Linux, iOS, Android)
