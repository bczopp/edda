# Odin - Main Process Service

## Übersicht

Odin ist der zentrale Orchestrator auf Midgard (Desktop), Alfheim (Mobile), Asgard (Homeserver) und Ragnarok (Terminal). Er koordiniert alle anderen Services und ist der Hauptprozess des Edda-Systems. Odin läuft NICHT auf Jotunheim (IoT-Devices), da diese zu klein sind und stattdessen Loki für Toolcalling-Funktionalität nutzen.

## Verantwortlichkeiten

### 1. User-Command Processing
- **Text-Input**: Empfängt Text-Input direkt vom Frontend
- **Voice-Input**: Empfängt Voice-Input von Huginn (STT) - wird zu Text transkribiert
- **Bild-Input**: Empfängt Bild-Daten von Huginn (keine Interpretation durch Huginn)
- **Video-Input**: Empfängt Video-Daten von Huginn (keine Interpretation durch Huginn)
- **Video-Stream-Input**: Empfängt Video-Stream-Daten von Huginn (keine Interpretation durch Huginn)
- **Einheitliche Verarbeitung**: Alle Input-Methoden werden verarbeitet
- Analysiert und interpretiert User-Commands
- **Vision-Model-Interpretation**: Nutzt Geri für Bild/Video-Interpretation (Vision-Model)
- **Plugin-Orchestrierung**: Entscheidet, ob Odin selbst antworten kann oder ob die Aufgabe an ein Plugin delegiert werden muss

### 2. Plugin-Orchestrierung & Decision Making
- **Einherjar Protocol**: Nutzt Einherjar Protocol, um verfügbare Funktionen aller Götter zu entdecken
- **Zuständigkeits-Erkennung**: Erkennt anhand von Funktionen, Keywords und Domains, welcher Gott zuständig ist
- **Dynamische Delegation**: Delegiert Aufgaben an zuständige Götter (Thor, Valkyries, Frigg, etc.)
- **Zuständigkeits-Weiterleitung**: Wenn ein Gott zuständig ist, bleibt er zuständig, bis er die Zuständigkeit zurückgibt
- **Rückweisungs-Mechanismus**: Götter können Requests zurückweisen, Odin weicht auf nächstwahrscheinlichste Wahl aus
- **Fallback-Mechanismus**: Bei Rückweisung wählt Odin alternative Götter basierend auf Einherjar Protocol
- **Einfache Fragen**: Odin kann selbst antworten (wenn es nur eine Frage ist)
- **Modulare Architektur**: Plugins sind optional und können je nach Bedarf hinzugefügt werden

### 3. Device State Management
- Verwaltet den aktuellen Zustand des Devices
- Trackt laufende Actions und Tasks
- Verwaltet Device-Konfiguration

### 4. Inter-Device Communication Coordination
- Koordiniert Kommunikation mit anderen Devices über Bifrost
- Verwaltet Device-Verbindungen
- Routet Messages zwischen Devices

### 5. Chat-Management und Chat-Leitung
- **Multiple Chats**: Platform muss ermöglichen, quasi beliebig viele Chats zu starten
- **Chat-Leitung an andere Götter**: Odin kann kurzzeitig/use-case-abhängig die Chat-Leitung an andere Götter abgeben
  - **Automatische Weiterleitung**: Odin verweist automatisch auf Frigg (oder andere Götter) und durch ein Flag bleibt der Chat bei Frigg
  - **Explizite Chat-Erstellung**: User kann explizit einen Chat mit Frigg erstellen, dann bleibt der Chat immer bei Frigg und kann auch nicht an Odin übergeben werden
  - **Direkte Eingabe**: Bei aktivem Flag werden User-Eingaben direkt an den zuständigen Gott (z.B. Frigg) geleitet, statt über Odin, der immer wieder entscheiden muss
- **Chat-Flags**: Flags in Settings steuern, ob ein Chat direkt an einen Gott geleitet wird oder über Odin läuft

## Service-Interfaces

### Inputs
- `RavenMessage` (von Frontend oder Huginn) - User Text/Voice/Bild/Video Input
  - **Text-Input**: Direkt vom Frontend als Text
  - **Voice-Input**: Von Huginn (STT) transkribiert zu Text
  - **Bild-Input**: Bild-Daten von Huginn (keine Interpretation durch Huginn)
  - **Video-Input**: Video-Daten von Huginn (keine Interpretation durch Huginn)
  - **Video-Stream-Input**: Video-Stream-Daten von Huginn (keine Interpretation durch Huginn)
- **Plugin-Ergebnisse (alle Plugins geben Rückmeldung an Odin)**:
  - `ThorResult` (von Thor) - Action Execution Results
  - `ValkyrieResult` (von Valkyries) - Coding Task Results (strukturierte Code-Änderungen)
  - `FriggResult` (von Frigg) - Healthcare Task Results
- `BifrostConnection` (von Bifrost) - Inter-Device Messages
- `ResponsibilityReturn` (von allen Göttern) - Zuständigkeits-Rückgabe
- `ResponsibilityRejection` (von allen Göttern) - Zuständigkeits-Rückweisung
- `CapabilityResponse` (von allen Göttern via Einherjar Protocol) - Verfügbare Funktionen

### Outputs
- `RavenMessage` (an Muninn) - System Responses
- **Plugin-Requests (alle Plugins erhalten Requests von Odin)**:
  - `ThorAction` (an Thor) - Actions zur Ausführung (via gRPC)
  - `ValkyrieTask` (an Valkyries) - Coding Tasks (via gRPC)
  - `FriggTask` (an Frigg) - Healthcare Tasks (via gRPC)
- `WolfRequest` (an Freki/Geri) - AI Service Requests (via gRPC)
- `SelectionRequest` (an Skuld) - LLM-Selection Requests (via gRPC)
- `ImageAnalysisRequest` (an Geri) - Bild-Interpretation via Vision-Model
- `VideoAnalysisRequest` (an Geri) - Video-Interpretation via Vision-Model
- `ResponsibilityRequest` (an alle Götter) - Zuständigkeits-Übernahme
- `CapabilityRequest` (an alle Götter via Einherjar Protocol) - Funktions-Offenlegung

### Event-Dispatcher-System

**Pro Platform ein Event-Dispatcher:**
- **Zentrale Event-Verwaltung**: Jede Platform (Midgard, Alfheim, Asgard, Ragnarok) hat einen `event_dispatcher`
- **Event-Registrierung**: Odin und alle Plugins registrieren sich für Events, die für sie relevant sind
- **Event-Publishing**: Services und Plugins können Events publizieren
- **Event-Subscription**: Odin und Plugins abonnieren Events basierend auf ihren Interessen

**Event-Flow:**
1. **Event wird publiziert**: Service/Plugin publiziert Event (z.B. `ThorActionRequest`, `FileChanged`, etc.)
2. **Event-Dispatcher routet**: Event-Dispatcher routet Event an alle registrierten Subscriber
3. **Plugin empfängt Event**: Plugin empfängt Event und stellt es auf interne FIFO-Queue
4. **Plugin verarbeitet Event**: Plugin verarbeitet Events aus Queue (FIFO) und erstellt Folge-Events oder Return-Events
5. **Return-Event wird dispatched**: Plugin publiziert Return-Event, das wiederum an Odin/andere Services geroutet wird

**Platform-Capabilities für Service-Discovery:**
- **Capability-Query**: Plugins können über Platform-Capabilities herausfinden, welche Services und Funktionen verfügbar sind
- **Service-Discovery**: Plugins können Events erstellen, die direkt zu bestimmten Services gehen (z.B. direkt zu Thor)
- **Dynamische Event-Erstellung**: Plugins erstellen Events basierend auf verfügbaren Services

**Beispiel: Valkyries → Thor:**
- Brünnhilde fragt Platform-Capabilities: "Welche Services sind verfügbar?"
- Platform antwortet: "Thor verfügbar, kann `FILE_OPERATION` Actions ausführen"
- Brünnhilde erstellt `ThorActionRequest`-Event
- Event wird an Thor geroutet (via Event-Dispatcher)
- Thor verarbeitet Event und publiziert `ThorActionResult`-Event
- Event wird zurück an Brünnhilde geroutet

### gRPC Service Communication

**Odin kommuniziert mit Services via gRPC:**
- **Odin ↔ Thor**: gRPC für Action-Execution (auch via Events möglich)
- **Odin ↔ Freki**: gRPC für RAG-Services
- **Odin ↔ Geri**: gRPC für LLM-Services und Vision-Model-Interpretation
- **Odin ↔ Skuld**: gRPC für LLM-Selection
- **Odin ↔ Plugins**: Event-basierte Kommunikation über Event-Dispatcher (primär), gRPC als Fallback
  - **Standard-Interface**: Einherjar Protocol (gRPC) für Funktions-Offenlegung und Responsibility Service (gRPC) für Zuständigkeits-Management
  - **Event-basiert**: Primäre Kommunikation über Event-Dispatcher
  - **Plugin-Fehler**: Error-Handling mit Events, Fallback zu gRPC bei Bedarf
- **Odin ↔ Alle Götter**: Einherjar Protocol (gRPC) für Funktions-Offenlegung
- **Odin ↔ Alle Götter**: Responsibility Service (gRPC) für Zuständigkeits-Management

**Service-Discovery:**
- **Service-Unabhängigkeit**: Services sind unabhängig von Platformen implementiert - ermöglicht flexible Entscheidungen, welche Services auf welcher Platform verfügbar sind
- **Innerhalb einer Platform**: Services können via gRPC kommunizieren, wenn nötig. Direkte Aufrufe sind auch möglich, wenn das performanter ist. Platform entscheidet flexibel über Kommunikationsmethode.
- **Platformübergreifend**: Sowohl Bifrost als auch gRPC müssen unterstützt werden. Bifrost für Connection-Establishment, dann gRPC für Service-Kommunikation.
- **Service-Discovery durch Platform**: Platform propagiert alle Methoden, die Odin als public ermittelt von allen Göttern, die auf der Platform vorhanden sind. Odin nutzt Einherjar Protocol zur Funktions-Entdeckung.
- **Externe Kommunikation**: Von außen wird niemals direkt mit einem Gott geredet - alle Kommunikation läuft über die Platform.

**Connection-Management:**
- **Connection-Pooling**: Wiederverwendung von Verbindungen für bessere Performance
- **Connection Reuse**: Connections werden effizient wiederverwendet
- **Automatische Reconnection**: Kombination aus sofortigem Versuch + Exponential Backoff
  - Sofortiger Reconnect-Versuch bei Verbindungsabbruch
  - Nach erstem Fehler beginnt Exponential Backoff
  - Maximale Wartezeit (z.B. 60 Sekunden)
  - Kontinuierliche Versuche zur Wiederherstellung
- **Connection Monitoring**: Verbindungsstatus wird überwacht

**Error-Handling in gRPC:**
- **gRPC Status-Codes**: gRPC-Fehler werden über Status-Codes behandelt
- **Retry-Mechanismen**: Automatischer Retry mit Exponential Backoff (siehe gemeinsame Klärungspunkte)
- **Timeout-Konfiguration**: Adaptive Timeouts mit Minimum/Maximum
- **Fallback**: Bei Fehler Fallback zu alternativen Routen/Providern

**Vorteile:**
- **Type-Safe**: Protobuf garantiert korrekte Service-Interfaces
- **Bessere Performance**: HTTP/2, Binary-Format, weniger Overhead
- **Streaming**: Built-in Streaming für große Datenmengen
- **Microservices-Architektur**: Jeder Service ist unabhängig

**Wichtig**: Odin orchestriert Plugins modulär:
- **Thor**: Plugin für Action-Execution (wenn Funktionen ausgeführt werden müssen)
- **Valkyries**: Plugin für Coding-Aufgaben (wenn verfügbar)
- **Frigg**: Plugin für Gesundheitsfragen (wenn verfügbar)
- **Modular**: Plugins sind optional und können je nach Bedarf und Verfügbarkeit hinzugefügt werden

## Workflow

1. **User Input empfangen**
   - **Text-Input**: Frontend sendet `RavenMessage` direkt mit Text
   - **ODER Voice-Input**: Huginn sendet `RavenMessage` mit transkribiertem Text (aus Voice-Input)
   - **ODER Bild-Input**: Huginn sendet Bild-Daten (keine Interpretation durch Huginn)
   - **ODER Video-Input**: Huginn sendet Video-Daten (keine Interpretation durch Huginn)
   - **ODER Video-Stream-Input**: Huginn sendet Video-Stream-Daten (keine Interpretation durch Huginn)
   - Odin analysiert den Command (unabhängig von Input-Methode)

2. **Vision-Model-Interpretation (falls Bild/Video)**
   - Falls Bild/Video/Video-Stream: Odin sendet Daten an Geri via gRPC für Vision-Model-Interpretation
   - **Video-Verarbeitung**: 
     - **Streaming für große Dateien**: Große Video-Dateien werden via gRPC-Streaming verarbeitet (nicht vollständiger Upload)
     - **Dateigrößen-Limits**: Abhängig von verfügbarem Speicher und Netzwerk-Bandbreite
     - **Video-Formate**: Geri unterstützt verschiedene Video-Formate (MP4, AVI, etc.)
   - **Vision-Model-Auswahl**:
     - Geri wählt passendes Vision-Model basierend auf Dateigröße, Format und verfügbaren Models
     - **Fallback-Mechanismen**: Bei Model-Ausfall wird automatisch auf alternatives Model ausgewichen
   - Geri interpretiert Bild/Video mit Vision-Model (GPT-4V, Claude Vision, etc.)
   - Odin erhält Analyse-Ergebnis von Geri
   - **Vision-Model-Response-Interpretation**: Odin interpretiert Vision-Model-Responses und integriert sie in Request-Kontext
   - **Verhaltensmuster-Erkennung** (optional, mit User-Zustimmung): Odin erkennt Verhaltensmuster aus Video-Streams

3. **Context Enrichment (optional)**
   - Falls nötig: Request an Freki (RAG) via gRPC für Kontext-Anreicherung
   - Freki sucht relevante Dokumente und erstellt `RAGContext`
   - Odin erhält `WolfResponse` mit `ragContext` von Freki
   - Odin speichert `ragContext` für LLM-Request

4. **LLM-Selection (optional, wenn Provider/Marketplace aktiv)**
   - **Provider-Suche durch Yggdrasil**: Yggdrasil ist für Provider-Suche zuständig, Odin erhält nur die Provider-Liste
   - **Netzwerkplan-Erstellung**:
     - **On-the-fly mit Cache**: Odin erstellt Netzwerkplan on-the-fly, aber cached ihn für bessere Performance
     - **Cache-Aktualisierung**: Netzwerkplan wird bei Netzwerk-Änderungen aktualisiert (Service-Start/Stop, Device-Verbindungen)
     - **Netzwerk-Änderungen während Request**: Odin verwendet gecachten Plan, aktualisiert nach Request-Abschluss
   - Odin fragt Skuld via gRPC: "Welches LLM/Device für diesen Request?"
   - **Provider-Auswahl**:
     - **Lokale vs. Cloud-LLMs**: Skuld entscheidet basierend auf Effizienz, Kosten, Latenz (keine Bevorzugung)
     - **Prioritäten**: Nur User-Präferenzen aus Konfiguration, sonst Effizienz-basiert
     - **Provider-Ausfälle**: Automatischer Fallback zu alternativen Providern
   - Skuld analysiert Netzwerkplan und empfiehlt optimales LLM/Device
   - Odin nutzt diese Information für Request-Routing
   - **Fallback-Hierarchie**: Wenn Provider-Liste leer → bestes verfügbares LLM im Netzwerk, letzter Fallback ist lokales LLM

5. **Einherjar Protocol: Funktions-Entdeckung**
   - Odin fragt alle Götter via Einherjar Protocol nach verfügbaren Funktionen
   - Odin erhält `CapabilityResponse` von allen Göttern mit:
     - Verfügbare Funktionen
     - Zweck des Gottes
     - Zuständigkeits-Domains
     - Responsibility-Keywords

6. **Gott-Auswahl & Zuständigkeits-Weiterleitung**
   - Odin analysiert Request, Vision-Analyse, RAG-Context und Einherjar Protocol-Daten
   - Odin wählt wahrscheinlichsten Gott basierend auf:
     - Funktionen und Keywords
     - Zuständigkeits-Domains
     - Request-Inhalt
   - Odin sendet `ResponsibilityRequest` an gewählten Gott
   - **Gott-Entscheidung**:
     - **Übernahme**: Gott bestätigt Zuständigkeit mit `ResponsibilityResponse`
     - **Rückweisung**: Gott weist zurück mit `ResponsibilityRejection` und Hinweis auf besseren Gott
   - **Fallback bei Rückweisung**: Odin weicht auf nächstwahrscheinlichste Wahl aus

7. **Response-Generierung**
   - **Wichtig: Alle Plugins geben Rückmeldung an Odin**: Alle Plugins (Thor, Valkyries, Frigg) geben ihre Ergebnisse direkt an Odin zurück
   - **Zuständiger Gott generiert Response**: Zuständiger Gott (z.B. Frigg, Thor, Valkyries) generiert Response und gibt sie an Odin zurück
     - **Thor**: Gibt `ThorResult` an Odin zurück (Action Execution Results)
     - **Valkyries**: Gibt `ValkyrieResult` an Odin zurück (strukturierte Code-Änderungen)
     - **Frigg**: Gibt `FriggResult` an Odin zurück (Healthcare Task Results)
   - **Odin generiert Response**: Falls kein Gott zuständig oder einfache Frage, Odin generiert Response selbst
   - **LLM Processing** (falls nötig): Request an Geri (LLM) via gRPC mit angereichertem Prompt
     - Odin sendet `WolfRequest` mit `prompt` und optional `ragContext` (von Freki)
     - Geri fügt RAG-Context in LLM-Prompt ein (siehe Geri RAG-Context-Integration)
     - Geri verwaltet Context-Window (siehe Geri Context-Window-Management)
   - Geri nutzt gewähltes LLM/Device (basierend auf Skuld-Empfehlung)
   - Warten auf LLM-Response (Streaming möglich via gRPC)

8. **Zuständigkeits-Rückgabe**
   - Gott analysiert Request, erkennt dass es nicht mehr in seinem Bereich ist
   - Gott sendet `ResponsibilityReturn` an Odin
   - Odin übernimmt wieder Entscheidungsfindung

9. **Response Generation**
   - Erstellt Response basierend auf Gott-Result oder eigener Antwort
   - Sendet `RavenMessage` an Muninn für TTS

## Abhängigkeiten

### Keine Core Library

- **WICHTIG**: Es gibt keine Edda Core Library
- **Separate Projekte**: Wenn gemeinsame Komponenten benötigt werden (DTOs, Protocols, Utils), sollte ein separates Projekt erstellt werden
- **Selektive Nutzung**: Dies hält Apps klein, da genau gewählt werden kann, was benötigt wird
- **Keine Abhängigkeit**: Odin sollte nicht auf Dateien/Protocols/Utils aus dem `edda` Verzeichnis verweisen (KEIN PROJEKT - nur Metadaten-Sammlung)

### Service-Abhängigkeiten

- **Frontend**: Für Text-Input (optional, wenn Frontend vorhanden)
- **Huginn/Muninn**: STT/TTS Service (für Voice-Input und TTS-Output)
- **Freki**: RAG Service (optional)
- **Geri**: LLM Service (optional)
- **Bifrost**: Communication Service (optional)
- **Heimdall**: Security Service (optional)
- **Skuld**: LLM-Selection Service (muss installiert sein)
- **Vedrfolnir**: Connection Builder Client (optional, für Yggdrasil-Kommunikation)

### Plugin-Abhängigkeiten (optional, modular)

- **Thor**: Plugin für Action-Execution (wenn Funktionen ausgeführt werden müssen)
- **Valkyries**: Plugin für Coding-Aufgaben (wenn verfügbar)
- **Frigg**: Plugin für Gesundheitsfragen (wenn verfügbar)
- **Modular**: Plugins können je nach Bedarf und Verfügbarkeit hinzugefügt werden

## Einherjar Protocol

Das **Einherjar Protocol** ist ein Standard-Protokoll für alle Götter (Services und Plugins), um ihre verfügbaren gRPC-Funktionen und Zuständigkeits-Bereiche offenzulegen. Odin nutzt dieses Protokoll, um automatisch zu erkennen, welcher Gott für welche Art von Anfragen zuständig ist.

**Einherjar = Krieger Odins**: Die Services (Krieger) stehen Odin zur Verfügung und definieren ihre Aufgaben im Protocol. Das Einherjar Protocol kombiniert **Service-Discovery** (Funktions-Offenlegung) und **Responsibility Management** (Zuständigkeits-Definition) in einem Protokoll.

### Protokoll-Definition

Jeder Gott muss den `EinherjarProtocol` Service implementieren:

```protobuf
service EinherjarProtocol {
  rpc GetCapabilities(CapabilityRequest) returns (CapabilityResponse);
}

message CapabilityResponse {
  string god_name = 1;  // z.B. "Thor", "Frigg", "Valkyries"
  string purpose = 2;   // Zweck des Gottes (z.B. "Action Execution", "Healthcare", "Coding")
  repeated FunctionDefinition functions = 3;
  repeated string responsibility_domains = 4;  // Für welche Art von Anfragen ist dieser Gott zuständig
}

message FunctionDefinition {
  string name = 1;
  string description = 2;
  repeated ParameterDefinition parameters = 3;
  string return_type = 4;
  repeated string capabilities = 5;
  repeated string responsibility_keywords = 6;  // Keywords, die auf Zuständigkeit hinweisen
}
```

### Funktions-Entdeckung

**Caching und Aktualisierung:**
- **Funktions-Definitionen werden gecacht**: Odin cached Capability-Responses für bessere Performance
- **Event-basierte Aktualisierung**: Bei Service-Start/Stop werden Capabilities aktualisiert
- **Polling als Fallback**: Periodisches Polling (z.B. alle 5 Minuten) als Fallback für verpasste Events
- **Änderungen in Funktions-Definitionen**: Services können Capability-Updates signalisieren, Odin aktualisiert Cache

### Zuständigkeits-Erkennung

**Keyword-basierte Erkennung:**
- **Multi-Faktor-Bewertung**: Odin bewertet Götter basierend auf:
  - **Responsibility-Keywords**: Match-Score für Keywords im Request
  - **Responsibility-Domains**: Domain-Match für Request-Typ
  - **Function-Definitionen**: Relevanz der verfügbaren Funktionen
  - **Request-Inhalt**: Semantische Analyse des Request-Inhalts
- **Priorisierung bei mehreren passenden Göttern**: Götter werden nach Relevanz-Score sortiert, höchster Score wird zuerst angefragt
- **Relevanz-Bewertung**: Gewichteter Score aus Keywords, Domains, Funktionen und Request-Inhalt

Odin nutzt das Einherjar Protocol, um:
1. **Automatische Funktions-Entdeckung**: Odin kann automatisch alle verfügbaren Funktionen entdecken
2. **Zuständigkeits-Erkennung**: Odin erkennt anhand von Funktionen, Keywords und Domains, welcher Gott zuständig ist
3. **Dynamische Delegation**: Odin delegiert Aufgaben basierend auf Einherjar Protocol-Daten
4. **Fallback-Mechanismus**: Bei Rückweisung wählt Odin alternative Götter basierend auf Einherjar Protocol

## Responsibility Service

Das **Responsibility Service** ist ein Standard-Service für alle Götter, um Zuständigkeit zu übernehmen, zurückzugeben oder zurückzuweisen.

**Zusammenhang mit Einherjar Protocol**: 
- **Einherjar Protocol** definiert **WAS** ein Service kann (Funktionen, Zuständigkeits-Domains, Keywords)
- **Responsibility Service** verwaltet **WER** aktuell zuständig ist (Take/Return/Reject)
- Beide arbeiten zusammen: Einherjar zeigt verfügbare Services, Responsibility verwaltet die aktuelle Zuständigkeit

### Service-Definition

```protobuf
service ResponsibilityService {
  rpc TakeResponsibility(ResponsibilityRequest) returns (ResponsibilityResponse);
  rpc ReturnResponsibility(ResponsibilityReturn) returns (ResponsibilityAcknowledgment);
  rpc RejectResponsibility(ResponsibilityRejection) returns (ResponsibilityAcknowledgment);
}
```

### Zuständigkeits-Management

- **State Tracking**: Odin trackt aktuell zuständigen Gott
- **Automatische Weiterleitung**: Requests werden automatisch an zuständigen Gott weitergeleitet
- **Timeout-Mechanismus**: Zuständigkeit bleibt bestehen bis:
  - Gott gibt Zuständigkeit explizit zurück (`ReturnResponsibility`)
  - Request ist abgeschlossen und Gott signalisiert Ende
  - Timeout (z.B. 30 Minuten Inaktivität) - dann automatische Rückgabe
- **Nicht-Antwort-Handling**: Wenn Gott nicht antwortet:
  - Retry mit Exponential Backoff
  - Nach mehreren Fehlversuchen: Zuständigkeit wird zurückgenommen, Fallback zu nächstwahrscheinlichstem Gott
- **Zuständigkeits-Rückgabe**: 
  - **Explizit**: Gott sendet `ReturnResponsibility` wenn Request außerhalb seines Bereichs ist
  - **Automatisch**: Nach Request-Abschluss oder Timeout
- **Rückweisungs-Mechanismus**: 
  - **Nächstwahrscheinlichste Wahl**: Odin wählt nächsten Gott aus sortierter Liste (basierend auf Relevanz-Score)
  - **Maximale Rückweisungen**: Maximal 3-5 Rückweisungen, dann Fehler an User
  - **Alle Götter zurückweisen**: Odin generiert Response selbst oder gibt Fehler zurück

## Plugin-Interface

Alle Plugins müssen das `OdinPlugin`-Interface implementieren (in Rust als Trait), damit Odin sie verstehen und orchestrieren kann. Zusätzlich müssen alle Plugins das **Einherjar Protocol** und das **Responsibility Service** implementieren.

### Interface-Definition

```rust
pub trait OdinPlugin {
    /// Gibt den Titel des Plugins zurück
    fn get_title(&self) -> String;
    
    /// Gibt die Beschreibung des Plugins zurück
    fn get_description(&self) -> String;
    
    /// Gibt die verfügbaren Funktionen des Plugins zurück (Function Call Protocol)
    /// Odin nutzt diese Informationen, um schnell und sicher Entscheidungen zu treffen
    /// ohne den Plugin-Code prüfen zu müssen
    fn get_functions(&self) -> Vec<FunctionDefinition>;
}
```

### Einherjar Protocol Integration

Plugins müssen zusätzlich das Einherjar Protocol implementieren:
- **GetCapabilities**: Gibt alle verfügbaren Funktionen, Zweck und Zuständigkeits-Domains zurück
- **Responsibility Service**: Implementiert TakeResponsibility, ReturnResponsibility, RejectResponsibility

### Plugin-Erkennung durch Odin

Odin nutzt das Einherjar Protocol, um:
1. **Automatische Funktions-Entdeckung**: Odin kann automatisch alle verfügbaren Funktionen entdecken
2. **Zuständigkeits-Erkennung**: Odin erkennt anhand von Funktionen, Keywords und Domains, welcher Gott zuständig ist
3. **Dynamische Delegation**: Odin delegiert Aufgaben basierend auf Einherjar Protocol-Daten
4. **Keine Code-Prüfung nötig**: Odin muss nicht den Plugin-Code analysieren, um zu verstehen, was das Plugin kann

### Plugin-Entwicklung

User können eigene Plugins entwickeln, die das `OdinPlugin`-Interface implementieren. Plugins können:
- Über den Marketplace verkauft werden
- Lokal installiert werden
- Mit anderen Plugins kombiniert werden

#### FunctionDefinition

Jede Funktion muss als `FunctionDefinition` beschrieben werden:

```rust
pub struct FunctionDefinition {
    pub name: String,
    pub description: String,
    pub parameters: JsonSchema,
    pub return_type: ReturnType,
    pub capabilities: Vec<Capability>,
}
```

#### Function Call Protocol

Das Function Call Protocol basiert auf dem OpenAI Function Calling Format:

```rust
pub struct JsonSchema {
    pub r#type: String,  // "object", "string", "number", etc.
    pub properties: HashMap<String, Property>,
    pub required: Vec<String>,
}

pub struct Property {
    pub r#type: String,
    pub description: String,
    pub enum_values: Option<Vec<String>>,
}
```

#### Beispiel: Einfaches Plugin

```rust
use edda::plugin::{OdinPlugin, FunctionDefinition};

pub struct MyPlugin;

impl OdinPlugin for MyPlugin {
    fn get_title(&self) -> String {
        "My Custom Plugin".to_string()
    }
    
    fn get_description(&self) -> String {
        "Ein Beispiel-Plugin, das zeigt, wie Plugins implementiert werden.".to_string()
    }
    
    fn get_functions(&self) -> Vec<FunctionDefinition> {
        vec![
            FunctionDefinition {
                name: "my_function".to_string(),
                description: "Führt eine benutzerdefinierte Funktion aus.".to_string(),
                parameters: JsonSchema {
                    r#type: "object".to_string(),
                    properties: HashMap::new(),
                    required: vec![],
                },
                return_type: ReturnType::String,
                capabilities: vec![],
            },
        ]
    }
}
```

#### Plugin-Registrierung

**Lokale Installation**
- Plugins können lokal installiert werden, indem sie in das Plugin-Verzeichnis kopiert werden: `~/.edda/plugins/`

**Marketplace-Publishing**
1. **Plugin-Registrierung**: Developer registriert Plugin auf Yggdrasil
2. **Plugin-Beschreibung**: Titel, Beschreibung, Funktionen werden über Interface bereitgestellt
3. **Pricing**: Developer kann Preis für Plugin festlegen
4. **Versioning**: Plugin-Versionen werden verwaltet

#### Best Practices

**Plugin-Design**
- **Klarer Fokus**: Jedes Plugin sollte einen klaren, spezifischen Zweck haben
- **Gute Dokumentation**: Funktionen sollten klar beschrieben sein
- **Fehlerbehandlung**: Plugins sollten robuste Fehlerbehandlung haben
- **Performance**: Plugins sollten effizient sein

**Function Call Protocol**
- **Präzise Beschreibungen**: Funktionen sollten klar beschrieben sein
- **Vollständige Parameter**: Alle Parameter sollten dokumentiert sein
- **Capabilities**: Capabilities sollten klar definiert sein

### Marketplace-Integration

Plugins können über den Marketplace veröffentlicht und verkauft werden. Der Marketplace unterstützt:
- **Plugins**: Vollständige Plugin-Implementierungen
- **Skills**: Vordefinierte Funktionssammlungen
- **Rules**: Regel-basierte Konfigurationen
- **Commands**: Vordefinierte Command-Sets
- **Snippets für AGENTS.md**: Code-Snippets für Agent-Konfigurationen

**Plugin-Kategorien**
- **Action-Execution**: Plugins für Action-Execution (z.B. Thor)
- **Coding**: Plugins für Coding-Aufgaben (z.B. Valkyries)
- **Healthcare**: Plugins für Gesundheitsfragen (z.B. Frigg)
- **Custom**: Benutzerdefinierte Plugins

**Pricing**
- **Einmalige Zahlung**: Plugin kann einmalig gekauft werden
- **Abonnement**: Plugin kann als Abonnement angeboten werden
- **Kostenlos**: Plugin kann kostenlos angeboten werden

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

#### Settings-Befüllung bei Installation
- **Installation-Defaults**: Settings müssen bei der Installation (mindestens mit Default-Werten) befüllt werden
- **Jeder Gott hat LLM**: Jeder Gott hat ein LLM, um Dinge zu tun, aber auch bestimmten Code, der den Workflow darstellt
- **Default-Konfiguration**: Jeder Service/Plugin muss mit funktionsfähigen Default-Settings installiert werden können

### Odin-spezifische Settings

**Settings-Struktur (JSON-Format):**

```json
{
  "user_preferences": {
    "quality_level": "medium",  // "low" | "medium" | "high" | "custom"
    "max_cost": 0.10,            // Maximale Kosten pro Request (optional)
    "max_latency_ms": 5000,      // Maximale Antwortzeit in ms (optional)
    "model_preferences": [],     // Bevorzugte Modelle (optional)
    "provider_preferences": []   // Bevorzugte Provider (optional, nicht empfohlen)
  },
  "provider_selection": {
    "auto_select": true,         // Automatische Provider-Auswahl
    "min_quality": 0.7,         // Minimum-Quality-Filter (optional)
    "fair_distribution": true   // Fair Distribution aktivieren
  },
  "plugins": {
    "valkyries": {
      "enabled": true,
      "parallel_agents": 3,     // Anzahl paralleler Agents
      "llm_config": {}          // LLM-Konfiguration für Valkyries
    },
    "frigg": {
      "enabled": false,
      "chat_direct": false      // Direkte Chat-Leitung an Frigg
    }
  },
  "network": {
    "auto_connect": true,        // Automatische Verbindung zu anderen Devices
    "yggdrasil_enabled": false,  // Yggdrasil-Verbindung aktiviert
    "asgard_relay": true        // Asgard als Relay nutzen
  },
  "state_sync": {
    "enabled": true,            // State-Synchronisation aktiviert
    "sync_interval_ms": 1000,   // Sync-Intervall (optional)
    "selective_propagation": true // Selective Propagation aktiviert
  },
  "chat_flags": {
    "frigg_direct": false,      // Direkte Chat-Leitung an Frigg
    "valkyries_direct": false   // Direkte Chat-Leitung an Valkyries
  }
}
```

**Settings-Speicherung:**
- **Lokale Speicherung**: Settings werden lokal in JSON-Datei gespeichert (z.B. `~/.edda/odin/settings.json`)
- **Verschlüsselte Speicherung**: Sensitive Settings (z.B. API-Keys) werden verschlüsselt gespeichert
- **Schema-Validierung**: Settings werden beim Laden validiert (JSON-Schema)
- **Hot-Reload**: Settings können zur Laufzeit neu geladen werden (keine Neukompilierung nötig)

**Settings-Synchronisation zwischen Devices:**
- **Push-basiert**: Settings-Änderungen werden sofort an alle verbundenen Devices gesendet
- **Via Bifrost**: Settings werden über Bifrost synchronisiert (State-Synchronisation)
- **Conflict-Resolution**: Timestamp + Priority (wie bei State-Synchronisation)
- **Selective Sync**: Nur relevante Settings werden synchronisiert (z.B. nicht device-spezifische Settings)
- **Yggdrasil-Sync**: Falls mit Yggdrasil verbunden, Settings auch in Yggdrasil gespeichert (optional)

**Settings-API:**
- **Get Settings**: `get_settings() -> Settings`
- **Update Settings**: `update_settings(settings: Settings) -> Result`
- **Reload Settings**: `reload_settings() -> Result`
- **Validate Settings**: `validate_settings(settings: Settings) -> ValidationResult`

## Technische Anforderungen

- Event-driven Architecture
- Asynchrone Message Processing
- State Management
- Error Handling & Recovery
- Logging & Monitoring

## Integration

- **Midgard**: Läuft als Hauptprozess auf Desktop/Laptop (mit GUI-Frontend)
- **Alfheim**: Läuft als Hauptprozess auf Mobile (mit GUI-Frontend)
- **Asgard**: Läuft als Hauptprozess auf Homeserver (mit GUI-Frontend)
- **Ragnarok**: Läuft als Hauptprozess auf Terminal (mit TUI-Frontend statt GUI)
- **Jotunheim**: Kein Odin (zu klein, nutzt Loki für Toolcalling-Funktionalität)
- **Services**: Koordiniert alle Services (Huginn/Muninn, Freki, Geri, Bifrost, Heimdall, Skuld, Vedrfolnir)
- **Plugins**: Orchestriert Plugins modulär (Thor, Valkyries, Frigg) - alle optional
- **Modular**: Plugins können je nach Bedarf und Verfügbarkeit hinzugefügt werden
- **Yggdrasil**: Kommuniziert über Vedrfolnir und Ratatoskr-Protocol

### Cross-Device-Communication

**Odin kommuniziert mit Odin auf anderen Devices:**
- **Bifrost + gRPC**: Odin kommuniziert mit Odin auf anderen Devices über Bifrost (WebSocket) für Connection-Establishment, dann gRPC für Service-Kommunikation
- **Cross-Device Actions**: Device A sendet ThorAction via gRPC an Device B über Bifrost-Verbindung
- **Routing-System**: Yggdrasil kann als zentrales Routing-System fungieren, aber direkte Device-to-Device-Verbindungen sind auch möglich
- **Netzwerk-Partitionen**: 
  - **Fallback zu Yggdrasil-Relay**: Wenn direkte Verbindung nicht möglich, automatischer Fallback zu Yggdrasil-Relay
  - **Asgard als Relay**: Falls Asgard verfügbar, kann auch als Relay fungieren
  - **Automatische Route-Auswahl**: Odin wählt automatisch beste Route (direkt → Asgard → Yggdrasil)

**Yggdrasil-Integration:**
- **Verbindung**: Odin verbindet sich mit Yggdrasil über Bifrost (WebSocket) für Device-Relay und Event-Notifications, sowie gRPC für Request/Response-Patterns
- **Offline-Modi**: Devices können ohne Yggdrasil-Account starten und autonom funktionieren
- **Yggdrasil-Ausfälle**: Fallback zu lokalen Verbindungen oder Asgard-Relay

## RAG-System für Odin

**Projekt-Indexierung:**
- **RAG-Integration**: Odin nutzt RAG-System (ähnlich wie Freki) für Projekt-Indexierung
- **Stichwort-basierte Speicherung**: Informationen werden in Stichworten festgehalten, ohne Sinn zu verlieren
- **Kategorisierung und Gruppierung**: Informationen werden kategorisiert und gruppiert
- **Effiziente Suche**: Mit Stichworten gute Treffer erzielen, aber nicht alle Daten aus Datenbank laden müssen
- **Optimierte Datenbanken**: Verwendung von für RAG optimierten Datenbanken (z.B. Vector-Databases)

**RAG-Strategie:**
- **Stichwort-Extraktion**: Wichtige Informationen werden als Stichworte extrahiert
- **Kategorisierung**: Stichworte werden kategorisiert (z.B. "Frontend", "Backend", "Database", "API")
- **Gruppierung**: Ähnliche Stichworte werden gruppiert
- **Vector-Embeddings**: Stichworte werden als Embeddings gespeichert für semantische Suche
- **Hierarchische Struktur**: Informationen werden hierarchisch strukturiert (Kategorie → Gruppe → Stichwort)

**Context-Window-Optimierung:**
- **Selektive Datenladung**: Nur relevante Informationen werden geladen (basierend auf Stichwort-Suche)
- **Minimaler Memory-Footprint**: Kleine Context-Fenster durch effiziente Datenstrukturen
- **Schnelle Suche**: Stichwort-basierte Suche ist schnell und effizient

## Performance

### Caching-Strategien

**Gecachte Daten:**
- Funktions-Definitionen (Einherjar Protocol)
- Netzwerkpläne (on the fly, mit Cache)
- Häufig verwendete Responses
- Token-Validierungen und Permissions (Heimdall)
- Quality-Metriken (Eikthyrnir)
- RAG-Index (Stichworte, Kategorien, Embeddings)

**Cache-Invalidierung:**
- Event-basiert: Bei Capability-Updates, Netzwerkplan-Updates, Device-Status-Änderungen
- Timeout-basiert: Als Fallback, wenn Events fehlen
- Sofortige Invalidierung bei wichtigen Änderungen

**Cache-Sharing:**
- Kein direkter Cache-Sharing zwischen Devices
- State-Synchronisation für konsistenten State

### Service-Ausfall-Behandlung

**Innerhalb einer Platform:**
- Fallback ist unnötig - Services müssen existieren, so bauen wir sie ja
- Services sind Teil der Platform-Installation

**Platformübergreifend:**
- Netzwerkplan verwenden für Service-Discovery
- Falls mit Yggdrasil verbunden: Netzwerkplan an Yggdrasil übertragen
- **WICHTIG**: Netzwerkplan darf unter keinen Umständen anderen Usern zugänglich gemacht werden
- Asgard fungiert wie eine weitere Platform (Server-optimiert), ähnlich wie Midgard (Desktop-optimiert) und Alfheim (Mobile-optimiert)

**LLM-Auswahl:**
- **Provider-Suche durch Yggdrasil**: Yggdrasil ist für Provider-Suche zuständig, Odin erhält nur die Provider-Liste
- Bei Abfrage soll möglichst immer das beste verfügbare Model gewählt werden
- **Fallback-Hierarchie**: Wenn Provider-Liste leer → bestes verfügbares LLM im Netzwerk, letzter Fallback ist lokales LLM
- **Lokales LLM**: Wird bei Installation vom User explizit gewählt aus einer Liste mit für das Device passenden Modellen (unterschiedlich starke und ressourcenintensive Modelle mit Beschreibungen)
- Sicherstellung: Mit Installation einer Platform wird auch ein LLM mitgeliefert

**Fallback-Strategien (nur platformübergreifend):**
- Alternative Route: Falls direkte Verbindung fehlschlägt, Fallback zu Relay (Asgard/Yggdrasil)
- Alternative Provider: Falls Provider-Fehler, Fallback zu alternativem Provider
- Lokales LLM: Falls Cloud-LLM-Fehler, Fallback zu lokalem LLM (letztes Fallback)

**Service-Ausfall-Behandlung:**
- Automatischer Retry mit Exponential Backoff
- Sofortiger Fallback zu alternativen Services (nur platformübergreifend)
- User-Benachrichtigung bei komplettem Service-Ausfall

**User-Kommunikation:**
- Fehlermeldung an User, wenn alle Versuche fehlschlagen
- Error-Logging für Debugging
- User kann später erneut versuchen
- Transparente Fehlerbehandlung

### Concurrency und Resource-Management

**Prozess-Management:**
- **Non-Blocking Delegation**: Odin startet für Aufgaben, die er anderen Göttern übergibt, neue/parallele Prozesse (non-blocking)
- **Prozess-Isolation**: Thor, Valkyries und alle anderen Götter werden in neuen Prozessen gestartet
- **Sofortige Antwort bei eigenen Aufgaben**: Nur wenn Odin selbst antwortet (allgemeine Fragen, System-Fragen, Platform/Ecosystem-Fragen), gibt er sofort Antwort und blockiert nicht
- **Hintergrund-Chat-Management**: Für abgegebene Tasks werden Chats im Hintergrund behalten, damit Ergebnisse später empfangen werden können
- **Multiple Chats**: Ein neuer leerer Chat wird gestartet, damit Odin wieder bereit für weitere Anfragen ist (multiple Chats parallel möglich)
- **Chat-Leitung an andere Götter**: Odin kann kurzzeitig/use-case-abhängig die Chat-Leitung an andere Götter abgeben
  - **Automatische Weiterleitung**: Odin verweist automatisch auf Frigg (oder andere Götter) und durch ein Flag bleibt der Chat bei Frigg
  - **Explizite Chat-Erstellung**: User kann explizit einen Chat mit Frigg erstellen, dann bleibt der Chat immer bei Frigg und kann auch nicht an Odin übergeben werden
  - **Direkte Eingabe**: Bei aktivem Flag werden User-Eingaben direkt an den zuständigen Gott (z.B. Frigg) geleitet, statt über Odin, der immer wieder entscheiden muss

**Parallele Requests:**
- **Asynchrone Verarbeitung**: Odin kann mehrere Requests parallel verarbeiten (Event-driven Architecture)
- **Request-Queuing**: Requests werden in Queue gelegt, wenn alle Slots belegt sind
- **Resource-Limits**: 
  - **Maximale parallele Requests**: Konfigurierbar basierend auf verfügbaren Ressourcen
  - **CPU/Memory-Limits**: Odin respektiert System-Ressourcen-Limits
  - **Adaptive Limits**: Limits passen sich an verfügbare Ressourcen an
- **Request-Priorisierung**: Wichtige Requests (z.B. User-Input) haben höhere Priorität

### Performance-Optimierungen
- **Event-driven Architecture**: Asynchrone Event-Verarbeitung für bessere Performance
- **Parallel Processing**: Parallele Verarbeitung von mehreren Requests
- **Caching**: Intelligentes Caching für häufig verwendete Daten und Responses
- **Connection Pooling**: Effizientes Connection-Pooling für Service-Kommunikation
- **State Management**: Optimiertes State-Management für schnellen Zugriff
- **Resource Management**: Intelligentes Resource-Management für optimale Performance

### Performance-Monitoring

**Performance-Metriken:**
- Response-Zeiten (z.B. < 100ms für Standard-Requests)
- Durchsatz (z.B. 1000+ Requests/Sekunde)
- Resource-Usage (CPU, Memory, Disk, Network)
- Latency-Metriken
- Quality-Metriken (Eikthyrnir)

**Performance-Überwachung:**
- Monitoring von Performance-Metriken
- Performance-Tracking für alle Services
- Resource-Monitoring
- Connection-Quality-Monitoring

**Problem-Erkennung:**
- Kontinuierliche Überwachung
- Performance-Probleme werden erkannt
- Alerts bei Performance-Problemen
- Performance-Optimierung basierend auf Metriken

### Performance-Metriken
- Niedrige Latenz für Command-Processing (< 100ms für einfache Commands)
- Hoher Durchsatz für parallele Requests
- Effiziente Service-Koordination (minimaler Overhead)

## Monitoring & Logging

### Strukturiertes Logging

**Strukturiertes Logging:**
- Structured Logging mit strukturierten Daten
- Log Levels: Verschiedene Log-Level (DEBUG, INFO, WARN, ERROR, etc.)
- Context Tracking: Context wird mitgeloggt
- Log Rotation: Automatische Log-Rotation

**Implementierung:**
- Alle Services verwenden strukturiertes Logging
- Logging für Debugging und Monitoring
- Umfassendes Logging für alle Services

**Log-Synchronisation:**
- Logs werden nicht direkt zwischen Devices synchronisiert
- Jedes Device hat eigene Logs
- Audit-Logs werden für Compliance aufbewahrt
- Logs können für Debugging und Monitoring verwendet werden

## Datenschutz

### Datenschutz-Features
- **Lokale Verarbeitung**: Daten werden lokal verarbeitet, wo möglich
- **Minimale Datensammlung**: Nur notwendige Daten werden gespeichert
- **Keine Tracking-Daten**: Keine User-Tracking oder Analytics-Daten ohne Zustimmung
- **Datenverschlüsselung**: Sensible Daten werden verschlüsselt gespeichert
- **User Control**: User hat volle Kontrolle über seine Daten

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

**Audit-Logging für Compliance:**
- Vollständiges Audit-Logging aller Datenzugriffe
- Immutable Logs für Compliance
- Langfristige Aufbewahrung von Logs
- Compliance-Logging erfüllt GDPR-Anforderungen

**Privacy by Design:**
- Privacy ist von Anfang an integriert
- Privacy by Default: Default-Einstellungen sind privacy-friendly

### Data-Minimization

**Data Minimization:**
- Nur notwendige Daten werden gespeichert
- Nur notwendige Daten werden verarbeitet
- Purpose Limitation: Daten nur für spezifische Zwecke verwendet
- Storage Limitation: Daten nur so lange gespeichert wie nötig

**Schutz persönlicher Daten:**
- Verschlüsselung aller personenbezogenen Daten (at rest und in transit)
- Strikte Zugriffskontrolle
- Data Isolation: Strikte Isolation von anderen Datenbanken

**GDPR-konform:**
- Vollständige Einhaltung der GDPR-Anforderungen

### Compliance
- **GDPR-konform**: Unterstützung für GDPR-Anforderungen
- **Data Minimization**: Nur notwendige Daten werden gespeichert
- **Right to Deletion**: User kann alle Daten löschen
- **Transparency**: User wird über Datenverarbeitung informiert

## Sicherheit

### Authentication/Authorization

**Zentrales Permission-System:**
- Heimdall ist der zentrale Security Service
- Odin nutzt Heimdall für alle Authentication/Authorization-Requests

**Authentication:**
- Device-Authentifizierung via Heimdall
- Token-basierte Authentication (Heimdall-Tokens, Session-Tokens)
- Public/Private Key Pairs für Device-Authentifizierung
- Digital Signatures

**Authorization:**
- Permission Checking via Heimdall
- Role-Based Access Control (RBAC)
- Permission-based Access Control
- Resource-based Permissions
- Conditional Permissions

**Service-zu-Service:**
- Services authentifizieren sich via Heimdall
- Bifrost Connection Validation: Heimdall validiert alle Bifrost-Verbindungen

### Secure Storage

**Verschlüsselung:**
- At-Rest Encryption für alle lokalen Daten
- In-Transit Encryption für alle Datenübertragungen
- Moderne Verschlüsselungsalgorithmen (AES-256, etc.)

**Key Management:**
- Sichere Verwaltung von Cryptographic Keys via Heimdall
- Key Rotation: Regelmäßig + Event-basiert
- Sichere Key-Speicherung
- Key Access Control

**Secure Storage:**
- Keine Hardcoded Secrets
- Environment Variables oder secure secret management
- Verschlüsselte Speicherung von sensiblen Daten

### TLS/Encryption

**TLS 1.3:**
- TLS Encryption für alle Netzwerk-Verbindungen
- TLS Handshake für Verschlüsselung

**Certificate-Validierung:**
- Certificate-Validierung für alle WAN-Connections
- Certificate Validation für TLS-Zertifikate

**Secure Key Exchange:**
- Sichere Key-Exchange-Protokolle

### Input-Validation

**Input Validation:**
- Umfassende Validierung aller Inputs
- DTO Validation
- Input Sanitization
- Type Checking
- Schema Validation

**Injection-Schutz:**
- Message Sanitization zum Schutz vor Injection-Angriffen
- Output Encoding zur Verhinderung von Injection-Angriffen

**Security-Best-Practices:**
- Input-Validation ist fundamental requirement
- Alle Services validieren Inputs vor Verarbeitung

### Security-Features
- **Secure State Storage**: Verschlüsselte Speicherung von Device-State
- **Authentication Integration**: Integration mit Heimdall für sichere Authentication
- **Permission Checking**: Prüfung von Permissions für alle Actions
- **Input Validation**: Umfassende Validierung aller Inputs
- **Secure Communication**: Sichere Kommunikation mit allen Services
- **Audit Logging**: Logging aller Security-relevanten Events

### Security-Best-Practices
- **Keine Hardcoded Secrets**: Keine Hardcoded Passwords oder Keys
- **Secure Defaults**: Sichere Standard-Konfigurationen
- **Regular Updates**: Automatische Security-Updates
- **Vulnerability Scanning**: Regelmäßiges Scanning für bekannte Vulnerabilities
- **Fail-Safe**: Bei Fehlern wird sicherheitshalber verweigert (Deny statt Allow)

## Provider Selection

### Automatische Provider-Auswahl durch Odin

**User gibt Settings an**: Anspruch/Requirements in den Settings
**Odin wählt Provider**: Automatisch basierend auf Settings und aktuellen Provider-Angeboten
**Optimale Auswahl**: Odin berücksichtigt Preis, Qualität, Latency, Verfügbarkeit
**Transparenz**: User kann sehen, welcher Provider gewählt wurde (Marketplace als Übersicht)

### Zahlungsmethode erforderlich

**Als Consumer (Requester)**: Gültige Zahlungsmethode muss hinterlegt sein
- Für Bezahlung von Compute-Requests
- Automatische Abrechnung pro Request
- Pre-Authorization für schnelle Requests

**Als Provider**: Gültige Zahlungsmethode muss hinterlegt sein
- Für Auszahlung von Earnings
- Automatische Auszahlung (täglich/wöchentlich/monatlich)
- Steuerliche Dokumentation

**Beide Rollen**: User können gleichzeitig Consumer und Provider sein
- Separate Zahlungsmethoden möglich (aber nicht nötig)
- Netting möglich (Earnings gegen Costs)

### Marketplace als Übersicht

**Transparenz-Tool**: Zeigt alle verfügbaren Provider
**Vergleich**: Provider können verglichen werden
**Statistiken**: Provider-Statistiken (Qualität, Preis, Verfügbarkeit)
**Nicht für direkte Auswahl**: Direkte Provider-Auswahl ist möglich, aber nicht empfohlen

### User Settings

**Anspruch/Requirements (User konfiguriert)**
- **Quality Level**: Low (günstigste Option), Medium (Balance), High (beste Qualität), Custom (spezifische Anforderungen)
- **Max Cost**: Maximale Kosten pro Request
- **Max Latency**: Maximale Antwortzeit
- **Model Preferences**: Bevorzugte Modelle (optional)
- **Provider Preferences**: Bevorzugte Provider (optional, nicht empfohlen)

**Default Settings**
- **Empfohlene Einstellung**: "Medium" - Balance zwischen Preis und Qualität
- **Odin wählt optimal**: Basierend auf aktuellen Provider-Angeboten

### Odin's LLM-Selection mit Skuld

**1. Netzwerkplan-Erstellung**
- Odin erstellt Netzwerkplan (on the fly, mit Cache)
- Enthält: Verfügbare Devices, Models, Provider (wenn vorhanden), Capabilities, Quality-Metriken, Latency-Info, User-Preferences
- Cache-Strategie: Netzwerkplan wird gecacht, um wiederholte Erstellung zu vermeiden
- Cache-Invalidation: Bei Netzwerkplan-Updates, Device-Status-Änderungen, Quality-Metrik-Updates, Timeout
- **WICHTIG - Sicherheit**: Netzwerkplan darf unter keinen Umständen anderen Usern zugänglich gemacht werden
- **Yggdrasil-Übertragung**: Falls mit Yggdrasil verbunden: Netzwerkplan an Yggdrasil übertragen (nur für eigenen User/Netzwerk)

**2. Request-Analyse**
- Anforderung analysieren: Was wird benötigt?
- Context verstehen: Komplexität, Token-Schätzung
- Requirements extrahieren: Aus User-Settings

**3. Skuld fragt um Rat**
- Odin sendet `SelectionRequest` mit Netzwerkplan an Skuld
- Skuld analysiert Netzwerkplan und entscheidet:
  - Effektivster Weg (Routing, Latency)
  - Effektivstes Model (Quality, Performance)
  - Entspricht User-Vorgaben (Requirements, Preferences)
- Skuld antwortet mit `SelectionResponse` (Empfehlung für LLM/Device)

**4. Execution**
- Odin nutzt Skuld-Empfehlung für Request-Routing
- Request wird an gewähltes LLM/Device gesendet
- Monitoring: Überwachung der Ausführung
- Fallback: Bei Fehler automatisch zu alternativem LLM/Device

**5. Error Handling & Fallback**

**LLM-Auswahl-Prinzip:**
- **Beste verfügbare Model**: Bei Abfrage soll möglichst immer das beste verfügbare Model gewählt werden
- **Lokales LLM als letztes Fallback**: Lokales LLM ist immer das letzte Fallback (wird mit Installation mitgeliefert)
- **Sicherstellung**: Mit Installation einer Platform wird auch ein LLM mitgeliefert
- **Provider-Modelle**: Provider-Modelle können nur abgerufen werden, wenn User bei Yggdrasil registriert ist und mit Device/Netzwerk aktiv verbunden ist
- **Explizite Konfiguration**: Verbindung zu OpenAI etc. muss explizit in Konfiguration angegeben werden

**Kein passender Provider gefunden / Leere Provider-Liste**
- **Fallback-Strategie**: Hierarchische Fallback-Strategie
  1. **Bestes verfügbares LLM im Netzwerk**: Wenn Provider-Liste leer, wird bestes verfügbares LLM im Netzwerk verwendet
  2. **Lokales LLM**: Lokales LLM als letztes Fallback (wird bei Installation vom User gewählt)
  3. **Fehlermeldung**: Falls alles fehlschlägt, Fehlermeldung an User

**Provider-Ausfall während Ausführung**
- **Automatischer Retry**: Automatischer Retry mit anderem Provider (basierend auf Score)
- **Keine Entschädigung**: Ausgefallener Provider erhält keine Entschädigung für fehlgeschlagenen Request
- **User zahlt nicht**: User muss für fehlgeschlagenen Request nicht zahlen
- **Zahlung erst nach Erfolg**: Zahlung erfolgt erst nach erfolgreicher Antwort
- **Fallback zu lokalem LLM**: Wenn alle Provider ausfallen, Fallback zu lokalem LLM (letztes Fallback)

**Provider-Timeout**
- **Timeout-Handling**: Bei Timeout wird Request als fehlgeschlagen markiert
- **Automatischer Retry**: Retry mit anderem Provider
- **Fallback**: Falls alle Provider timeout, Fallback zu lokalem LLM (letztes Fallback)

**Edge Cases:**

**Alle Provider ausfallen:**
- **Fallback-Hierarchie**: 
  1. Bestes verfügbares LLM im Netzwerk (via Einherjar Protocol)
  2. Lokales LLM (garantiert verfügbar, da bei Installation installiert)
  3. Fehlermeldung an User (nur wenn auch lokales LLM nicht verfügbar - sollte nie passieren)
- **User-Benachrichtigung**: TTS-Meldung mit Begründung
- **Retry-Mechanismus**: System versucht kontinuierlich, Provider wieder zu erreichen

**Lokales LLM nicht verfügbar:**
- **Sollte nie passieren**: Lokales LLM wird bei Installation installiert und ist garantiert verfügbar
- **Fallback**: Falls doch nicht verfügbar (z.B. nach Deinstallation), System sucht im Netzwerk nach LLM
- **Fehlermeldung**: Falls auch kein Netzwerk-LLM verfügbar, Fehlermeldung an User
- **Installation-Erinnerung**: System erinnert User, lokales LLM zu installieren

**Provider-Preisschwankungen:**
- **Dynamische Preise**: Provider können Preise dynamisch ändern
- **Preis-Update**: Odin aktualisiert Preise regelmäßig (z.B. alle 5 Minuten)
- **Score-Neuberechnung**: Bei Preis-Änderungen wird Score neu berechnet
- **Provider-Wechsel**: Bei signifikanten Preis-Änderungen kann Provider gewechselt werden (nur bei neuen Requests)
- **Preis-Alerts**: User wird benachrichtigt bei signifikanten Preis-Änderungen (optional)

**Quality Metrics Update**
- **Kombination**: Sofort nach Request, aber auch periodische Aggregation
- **Sofort-Update**: Quality-Metriken werden sofort nach jedem Request aktualisiert
- **Periodische Aggregation**: Zusätzlich periodische Aggregation für langfristige Trends
- **Fair Distribution**: Quality-Metriken werden für Fair Distribution verwendet

### Scoring Algorithm

**Faktoren**
- **Preis** (30% Gewichtung): Niedriger Preis = höherer Score
- **Qualität** (25% Gewichtung): Höhere Qualität = höherer Score (basierend auf Quality-Metriken)
- **Latency** (20% Gewichtung): Niedrigere Latency = höherer Score
- **Verfügbarkeit** (15% Gewichtung): Höhere Verfügbarkeit = höherer Score
- **Fairness** (10% Gewichtung): Fair Distribution Score

**Scoring-Formel:**

```
score = (price_score * 0.30) + (quality_score * 0.25) + (latency_score * 0.20) + (availability_score * 0.15) + (fairness_score * 0.10)
```

**Normalisierung der Scores (0.0 - 1.0):**

**Price Score:**
```
price_score = 1.0 - (provider_price / max_price)
```
- `max_price`: Höchster Preis aller verfügbaren Provider
- Niedriger Preis = höherer Score (1.0 = günstigster, 0.0 = teuerster)

**Quality Score:**
```
quality_score = weighted_average_quality / 100.0
```
- `weighted_average_quality`: Gewichteter Durchschnitt der Quality-Metriken (0-100)
- Höhere Qualität = höherer Score

**Latency Score:**
```
latency_score = 1.0 - (provider_latency_ms / max_latency_ms)
```
- `max_latency_ms`: Maximale akzeptable Latency (z.B. 5000ms)
- Niedrigere Latency = höherer Score (1.0 = schnellster, 0.0 = langsamster)

**Availability Score:**
```
availability_score = provider_uptime_percentage / 100.0
```
- `uptime_percentage`: Verfügbarkeit in Prozent (0-100)
- Höhere Verfügbarkeit = höherer Score

**Fairness Score:**
```
fairness_score = 1.0 - (provider_usage_count / max_usage_count)
```
- `usage_count`: Anzahl der Requests für diesen Provider (letzte X Stunden)
- `max_usage_count`: Höchste Usage-Count aller Provider
- Niedrigere Usage = höherer Score (Fair Distribution)

**User-Settings-Anpassung:**

**Quality Level "Low":**
- Preis-Gewichtung: 50%
- Qualität-Gewichtung: 15%
- Andere Gewichtungen: Angepasst (Latency: 20%, Availability: 10%, Fairness: 5%)

**Quality Level "High":**
- Preis-Gewichtung: 15%
- Qualität-Gewichtung: 50%
- Andere Gewichtungen: Angepasst (Latency: 20%, Availability: 10%, Fairness: 5%)

**Quality Level "Medium":**
- Standard-Gewichtungen (wie oben)

**Quality-Metriken-Messung:**

**Automatische Bewertung (nach jedem Request):**
- **Latency**: Gemessen in Millisekunden (Response-Zeit)
- **Accuracy**: Automatische Bewertung basierend auf Response-Qualität (LLM-Response-Analyse)
- **Completeness**: Vollständigkeit der Response (Token-Count, Coverage)
- **Consistency**: Konsistenz mit vorherigen Responses

**User-Feedback (optional):**
- **Thumbs Up/Down**: Einfaches Feedback (optional)
- **Rating (1-5)**: Detailliertes Rating (optional)
- **Text-Feedback**: Freitext-Feedback (optional)

**Quality-Metrik-Berechnung:**
```
quality_metric = (latency_score * 0.20) + (accuracy_score * 0.40) + (completeness_score * 0.25) + (consistency_score * 0.15)
```

**Periodische Aggregation:**

**Gewichteter Durchschnitt (Time-Decay):**
```
weighted_average = Σ(quality_metric_i * weight_i) / Σ(weight_i)
weight_i = e^(-decay_rate * age_i)
```

- `decay_rate`: 0.1 (10% Decay pro Stunde)
- `age_i`: Alter des Requests in Stunden
- Neuere Requests haben exponentiell höheres Gewicht

**Aggregations-Intervall:**
- **Sofort-Update**: Nach jedem Request (für aktuellen Score)
- **Periodische Aggregation**: Stündlich (für langfristige Trends)
- **Rolling Window**: Letzte 24 Stunden werden berücksichtigt

**Yggdrasil-Kommunikation über Vedrfolnir**

- **Vedrfolnir-Integration**: Odin nutzt Vedrfolnir-Service für Yggdrasil-Kommunikation
- **Ratatoskr-Protocol**: Kommunikation über Ratatoskr-Protocol (Business-Logik)
- **Marketplace-Requests**: Provider-Registrierung, Provider-Abfrage, etc.
- **Payment-Requests**: Payment-Processing, Pre-Authorization, etc.
- **Business-Requests**: Allgemeine Business-Transaktionen

**User Settings Integration**
- **Quality Level "Low"**: Preis-Gewichtung erhöht (50%)
- **Quality Level "High"**: Qualität-Gewichtung erhöht (50%)
- **Quality Level "Medium"**: Ausgewogene Gewichtung
- **Max Cost**: Filtert Provider über Max Cost hinaus
- **Max Latency**: Filtert Provider über Max Latency hinaus

## Marketplace Quality Metrics

### Quality-Metric-Messung

**Kombination: Nach jedem Request + periodische Tests**

**Nach jedem Request**
- **Response-Qualität wird bewertet**: Nach jedem Request wird Response-Qualität automatisch bewertet
- **Sofortige Bewertung**: Bewertung erfolgt sofort nach Request-Abschluss
- **Kontinuierlich**: Quality-Metriken werden kontinuierlich aktualisiert

**Periodische Tests**
- **Regelmäßige Tests**: Provider werden regelmäßig getestet (z.B. täglich/wöchentlich)
- **Standardisierte Tests**: Standardisierte Test-Requests werden gesendet
- **Baseline**: Erstellt Baseline für Quality-Metriken
- **Ergänzend**: Ergänzt kontinuierliche Bewertung

### Quality-Metric-Aggregation

**Gewichteter Durchschnitt (neuere Requests haben höheres Gewicht)**

**Gewichtung basierend auf Zeit**
- **Neuere Requests**: Neuere Requests haben höheres Gewicht
- **Zeit-Decay**: Ältere Requests haben exponentiell abnehmendes Gewicht
- **Aktuell**: Aktuelle Performance wird stärker gewichtet

**Formel**
```
weighted_average = Σ(quality_score_i * weight_i) / Σ(weight_i)
weight_i = e^(-decay_rate * age_i)
```

### Quality-Bewertung nach jedem Request

**Kombination: Automatisch + optionales User-Feedback**

**Automatische Bewertung**
- **System bewertet**: System bewertet Response-Qualität automatisch
- **Metriken**: Verschiedene Metriken werden gemessen:
  - Antwortzeit (Latency)
  - Korrektheit (Accuracy)
  - Vollständigkeit (Completeness)
  - Konsistenz (Consistency)
- **Kontinuierlich**: Bewertung erfolgt bei jedem Request

**Optionales User-Feedback**
- **User kann Feedback abgeben**: User kann optional Feedback abgeben
- **Sofern abgegeben**: User-Feedback wird nur einbezogen, wenn User es abgibt
- **Nicht verpflichtend**: User muss kein Feedback abgeben
- **Zusätzliche Information**: User-Feedback ergänzt automatische Bewertung

### Quality-Metric-Updates

**Kombination: Sofort für wichtige Updates, Batch für Aggregation**

**Sofort für wichtige Updates**
- **Wichtige Änderungen**: Bei wichtigen Quality-Änderungen wird Metrik sofort aktualisiert
- **Signifikante Änderungen**: Bei signifikanten Änderungen (z.B. Quality-Drop) sofortige Aktualisierung
- **Alerts**: Bei wichtigen Änderungen werden Alerts gesendet

**Batch für Aggregation**
- **Periodische Aggregation**: Quality-Metriken werden periodisch aggregiert (z.B. stündlich/täglich)
- **Gewichteter Durchschnitt**: Gewichteter Durchschnitt wird berechnet
- **Effizient**: Reduziert Rechenaufwand

### Verwendung für Fair Distribution

**Kombination: Als Faktor + optionaler Filter**

**Als Faktor im Scoring-Algorithmus**
- **Quality-Gewichtung**: Quality-Metrik wird als Faktor im Scoring-Algorithmus verwendet
- **Höhere Quality = höherer Score**: Provider mit höherer Quality erhalten höheren Score
- **Standard**: Quality ist immer Teil des Scores

**Optionaler Filter**
- **Minimum-Quality**: User kann Minimum-Quality setzen
- **Filter**: Nur Provider über Minimum-Quality werden berücksichtigt
- **Optional**: Filter ist optional, nicht verpflichtend

## State Synchronisation

### State-Arten

**Alle State-Arten werden synchronisiert**
- **Device-State**: Status des Devices (online/offline, laufende Tasks, aktive Verbindungen)
- **Konfiguration-State**: User-Einstellungen, Präferenzen, Model-Auswahl
- **Action-State**: Status laufender Actions (welche Actions laufen, Fortschritt)
- **Network-State**: Verbundene Devices, Network-Topologie, Capabilities
- **Session-State**: Aktive Sessions, Token-Status, Authentifizierungs-Status

### State-Synchronisation

**Push-basiert (Sofortige Synchronisation)**

**Device sendet State-Updates sofort**
- **Bei Änderung**: Wenn State sich ändert, wird Update sofort gesendet
- **An alle verbundenen Devices**: State-Update wird an alle verbundenen Devices gesendet
- **Schnell**: Minimale Verzögerung
- **Konsistent**: Alle Devices haben aktuellen State

**Push-Mechanismus (Detailliert):**

**1. State-Änderung erkannt**
- Odin erkennt State-Änderung (Device-State, Konfiguration, Action-State, etc.)
- State-Update wird erstellt mit: `state_type`, `state_data`, `timestamp`, `device_id`, `priority`

**2. State-Update-Vorbereitung**
- State-Update wird serialisiert (JSON-Format)
- Update wird mit Device-Private-Key signiert
- Update wird mit Timestamp versehen

**3. State-Update-Versand**
- **Bifrost-Verbindungen**: Odin nutzt Bifrost, um State-Updates an verbundene Devices zu senden
- **Broadcast**: State-Update wird an alle verbundenen Devices gesendet (außer Jotunheim-Devices)
- **Reliable Delivery**: Bifrost garantiert zuverlässige Zustellung (Retry bei Fehler)

**4. State-Update-Empfang**
- Empfangendes Device validiert Signatur
- Device prüft Timestamp und Priority
- Device wendet State-Update an (siehe Conflict Resolution)

### Conflict Resolution

**Kombination: Timestamp + Priority**

**Timestamp-basiert**
- **Neuester State gewinnt**: State mit neuestem Timestamp gewinnt bei Konflikten
- **Chronologische Ordnung**: Garantiert chronologische Konsistenz
- **Standard**: Timestamp ist primäres Kriterium
- **Timestamp-Genauigkeit**: Millisekunden-Genauigkeit für bessere Konfliktlösung

**Priority-basiert**
- **Höhere Priorität gewinnt**: Bei gleichem Timestamp gewinnt höhere Priorität
- **Device-Priority**: Bestimmte Devices haben höhere Priorität (z.B. Asgard > Midgard > Alfheim)
  - **Asgard**: Priority 100 (höchste, da Homeserver)
  - **Midgard**: Priority 80 (Desktop)
  - **Alfheim**: Priority 60 (Mobile)
  - **Ragnarok**: Priority 70 (Terminal)
- **User-Priority**: User kann Priorität für bestimmte State-Updates setzen (0-100)
- **State-Type-Priority**: Bestimmte State-Types haben höhere Priorität
  - **Security-State**: Priority 90 (höchste)
  - **Configuration-State**: Priority 70
  - **Action-State**: Priority 50
  - **Session-State**: Priority 40

**Conflict-Resolution-Algorithmus:**
```
if (state1.timestamp > state2.timestamp):
    return state1
elif (state1.timestamp < state2.timestamp):
    return state2
else:  // Gleicher Timestamp
    if (state1.priority > state2.priority):
        return state1
    else:
        return state2
```

**Konflikt-Erkennung:**
- **State-Version**: Jeder State hat Versionsnummer
- **Konflikt-Detection**: System erkennt, wenn zwei Devices denselben State gleichzeitig ändern
- **Konflikt-Logging**: Konflikte werden geloggt für Debugging

### State-Persistenz

**Lokal auf jedem Device**

**Jedes Device speichert eigenen State**
- **Lokale Speicherung**: Jedes Device speichert State lokal (SQLite, etc.)
- **Unabhängig**: Jedes Device ist unabhängig
- **Schnell**: Keine Netzwerk-Latenz für lokale Zugriffe
- **Robust**: Funktioniert auch ohne Netzwerk

**Synchronisation**
- **Bei Verbindung**: State wird bei Verbindung synchronisiert
- **Bei Änderung**: State wird bei Änderung synchronisiert
- **Konsistenz**: Alle Devices haben konsistenten State

**State-Updates persistent machen:**
- **Transaction-basiert**: State-Updates werden als Transactions gespeichert
- **Atomic Writes**: State-Updates werden atomar geschrieben (ACID)
- **Write-Ahead-Logging**: WAL für bessere Performance
- **Sofortige Persistenz**: State-Updates werden sofort persistent gemacht (kein Buffering)

**Backup & Restore:**
- **Automatisches Backup**: State wird regelmäßig gesichert (z.B. täglich)
- **Backup-Speicherung**: Backups werden lokal gespeichert (verschlüsselt)
- **Restore-Mechanismus**: User kann State von Backup wiederherstellen
- **State-Export**: User kann State exportieren (für Migration zwischen Devices)

### State-Update-Propagation

**Selective (nur an relevante Devices)**

**Nur relevante Devices erhalten Updates**
- **Selective Propagation**: State-Updates werden nur an relevante Devices gesendet
- **Relevanz-Prüfung**: System prüft, welche Devices den State benötigen
- **Effizient**: Reduziert unnötige Netzwerk-Traffic
- **Jotunheim-Devices ausgeschlossen**: Jotunheim-Devices erhalten State-Updates nicht (brauchen sie nicht)

**Selective Propagation (Detailliert):**

**Relevanz-Prüfung:**
- **State-Type-Filter**: Bestimmte State-Types werden nur an bestimmte Devices gesendet
  - **Device-State**: Nur an Devices, die diesen State benötigen
  - **Configuration-State**: Nur an Devices, die diese Konfiguration nutzen
  - **Action-State**: Nur an Devices, die an dieser Action beteiligt sind
- **Device-Capabilities**: System prüft Device-Capabilities, ob Device State benötigt
- **User-Preferences**: User kann konfigurieren, welche Devices welche States erhalten

**Propagation-Strategie:**
- **Broadcast für wichtige States**: Security-States werden an alle Devices gesendet
- **Selective für spezifische States**: Action-States werden nur an beteiligte Devices gesendet
- **On-Demand**: Devices können State-Updates anfordern, wenn sie benötigt werden

**Propagation-Optimierung:**
- **Batching**: Mehrere State-Updates werden gebatcht, wenn möglich
- **Deduplizierung**: Duplizierte State-Updates werden entfernt
- **Compression**: State-Updates werden komprimiert für große States

## Implementierungs-Notizen

**Programmiersprache:**
- **Rust**: Für maximale Performance, Memory-Safety ohne GC, moderne Tooling, Cross-compilation
- **TypeScript nur im Frontend**: Nur GUI-Frontends (Midgard/Alfheim) nutzen TypeScript

**Technische Anforderungen:**
- Sollte als zentraler Event-Bus fungieren
- Muss thread-safe sein für parallele Requests
- Sollte Retry-Mechanismen für fehlgeschlagene Actions haben
- Muss Device-State persistent speichern können
- Muss alle Services koordinieren können
- Sollte robustes Error-Handling haben
- **Muss Skuld-Integration haben**: Nutzt Skuld für LLM-Auswahl (muss auf allen Devices installiert sein)
- **Muss Netzwerkplan-Erstellung haben**: Erstellt Netzwerkplan (on the fly, mit Cache)
- **Muss Vedrfolnir-Integration haben**: Nutzt Vedrfolnir für Yggdrasil-Kommunikation
- **Muss Ratatoskr-Protocol unterstützen**: Für Business-Logik-Kommunikation mit Yggdrasil
- **Muss Vision-Model-Support haben**: Nutzt Geri für Bild/Video-Interpretation
- **Muss Einherjar Protocol implementieren**: Als Consumer, fragt alle Götter nach Funktionen
- **Muss Responsibility Service nutzen**: Für Zuständigkeits-Management
- **Muss State Tracking haben**: Trackt aktuell zuständigen Gott
- **Muss Fallback-Mechanismus haben**: Bei Rückweisung wählt Odin alternative Götter
- **Muss Settings Management haben**: Verwaltung von User-Settings
- **Muss periodische Tests haben**: Für Quality-Metriken
- **Muss gewichteten Durchschnitt haben**: Neuere Requests höheres Gewicht
- **Muss automatische Bewertung haben**: Für Quality-Metriken
- **Muss optionales User-Feedback haben**: Sofern abgegeben
- **Muss sofortige Updates für wichtige Änderungen haben**: Quality-Metriken
- **Muss Batch-Aggregation haben**: Für Quality-Metriken
- **Muss Quality als Faktor im Scoring-Algorithmus haben**: Für Provider-Auswahl
- **Muss optionalen Quality-Filter haben**: Für User-Präferenzen
- **Muss Push-basierte State-Synchronisation haben**: Für Device-State
- **Muss Timestamp + Priority-basierte Conflict Resolution haben**: Für State-Konflikte
- **Muss lokale State-Persistenz haben**: Für Device-State
- **Muss Selective Propagation haben**: Nur an relevante Devices, nicht an Jotunheim-Devices
- **Performance**: Muss optimiert sein für schnelle Command-Processing und Service-Koordination
- **Datenschutz**: Muss Privacy-by-Design implementieren
- **Sicherheit**: Muss robuste Security-Mechanismen haben, insbesondere für State-Management

