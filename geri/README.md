# Geri - LLM Service

## Übersicht

Geri ist einer von Odins Wölfen und stellt den LLM (Large Language Model) Service bereit. Er verarbeitet Prompts, die bereits durch Freki (RAG) angereichert wurden.

## Verantwortlichkeiten

### 1. Model Management
- Verwaltet verfügbare LLM-Models
- Wählt passendes Model basierend auf Requirements
- Load Balancing zwischen Models

### 2. Prompt Processing
- Verarbeitet Prompts (mit oder ohne RAG-Context)
- Formatiert Prompts für verschiedene Model-APIs
- Verwaltet Context Windows

### 3. Vision-Model Support
- **Bild-Interpretation**: Unterstützt Vision-Models für Bild-Analyse (GPT-4V, Claude Vision, etc.)
- **Video-Interpretation**: Unterstützt Vision-Models für Video-Analyse
- **Video-Stream-Verarbeitung**: Unterstützt Streaming für große Video-Dateien
- **Integration mit Odin**: Odin nutzt Geri für Vision-Model-Interpretation von Bild/Video-Daten
- **Lokale Vision-Models**: Optional lokale Vision-Models

### 3. Model Selection & Load Balancing

**Wichtig**: Geri ist es egal, zu welchem Model es verbinden kann. Die Model-Auswahl kommt aus der Konfiguration:
- **Konfigurationsquellen**:
  - Vom Device selbst (lokale Konfiguration)
  - Vom verbundenen Desktop/Server (Midgard/Asgard)
- **User-Auswahl**:
  - User kann explizit Model wählen
  - Oder automatische Auswahl (beste Wahl für aktuelle Situation)
- **Keine Bevorzugung**: Cloud LLM Provider werden NICHT bevorzugt. Sie sind nur verfügbar, wenn User entsprechende API-Keys/Credentials hinterlegt hat

**Model-Auswahl basierend auf**:
  - Prompt-Komplexität
  - Verfügbare Hardware
  - Cost Constraints
  - Latency Requirements
  - User-Präferenzen (aus Konfiguration)
  - Verfügbare Models (nur Models mit gültigen Credentials)
  - **Multi-Faktor-Bewertung**: Größe des Models, Hardware des Providers, Zuverlässigkeit/Uptime, Ping, Entfernung (für Energie-Effizienz)
  - **Effizienz**: Immer das effizienteste Model wird gewählt, aber mit Load-Balancing
  - **Load-Balancing**: Nicht alle Requests gehen an einen Provider, auch wenn er das beste Model hat (außer User hat explizite Anforderungen)

## Multi-Faktor-Bewertung für Model-Auswahl

**Bewertungs-Faktoren:**

**1. Model-Größe (20% Gewichtung)**
- **Größeres Model = höhere Qualität**: Größere Models generieren bessere Responses
- **Bewertung**: `model_size_score = model_parameter_count / max_parameter_count`
- **Normalisierung**: Basierend auf größtem verfügbarem Model

**2. Hardware des Providers (15% Gewichtung)**
- **GPU-Verfügbarkeit**: GPU-beschleunigte Models sind schneller
- **Hardware-Qualität**: Bessere Hardware = bessere Performance
- **Bewertung**: `hardware_score = (gpu_available ? 1.0 : 0.5) * hardware_quality_factor`
- **Hardware-Quality-Factor**: 0.5 (CPU) bis 1.0 (High-End GPU)

**3. Zuverlässigkeit/Uptime (20% Gewichtung)**
- **Uptime-Percentage**: Verfügbarkeit des Providers in letzter Zeit
- **Fehlerrate**: Anzahl fehlgeschlagener Requests / Gesamt-Requests
- **Bewertung**: `reliability_score = (uptime_percentage / 100.0) * (1.0 - error_rate)`

**4. Ping/Latency (25% Gewichtung)**
- **Network-Latency**: Ping-Zeit zum Provider
- **Niedrigere Latency = höherer Score**
- **Bewertung**: `latency_score = 1.0 - (ping_ms / max_acceptable_ping_ms)`
- **Max-Acceptable-Ping**: 1000ms (konfigurierbar)

**5. Entfernung/Energie-Effizienz (10% Gewichtung)**
- **Geografische Entfernung**: Nähere Provider = weniger Energie für Datenübertragung
- **Lokale Models**: Lokale Models haben höchsten Score (Entfernung = 0)
- **Bewertung**: `distance_score = 1.0 - (distance_km / max_distance_km)`
- **Lokale Models**: `distance_score = 1.0`

**6. Cost (10% Gewichtung)**
- **Niedrigere Kosten = höherer Score**
- **Bewertung**: `cost_score = 1.0 - (cost_per_token / max_cost_per_token)`

**Effizienz-Berechnung:**
```
efficiency_score = (model_size_score * 0.20) + 
                   (hardware_score * 0.15) + 
                   (reliability_score * 0.20) + 
                   (latency_score * 0.25) + 
                   (distance_score * 0.10) + 
                   (cost_score * 0.10)
```

**Effizienz-Messung:**
- **Tokens pro Sekunde**: Wie viele Tokens kann das Model pro Sekunde generieren?
- **Energie-Verbrauch**: Geschätzter Energie-Verbrauch pro Request
- **Effizienz-Metrik**: `tokens_per_second / energy_consumption`

**Load-Balancing:**

**Round-Robin mit Gewichtung:**
- **Gewichtete Auswahl**: Provider werden basierend auf Efficiency-Score gewichtet ausgewählt
- **Verteilung**: Nicht alle Requests gehen an besten Provider
- **Gewichtungs-Formel**: `selection_probability = efficiency_score / sum(all_efficiency_scores)`
- **Zweck**: Verhindert Überlastung einzelner Provider

**Load-Balancing-Parameter:**
- **Max-Requests pro Provider**: Max. X Requests pro Minute pro Provider
- **Load-Threshold**: Wenn Provider über 80% Auslastung, wird weniger gewichtet
- **Automatische Anpassung**: Gewichtungen werden dynamisch angepasst basierend auf aktueller Last

### 4. Provider Integration
- **Lokale Modelle**: Ollama, LM Studio, Custom Local Models
- **Cloud LLMs**: OpenAI, Anthropic, Google, etc. (nur wenn API-Keys/Credentials hinterlegt sind)
- **Unified API**: Einheitliche API für alle Provider
- **Keine Hierarchie**: Lokale und Cloud Models haben gleiche Priorität

### 5. Valkyries LLM-Konfiguration
- **Per Default**: Alle Valkyries nutzen dasselbe LLM (konfigurierbar)
- **Individuelle Konfiguration**: Jede Valkyrie kann ein eigenes LLM konfiguriert bekommen
- **Konfiguration über Geri**: Geri verwaltet die LLM-Konfiguration für alle Valkyries
- **Use-Case-spezifisch**: Verschiedene Valkyries können verschiedene Models nutzen (z.B. spezialisierte Coding-Models)
- **Gilt für alle Installationen**: Diese Konfigurationsmöglichkeit gilt für alle Valkyries-Installationen (auch außerhalb von Ragnarok)

## Service-Interfaces

### Inputs
- `WolfRequest` mit `modelType: LLM`, `prompt` und optional `ragContext`
- `ImageAnalysisRequest` (von Odin) - Bild-Daten für Vision-Model-Interpretation
- `VideoAnalysisRequest` (von Odin) - Video-Daten für Vision-Model-Interpretation
- `VideoStreamChunk` (von Odin) - Video-Stream-Daten für Vision-Model-Interpretation
- Model Configuration Requests

### Outputs
- `WolfResponse` mit generiertem Text, Tokens, Latency, Cost
- `ImageAnalysisResponse` (an Odin) - Ergebnis der Bild-Analyse
- `VideoAnalysisResponse` (an Odin) - Ergebnis der Video-Analyse
- `VideoAnalysisChunk` (an Odin) - Streaming-Ergebnisse der Video-Analyse

## Workflow

1. **Request empfangen**
   - Odin sendet `WolfRequest` mit Prompt und optional `ragContext`
   - Geri analysiert Requirements

2. **Model Selection**
   - Liest verfügbare Models aus Konfiguration (Device oder verbundener Desktop/Server)
   - Wählt passendes Model basierend auf:
     - Prompt-Länge + RAG-Context
     - Verfügbare Hardware
     - Cost Constraints
     - User-Präferenzen (aus Konfiguration)
     - Verfügbarkeit (nur Models mit gültigen Credentials)
   - **Automatische oder explizite Auswahl**: User kann wählen oder automatisch wählen lassen

3. **Prompt Formatting & RAG-Context-Integration**
   - Formatiert Prompt für gewähltes Model
   - Fügt System-Prompts hinzu
   - **RAG-Context-Integration** (falls vorhanden):
     - RAG-Context wird in strukturiertem Format in Prompt eingefügt
     - **Prompt-Template**:
       ```
       System: [System-Prompt]
       
       Context:
       [RAG-Context-Dokumente hier]
       
       User: [User-Prompt]
       ```
     - **Context-Position**: RAG-Context wird zwischen System-Prompt und User-Prompt eingefügt
     - **Context-Formatierung**: Dokumente werden mit Metadaten formatiert (siehe Freki Context-Formatting)
   - **Context-Window-Management** (siehe unten)

4. **LLM Call**
   - Ruft gewähltes Model auf (lokal oder Cloud)
   - Streamt Response falls unterstützt
   - Trackt Tokens und Latency

5. **Response Processing**
   - Verarbeitet LLM-Response
   - Extrahiert relevante Information
   - Berechnet Cost (für Cloud-Provider)

6. **Response**
   - Sendet `WolfResponse` zurück an Odin
   - Enthält generierten Text, Tokens, Latency, Cost

## Provider Support

### Local Providers
- **Ollama**: Lokale Modelle, verschiedene Größen
- **LM Studio**: Lokale Modelle mit API
- **Custom Local Models**: Direkte Integration

### Cloud Providers
- **OpenAI**: GPT-4, GPT-3.5, etc.
- **Anthropic**: Claude Models
- **Google**: Gemini Models
- **Other**: Cohere, Mistral AI, etc.

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

### Geri-spezifische Settings

**Settings-Struktur (JSON-Format):**

```json
{
  "models": {
    "local": [
      {
        "id": "llama-3-8b",
        "name": "Llama 3 8B",
        "type": "local",
        "provider": "ollama",
        "endpoint": "http://localhost:11434",
        "enabled": true
      }
    ],
    "cloud": [
      {
        "id": "gpt-4",
        "name": "GPT-4",
        "type": "cloud",
        "provider": "openai",
        "api_key": "encrypted:...",  // Verschlüsselt gespeichert
        "endpoint": "https://api.openai.com/v1",
        "enabled": true
      }
    ]
  },
  "default_model": {
    "local": "llama-3-8b",
    "cloud": "gpt-4"
  },
  "vision_models": {
    "enabled": true,
    "default_model": "gpt-4-vision",
    "max_image_size_mb": 20,
    "supported_formats": ["jpg", "png", "webp"]
  },
  "fallback": {
    "enabled": true,
    "fallback_to_local": true,
    "fallback_to_network": true
  },
  "performance": {
    "max_concurrent_requests": 5,
    "timeout_ms": 30000,
    "retry_count": 3
  }
}
```

**Model-Konfiguration:**
- **Lokale Models**: Konfiguration für lokale Models (Ollama, LM Studio, etc.)
- **Cloud-Models**: Konfiguration für Cloud-Models (OpenAI, Anthropic, etc.)
- **Model-Registry**: Models werden in Registry verwaltet
- **Model-Versionierung**: Unterstützung für mehrere Model-Versionen

**API-Keys-Speicherung:**
- **Verschlüsselte Speicherung**: API-Keys werden verschlüsselt gespeichert (OS-spezifisch: Keychain, Credential Manager)
- **Keine Plain-Text-Speicherung**: API-Keys werden niemals im Klartext gespeichert
- **Key-Rotation**: API-Keys können rotiert werden (alte Keys werden entfernt)
- **Access-Control**: Nur Geri-Service hat Zugriff auf API-Keys

**Konfiguration vom verbundenen Desktop/Server:**
- **Asgard-Integration**: Wenn Asgard verfügbar, kann Model-Konfiguration von Asgard übernommen werden
- **Network-Model-Discovery**: Geri kann Models im Netzwerk entdecken (via Einherjar Protocol)
- **Remote-Model-Access**: Geri kann auf Models auf anderen Devices zugreifen (via gRPC)
- **Konfiguration-Sync**: Model-Konfiguration wird zwischen Devices synchronisiert (optional)

**Settings-Speicherung:**
- **Lokale Speicherung**: Settings werden lokal in JSON-Datei gespeichert (z.B. `~/.edda/geri/settings.json`)
- **Verschlüsselte API-Keys**: API-Keys werden in OS-spezifischem Secure Storage gespeichert
- **Schema-Validierung**: Settings werden beim Laden validiert
- **Hot-Reload**: Settings können zur Laufzeit neu geladen werden

## Technische Anforderungen

### Model Management
- **Model-Registry**:
  - **Verwaltung**: Model-Registry wird zentral verwaltet (lokal pro Device oder über Midgard/Asgard)
  - **Model-Versionierung**: Models haben Versionsnummern, mehrere Versionen können gleichzeitig verfügbar sein
  - **Model-Updates**: Model-Updates werden automatisch erkannt und integriert
  - **Registry-Synchronisation**: Model-Registry wird zwischen Devices synchronisiert (optional)
- **Model Versioning**: Unterstützung für mehrere Model-Versionen gleichzeitig
- **Model Health Monitoring**: Kontinuierliche Überwachung der Model-Verfügbarkeit und Performance
- **Automatic Fallback**: Automatischer Fallback zu alternativen Models bei Ausfällen

### Prompt Engineering
- System Prompt Templates
- Few-Shot Examples
- Prompt Optimization
- Context Window Management

## RAG-Context-Integration

### Context-Einfügung in LLM-Prompt

**Prompt-Struktur mit RAG-Context:**
```
System: [System-Prompt mit Anweisungen für RAG-Context-Nutzung]

Context:
[Dokument 1: document_id]
Relevanter Dokument-Content...

[Dokument 2: document_id]
Relevanter Dokument-Content...

User: [User-Prompt]
```

**Context-Position:**
- RAG-Context wird zwischen System-Prompt und User-Prompt eingefügt
- Ermöglicht LLM, Context vor User-Prompt zu verarbeiten
- System-Prompt kann Anweisungen enthalten, wie Context zu nutzen ist

**Context-Formatierung:**
- Dokumente werden mit eindeutigen IDs formatiert
- Metadaten werden beibehalten für Traceability
- Dokumente werden nach Relevanz-Score sortiert (höchste zuerst)

### Context-Window-Management

**Problem**: LLMs haben begrenzte Context-Window-Größe (z.B. 4K, 8K, 32K, 128K Tokens)

**Lösung: Hierarchische Context-Optimierung**

**1. Context-Größen-Prüfung**
- Geri berechnet Token-Anzahl für: System-Prompt + RAG-Context + User-Prompt
- Prüft gegen Model Context-Window-Limit
- Reserviert Platz für Response (z.B. 20% des Context-Windows)

**2. Context-Truncation (wenn zu groß)**

**Priorisierung:**
- **System-Prompt**: Wird nie gekürzt (kritisch für Verhalten)
- **User-Prompt**: Wird nie gekürzt (User-Input ist wichtig)
- **RAG-Context**: Wird gekürzt wenn nötig

**Truncation-Strategie:**
- **Relevanz-basiert**: Dokumente mit niedrigstem Relevanz-Score werden zuerst entfernt
- **Chunk-basiert**: Ganze Dokument-Chunks werden entfernt (keine Teil-Chunks)
- **Minimale Dokument-Anzahl**: Mindestens 1 Dokument bleibt erhalten (auch wenn sehr groß)
- **Token-Limit**: Context wird so gekürzt, dass System-Prompt + Context + User-Prompt + Response-Reserve < Context-Window

**3. Context-Optimierung (vor Truncation)**

**Intelligente Optimierung:**
- **Deduplizierung**: Ähnliche/duplizierte Dokument-Passagen werden entfernt
- **Zusammenfassung**: Sehr lange Dokumente können optional zusammengefasst werden (nur wenn nötig)
- **Relevanz-Filterung**: Dokumente unter Relevanz-Threshold werden entfernt

**4. Fallback-Strategien**

**Wenn Context immer noch zu groß:**
- **Model-Wechsel**: Falls möglich, Wechsel zu Model mit größerem Context-Window
- **Multi-Pass-Processing**: Context wird in mehreren Passes verarbeitet (komplex, nur als letzter Fallback)
- **User-Benachrichtigung**: User wird informiert, dass nicht alle Dokumente verwendet werden konnten

**5. Context-Window-Reservierung**

**Reservierter Platz:**
- **System-Prompt**: ~500-1000 Tokens (variiert je nach Model)
- **Response-Reserve**: 20% des Context-Windows (für LLM-Response)
- **User-Prompt**: Vollständig reserviert (nie gekürzt)
- **RAG-Context**: Restlicher verfügbarer Platz

**Beispiel-Berechnung:**
```
Context-Window: 32K Tokens
- System-Prompt: 1K Tokens
- Response-Reserve: 6.4K Tokens (20%)
- User-Prompt: 2K Tokens
- Verfügbar für RAG-Context: 22.6K Tokens
```

### Context-Window-Monitoring

**Metriken:**
- **Context-Usage**: Wie viel vom Context-Window wird genutzt?
- **Truncation-Rate**: Wie oft muss Context gekürzt werden?
- **Dokument-Ausschluss**: Wie viele Dokumente wurden ausgeschlossen?

**Logging:**
- Alle Context-Truncations werden geloggt
- Dokument-Ausschlüsse werden dokumentiert
- User kann sehen, welche Dokumente verwendet wurden

### Cost Management
- Token Counting
- Cost Calculation
- Budget Limits
- Cost Tracking per Request

### Caching-Strategien

**Gecachte Daten:**
- Häufig verwendete Responses für ähnliche Prompts
- Model-Konfigurationen
- Provider-Status und Verfügbarkeit

**Cache-Invalidierung:**
- Event-basiert: Bei Model-Updates, Provider-Status-Änderungen
- Timeout-basiert: Als Fallback, wenn Events fehlen
- Sofortige Invalidierung bei wichtigen Änderungen

**Cache-Sharing:**
- Kein direkter Cache-Sharing zwischen Devices
- Jedes Device hat eigenen Cache für optimale Performance

### Performance
- **Streaming Support**: 
  - **LLM-Response-Streaming**: Streaming für LLM-Responses (wenn vom Provider unterstützt)
  - **Provider-Unterstützung**: Streaming für alle Provider, die es unterstützen
  - **Streaming-Fehler**: Retry-Mechanismen bei Streaming-Fehlern
- **Parallel Requests**: Unterstützung für parallele Requests an verschiedene Models
- **Request Queuing**: 
  - **Request-Queuing**: Requests werden in Queue gelegt bei hoher Last
  - **Prioritäten**: Prioritäten in Queues (wichtige Requests haben höhere Priorität)
  - **Queue-Backlog**: Intelligentes Handling von Queue-Backlog (automatische Skalierung)
- **Caching**: Caching für ähnliche Prompts für bessere Performance

### Performance-Monitoring

**Performance-Monitoring:**
- Performance-Metriken: Response-Zeiten, Durchsatz, Resource-Usage
- Performance-Tracking für alle LLM-Requests
- Kontinuierliche Überwachung und Performance-Optimierung
- Alerts bei Performance-Problemen

## Monitoring & Logging

### Strukturiertes Logging

**Strukturiertes Logging:**
- Structured Logging mit strukturierten Daten
- Log Levels: Verschiedene Log-Level (DEBUG, INFO, WARN, ERROR, etc.)
- Context Tracking: Context wird mitgeloggt
- Log Rotation: Automatische Log-Rotation
- Umfassendes Logging für Debugging und Monitoring

## Cloud LLM Fallback zu Local LLMs

### Automatischer Fallback
- **Limit erreicht**: Wenn Cloud LLM Limit erreicht ist, automatischer Fallback zu lokalem LLM
- **Stärkstes lokales LLM**: Bestes verfügbares lokales LLM wird identifiziert (Multi-Faktor-Bewertung)
- **Im Netzwerk suchen**: System sucht auch im Netzwerk nach besten LLM (z.B. Smartphone nutzt Desktop-LLM)
- **User-Benachrichtigung**: TTS-Meldung mit Begründung ("Ich nutze jetzt lokales Modell, da Cloud-Limit erreicht")

### Stärkstes lokales LLM identifizieren

**Multi-Faktor-Bewertung für lokale LLMs:**
- **Model-Größe**: Größere lokale Models haben höhere Priorität
- **Hardware-Verfügbarkeit**: GPU-beschleunigte Models bevorzugt
- **Performance**: Tokens pro Sekunde
- **Verfügbarkeit**: Model muss verfügbar und lauffähig sein

**Bewertungs-Formel für lokale LLMs:**
```
local_llm_score = (model_size_score * 0.40) + 
                  (hardware_score * 0.30) + 
                  (performance_score * 0.30)
```

**Netzwerk-Suche nach LLMs:**

**Service-Discovery:**
- **Einherjar Protocol**: Geri nutzt Einherjar Protocol, um verfügbare LLMs im Netzwerk zu finden
- **Capability-Response**: Jedes Device mit LLM meldet Capabilities via Einherjar
- **LLM-Metadaten**: Model-Name, Größe, Hardware, Performance-Metriken werden gemeldet

**Netzwerk-LLM-Bewertung:**
- **Network-Latency**: Ping-Zeit zum Netzwerk-Device
- **Model-Qualität**: Model-Größe und Performance
- **Device-Verfügbarkeit**: Device muss online und verfügbar sein
- **Bewertung**: Kombination aus Model-Qualität und Network-Latency

**Priorisierung:**
1. **Lokales Device-LLM**: Zuerst lokales LLM auf eigenem Device
2. **Netzwerk-LLM (niedrige Latency)**: LLM auf anderem Device im Netzwerk (< 50ms Ping)
3. **Netzwerk-LLM (höhere Latency)**: LLM auf anderem Device im Netzwerk (> 50ms Ping)
4. **Cloud-LLM**: Nur wenn keine lokalen/Netzwerk-LLMs verfügbar

### TTS-Benachrichtigung implementieren

**Benachrichtigungs-Workflow:**
1. **Fallback-Erkennung**: Geri erkennt, dass Fallback zu lokalem LLM nötig ist
2. **Benachrichtigungs-Text**: Geri generiert Benachrichtigungs-Text
   - Beispiel: "Ich nutze jetzt lokales Modell, da Cloud-Limit erreicht"
   - Beispiel: "Ich nutze jetzt Desktop-LLM, da lokales Modell nicht verfügbar"
3. **An Odin senden**: Geri sendet Benachrichtigung an Odin
4. **Odin → Muninn**: Odin sendet `RavenMessage` mit Benachrichtigung an Muninn
5. **TTS-Ausgabe**: Muninn gibt Benachrichtigung via TTS aus

**Benachrichtigungs-Typen:**
- **Cloud-Limit erreicht**: "Ich nutze jetzt lokales Modell, da Cloud-Limit erreicht"
- **Cloud-Provider-Ausfall**: "Ich nutze jetzt lokales Modell, da Cloud-Provider nicht verfügbar"
- **Netzwerk-LLM verwendet**: "Ich nutze jetzt Desktop-LLM für bessere Qualität"
- **Lokales LLM verwendet**: "Ich nutze jetzt lokales Modell"

**Benachrichtigungs-Einstellungen:**
- **User kann deaktivieren**: User kann TTS-Benachrichtigungen deaktivieren
- **Nur wichtige Events**: Nur bei wichtigen Events (Limit erreicht, Provider-Ausfall)
- **Nicht bei jedem Fallback**: Nicht bei jedem einzelnen Fallback, nur bei wichtigen Änderungen

### Lokales LLM ist Standard
- **Jedes Device muss lokales LLM haben**: Beim Installieren wird lokales LLM installiert und lauffähig gemacht
- **Fallback darf nicht fehlschlagen**: Es darf nicht passieren, dass kein lokales LLM verfügbar ist
- **Default**: Lokales LLM ist der Standard-Fallback, wenn nichts anderes geht

### Automatische Rückkehr zu Cloud LLM
- **Monatliches Reset**: Kontingent wird monatlich zurückgesetzt (exakt nach einem Monat ab Vertragsabschluss)
- **Automatisch auf Yggdrasil**: Reset erfolgt automatisch auf Yggdrasil
- **Nur bei aktiver Zahlung**: Reset nur, solange User noch dafür zahlt
- **Automatische Rückkehr**: Nach Reset automatische Rückkehr zu Cloud LLM (wenn verfügbar)

## gRPC Communication

**gRPC Service Communication:**
- **Odin ↔ Geri**: gRPC für LLM-Services und Vision-Model-Interpretation
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
- **Fallback**: Bei Fehler Fallback zu alternativen Providern/Models

## Abhängigkeiten

### Keine Core Library

- **WICHTIG**: Es gibt keine Edda Core Library
- **Separate Projekte**: Wenn gemeinsame Komponenten benötigt werden (DTOs wie WolfRequest/WolfResponse, Protocols, Utils), sollte ein separates Projekt erstellt werden
- **Selektive Nutzung**: Dies hält Apps klein, da genau gewählt werden kann, was benötigt wird
- **Keine Abhängigkeit**: Geri sollte nicht auf Dateien/Protocols/Utils aus dem `edda` Verzeichnis verweisen (KEIN PROJEKT - nur Metadaten-Sammlung)

### Service-Abhängigkeiten

- **Odin**: Für Requests
- **Freki**: Für RAG-Context (optional)
- **Model Providers**: Lokal oder Cloud
- **Hardware Resources**: Für lokale Models

## Integration

- **Odin**: 
  - Empfängt `WolfRequest` von Odin, sendet `WolfResponse` zurück
  - Empfängt `ImageAnalysisRequest` von Odin, sendet `ImageAnalysisResponse` zurück
  - Empfängt `VideoAnalysisRequest` von Odin, sendet `VideoAnalysisResponse` zurück
  - Empfängt `VideoStreamChunk` von Odin, sendet `VideoAnalysisChunk` zurück
- **Freki**: Verwendet RAG-Context von Freki (optional)
- **Huginn**: Huginn gibt Bild/Video-Daten an Odin weiter, Odin nutzt Geri für Interpretation
- **Midgard**: Lokale Models mit voller Hardware-Nutzung
- **Alfheim**: Cloud-First Approach, lokale Models optional
- **Asgard**: Lokale Models mit erweiterten Features
- **Yggdrasil**: Für Subscription-Management und Cloud-LLM-Limits

## Service-Ausfall-Behandlung

**Innerhalb einer Platform:**
- Fallback ist unnötig - Services müssen existieren, so bauen wir sie ja
- Services sind Teil der Platform-Installation

**Platformübergreifend:**
- Netzwerkplan verwenden für Model-Discovery
- Falls mit Yggdrasil verbunden: Netzwerkplan an Yggdrasil übertragen
- **WICHTIG**: Netzwerkplan darf unter keinen Umständen anderen Usern zugänglich gemacht werden

**LLM-Auswahl:**
- Bei Abfrage soll möglichst immer das beste verfügbare Model gewählt werden
- Lokales LLM ist immer das letzte Fallback (wird mit Installation mitgeliefert)
- Sicherstellung: Mit Installation einer Platform wird auch ein LLM mitgeliefert

**Fallback-Strategien (nur platformübergreifend):**
- Alternative Provider: Falls Provider-Fehler, Fallback zu alternativem Provider
- Lokales LLM: Falls Cloud-LLM-Fehler, Fallback zu lokalem LLM (letztes Fallback)

**Service-Ausfall-Behandlung:**
- Automatischer Retry mit Exponential Backoff
- Sofortiger Fallback zu alternativen Providern/Models (nur platformübergreifend)
- User-Benachrichtigung bei komplettem Service-Ausfall

**User-Kommunikation:**
- Fehlermeldung an User, wenn alle Versuche fehlschlagen
- Error-Logging für Debugging
- User kann später erneut versuchen
- Transparente Fehlerbehandlung

## Datenschutz

### GDPR-Compliance

**Right to Deletion:**
- User kann alle Daten löschen ("Right to be forgotten")
- Sichere Datenlöschung
- Automatische Löschung nach Retention-Policy

**User-Rechte:**
- Right to Access: User können ihre Daten abrufen
- Right to Rectification: User können ihre Daten korrigieren
- Right to Data Portability: User können ihre Daten exportieren
- Right to Object: User können der Datenverarbeitung widersprechen

**Data-Minimization:**
- Nur notwendige Daten werden gespeichert
- Nur notwendige Daten werden verarbeitet
- Purpose Limitation: Daten nur für spezifische Zwecke verwendet
- Storage Limitation: Daten nur so lange gespeichert wie nötig

### Datenschutz-Features
- **Lokale Verarbeitung**: Prompts werden bevorzugt lokal verarbeitet
- **Minimale Datensammlung**: Nur notwendige Daten werden gespeichert
- **Keine Tracking-Daten**: Keine User-Tracking oder Analytics-Daten ohne Zustimmung
- **Prompt-Privacy**: Prompts bleiben lokal, werden nicht an Dritte weitergegeben (außer bei Cloud-LLMs mit Zustimmung)
- **User Control**: User hat volle Kontrolle über LLM-Nutzung und Cloud-Services

### Compliance
- **GDPR-konform**: Unterstützung für GDPR-Anforderungen
- **Data Minimization**: Nur notwendige Daten werden verarbeitet
- **Right to Deletion**: User kann alle Prompt-Daten löschen
- **Transparency**: User wird über LLM-Nutzung informiert
- **Consent**: Explizite Zustimmung für Cloud-LLM-Services

## Sicherheit

### Security-Features
- **Secure Prompt Storage**: Verschlüsselte Speicherung von Prompts (optional)
- **TLS Encryption**: Alle Cloud-Verbindungen sind verschlüsselt (TLS 1.3)
- **Authentication**: Sichere Authentifizierung für Cloud-LLM-Services
- **Input Validation**: Validierung aller Prompt-Inputs
- **Secure Key Storage**: Sichere Speicherung von API-Keys für Cloud-LLMs
- **Prompt Sanitization**: Sanitization von Prompts zum Schutz vor Injection-Angriffen
- **Audit Logging**: Logging aller LLM-Requests für Security-Audits

### Security-Best-Practices
- **Keine Hardcoded Secrets**: Keine Hardcoded API-Keys oder Credentials
- **Secure Defaults**: Sichere Standard-Konfigurationen
- **Regular Updates**: Automatische Security-Updates
- **Vulnerability Scanning**: Regelmäßiges Scanning für bekannte Vulnerabilities
- **Rate Limiting**: Rate Limiting für Cloud-LLM-Requests zum Schutz vor Abuse

## Implementierungs-Notizen

- Sollte Provider-Abstraktion haben (Plugin-Architektur)
- Muss effizientes Context Window Management haben
- Sollte Streaming für bessere UX unterstützen
- Muss Cost-Tracking und Budget-Limits haben
- Sollte Retry-Mechanismen für fehlgeschlagene Requests haben
- Muss Monitoring für Model-Performance haben
- **Muss lokales LLM garantieren**: Jedes Device muss lokales LLM haben
- **Muss Multi-Faktor-Bewertung implementieren**: Für Model-Auswahl
- **Muss Load-Balancing haben**: Verhindert Überlastung einzelner Provider
- **Muss TTS-Benachrichtigung haben**: Bei Fallback zu lokalem LLM
- **Muss Vision-Model-Support haben**: Unterstützt Vision-Models für Bild/Video-Interpretation (GPT-4V, Claude Vision, etc.)
- **Muss Video-Stream-Verarbeitung haben**: Unterstützt Streaming für große Video-Dateien
- **Muss lokale Vision-Models unterstützen**: Optional lokale Vision-Models
- **Performance**: Muss optimiert sein für schnelle LLM-Responses und effizientes Context-Management
- **Datenschutz**: Muss Privacy-by-Design implementieren und Prompt-Privacy gewährleisten
- **Sicherheit**: Muss robuste Security-Mechanismen haben, insbesondere für Prompt-Verarbeitung

