# Analyse: Unklare Punkte in den Projekten

## Zusammenfassung

Nach Durchsicht aller Projekt-READMEs wurden folgende unklare oder unvollständige Aspekte identifiziert:

---

## 1. Technische Implementierungsdetails

### 1.1 Edda Core Library
- **Unklar**: Konkrete DTO-Strukturen sind nicht vollständig definiert
  - Welche Felder haben `RavenMessage`, `WolfRequest`, `ThorAction` genau?
  - Wie sehen die TypeScript-Interfaces aus?
  - Welche Validierungsregeln gelten für welche DTOs?

- **Unklar**: Protocol-Implementierungsdetails
  - Bifrost Protocol: Welche Message-Types genau? Wie sieht das JSON-Format aus?
  - Jötnar Protocol: Wie genau funktioniert MessagePack-Serialisierung?
  - Versionierung: Wie werden Protocol-Versionen gehandhabt?

### 1.2 Odin - Provider Selection
- **Unklar**: Konkreter Scoring-Algorithmus
  - Wie genau wird der Score berechnet? (Formel ist beschrieben, aber Implementierung fehlt)
  - Wie werden Quality-Metriken genau gemessen? (automatisch + User-Feedback - aber wie?)
  - Wie funktioniert die periodische Aggregation genau?

- **Unklar**: Fallback-Mechanismen
  - Was passiert genau, wenn kein Provider gefunden wird?
  - Wie funktioniert die Queue-Implementierung?
  - Wie wird das lokale LLM identifiziert und ausgewählt?

### 1.3 Thor - Conflict Resolution
- **Unklar**: Konkrete Lock-Implementierung
  - Wie genau funktioniert Distributed Locking über Asgard/Yggdrasil?
  - Welche Lock-Types gibt es? (Read/Write Locks?)
  - Wie wird Deadlock-Detection implementiert?

- **Unklar**: Priority-System
  - Wie genau wird System-Priority berechnet?
  - Wie funktioniert User-Override genau?
  - Was passiert bei Priority-Konflikten?

### 1.4 Bifrost - Connection/Authentication Protocol
- **Unklar**: Konkrete Implementierung
  - Wie genau funktioniert das Challenge-Response-Protokoll?
  - Wie werden Public/Private Keys generiert und gespeichert?
  - Wie funktioniert Token-Management genau? (Refresh-Token, etc.)

- **Unklar**: Gast-Netzwerk-Implementierung
  - Wie genau wird Isolation technisch umgesetzt?
  - Wie funktioniert Datentransfer-Erlaubnis genau?
  - Wie wird mehrfache Bestätigung für expliziten Zugang implementiert?

### 1.5 Heimdall - Token Management
- **Unklar**: Konkrete Token-Struktur
  - Was genau ist in einem HeimdallToken enthalten?
  - Wie funktioniert Token-Rotation genau?
  - Wie wird Token-Leak Detection implementiert?

### 1.6 Freki - RAG Service
- **Unklar**: Embedding-Model-Auswahl
  - Wie wird das "einheitliche Model" standardmäßig gewählt?
  - Wie funktioniert typ-spezifische Model-Konfiguration genau?
  - Wie werden Embeddings gecacht?

- **Unklar**: Chunking-Implementierung
  - Wie genau funktioniert semantisches Chunking? (Welche Bibliothek/Methode?)
  - Wie wird Overlap implementiert?
  - Wie werden Dokument-Updates genau gehandhabt?

### 1.7 Geri - LLM Service
- **Unklar**: Multi-Faktor-Bewertung
  - Wie genau werden die verschiedenen Faktoren gewichtet?
  - Wie wird "Effizienz" gemessen?
  - Wie funktioniert Load-Balancing genau?

- **Unklar**: Cloud-LLM Fallback
  - Wie wird das "stärkste lokale LLM" identifiziert?
  - Wie funktioniert die Suche im Netzwerk nach LLMs?
  - Wie wird TTS-Benachrichtigung implementiert?

### 1.8 Valkyries - Coding Agent
- **Unklar**: Statement-System
  - Wie genau sehen Statements aus? (Format, Struktur)
  - Wie sammelt Brünhild Statements?
  - Wie funktioniert Vollständigkeitsprüfung genau?

- **Unklar**: Multi-Instance-Orchestration
  - Wie werden mehrere Instanzen einer Valkyrie koordiniert?
  - Wie werden Ressourcen zwischen Instanzen verwaltet?
  - Wie werden Konflikte zwischen Instanzen gelöst?

- **Unklar**: Queue-System-Implementierung
  - Wie funktioniert Queue zwischen Thor und Brünhild?
  - Wie funktioniert interne Queue zwischen Brünhild und Valkyries?
  - Wie wird Queue-Priorisierung gehandhabt?
  - Wie werden Queue-Failures und Retries gehandhabt?

### 1.9 Jötnar - IoT Devices
- **Unklar**: Toolcalling-Protocol-Implementierung
  - Wie genau funktioniert MessagePack-Serialisierung?
  - Wie wird Streaming implementiert?
  - Wie funktioniert Capability-Negotiation genau?

- **Unklar**: OTA-Update-Mechanismus
  - Wie genau funktioniert Update-Verteilung über Asgard/Yggdrasil?
  - Wie wird Update-Verifikation implementiert?
  - Wie funktioniert Rollback genau?

---

## 2. Integrationen zwischen Services

### 2.1 Odin ↔ Thor ↔ Valkyries/Frigg
- **Unklar**: Queue-System-Implementierung
  - Wie genau funktioniert Queue-basierte Kommunikation?
  - Welche Queue-Types gibt es? (Task-Queue, Ergebnis-Queue)
  - Wie wird Queue-Management implementiert? (Enqueue, Dequeue, Status-Tracking)
  - Wie funktioniert Priorisierung in Queues?
  - Wie werden Queue-Failures gehandhabt?

### 2.2 Odin ↔ Geri ↔ Freki
- **Unklar**: RAG-Context-Integration
  - Wie genau wird RAG-Context in LLM-Prompt eingefügt?
  - Wie wird Context-Window-Management gehandhabt?
  - Was passiert, wenn RAG-Context zu groß ist?

### 2.3 Bifrost ↔ Heimdall
- **Unklar**: Connection-Validation-Workflow
  - Wie genau funktioniert die Validierung?
  - Wie wird Connection-Status überwacht?
  - Wie funktioniert Connection-Blocking?

### 2.4 Asgard ↔ Jötnar
- **Unklar**: Capability-Synchronisation
  - Wie genau funktioniert die zentrale Synchronisation über Asgard?
  - Wie wird der "leitende Server" bei mehreren Asgard-Servern bestimmt?
  - Wie werden Capability-Updates propagiert?

---

## 3. Konfiguration und Settings

### 3.1 User Settings (Odin)
- **Unklar**: Konkrete Settings-Struktur
  - Wie sehen Settings genau aus? (JSON-Struktur?)
  - Wie werden Settings gespeichert und synchronisiert?
  - Wie werden Settings zwischen Devices synchronisiert?

### 3.2 Model-Konfiguration (Geri)
- **Unklar**: Konfigurationsformat
  - Wie werden Model-Konfigurationen gespeichert?
  - Wie funktioniert Konfiguration vom verbundenen Desktop/Server?
  - Wie werden API-Keys sicher gespeichert?

### 3.3 Valkyries LLM-Konfiguration
- **Unklar**: Konfigurationsformat
  - Wie sieht die Konfigurationsdatei genau aus?
  - Wie wird Konfiguration pro Valkyrie gespeichert?
  - Wie funktioniert Konfiguration außerhalb von Ragnarok?

---

## 4. Workflows und Edge Cases

### 4.1 Provider Selection (Odin)
- **Unklar**: Edge Cases
  - Was passiert, wenn alle Provider ausfallen?
  - Was passiert, wenn lokales LLM nicht verfügbar ist?
  - Wie wird mit Provider-Preisschwankungen umgegangen?

### 4.2 State Synchronisation (Odin)
- **Unklar**: Konkrete Implementierung
  - Wie genau funktioniert Push-basierte Synchronisation?
  - Wie werden State-Konflikte gelöst? (Timestamp + Priority - aber wie genau?)
  - Wie funktioniert Selective Propagation genau?

### 4.3 Error Recovery (Thor, Bifrost)
- **Unklar**: Konkrete Retry-Logik
  - Wie genau funktioniert Exponential Backoff?
  - Wie werden Retry-Limits gesetzt?
  - Wie funktioniert Fallback-Routing genau?

### 4.4 Network Expansion (Phase 4)
- **Unklar**: NAT-Traversal-Implementierung
  - Wie genau funktioniert STUN/TURN/ICE?
  - Welche Bibliotheken werden verwendet?
  - Wie funktioniert Fallback auf manuelle Konfiguration?

---

## 5. Datenbank und Persistenz

### 5.1 Asgard Database Schema
- **Unklar**: Vollständiges Schema
  - Welche Tabellen genau?
  - Welche Indizes?
  - Wie werden Migrations gehandhabt?

### 5.2 Yggdrasil Database Schema
- **Unklar**: Vollständiges Schema
  - Marketplace-Tabellen: Wie genau?
  - Transaction-Tabellen: Wie genau?
  - Wie werden Quality-Metriken gespeichert?

### 5.3 Frigg Healthcare Database
- **Unklar**: Vollständiges Schema
  - Welche Tabellen genau?
  - Wie wird Verschlüsselung implementiert?
  - Wie funktioniert schneller User-Datenzugriff genau?

### 5.4 Lokale State-Persistenz (Odin)
- **Unklar**: Implementierung
  - Wie wird State lokal gespeichert? (SQLite? JSON?)
  - Wie funktioniert Backup & Restore?
  - Wie werden State-Updates persistent gemacht?

---

## 6. API-Endpunkte

### 6.1 Yggdrasil API
- **Unklar**: Vollständige API-Spezifikation
  - Welche Request/Response-Formate genau?
  - Welche Authentifizierung?
  - Welche Rate-Limits?

### 6.2 Asgard API
- **Unklar**: Vollständige API-Spezifikation
  - Welche Endpunkte genau?
  - Wie funktioniert Web-Dashboard-Integration?
  - Wie funktioniert API-Authentifizierung?

---

## 7. Performance und Skalierung

### 7.1 Performance-Metriken
- **Unklar**: Konkrete Zielwerte
  - Sind die angegebenen Metriken (z.B. < 100ms) realistisch?
  - Wie werden Metriken gemessen?
  - Welche Monitoring-Tools werden verwendet?

### 7.2 Skalierung
- **Unklar**: Skalierungsstrategien
  - Wie skaliert Yggdrasil genau? (Horizontal Scaling - aber wie?)
  - Wie funktioniert Database Sharding?
  - Wie funktioniert Load Balancing genau?

---

## 8. Sicherheit

### 8.1 Verschlüsselung
- **Unklar**: Konkrete Implementierung
  - Welche Verschlüsselungsalgorithmen?
  - Wie werden Keys generiert und gespeichert?
  - Wie funktioniert Key-Rotation?

### 8.2 Authentication/Authorization
- **Unklar**: Konkrete Implementierung
  - Wie funktioniert OAuth-Integration genau?
  - Wie funktioniert Email/Code-Verifizierung?
  - Wie funktioniert RBAC genau?

### 8.3 Network Isolation (Yggdrasil)
- **Unklar**: Technische Umsetzung
  - Wie wird VPC-Isolation implementiert?
  - Wie funktioniert Kubernetes Network Policies?
  - Wie wird Cross-Network-Zugriff verhindert?

---

## 9. Deployment und Infrastructure

### 9.1 Deployment-Strategien
- **Unklar**: Konkrete Deployment-Pipelines
  - Wie werden Updates deployed?
  - Wie funktioniert Rollback?
  - Wie werden Zero-Downtime-Deployments gehandhabt?

### 9.2 Monitoring und Logging
- **Unklar**: Konkrete Tools
  - Welche Monitoring-Tools werden verwendet?
  - Wie funktioniert Centralized Logging?
  - Wie werden Alerts konfiguriert?

---

## 10. Testing

### 10.1 Test-Strategien
- **Unklar**: Test-Abdeckung
  - Welche Test-Types werden verwendet? (Unit, Integration, E2E?)
  - Wie werden Tests für verteilte Systeme geschrieben?
  - Wie werden Edge Cases getestet?

---

## 11. Dokumentation

### 11.1 API-Dokumentation
- **Unklar**: Dokumentationsformat
  - OpenAPI/Swagger?
  - Wie werden API-Dokumentationen generiert?
  - Wo werden Dokumentationen gehostet?

### 11.2 User-Dokumentation
- **Unklar**: Dokumentationsstruktur
  - Welche Dokumentations-Tools?
  - Wie werden Tutorials strukturiert?
  - Wie werden Video-Tutorials integriert?

---

## 12. Business-Logik

### 12.1 Marketplace (Yggdrasil)
- **Unklar**: Konkrete Implementierung
  - Wie funktioniert Fair Distribution Algorithm genau?
  - Wie werden Quality-Metriken aggregiert?
  - Wie funktioniert Transaction-Settlement genau?

### 12.2 Subscription System (Yggdrasil)
- **Unklar**: Konkrete Implementierung
  - Wie funktioniert Token-Limit-Enforcement?
  - Wie funktioniert monatliches Reset?
  - Wie funktioniert automatischer Fallback?

### 12.3 Payment Integration (Yggdrasil)
- **Unklar**: Konkrete Implementierung
  - Wie funktioniert Pre-Authorization?
  - Wie funktioniert Payout-Processing?
  - Wie funktioniert Netting?

---

## Priorisierung

### Hoch (Kritisch für MVP)
1. Edda Core Library DTOs und Protocols
2. Odin Provider Selection Algorithm
3. Bifrost Connection/Authentication Protocol
4. Thor Conflict Resolution
5. State Synchronisation (Odin)

### Mittel (Wichtig für Phase 1-2)
6. Freki RAG Chunking und Embedding
7. Geri Multi-Faktor-Bewertung
8. Heimdall Token Management
9. Asgard Database Schema
10. Queue-System zwischen Services (Thor ↔ Valkyries/Frigg)

### Niedrig (Kann später geklärt werden)
11. Performance-Monitoring-Tools
12. Deployment-Pipelines
13. API-Dokumentationsformat
14. Test-Strategien
15. User-Dokumentationsstruktur

---

## Empfehlungen

1. **Schnell klären**: DTOs und Protocols (Edda Core) - diese sind Basis für alles andere
2. **Detaillieren**: Provider Selection Algorithm - kritisch für Marketplace
3. **Spezifizieren**: Database Schemas - wichtig für Persistenz
4. **Definieren**: Event-System - wichtig für Service-Integration
5. **Dokumentieren**: API-Endpunkte - wichtig für Integration

---

## Nächste Schritte

1. DTOs und Protocols in Edda Core Library vollständig definieren
2. Provider Selection Algorithm detailliert spezifizieren
3. Database Schemas für alle Services definieren
4. Queue-System zwischen Services spezifizieren (Thor ↔ Valkyries/Frigg)
5. API-Endpunkte vollständig dokumentieren
6. Konfigurationsformate definieren
7. Error-Handling-Strategien detaillieren
8. Performance-Metriken konkretisieren

