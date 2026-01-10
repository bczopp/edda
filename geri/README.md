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
- Model Configuration Requests

### Outputs
- `WolfResponse` mit generiertem Text, Tokens, Latency, Cost

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

3. **Prompt Formatting**
   - Formatiert Prompt für gewähltes Model
   - Fügt System-Prompts hinzu
   - Verwaltet Context Window

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

## Technische Anforderungen

### Model Management
- Model Registry
- Model Versioning
- Model Health Monitoring
- Automatic Fallback

### Prompt Engineering
- System Prompt Templates
- Few-Shot Examples
- Prompt Optimization
- Context Window Management

### Cost Management
- Token Counting
- Cost Calculation
- Budget Limits
- Cost Tracking per Request

### Performance
- Streaming Support
- Parallel Requests
- Request Queuing
- Caching für ähnliche Prompts

## Cloud LLM Fallback zu Local LLMs

### Automatischer Fallback
- **Limit erreicht**: Wenn Cloud LLM Limit erreicht ist, automatischer Fallback zu lokalem LLM
- **Stärkstes lokales LLM**: Bestes verfügbares lokales LLM wird identifiziert (Multi-Faktor-Bewertung)
- **Im Netzwerk suchen**: System sucht auch im Netzwerk nach besten LLM (z.B. Smartphone nutzt Desktop-LLM)
- **User-Benachrichtigung**: TTS-Meldung mit Begründung ("Ich nutze jetzt lokales Modell, da Cloud-Limit erreicht")

### Lokales LLM ist Standard
- **Jedes Device muss lokales LLM haben**: Beim Installieren wird lokales LLM installiert und lauffähig gemacht
- **Fallback darf nicht fehlschlagen**: Es darf nicht passieren, dass kein lokales LLM verfügbar ist
- **Default**: Lokales LLM ist der Standard-Fallback, wenn nichts anderes geht

### Automatische Rückkehr zu Cloud LLM
- **Monatliches Reset**: Kontingent wird monatlich zurückgesetzt (exakt nach einem Monat ab Vertragsabschluss)
- **Automatisch auf Yggdrasil**: Reset erfolgt automatisch auf Yggdrasil
- **Nur bei aktiver Zahlung**: Reset nur, solange User noch dafür zahlt
- **Automatische Rückkehr**: Nach Reset automatische Rückkehr zu Cloud LLM (wenn verfügbar)

## Abhängigkeiten

- **Odin**: Für Requests
- **Freki**: Für RAG-Context (optional)
- **Model Providers**: Lokal oder Cloud
- **Hardware Resources**: Für lokale Models
- **Edda Core Library**: DTOs (WolfRequest, WolfResponse)

## Integration

- **Odin**: Empfängt `WolfRequest` von Odin, sendet `WolfResponse` zurück
- **Freki**: Verwendet RAG-Context von Freki (optional)
- **Midgard**: Lokale Models mit voller Hardware-Nutzung
- **Alfheim**: Cloud-First Approach, lokale Models optional
- **Asgard**: Lokale Models mit erweiterten Features
- **Yggdrasil**: Für Subscription-Management und Cloud-LLM-Limits

## Datenschutz

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
- **Performance**: Muss optimiert sein für schnelle LLM-Responses und effizientes Context-Management
- **Datenschutz**: Muss Privacy-by-Design implementieren und Prompt-Privacy gewährleisten
- **Sicherheit**: Muss robuste Security-Mechanismen haben, insbesondere für Prompt-Verarbeitung

