# IMPLEMENTATION_PLAN - Frigg (Healthcare Plugin)

## Übersicht

Dieser Plan beschreibt die kleinstmöglichen Schritte zur Implementierung von Frigg - dem Healthcare Plugin für Odin. Frigg ist ausschließlich für persönliche und Gesundheitsfragen (mentale und körperliche) zuständig und bietet zertifizierte Kurse, die von Krankenkassen genehmigt und bezahlt werden können.

**Mythologische Bedeutung**: Frigg ist Göttin der Fürsorge und Mutterschaft.

**Programmiersprache**: Rust

**Plugin-Architektur**: Frigg ist ein optionales Plugin, das modular zu Odin hinzugefügt werden kann.

## Entschiedene Konfiguration

### Database-Wahl
✅ **ENTSCHEIDUNG**: PostgreSQL
**Begründung**: Enterprise-Grade, robust, verschlüsselbar, beste Compliance-Features für Healthcare-Daten

### Encryption-Library
✅ **ENTSCHEIDUNG**: ring
**Begründung**: Moderne Rust-Crypto, beste Performance, battle-tested, verwendet von Firefox/Cloudflare

### RAG-Integration
✅ **ENTSCHEIDUNG**: Freki-Code wiederverwenden mit separater Vector-Database
**Begründung**: Konsistenz im System, keine Code-Duplikation, robuste Integration

### Fulla-Service
✅ **ENTSCHEIDUNG**: Ja - Fulla als separater Service
**Begründung**: Bessere Trennung, modulare Architektur, klare Verantwortlichkeiten

### Certification-Storage
✅ **ENTSCHEIDUNG**: Beides kombiniert (gRPC für CRUD, Ratatoskr für Events)
**Begründung**: gRPC für Robustheit bei Transaktionen, Ratatoskr für Real-time-Benachrichtigungen

### Protobuf-Rust-Tool
✅ **ENTSCHEIDUNG**: prost + tonic
**Begründung**: Moderne Rust-Lösung, async-native, beste gRPC-Integration

---

## Phase 1: Projekt-Setup & Grundstruktur

### 1.1 Projekt-Initialisierung

**Abhängigkeiten**: Keine
**Erforderliche USER-Eingaben**: Database-Wahl, Fulla-Service-Entscheidung

#### 1.1.1 Cargo-Workspace erstellen
- [ ] `Cargo.toml` mit Workspace erstellen
  - `frigg/` (Main Healthcare Plugin)
  - `fulla/` (Optional Data Service - falls separater Service gewählt)
- [ ] Basis-Dependencies für Frigg definieren
  - Async Runtime (tokio)
  - gRPC (tonic, prost)
  - Database (sqlx mit PostgreSQL/SQLite)
  - Encryption (ring/rust-crypto/sodiumoxide)
  - Serialization (serde, serde_json)
  - Logging (tracing, tracing-subscriber)
  - Error-Handling (anyhow, thiserror)
- [ ] `.gitignore` erstellen

#### 1.1.2 Frigg Verzeichnisstruktur erstellen
- [ ] `frigg/src/main.rs` erstellen
- [ ] `frigg/src/lib.rs` erstellen
- [ ] `frigg/src/course/` für Course-Management erstellen
- [ ] `frigg/src/certification/` für Certification-Engine erstellen
- [ ] `frigg/src/insurance/` für Insurance-Integration erstellen
- [ ] `frigg/src/progress/` für Progress-Tracking erstellen
- [ ] `frigg/src/database/` für Database-Operations erstellen
- [ ] `frigg/src/encryption/` für Encryption-Operations erstellen
- [ ] `frigg/src/rag/` für eigenes RAG erstellen
- [ ] `frigg/src/plugin/` für Plugin-Interface erstellen
- [ ] `frigg/src/grpc/` für gRPC-Service erstellen
- [ ] `frigg/src/utils/` für Utilities erstellen
- [ ] `frigg/config/` für Konfigurationsdateien erstellen
- [ ] `frigg/tests/` für Tests erstellen

#### 1.1.3 Fulla Verzeichnisstruktur erstellen (Optional)
❓ **HINWEIS**: Nur wenn separater Fulla-Service gewählt wurde
- [ ] `fulla/src/main.rs` erstellen
- [ ] `fulla/src/lib.rs` erstellen
- [ ] `fulla/src/data/` für Datenbereitstellung erstellen
- [ ] `fulla/src/treatment_plans/` für Behandlungspläne erstellen
- [ ] `fulla/src/grpc/` für gRPC-Service erstellen
- [ ] `fulla/config/` für Konfigurationsdateien erstellen
- [ ] `fulla/tests/` für Tests erstellen

#### 1.1.4 Build-System einrichten
- [ ] Build-Scripts in `Cargo.toml` definieren
- [ ] Code-Generierungs-Pipeline einrichten (Protobuf → Rust)
- [ ] Cargo-Features definieren (z.B. `postgres`, `sqlite`, `with-fulla`)

### 1.2 Test-Infrastruktur

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.2.1 Container-Setup für Tests
- [ ] `Dockerfile` für Test-Umgebung erstellen
- [ ] Docker Compose für Test-Services konfigurieren
  - Database-Container (PostgreSQL/SQLite)
  - Mock-Odin-Service
  - Mock-Yggdrasil-Service
  - Vector-Database-Container (für Frigg-RAG)
  - Mock-Insurance-Provider-API
- [ ] Test-Container-Startup-Scripts erstellen
- [ ] **WICHTIG**: Alle Tests müssen in Containern laufen - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren

#### 1.2.2 Test-Framework konfigurieren
- [ ] Test-Dependencies hinzufügen (tokio-test, mockall, etc.)
- [ ] Test-Utilities und Helpers erstellen
- [ ] Mock-Setup für Services
- [ ] Test-Data-Generators für Healthcare-Data erstellen
- [ ] Encryption-Test-Utilities erstellen

#### 1.2.3 CI/CD-Pipeline
- [ ] GitHub Actions / GitLab CI Workflow erstellen
- [ ] Automatische Test-Ausführung bei Commits konfigurieren
- [ ] Code-Coverage-Reporting einrichten (cargo-tarpaulin)
- [ ] Security-Scanning integrieren
- [ ] Linting und Formatting (cargo clippy, cargo fmt)

### 1.3 Projekt-Konfiguration

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.3.1 Settings-System Design
- [ ] Settings-Schema definieren (JSON oder TOML)
- [ ] Settings-Struktur entwerfen (Frigg-spezifisch)
  - gRPC-Port
  - Database-Konfiguration (isolierte Healthcare-DB)
  - RAG-Konfiguration (eigenes RAG mit separater DB)
  - Encryption-Settings
  - Insurance-Integration-Settings
  - Certification-Settings
  - Chat-Leitung-Flags

#### 1.3.2 Settings-Validierung
- [ ] Rust-Structs für Settings definieren
- [ ] Tests für Settings-Validierung schreiben
- [ ] Settings-Validator implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 1.3.3 Settings-Loader
- [ ] Tests für Settings-Loader schreiben
- [ ] Settings-Loader implementieren (TDD)
- [ ] Hot-Reload-Mechanismus implementieren (TDD)
- [ ] Tests ausführen und bestehen

---

## Phase 2: Database Setup (Isolierte Healthcare-DB)

### 2.1 Database Configuration

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)
**Erforderliche USER-Eingaben**: Database-Wahl

#### 2.1.1 Database-Client Setup
- [ ] Database-Dependencies hinzufügen (sqlx)
- [ ] Tests für Database-Connection schreiben
- [ ] Database-Connection-Manager implementieren (TDD)
  - Connection-Pooling
  - Connection-Timeout
  - Connection-Retry
- [ ] Tests ausführen und bestehen

#### 2.1.2 Database-Migration System
- [ ] Migration-Framework konfigurieren (sqlx-cli)
- [ ] Initial-Migrations erstellen (siehe README.md Schema)
  - `users` Table
  - `course_progress` Table
  - `treatment_plans` Table
  - `health_records` Table
  - Alle Indizes erstellen
- [ ] Tests für Migrations schreiben
- [ ] Migration-Runner implementieren (TDD)
- [ ] Tests ausführen und bestehen

### 2.2 Database Models

**Abhängigkeiten**: 2.1 (Database Configuration)

#### 2.2.1 User Model
- [ ] Tests für User-Model schreiben
- [ ] `User` Struct definieren
- [ ] CRUD-Operations implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 2.2.2 Course-Progress Model
- [ ] Tests für Course-Progress-Model schreiben
- [ ] `CourseProgress` Struct definieren
- [ ] CRUD-Operations implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 2.2.3 Treatment-Plan Model
- [ ] Tests für Treatment-Plan-Model schreiben
- [ ] `TreatmentPlan` Struct definieren
- [ ] CRUD-Operations implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 2.2.4 Health-Record Model
- [ ] Tests für Health-Record-Model schreiben
- [ ] `HealthRecord` Struct definieren
- [ ] CRUD-Operations implementieren (TDD)
- [ ] Tests ausführen und bestehen

---

## Phase 3: Encryption Layer

### 3.1 Encryption Setup

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)
**Erforderliche USER-Eingaben**: Encryption-Library-Wahl

#### 3.1.1 Encryption Manager
- [ ] Tests für Encryption-Manager schreiben
- [ ] `EncryptionManager` implementieren (TDD)
  - AES-256-GCM Encryption
  - Decryption
  - Key-Management
- [ ] Tests ausführen und bestehen

#### 3.1.2 Key-Storage Integration
- [ ] Tests für Key-Storage schreiben
- [ ] `KeyStorage` implementieren (TDD)
  - Keys in OS-Secure-Storage speichern (Platform-spezifisch)
  - Key-Loading
  - Key-Rotation-Support
- [ ] Tests ausführen und bestehen

#### 3.1.3 Key-Rotation Manager
- [ ] Tests für Key-Rotation schreiben
- [ ] `KeyRotationManager` implementieren (TDD)
  - Automatische Key-Rotation (alle 90 Tage)
  - Re-Encryption von Daten mit neuem Key
  - Old-Key-Preservation für Backups
- [ ] Tests ausführen und bestehen

### 3.2 Column-Level Encryption

**Abhängigkeiten**: 3.1 (Encryption Setup), 2.2 (Database Models)

#### 3.2.1 Encrypted-Column Wrapper
- [ ] Tests für Encrypted-Column schreiben
- [ ] `EncryptedColumn<T>` implementieren (TDD)
  - Transparente Encryption bei Speicherung
  - Transparente Decryption beim Laden
  - Serialization/Deserialization
- [ ] Tests ausführen und bestehen

#### 3.2.2 Database-Model-Integration
- [ ] Tests für encrypted Database-Models schreiben
- [ ] Alle sensiblen Felder mit `EncryptedColumn<T>` wrappen
- [ ] Tests ausführen und bestehen

---

## Phase 4: Protobuf & gRPC Setup

### 4.1 Protobuf Definitions

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 4.1.1 Shared Protobuf-Projekt verwenden
❓ **HINWEIS**: Protobuf-Definitions sollten in einem separaten Projekt sein
- [ ] Frigg als Dependency zu Protobuf-Projekt hinzufügen
- [ ] Protobuf-Definitions importieren

#### 4.1.2 Plugin-Interface Protocol
- [ ] `OdinPlugin.proto` definieren (falls nicht vorhanden)
  - `GetTitle()` RPC
  - `GetDescription()` RPC
  - `GetFunctions()` RPC (Function Call Protocol)
- [ ] Code-Generierung konfigurieren

#### 4.1.3 Einherjar Protocol
- [ ] `EinherjarProtocol.proto` verwenden (falls nicht vorhanden)
  - `GetCapabilities()` RPC
  - `Capability` Message
  - `ResponsibilityDomain` Message
- [ ] Code-Generierung konfigurieren

#### 4.1.4 Responsibility Service Protocol
- [ ] `ResponsibilityService.proto` definieren (falls nicht vorhanden)
  - `TakeResponsibility()` RPC
  - `ReturnResponsibility()` RPC
  - `RejectResponsibility()` RPC
  - `ResponsibilityRequest` Message
  - `ResponsibilityResponse` Message
- [ ] Code-Generierung konfigurieren

#### 4.1.5 Healthcare Service Protocol
- [ ] `HealthcareService.proto` definieren
  - `ProcessHealthcareRequest()` RPC
  - `GetCourseProgress()` RPC
  - `GetTreatmentPlans()` RPC
  - Weitere Healthcare-RPCs
- [ ] Code-Generierung konfigurieren

### 4.2 gRPC Server Implementation

**Abhängigkeiten**: 4.1 (Protobuf Definitions)

#### 4.2.1 gRPC Server Setup
- [ ] Tests für gRPC-Server-Setup schreiben
- [ ] gRPC-Server-Setup implementieren (TDD)
  - tonic-Server konfigurieren
  - TLS-Support
  - Health-Check-Service
- [ ] Tests ausführen und bestehen

#### 4.2.2 Plugin-Interface Service
- [ ] Tests für Plugin-Interface-Service schreiben
- [ ] `OdinPluginServiceImpl` implementieren (TDD)
  - `GetTitle()` → "Frigg - Healthcare Plugin"
  - `GetDescription()` → Healthcare-Beschreibung
  - `GetFunctions()` → Liste verfügbarer Healthcare-Funktionen
- [ ] Tests ausführen und bestehen

#### 4.2.3 Einherjar Protocol Service
- [ ] Tests für Einherjar-Service schreiben
- [ ] `EinherjarProtocolServiceImpl` implementieren (TDD)
  - `GetCapabilities()` → Frigg's Capabilities
  - Responsibility-Domains definieren (Healthcare, Mental Health, Physical Health)
  - Responsibility-Keywords definieren
- [ ] Tests ausführen und bestehen

#### 4.2.4 Responsibility Service
- [ ] Tests für Responsibility-Service schreiben
- [ ] `ResponsibilityServiceImpl` implementieren (TDD)
  - `TakeResponsibility()` → Zuständigkeit übernehmen
  - `ReturnResponsibility()` → Zuständigkeit zurückgeben
  - `RejectResponsibility()` → Zuständigkeit zurückweisen
- [ ] Tests ausführen und bestehen

#### 4.2.5 Healthcare Service
- [ ] Tests für Healthcare-Service schreiben
- [ ] `HealthcareServiceImpl` implementieren (TDD)
  - `ProcessHealthcareRequest()` → Healthcare-Request verarbeiten
  - `GetCourseProgress()` → Course-Progress abrufen
  - `GetTreatmentPlans()` → Treatment-Plans abrufen
- [ ] Tests ausführen und bestehen

---

## Phase 5: Course Management System

### 5.1 Course-Model & Storage

**Abhängigkeiten**: 2.2 (Database Models), 3.2 (Column-Level Encryption)

#### 5.1.1 Course Model
- [ ] Tests für Course-Model schreiben
- [ ] `Course` Struct definieren
  - course_id, title, description, modules, etc.
- [ ] CRUD-Operations implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 5.1.2 Module Model
- [ ] Tests für Module-Model schreiben
- [ ] `Module` Struct definieren
  - module_id, title, content, quizzes, etc.
- [ ] CRUD-Operations implementieren (TDD)
- [ ] Tests ausführen und bestehen

### 5.2 Course-Content-Management

**Abhängigkeiten**: 5.1 (Course-Model & Storage)

#### 5.2.1 Content-Manager
- [ ] Tests für Content-Manager schreiben
- [ ] `ContentManager` implementieren (TDD)
  - Content erstellen
  - Content aktualisieren
  - Content-Versioning
- [ ] Tests ausführen und bestehen

#### 5.2.2 Quiz/Assessment-System
- [ ] Tests für Quiz-System schreiben
- [ ] `QuizManager` implementieren (TDD)
  - Quiz erstellen
  - Quiz-Fragen verwalten
  - Quiz-Antworten validieren
  - Quiz-Scores berechnen
- [ ] Tests ausführen und bestehen

---

## Phase 6: Progress Tracking

### 6.1 Progress-Tracking-System

**Abhängigkeiten**: 5.1 (Course-Model), 2.2 (Database Models)

#### 6.1.1 Progress-Tracker
- [ ] Tests für Progress-Tracker schreiben
- [ ] `ProgressTracker` implementieren (TDD)
  - User-Progress überwachen
  - Module-Completion tracken
  - Course-Completion tracken
  - Progress-Updates verarbeiten
- [ ] Tests ausführen und bestehen

#### 6.1.2 Completion-Manager
- [ ] Tests für Completion-Manager schreiben
- [ ] `CompletionManager` implementieren (TDD)
  - Module-Completion registrieren
  - Course-Completion registrieren
  - Completion-Validation
- [ ] Tests ausführen und bestehen

### 6.2 Analytics

**Abhängigkeiten**: 6.1 (Progress-Tracking)

#### 6.2.1 Analytics-Engine
- [ ] Tests für Analytics schreiben
- [ ] `AnalyticsEngine` implementieren (TDD)
  - Progress-Analytics
  - Completion-Rate-Analytics
  - Time-Spent-Analytics
- [ ] Tests ausführen und bestehen

---

## Phase 7: Certification Engine

### 7.1 Certification-Generation

**Abhängigkeiten**: 6.1 (Progress-Tracking), 5.1 (Course-Model)

#### 7.1.1 Certification-Generator
- [ ] Tests für Certification-Generator schreiben
- [ ] `CertificationGenerator` implementieren (TDD)
  - Certification erstellen (nach Course-Completion)
  - Certification-ID generieren
  - Certification-Data strukturieren
- [ ] Tests ausführen und bestehen

#### 7.1.2 Certification-Template-System
- [ ] Tests für Certificate-Templates schreiben
- [ ] `CertificateTemplateManager` implementieren (TDD)
  - Certificate-Templates verwalten
  - Template-Rendering (mit User-Daten)
- [ ] Tests ausführen und bestehen

### 7.2 Certification-Validation

**Abhängigkeiten**: 7.1 (Certification-Generation)

#### 7.2.1 Certification-Validator
- [ ] Tests für Certification-Validation schreiben
- [ ] `CertificationValidator` implementieren (TDD)
  - Certification-Authentizität validieren
  - Digital-Signature-Validation
  - Expiration-Check
- [ ] Tests ausführen und bestehen

### 7.3 Yggdrasil Certification-Storage Integration

**Abhängigkeiten**: 7.1 (Certification-Generation)
**Erforderliche USER-Eingaben**: Certification-Storage-Integration (gRPC/Ratatoskr)

#### 7.3.1 Yggdrasil Certification-Client
- [ ] Tests für Yggdrasil-Certification-Client schreiben
- [ ] `YggdrasilCertificationClient` implementieren (TDD)
  - Certification zu Yggdrasil hochladen
  - Certification von Yggdrasil abrufen
  - Certification-Synchronisation
- [ ] Tests ausführen und bestehen

---

## Phase 8: Insurance Integration

### 8.1 Yggdrasil Insurance-Integration

**Abhängigkeiten**: 4.2 (gRPC Server), 2.2 (Database Models)

#### 8.1.1 Yggdrasil Insurance-Client
- [ ] Tests für Yggdrasil-Insurance-Client schreiben
- [ ] `YggdrasilInsuranceClient` implementieren (TDD)
  - Freischaltcode-Validation via Yggdrasil
  - Plan-Activation-Request an Yggdrasil
  - Insurance-Provider-Response verarbeiten
- [ ] Tests ausführen und bestehen

### 8.2 Activation-Code-System

**Abhängigkeiten**: 8.1 (Yggdrasil Insurance-Integration)

#### 8.2.1 Activation-Code-Validator
- [ ] Tests für Activation-Code-Validation schreiben
- [ ] `ActivationCodeValidator` implementieren (TDD)
  - Activation-Code-Format validieren
  - Code an Yggdrasil zur Validation senden
  - Validation-Response verarbeiten
- [ ] Tests ausführen und bestehen

#### 8.2.2 Plan-Activation-Manager
- [ ] Tests für Plan-Activation schreiben
- [ ] `PlanActivationManager` implementieren (TDD)
  - Pläne nach Validation freischalten
  - Aktivierte Pläne verwalten
  - Aktivierungs-Status speichern
- [ ] Tests ausführen und bestehen

### 8.3 Insurance-Workflow

**Abhängigkeiten**: 8.2 (Activation-Code-System), 5.1 (Course-Model)

#### 8.3.1 Insurance-Workflow-Manager
- [ ] Tests für Insurance-Workflow schreiben
- [ ] `InsuranceWorkflowManager` implementieren (TDD)
  - Kompletter Freischaltungs-Workflow
  - User-Code-Eingabe → Yggdrasil-Validation → Plan-Activation
  - User-Benachrichtigung bei Erfolg/Fehler
- [ ] Tests ausführen und bestehen

---

## Phase 9: Frigg RAG Implementation

### 9.1 RAG-Setup (Eigenes RAG für Frigg)

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)
**Erforderliche USER-Eingaben**: RAG-Integration-Entscheidung

#### 9.1.1 Separate Vector-Database für Frigg-RAG
- [ ] Vector-Database-Container konfigurieren (separate Instance)
- [ ] Tests für Vector-DB-Connection schreiben
- [ ] Vector-DB-Client implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 9.1.2 RAG-Components (falls Freki-Code wiederverwendet)
❓ **HINWEIS**: Abhängig von USER-Antwort zur RAG-Integration
- [ ] Falls Option A: Freki-Code als Library importieren und konfigurieren
- [ ] Falls Option B: Eigene RAG-Implementierung (ähnlich wie Freki-Phasen)
- [ ] Falls Option C: Lightweight-RAG implementieren
- [ ] Tests für RAG-Integration schreiben und ausführen

### 9.2 Healthcare-Document-Indexing

**Abhängigkeiten**: 9.1 (RAG-Setup)

#### 9.2.1 Healthcare-Document-Parser
- [ ] Tests für Healthcare-Document-Parsing schreiben
- [ ] Healthcare-specific Document-Parser implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 9.2.2 Healthcare-Document-Indexing
- [ ] Tests für Healthcare-Document-Indexing schreiben
- [ ] Healthcare-Document-Indexing-Pipeline implementieren (TDD)
- [ ] Tests ausführen und bestehen

---

## Phase 10: Odin Integration (Plugin-Registration)

### 10.1 Plugin-Loading

**Abhängigkeiten**: 4.2 (gRPC Server Implementation)

#### 10.1.1 Plugin-Interface Implementation
- [ ] Tests für Plugin-Interface schreiben
- [ ] Plugin-Interface für Odin implementieren (TDD)
  - Plugin-Metadata bereitstellen
  - Plugin-Loading-Hooks
  - Plugin-Initialization
- [ ] Tests ausführen und bestehen

### 10.2 Responsibility-Management

**Abhängigkeiten**: 4.2.4 (Responsibility Service)

#### 10.2.1 Responsibility-Analyzer
- [ ] Tests für Responsibility-Analyzer schreiben
- [ ] `ResponsibilityAnalyzer` implementieren (TDD)
  - Requests analysieren (gesundheitsbezogen oder nicht)
  - Zuständigkeits-Entscheidung treffen
  - Keywords und Context-basierte Erkennung
- [ ] Tests ausführen und bestehen

#### 10.2.2 Responsibility-State-Manager
- [ ] Tests für Responsibility-State schreiben
- [ ] `ResponsibilityStateManager` implementieren (TDD)
  - Zuständigkeits-Status verwalten (active, returned, rejected)
  - Zuständigkeits-Historie tracken
- [ ] Tests ausführen und bestehen

### 10.3 Chat-Leitung

**Abhängigkeiten**: 10.2 (Responsibility-Management)

#### 10.3.1 Chat-Routing-Manager
- [ ] Tests für Chat-Routing schreiben
- [ ] `ChatRoutingManager` implementieren (TDD)
  - Automatische Chat-Weiterleitung von Odin zu Frigg
  - Explizite Frigg-Chat-Erstellung
  - Chat-Leitung-Flags in Settings
- [ ] Tests ausführen und bestehen

---

## Phase 11: Fulla Integration (Optional)

**Abhängigkeiten**: 1.1.3 (Fulla Verzeichnisstruktur) - falls gewählt

❓ **HINWEIS**: Diese Phase wird nur ausgeführt, wenn Fulla als separater Service gewählt wurde.

### 11.1 Fulla gRPC Service

#### 11.1.1 Fulla Service Implementation
- [ ] Tests für Fulla-Service schreiben
- [ ] `FullaServiceImpl` implementieren (TDD)
  - `GetHealthcareData()` RPC
  - `GetTreatmentPlan()` RPC
  - Data-Bereitstellung
- [ ] Tests ausführen und bestehen

### 11.2 Frigg ↔ Fulla Communication

**Abhängigkeiten**: 11.1 (Fulla Service)

#### 11.2.1 Fulla-Client (in Frigg)
- [ ] Tests für Fulla-Client schreiben
- [ ] `FullaClient` implementieren (TDD)
  - gRPC-Connection zu Fulla
  - Data-Requests an Fulla
- [ ] Tests ausführen und bestehen

---

## Phase 12: Security & Access Control

### 12.1 Access-Control-System

**Abhängigkeiten**: 3.1 (Encryption Layer), 2.2 (Database Models)

#### 12.1.1 Access-Control-Manager
- [ ] Tests für Access-Control schreiben
- [ ] `AccessControlManager` implementieren (TDD)
  - User-Identity-basierte Access-Control
  - Role-based Access-Control
  - Permission-Checks
- [ ] Tests ausführen und bestehen

#### 12.1.2 User-Authentication via Heimdall
- [ ] Tests für User-Authentication schreiben
- [ ] `AuthenticationManager` implementieren (TDD)
  - Integration mit Heimdall
  - Token-Validation
  - User-Identity-Verification
- [ ] Tests ausführen und bestehen

### 12.2 Audit-Logging

**Abhängigkeiten**: 2.2 (Database Models)

#### 12.2.1 Audit-Logger
- [ ] Tests für Audit-Logging schreiben
- [ ] `AuditLogger` implementieren (TDD)
  - Data-Access-Events loggen
  - Data-Modification-Events loggen
  - User-Authentication-Events loggen
  - Security-Events loggen
- [ ] Tests ausführen und bestehen

### 12.3 Threat-Detection

**Abhängigkeiten**: 12.2 (Audit-Logging)

#### 12.3.1 Anomaly-Detector
- [ ] Tests für Anomaly-Detection schreiben
- [ ] `AnomalyDetector` implementieren (TDD)
  - Verdächtige Zugriffe erkennen
  - Anomaly-Score berechnen
  - Security-Alerts auslösen
- [ ] Tests ausführen und bestehen

---

## Phase 13: GDPR Compliance Features

### 13.1 Right-to-Access

**Abhängigkeiten**: 2.2 (Database Models)

#### 13.1.1 Data-Access-Manager
- [ ] Tests für Data-Access schreiben
- [ ] `DataAccessManager` implementieren (TDD)
  - User-Daten abrufen (alle Gesundheitsdaten)
  - Strukturiertes Export-Format
- [ ] Tests ausführen und bestehen

### 13.2 Right-to-Deletion

**Abhängigkeiten**: 2.2 (Database Models), 3.1 (Encryption)

#### 13.2.1 Data-Deletion-Manager
- [ ] Tests für Data-Deletion schreiben
- [ ] `DataDeletionManager` implementieren (TDD)
  - Sichere Datenlöschung ("Right to be forgotten")
  - Cascade-Deletion
  - Backup-Cleanup
- [ ] Tests ausführen und bestehen

### 13.3 Right-to-Portability

**Abhängigkeiten**: 2.2 (Database Models)

#### 13.3.1 Data-Export-Manager
- [ ] Tests für Data-Export schreiben
- [ ] `DataExportManager` implementieren (TDD)
  - Gesundheitsdaten exportieren (JSON, CSV, PDF)
  - Export-Format standardisieren
  - Export-Encryption (optional)
- [ ] Tests ausführen und bestehen

### 13.4 Right-to-Rectification

**Abhängigkeiten**: 2.2 (Database Models)

#### 13.4.1 Data-Correction-Manager
- [ ] Tests für Data-Correction schreiben
- [ ] `DataCorrectionManager` implementieren (TDD)
  - User kann Daten korrigieren
  - Correction-History tracken
- [ ] Tests ausführen und bestehen

---

## Phase 14: Backup & Recovery

### 14.1 Backup-System

**Abhängigkeiten**: 2.1 (Database Configuration), 3.1 (Encryption)

#### 14.1.1 Backup-Manager
- [ ] Tests für Backup schreiben
- [ ] `BackupManager` implementieren (TDD)
  - Regelmäßige verschlüsselte Backups
  - Backup-Schedule
  - Backup-Rotation
- [ ] Tests ausführen und bestehen

### 14.2 Recovery-System

**Abhängigkeiten**: 14.1 (Backup-System)

#### 14.2.1 Recovery-Manager
- [ ] Tests für Recovery schreiben
- [ ] `RecoveryManager` implementieren (TDD)
  - Database-Restore aus Backup
  - Restore-Validation
  - Recovery-Testing
- [ ] Tests ausführen und bestehen

---

## Phase 15: Performance Optimization

### 15.1 Query-Optimization

**Abhängigkeiten**: 2.2 (Database Models)

#### 15.1.1 Query-Optimizer
- [ ] Performance-Tests für Queries schreiben
- [ ] Queries optimieren
  - User-spezifische Indizes nutzen
  - Composite-Indizes nutzen
  - Query-Plans analysieren
- [ ] Performance-Tests ausführen und Benchmarks erreichen (< 50ms)

### 15.2 Caching

**Abhängigkeiten**: 2.2 (Database Models)

#### 15.2.1 Cache-Manager
- [ ] Tests für Cache-Manager schreiben
- [ ] `CacheManager` implementieren (TDD)
  - Frequently-accessed Health-Data cachen
  - Cache-Invalidation bei Updates
  - TTL-basierte Expiration
- [ ] Tests ausführen und bestehen

---

## Phase 16: Monitoring & Logging

### 16.1 Structured Logging

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 16.1.1 Logging Setup
- [ ] Structured-Logging konfigurieren (tracing)
- [ ] Healthcare-specific Log-Levels
- [ ] GDPR-compliant Logging (keine sensitiven Daten in Logs)
- [ ] Log-Rotation konfigurieren

### 16.2 Performance Monitoring

**Abhängigkeiten**: 15.1 (Performance Optimization)

#### 16.2.1 Metrics Collector
- [ ] Tests für Metrics-Collector schreiben
- [ ] `MetricsCollector` implementieren (TDD)
  - Performance-Metriken sammeln
  - Database-Query-Performance tracken
  - Resource-Usage tracken
- [ ] Tests ausführen und bestehen

---

## Phase 17: Documentation

### 17.1 API Documentation

**Abhängigkeiten**: 4.2 (gRPC Server Implementation)

#### 17.1.1 gRPC Service Documentation
- [ ] gRPC-Service-Documentation erstellen
- [ ] Healthcare-API dokumentieren
- [ ] Insurance-Integration dokumentieren

### 17.2 Compliance Documentation

**Abhängigkeiten**: 13.1-13.4 (GDPR Features)

#### 17.2.1 GDPR-Compliance Documentation
- [ ] GDPR-Compliance-Features dokumentieren
- [ ] Privacy-Policy erstellen
- [ ] Data-Processing-Agreement erstellen

---

## Phase 18: Testing & Quality Assurance

### 18.1 Integration Testing

**Abhängigkeiten**: Alle vorherigen Phasen

#### 18.1.1 End-to-End Tests
- [ ] E2E-Tests für Healthcare-Workflows schreiben
  - Course-Enrollment → Progress → Completion → Certification
  - Activation-Code → Plan-Activation → Course-Access
- [ ] E2E-Tests ausführen und bestehen

### 18.2 Security Testing

**Abhängigkeiten**: 12.1 (Access-Control), 3.1 (Encryption)

#### 18.2.1 Security Test Suite
- [ ] Security-Tests ausführen
  - Encryption-Tests
  - Access-Control-Tests
  - Unauthorized-Access-Prevention (100% Coverage)
- [ ] Security-Tests bestehen

### 18.3 GDPR Compliance Testing

**Abhängigkeiten**: 13.1-13.4 (GDPR Features)

#### 18.3.1 GDPR Test Suite
- [ ] GDPR-Compliance-Tests ausführen
  - Right-to-Access-Tests
  - Right-to-Deletion-Tests
  - Right-to-Portability-Tests
  - Right-to-Rectification-Tests
- [ ] GDPR-Tests bestehen

---

## Zusammenfassung

**Gesamtanzahl Phasen**: 18
**Gesamtanzahl Schritte**: ~250+

**Kritische Abhängigkeiten**:
1. Database-Wahl (isolierte Healthcare-DB)
2. Encryption-Library-Wahl
3. RAG-Integration-Entscheidung
4. Fulla-Service-Entscheidung
5. Certification-Storage-Integration

**Offene Fragen für USER**:
1. Database-Wahl (PostgreSQL empfohlen, SQLite, Healthcare-DB)
2. Encryption-Library (ring, rust-crypto, sodiumoxide)
3. RAG-Integration (Freki wiederverwenden, eigene Impl., Lightweight)
4. Fulla als separater Service? (Ja, Nein, Später)
5. Certification-Storage-Integration (gRPC, Ratatoskr, Beides)

**Hinweise**:
- Alle Schritte folgen TDD
- Alle Tests in Containern
- Security ist kritisch: Encryption, Access-Control, Audit-Logging
- GDPR-Compliance ist erforderlich: Vollständige GDPR-Features
- Performance: < 50ms User-Data-Access
- Isolierte Database: Komplett getrennt für Healthcare-Daten
