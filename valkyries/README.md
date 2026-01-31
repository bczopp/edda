# Valkyries - Coding Agent Plugin

## Übersicht

Valkyries ist ein Plugin für Coding-Aufgaben, das von Odin orchestriert wird. Wenn Coding-Aufgaben erkannt werden, delegiert Odin die Aufgabe an Valkyries (wenn verfügbar). Brünhild führt die Valkyries an und koordiniert Sub-Agents für verschiedene Aufgaben.

**Plugin-Architektur**: Valkyries ist ein optionales Plugin, das modular zur Odin-Installation hinzugefügt werden kann. Odin entscheidet selbst, ob eine Aufgabe an Valkyries delegiert werden muss (bei Coding-Aufgaben) oder ob Odin selbst antworten kann (bei einfachen Fragen).

**Plugin-Interface**: Valkyries implementiert das `OdinPlugin`-Interface mit:
- `get_title()`: Gibt "Valkyries - Coding Agent" zurück
- `get_description()`: Beschreibt die Coding-Agent-Funktionalität
- `get_functions()`: Gibt alle verfügbaren Coding-Funktionen zurück (Function Call Protocol)

**Einherjar Protocol**: Valkyries implementiert das Einherjar Protocol, um ihre verfügbaren Funktionen und Zuständigkeits-Domains offenzulegen. Odin nutzt diese Informationen, um automatisch zu erkennen, wann Valkyries zuständig ist.

**Responsibility Service**: Valkyries implementiert das Responsibility Service, um Zuständigkeit zu übernehmen, zurückzugeben oder zurückzuweisen. Wenn eine Aufgabe nicht mehr Coding-Aufgaben sind, gibt Valkyries die Zuständigkeit an Odin zurück.

### Valkyries-Struktur

**Gesamt: 12 spezialisierte Valkyries + Brünnhilde (Lead Agent) = 13 Agents**

Jede Valkyrie hat einen engen, spezialisierten Fokus, um das Context-Fenster klein zu halten. Die Namen und Aufgaben entsprechen ihren mythologischen Rollen:

1. **Brünnhilde (Brynhildr)** - "Kampfpanzerin/Schutzkämpferin" → Lead Agent (Orchestrierung, Task-Decomposition, Quality Assurance, Schutz des Projekts)
2. **Gunnr** - "Krieg/Schlacht" → Test Agent (Kampf gegen Bugs, Qualitätssicherung)
3. **Hildr** - "Kampf" → Security Agent (Kampf gegen Angriffe, Sicherheitsprotokolle)
4. **Skögul** - "die Rüttelnde" → Performance Agent (System optimieren, Performance erschüttern)
5. **Hrist** - "die Erschütternde" → Refactoring Agent (Code erschüttern/verbessern, Code-Qualität)
6. **Mist** - "Nebel" → DevOps Agent (Cloud-Infrastructure, verschleierte Deployment-Umgebungen)
7. **Skeggjöld** - "Axt-Zeit" → Configuration Agent (Schnelle Konfiguration, Build-Zeit-Optimierung)
8. **Göll** - "Tumult/Kampf" → Frontend Agent (Dynamische UI, interaktive Komponenten)
9. **Geirskögul** - "Speerträgerin" → Database Agent (Durchdringende Datenbankarbeit, präzise Queries)
10. **Þrúðr (Thrúd)** - "Stärke" → Backend Agent (Robuste Backend-Logik, starke Services)
11. **Hlökk** - "Lärm/Kampfgetöse" → Documentation Agent (Kommunikation, laute Dokumentation)
12. **Róta** - "Sturm/Aufruhr" → API Design Agent (Echtzeit-Systeme, Event-Architekturen, stürmische APIs)
13. **Sigrún** - "Geheimnis des Sieges" → Performance/Optimization Agent (Algorithmus-Optimierung, Erfolgsgeheimnisse)

**Prinzip**: Jede Valkyrie arbeitet nur mit ihrem spezialisierten Bereich, um Context-Fenster klein und fokussiert zu halten. Die mythologischen Namen spiegeln ihre Aufgaben wider.

### Schnellübersicht: Alle Valkyries und ihre Aufgaben

| Valkyrie | Mythologische Bedeutung | Software-Aufgabe | Spezialisierung |
|----------|------------------------|------------------|-----------------|
| **Brünnhilde** | "Kampfpanzerin" | Lead Agent | Orchestrierung, Task-Decomposition, Quality Assurance |
| **Gunnr** | "Krieg/Schlacht" | Test Agent | Unit/Integration/E2E-Tests, Qualitätssicherung |
| **Hildr** | "Kampf" | Security Agent | Security-Reviews, Vulnerability-Scanning, Verteidigung |
| **Skögul** | "die Rüttelnde" | Performance Agent | Performance-Analyse, Optimierung, Profiling |
| **Hrist** | "die Erschütternde" | Refactoring Agent | Code-Qualität, Clean Code, Transformation |
| **Mist** | "Nebel" | DevOps Agent | Docker, CI/CD, Cloud-Infrastructure |
| **Skeggjöld** | "Axt-Zeit" | Configuration Agent | Config-Files, Environment-Setup, Build-Config |
| **Göll** | "Tumult/Kampf" | Frontend Agent | UI-Komponenten, Styling, State-Management |
| **Geirskögul** | "Speerträgerin" | Database Agent | Schema-Design, Migrations, Query-Optimierung |
| **Þrúðr** | "Stärke" | Backend Agent | APIs, Business-Logic, Services |
| **Hlökk** | "Lärm/Kampfgetöse" | Documentation Agent | Code-Docs, API-Docs, README |
| **Róta** | "Sturm/Aufruhr" | API Design Agent | API-Design, Contracts, Echtzeit-Systeme |
| **Sigrún** | "Geheimnis des Sieges" | Optimization Agent | Algorithmus-Optimierung, Erfolgsgeheimnisse |

## Projektstruktur

```
valkyries/
├── src/
│   ├── cli/            # CLI Interface
│   │   ├── main.ts
│   │   ├── commands/
│   │   └── prompts/
│   ├── agents/         # Agent Implementations
│   │   ├── brünnhilde/ # Brünnhilde - Lead Agent
│   │   ├── gunnr/      # Gunnr - Test Agent
│   │   ├── hildr/      # Hildr - Security Agent
│   │   ├── skögul/     # Skögul - Performance Agent
│   │   ├── hrist/      # Hrist - Refactoring Agent
│   │   ├── mist/       # Mist - DevOps Agent
│   │   ├── skeggjöld/  # Skeggjöld - Configuration Agent
│   │   ├── göll/       # Göll - Frontend Agent
│   │   ├── geirskögul/ # Geirskögul - Database Agent
│   │   ├── thrúd/      # Þrúðr - Backend Agent
│   │   ├── hlökk/      # Hlökk - Documentation Agent
│   │   ├── róta/       # Róta - API Design Agent
│   │   └── sigrún/     # Sigrún - Optimization Agent
│   ├── services/       # Services
│   │   ├── git/
│   │   ├── llm/
│   │   ├── file/
│   │   └── execution/
│   └── utils/
├── config/
└── tests/
```

## Komponenten

### Brünnhilde (Brynhildr) - Lead Coding Agent

**Mythologische Bedeutung**: "Kampfpanzerin" oder "Schutzkämpferin" - Die mächtigste und bekannteste Walküre, die für Schutz und strategische Führung steht.

#### Spezialisierte Verantwortlichkeiten

**1. Task Analysis & Decomposition**
- **Task-Analyse**: 
  - **Komplexitäts-Analyse**: Analysiert eingehende Tasks auf Komplexität, Abhängigkeiten und Anforderungen
  - **Decomposition-Algorithmen**: Graph-basierte Decomposition-Algorithmen für Task-Zerlegung
  - **Abhängigkeits-Erkennung**: Automatische Erkennung von Abhängigkeiten zwischen Sub-Tasks (Dependency-Graph)
- **Dependency-Mapping**: Identifiziert Abhängigkeiten zwischen Sub-Tasks (DAG - Directed Acyclic Graph)
- **Task-Zerlegung**: Zerlegt komplexe Tasks in atomare, ausführbare Sub-Tasks
- **Prioritätsbestimmung**: Bestimmt Prioritäten für Sub-Tasks basierend auf Abhängigkeiten
- **Resource-Schätzung**: Schätzt benötigte Ressourcen für jeden Sub-Task

**2. Sub-Agent Orchestration**
- **Agent-Auswahl**: Entscheidet, welche Valkyries für welche Sub-Tasks benötigt werden
- **Multi-Instance-Planung**: Plant, wie viele Instanzen jeder Valkyrie benötigt werden
- **Parallelisierungs-Strategie**: Bestimmt, welche Tasks parallel ausgeführt werden können
- **Sequenzielle Koordination**: Koordiniert sequenzielle Tasks mit Abhängigkeiten
- **Load-Balancing**: Verteilt Tasks gleichmäßig auf verfügbare Instanzen

**3. Workflow Management**
- **Workflow-Erstellung**: Erstellt detaillierte Workflows für Task-Ausführung
- **Progress-Tracking**: Verfolgt Fortschritt aller laufenden Sub-Tasks
- **Deadline-Management**: Verwaltet Deadlines und Timeouts für Tasks
- **Retry-Strategien**: Plant Retry-Strategien für fehlgeschlagene Tasks
- **Rollback-Planung**: Plant Rollback-Strategien für kritische Fehler

**4. Quality Assurance & Verification**
- **Code-Review-Koordination**: Koordiniert Code-Reviews durch spezialisierte Agents
- **Test-Verification**: Verifiziert, dass alle Tests erfolgreich sind
- **Integration-Checking**: Prüft Integration zwischen Frontend, Backend und anderen Komponenten
- **Completeness-Verification**: Verifiziert, dass alle Anforderungen erfüllt wurden
- **Quality-Metriken**: Sammelt und analysiert Quality-Metriken von allen Valkyries

**5. Statement Collection & Analysis**
- **Statement-Sammlung**: Sammelt Statements von allen Valkyries nach Abschluss
- **Statement-Analyse**: Analysiert Statements auf Vollständigkeit und Konsistenz
- **Gap-Erkennung**: Erkennt Lücken zwischen erwarteten und tatsächlichen Ergebnissen
- **Iterations-Planung**: Plant zusätzliche Iterationen für fehlende Teile

**6. Communication & Coordination**
- **Event-basierte Kommunikation**: Kommuniziert mit Odin und anderen Services über Event-Dispatcher
- **Event-Registrierung**: Registriert sich für relevante Events (z.B. `CodingTaskRequest`, `FileChanged`, etc.)
- **Interne FIFO-Queue**: Events werden plugin-intern auf FIFO-Queue gestellt und abgearbeitet
- **Event-Publishing**: Publiziert Events für Folge-Events oder Return-Events
- **Platform-Capabilities**: Nutzt Platform-Capabilities für Service-Discovery (z.B. Thor für Actions)
- **Status-Updates**: Sendet regelmäßige Status-Updates via Events
- **Error-Reporting**: Meldet Fehler und Probleme via Events

**7. Resource Management**
- **Resource-Allokation**: Allokiert Ressourcen für Sub-Agents
- **Resource-Überwachung**: Überwacht Resource-Usage aller Instanzen
- **Resource-Optimierung**: Optimiert Resource-Allokation für beste Performance
- **Cleanup-Koordination**: Koordiniert Cleanup von Ressourcen nach Task-Abschluss

#### Kanban-Workflow

**Kanban-Board-Erstellung:**
- **Brünnhilde erstellt Kanban-Board**: Nach Task-Analyse erstellt Brünnhilde ein Kanban-Board mit allen Tasks und Iterationen
- **Board-Struktur**: 
  - **Backlog**: Alle Tasks, die noch nicht begonnen wurden
  - **In Progress**: Tasks, die aktuell bearbeitet werden
  - **Review**: Tasks, die auf Review warten
  - **Done**: Abgeschlossene Tasks
- **Task-Priorisierung**: Tasks werden nach Dependencies und Priorität sortiert
- **Iteration-Tracking**: Iterationen werden als separate Spalten oder Tags verwaltet

**Task-Abarbeitung:**
- **Valkyries wählen Tasks**: Valkyries wählen Tasks aus Board basierend auf:
  - **Eignung**: Wie gut ist die Valkyrie für den Task geeignet?
  - **Verfügbarkeit**: Ist die Valkyrie verfügbar?
  - **User-Konfiguration**: Wie viele parallele Agents hat User konfiguriert?
- **Parallele Bearbeitung**: Mehrere Valkyries können parallel arbeiten (basierend auf User-Konfiguration)
- **Task-Status-Updates**: Valkyries aktualisieren Task-Status im Kanban-Board
- **Dependency-Management**: Tasks mit Dependencies werden erst gestartet, wenn Dependencies erfüllt sind

**Workflow (Detailliert)**
1. **Task empfangen**: Task wird via Event-Dispatcher empfangen (Event wird auf interne FIFO-Queue gestellt)
2. **Task-Analyse**: Task wird analysiert (Komplexität, Abhängigkeiten, Anforderungen)
3. **RAG-Indexierung**: Projekt wird indexiert (falls noch nicht geschehen) für besseren Überblick
4. **Task-Decomposition**: Task wird in Sub-Tasks zerlegt mit Dependency-Mapping
5. **Kanban-Board-Erstellung**: Brünnhilde erstellt Kanban-Board mit allen Tasks und Iterationen
6. **Agent-Auswahl & Instanzierung**: Entscheidet, welche Valkyries benötigt werden, startet Instanzen (basierend auf User-Konfiguration)
7. **Task-Auswahl durch Valkyries**: Valkyries wählen Tasks aus Board basierend auf Eignung und Verfügbarkeit
8. **Parallele Task-Abarbeitung**: Mehrere Valkyries arbeiten parallel an verschiedenen Tasks
9. **Progress-Monitoring**: Brünnhilde überwacht Fortschritt aller laufenden Tasks im Kanban-Board
10. **Statement-Collection**: Sammelt Statements von allen Valkyries nach Abschluss
11. **Quality-Verification**: Prüft Qualität und Vollständigkeit aller Ergebnisse
12. **Iteration (falls nötig)**: Plant und delegiert zusätzliche Tasks für fehlende Teile (neue Tasks im Kanban-Board)
13. **Task-Completion**: Task wird abgeschlossen, Results werden via Event zurückgegeben

### Sub-Valkyries

**Wichtig**: Jede Valkyrie hat ihre eigene, klar definierte Aufgabe, um Context-Fenster klein und fokussiert zu halten. Nach Abschluss gibt jede Valkyrie ein kurzes Statement ab.

#### Göll - Frontend Agent

**Mythologische Bedeutung**: "Tumult" oder "Kampf" - Steht für dynamische, kämpferische Energie und Bewegung.

**Spezialisierte Verantwortlichkeiten:**
- **UI-Komponenten-Generierung**: Erstellt React/Vue/Angular-Komponenten
- **Styling & Layout**: Generiert CSS, SCSS, Tailwind-Klassen, Layout-Strukturen
- **State-Management**: Implementiert State-Management (Redux, Zustand, Context API)
- **Routing**: Erstellt Routing-Strukturen und Navigation
- **Form-Handling**: Implementiert Formulare mit Validierung
- **API-Integration**: Integriert Frontend mit Backend-APIs
- **Responsive Design**: Erstellt responsive Designs für verschiedene Bildschirmgrößen
- **Accessibility**: Implementiert Accessibility-Features (ARIA, Keyboard-Navigation)
- **Performance-Optimierung**: Optimiert Bundle-Size, Lazy-Loading, Code-Splitting

**Spezialisierte Tools & Technologien:**
- **Frameworks**: React, Vue.js, Angular, Svelte
- **Styling**: CSS, SCSS, Tailwind CSS, Styled Components, CSS Modules
- **State**: Redux, Zustand, MobX, Context API, Pinia
- **Routing**: React Router, Vue Router, Angular Router
- **Build-Tools**: Vite, Webpack, Parcel, esbuild
- **Testing**: Jest, Vitest, React Testing Library, Cypress (für E2E)
- **Type-Safety**: TypeScript, PropTypes, Flow

**Context-Isolation:**
- Arbeitet nur mit Frontend-relevantem Code
- Kein Zugriff auf Backend-Code oder Datenbank-Schemas
- Minimale Abhängigkeiten zu anderen Valkyries (nur API-Contracts)

**Multi-Instance-Szenarien:**
- Eine Instanz pro Komponente (bei großen Tasks)
- Eine Instanz pro Feature-Modul
- Parallele Bearbeitung von unabhängigen UI-Bereichen

**Statement-Format:**
- "Frontend: [Anzahl] Komponenten erstellt, [Anzahl] Stylesheets generiert, Routing implementiert"

#### Þrúðr (Thrúd) - Backend Agent

**Mythologische Bedeutung**: "Stärke" - Tochter Thors, steht für Robustheit, Kraft und Ausdauer.

**Spezialisierte Verantwortlichkeiten:**
- **API-Entwicklung**: Erstellt REST/GraphQL/gRPC APIs
- **Datenbank-Integration**: Implementiert Datenbank-Zugriffe, ORM-Mappings
- **Business-Logic**: Implementiert Geschäftslogik und Service-Layer
- **Authentication & Authorization**: Implementiert Auth-Systeme (JWT, OAuth, etc.)
- **Middleware-Entwicklung**: Erstellt Middleware für Request-Processing
- **Error-Handling**: Implementiert umfassendes Error-Handling
- **Data-Validation**: Implementiert Input-Validierung und Sanitization
- **Caching-Strategien**: Implementiert Caching-Layer (Redis, Memcached)
- **Background-Jobs**: Erstellt Background-Job-Systeme (Queues, Workers)

**Spezialisierte Tools & Technologien:**
- **Frameworks**: Express, FastAPI, Spring Boot, NestJS, Django, Flask
- **Datenbanken**: PostgreSQL, MySQL, MongoDB, Redis, SQLite
- **ORM/ODM**: Prisma, TypeORM, Sequelize, Mongoose, SQLAlchemy
- **API-Formate**: REST, GraphQL, gRPC, WebSocket
- **Authentication**: JWT, OAuth2, Passport.js, Auth0
- **Testing**: Jest, pytest, Mocha, JUnit
- **Validation**: Zod, Joi, class-validator, Pydantic

**Context-Isolation:**
- Arbeitet nur mit Backend-relevantem Code
- Kein Zugriff auf Frontend-Code oder UI-Komponenten
- Kennt API-Contracts für Frontend-Integration

**Multi-Instance-Szenarien:**
- Eine Instanz pro Service/Microservice
- Eine Instanz pro API-Endpoint-Gruppe
- Parallele Bearbeitung von unabhängigen Backend-Modulen

**Statement-Format:**
- "Backend: [Anzahl] API-Endpoints erstellt, [Anzahl] Services implementiert, Datenbank-Schema aktualisiert"

#### Gunnr - Test Agent

**Mythologische Bedeutung**: "Krieg" oder "Schlacht" - Steht für den Kampf gegen Fehler, Qualitätssicherung und den Sieg über Bugs.

**Spezialisierte Verantwortlichkeiten:**
- **Unit-Test-Generierung**: Erstellt Unit-Tests für einzelne Funktionen/Komponenten
- **Integration-Test-Generierung**: Erstellt Integration-Tests für Service-Interaktionen
- **E2E-Test-Generierung**: Erstellt End-to-End-Tests für User-Flows
- **Test-Data-Generierung**: Erstellt Mock-Daten und Test-Fixtures
- **Test-Coverage-Analyse**: Analysiert Code-Coverage und identifiziert Lücken
- **Test-Refactoring**: Refactoriert bestehende Tests für bessere Wartbarkeit
- **Performance-Tests**: Erstellt Performance- und Load-Tests
- **Security-Tests**: Erstellt Security-Tests (Penetration-Tests, Vulnerability-Scans)

**Spezialisierte Tools & Technologien:**
- **Unit-Testing**: Jest, Vitest, pytest, JUnit, Mocha, RSpec
- **E2E-Testing**: Cypress, Playwright, Selenium, Puppeteer
- **Integration-Testing**: Supertest, Testcontainers, WireMock
- **Mocking**: Sinon, Mockito, unittest.mock, MSW (Mock Service Worker)
- **Coverage**: Istanbul, Coverage.py, JaCoCo
- **Performance**: k6, Artillery, JMeter, Locust
- **Security**: OWASP ZAP, Burp Suite (Integration)

**Context-Isolation:**
- Arbeitet nur mit Test-relevantem Code
- Kennt Code-Struktur für Test-Generierung
- Minimaler Zugriff auf Produktions-Code (nur für Test-Erstellung)

**Multi-Instance-Szenarien:**
- Eine Instanz pro Test-Suite (Unit, Integration, E2E)
- Eine Instanz pro Modul/Feature
- Parallele Bearbeitung von unabhängigen Test-Bereichen

**Statement-Format:**
- "Tests: [Anzahl] Unit-Tests, [Anzahl] Integration-Tests, [Anzahl] E2E-Tests erstellt, Coverage: [Prozent]%"

#### Hlökk - Documentation Agent

**Mythologische Bedeutung**: "Lärm" oder "Kampfgetöse" - Steht für Kommunikation, laute Verkündigung und klare Dokumentation.

**Spezialisierte Verantwortlichkeiten:**
- **Code-Dokumentation**: Generiert JSDoc/TSDoc/Python-Docstrings
- **API-Dokumentation**: Erstellt OpenAPI/Swagger/GraphQL-Schema-Dokumentation
- **README-Generierung**: Erstellt und aktualisiert README-Dateien
- **Architektur-Dokumentation**: Dokumentiert System-Architektur und Design-Entscheidungen
- **User-Guides**: Erstellt Benutzerhandbücher und Tutorials
- **Changelog-Verwaltung**: Verwaltet CHANGELOG.md und Release-Notes
- **Code-Comments**: Fügt sinnvolle Code-Kommentare hinzu
- **Diagramm-Generierung**: Erstellt Architektur-Diagramme (Mermaid, PlantUML)

**Spezialisierte Tools & Technologien:**
- **Markup**: Markdown, reStructuredText, AsciiDoc
- **API-Docs**: OpenAPI/Swagger, GraphQL Schema, Postman Collections
- **Code-Docs**: JSDoc, TSDoc, Sphinx, Doxygen
- **Diagramme**: Mermaid, PlantUML, Graphviz
- **Generatoren**: Docusaurus, MkDocs, Sphinx, GitBook
- **Formatting**: Prettier, Markdownlint

**Context-Isolation:**
- Arbeitet nur mit Dokumentations-relevantem Code
- Analysiert Code-Struktur für Dokumentations-Generierung
- Minimaler Zugriff auf Code-Implementierung (nur für Dokumentation)

**Multi-Instance-Szenarien:**
- Eine Instanz pro Dokumentations-Bereich (API, User-Guide, Architecture)
- Eine Instanz pro Modul/Feature
- Parallele Bearbeitung von unabhängigen Dokumentations-Bereichen

**Statement-Format:**
- "Dokumentation: [Anzahl] README-Dateien aktualisiert, API-Dokumentation generiert, [Anzahl] Code-Kommentare hinzugefügt"

#### Geirskögul - Database Agent

**Mythologische Bedeutung**: "Speerträgerin" - Steht für Präzision, Durchdringung und gezielte Aktionen.

**Spezialisierte Verantwortlichkeiten:**
- **Schema-Design**: Erstellt und optimiert Datenbank-Schemas
- **Migration-Generierung**: Erstellt Datenbank-Migrations (up/down)
- **Query-Optimierung**: Optimiert SQL-Queries für Performance
- **Index-Management**: Erstellt und verwaltet Datenbank-Indizes
- **Data-Modeling**: Erstellt Entity-Relationship-Diagramme
- **Seed-Data-Generierung**: Erstellt Seed-Daten für Development/Testing
- **Backup-Strategien**: Plant Backup- und Recovery-Strategien
- **Connection-Pooling**: Konfiguriert Connection-Pooling

**Spezialisierte Tools & Technologien:**
- **Datenbanken**: PostgreSQL, MySQL, MongoDB, SQLite, Redis
- **ORM/ODM**: Prisma, TypeORM, Sequelize, Mongoose, SQLAlchemy, Drizzle
- **Migration-Tools**: Prisma Migrate, TypeORM Migrations, Alembic, Flyway
- **Query-Tools**: SQL, NoSQL-Queries, Query-Builder
- **Schema-Tools**: pgAdmin, MySQL Workbench, MongoDB Compass

**Context-Isolation:**
- Arbeitet nur mit Datenbank-relevantem Code
- Kein Zugriff auf Frontend-Code oder Business-Logic
- Kennt API-Contracts für Datenbank-Integration

**Multi-Instance-Szenarien:**
- Eine Instanz pro Datenbank-Schema
- Eine Instanz pro Migration-Set
- Parallele Bearbeitung von unabhängigen Datenbank-Bereichen

**Statement-Format:**
- "Database: Schema erstellt, [Anzahl] Migrations generiert, [Anzahl] Indizes optimiert"

#### Róta - API Design Agent

**Mythologische Bedeutung**: "Sturm" oder "Aufruhr" - Steht für Echtzeit-Events, dynamische Systeme und stürmische Aktivität.

**Spezialisierte Verantwortlichkeiten:**
- **API-Design**: Entwirft REST/GraphQL/gRPC API-Strukturen
- **Contract-Definition**: Erstellt API-Contracts (OpenAPI, GraphQL Schema)
- **Endpoint-Design**: Plant API-Endpoints mit Request/Response-Schemas
- **Versioning-Strategien**: Implementiert API-Versioning
- **Rate-Limiting-Design**: Plant Rate-Limiting-Strategien
- **Error-Response-Design**: Standardisiert Error-Response-Formate
- **API-Dokumentation-Struktur**: Erstellt Struktur für API-Dokumentation
- **Mock-Server-Generierung**: Generiert Mock-Server für Frontend-Entwicklung

**Spezialisierte Tools & Technologien:**
- **API-Formate**: REST, GraphQL, gRPC, WebSocket
- **Contract-Tools**: OpenAPI/Swagger, GraphQL Schema, Protocol Buffers
- **Design-Tools**: Postman Collections, Insomnia, API Blueprint
- **Mocking**: MSW, json-server, Mockoon, WireMock
- **Validation**: JSON Schema, Zod, Joi

**Context-Isolation:**
- Arbeitet nur mit API-Design-relevantem Code
- Definiert Contracts, die von Backend und Frontend verwendet werden
- Kein Zugriff auf Implementierungs-Details

**Multi-Instance-Szenarien:**
- Eine Instanz pro API-Version
- Eine Instanz pro Service/Microservice
- Parallele Bearbeitung von unabhängigen API-Bereichen

**Statement-Format:**
- "API Design: [Anzahl] Endpoints entworfen, OpenAPI-Schema erstellt, Mock-Server generiert"

#### Hildr - Security Agent

**Mythologische Bedeutung**: "Kampf" - Steht für den Kampf gegen Angriffe, Schutz und Verteidigung.

**Spezialisierte Verantwortlichkeiten:**
- **Security-Reviews**: Führt automatische Security-Reviews durch
- **Vulnerability-Scanning**: Scannt Code auf bekannte Vulnerabilities
- **Authentication-Review**: Prüft Authentication-Implementierungen
- **Authorization-Review**: Prüft Authorization-Logic
- **Input-Validation-Review**: Prüft Input-Validierung und Sanitization
- **Secret-Management**: Prüft auf Hardcoded Secrets, empfiehlt Secret-Management
- **Dependency-Scanning**: Scannt Dependencies auf Security-Issues
- **Security-Best-Practices**: Implementiert Security-Best-Practices

**Spezialisierte Tools & Technologien:**
- **Scanning**: OWASP ZAP, Snyk, bun audit, pip-audit, Safety
- **SAST**: SonarQube, CodeQL, Semgrep, Bandit
- **Dependency-Scanning**: Dependabot, Renovate, Snyk
- **Secret-Detection**: GitGuardian, TruffleHog, detect-secrets
- **Security-Frameworks**: OWASP Top 10, CWE, CVE

**Context-Isolation:**
- Arbeitet nur mit Security-relevantem Code
- Analysiert Code auf Security-Issues
- Minimaler Zugriff auf Code-Implementierung (nur für Security-Analyse)

**Multi-Instance-Szenarien:**
- Eine Instanz pro Security-Bereich (Auth, Input-Validation, etc.)
- Eine Instanz pro Modul/Service
- Parallele Bearbeitung von unabhängigen Security-Bereichen

**Statement-Format:**
- "Security: [Anzahl] Vulnerabilities gefunden, [Anzahl] Security-Issues behoben, Dependency-Scan durchgeführt"

#### Skögul - Performance Agent

**Mythologische Bedeutung**: "die Rüttelnde" - Steht für Bewegung, Optimierung und das Erschüttern von Ineffizienzen.

**Spezialisierte Verantwortlichkeiten:**
- **Performance-Analyse**: Analysiert Code auf Performance-Bottlenecks
- **Profiling**: Führt Code-Profiling durch
- **Optimierung**: Optimiert langsame Code-Stellen
- **Caching-Strategien**: Implementiert Caching-Strategien
- **Database-Query-Optimierung**: Optimiert langsame Datenbank-Queries
- **Bundle-Size-Optimierung**: Optimiert Frontend-Bundle-Sizes
- **Lazy-Loading**: Implementiert Lazy-Loading-Strategien
- **Performance-Metriken**: Sammelt und analysiert Performance-Metriken

**Spezialisierte Tools & Technologien:**
- **Profiling**: Chrome DevTools, Node.js Profiler, Python cProfile
- **Performance-Testing**: Lighthouse, WebPageTest, k6, Artillery
- **Bundle-Analysis**: webpack-bundle-analyzer, source-map-explorer
- **Caching**: Redis, Memcached, CDN-Konfiguration
- **Monitoring**: Performance Monitoring Tools, APM

**Context-Isolation:**
- Arbeitet nur mit Performance-relevantem Code
- Analysiert Code auf Performance-Issues
- Minimaler Zugriff auf Code-Implementierung (nur für Performance-Analyse)

**Multi-Instance-Szenarien:**
- Eine Instanz pro Performance-Bereich (Frontend, Backend, Database)
- Eine Instanz pro Modul/Service
- Parallele Bearbeitung von unabhängigen Performance-Bereichen

**Statement-Format:**
- "Performance: [Anzahl] Bottlenecks identifiziert, [Anzahl] Optimierungen durchgeführt, Bundle-Size um [Prozent]% reduziert"

#### Mist - DevOps Agent

**Mythologische Bedeutung**: "Nebel" - Steht für verschleierte Umgebungen, Cloud-Infrastructure und mysteriöse Deployment-Prozesse.

**Spezialisierte Verantwortlichkeiten:**
- **Docker-Containerisierung**: Erstellt Dockerfiles und docker-compose.yml
- **CI/CD-Pipeline**: Erstellt CI/CD-Pipelines (GitHub Actions, GitLab CI, etc.)
- **Infrastructure-as-Code**: Erstellt IaC-Skripte (Terraform, CloudFormation)
- **Deployment-Skripte**: Erstellt Deployment-Skripte und -Konfigurationen
- **Environment-Konfiguration**: Konfiguriert Development/Staging/Production-Environments
- **Monitoring-Setup**: Konfiguriert Monitoring und Logging
- **Kubernetes-Manifests**: Erstellt Kubernetes-Manifests (falls benötigt)
- **Build-Konfiguration**: Konfiguriert Build-Prozesse

**Spezialisierte Tools & Technologien:**
- **Containerization**: Docker, Podman, containerd
- **Orchestration**: Kubernetes, Docker Swarm, Nomad
- **CI/CD**: GitHub Actions, GitLab CI, Jenkins, CircleCI, Azure DevOps
- **IaC**: Terraform, CloudFormation, Pulumi, Ansible
- **Cloud**: AWS, Azure, GCP, Cloudflare
- **Monitoring**: Prometheus, Grafana, Datadog, New Relic

**Context-Isolation:**
- Arbeitet nur mit DevOps-relevantem Code
- Kein Zugriff auf Application-Code (nur Deployment-Konfiguration)
- Kennt Deployment-Anforderungen

**Multi-Instance-Szenarien:**
- Eine Instanz pro Environment (Dev, Staging, Prod)
- Eine Instanz pro Service/Microservice
- Parallele Bearbeitung von unabhängigen DevOps-Bereichen

**Statement-Format:**
- "DevOps: Dockerfile erstellt, CI/CD-Pipeline konfiguriert, [Anzahl] Environments eingerichtet"

#### Hrist - Refactoring Agent

**Mythologische Bedeutung**: "die Erschütternde" - Steht für das Erschüttern und Verbessern von Code, Transformation und Erneuerung.

**Spezialisierte Verantwortlichkeiten:**
- **Code-Qualität-Analyse**: Analysiert Code auf Code-Smells
- **Refactoring-Planung**: Plant Refactoring-Strategien
- **Code-Struktur-Optimierung**: Optimiert Code-Struktur und -Organisation
- **Clean-Code-Prinzipien**: Wendet Clean-Code-Prinzipien an
- **Design-Pattern-Implementierung**: Implementiert Design-Patterns
- **Legacy-Code-Modernisierung**: Modernisiert Legacy-Code
- **Code-Duplikation-Eliminierung**: Eliminiert Code-Duplikation
- **Naming-Conventions**: Verbessert Naming-Conventions

**Spezialisierte Tools & Technologien:**
- **Code-Analysis**: SonarQube, ESLint, Pylint, RuboCop
- **Refactoring-Tools**: IDE-Refactoring-Tools, jscodeshift, codemods
- **Metrics**: Code-Complexity-Metriken, Maintainability-Index
- **Pattern-Libraries**: Design-Pattern-Implementierungen

**Context-Isolation:**
- Arbeitet nur mit Code-Qualität-relevantem Code
- Analysiert Code-Struktur für Refactoring
- Minimaler Zugriff auf Code-Implementierung (nur für Refactoring)

**Multi-Instance-Szenarien:**
- Eine Instanz pro Refactoring-Bereich (Module, Services, etc.)
- Eine Instanz pro Code-Qualität-Aspekt
- Parallele Bearbeitung von unabhängigen Refactoring-Bereichen

**Statement-Format:**
- "Refactoring: [Anzahl] Code-Smells behoben, [Anzahl] Design-Patterns implementiert, Code-Duplikation um [Prozent]% reduziert"

#### Skeggjöld - Configuration Agent

**Mythologische Bedeutung**: "Axt-Zeit" - Steht für schnelle, präzise Aktionen, Zeit-Optimierung und effiziente Konfiguration.

**Spezialisierte Verantwortlichkeiten:**
- **Config-File-Generierung**: Erstellt Konfigurationsdateien (.env, config.json, etc.)
- **Environment-Variable-Management**: Verwaltet Environment-Variablen
- **Build-Konfiguration**: Konfiguriert Build-Tools (webpack.config.js, tsconfig.json, etc.)
- **Linter-Konfiguration**: Konfiguriert Linter (ESLint, Prettier, etc.)
- **Test-Konfiguration**: Konfiguriert Test-Frameworks (Jest, pytest, etc.)
- **Editor-Konfiguration**: Erstellt Editor-Konfigurationen (.editorconfig, .vscode/)
- **Package-Management**: Verwaltet package.json, requirements.txt, etc.
- **Secret-Management-Konfiguration**: Konfiguriert Secret-Management

**Spezialisierte Tools & Technologien:**
- **Config-Formate**: JSON, YAML, TOML, .env, XML
- **Build-Tools**: webpack, Vite, Rollup, esbuild
- **Linters**: ESLint, Prettier, Pylint, RuboCop
- **Package-Manager**: bun (für TypeScript/Frontend), pip, poetry, cargo
- **Secret-Management**: Vault, AWS Secrets Manager, Azure Key Vault

**Context-Isolation:**
- Arbeitet nur mit Konfigurations-Dateien
- Kein Zugriff auf Application-Code
- Kennt Konfigurations-Anforderungen

**Multi-Instance-Szenarien:**
- Eine Instanz pro Konfigurations-Bereich (Build, Lint, Test, etc.)
- Eine Instanz pro Environment
- Parallele Bearbeitung von unabhängigen Konfigurations-Bereichen

**Statement-Format:**
- "Configuration: [Anzahl] Config-Dateien erstellt, Environment-Variablen konfiguriert, Build-Tools eingerichtet"

## Valkyries-Übersicht mit mythologischen Namen

**Gesamt: 12 spezialisierte Valkyries + Brünnhilde (Lead Agent)**

### Vollständige Liste der Valkyries und ihre mythologischen Aufgaben:

1. **Brünnhilde (Brynhildr)** - "Kampfpanzerin/Schutzkämpferin"
   - **Mythologie**: Die mächtigste Walküre, Beschützerin, strategische Führerin
   - **Software-Aufgabe**: Lead Agent - Orchestrierung, Task-Decomposition, Quality Assurance, Schutz des gesamten Projekts

2. **Gunnr** - "Krieg/Schlacht"
   - **Mythologie**: Kämpferin im Krieg, Siegerin über Feinde
   - **Software-Aufgabe**: Test Agent - Kampf gegen Bugs, Qualitätssicherung, Sieg über Fehler

3. **Hildr** - "Kampf"
   - **Mythologie**: Kämpferin, Verteidigerin
   - **Software-Aufgabe**: Security Agent - Kampf gegen Angriffe, Sicherheitsprotokolle, Verteidigung des Systems

4. **Skögul** - "die Rüttelnde"
   - **Mythologie**: Erschüttert und bewegt Dinge, bringt Bewegung
   - **Software-Aufgabe**: Performance Agent - System optimieren, Performance erschüttern, Bewegung in langsame Systeme bringen

5. **Hrist** - "die Erschütternde"
   - **Mythologie**: Erschüttert und transformiert
   - **Software-Aufgabe**: Refactoring Agent - Code erschüttern/verbessern, Code-Qualität, Transformation

6. **Mist** - "Nebel"
   - **Mythologie**: Verschleiert, verhüllt, mysteriös
   - **Software-Aufgabe**: DevOps Agent - Cloud-Infrastructure, verschleierte Deployment-Umgebungen, mysteriöse Infrastruktur

7. **Skeggjöld** - "Axt-Zeit"
   - **Mythologie**: Schnelle, präzise Aktionen, Zeit-Optimierung
   - **Software-Aufgabe**: Configuration Agent - Schnelle Konfiguration, Build-Zeit-Optimierung, präzise Setup

8. **Göll** - "Tumult/Kampf"
   - **Mythologie**: Dynamische Energie, Bewegung, Kampf
   - **Software-Aufgabe**: Frontend Agent - Dynamische UI, interaktive Komponenten, kämpferische Benutzeroberflächen

9. **Geirskögul** - "Speerträgerin"
   - **Mythologie**: Präzision, Durchdringung, gezielte Aktionen
   - **Software-Aufgabe**: Database Agent - Durchdringende Datenbankarbeit, präzise Queries, gezielte Datenbank-Operationen

10. **Þrúðr (Thrúd)** - "Stärke"
    - **Mythologie**: Tochter Thors, Robustheit, Kraft, Ausdauer
    - **Software-Aufgabe**: Backend Agent - Robuste Backend-Logik, starke Services, ausdauernde APIs

11. **Hlökk** - "Lärm/Kampfgetöse"
    - **Mythologie**: Kommunikation, laute Verkündigung
    - **Software-Aufgabe**: Documentation Agent - Kommunikation, laute Dokumentation, klare Verkündigung

12. **Róta** - "Sturm/Aufruhr"
    - **Mythologie**: Stürmische Aktivität, Echtzeit-Events
    - **Software-Aufgabe**: API Design Agent - Echtzeit-Systeme, Event-Architekturen, stürmische APIs

13. **Sigrún** - "Geheimnis des Sieges"
    - **Mythologie**: Sieg, Erfolg, Geheimnisse des Erfolgs
    - **Software-Aufgabe**: Performance/Optimization Agent - Algorithmus-Optimierung, Erfolgsgeheimnisse, Sieg über Ineffizienz

**Jede Valkyrie hat einen engen, spezialisierten Fokus, um das Context-Fenster klein zu halten. Die mythologischen Namen und Bedeutungen spiegeln ihre Software-Entwicklungsaufgaben wider.**

## RAG-System für Valkyries

**Projekt-Indexierung:**
- **RAG-Integration**: Jede Valkyrie hat Zugriff auf RAG-System für Projekt-Indexierung
- **Projekt-Indexierung**: Projekt wird indexiert, damit immer ein guter Überblick besteht
- **Stichwort-basierte Speicherung**: Informationen werden in Stichworten festgehalten, ohne Sinn zu verlieren
- **Kategorisierung und Gruppierung**: Informationen werden kategorisiert und gruppiert
- **Effiziente Suche**: Mit Stichworten gute Treffer erzielen, aber nicht alle Daten aus Datenbank laden müssen
- **Optimierte Datenbanken**: Verwendung von für RAG optimierten Datenbanken (z.B. Vector-Databases)

**RAG-Strategie:**
- **Stichwort-Extraktion**: Wichtige Code-Informationen werden als Stichworte extrahiert
- **Kategorisierung**: Stichworte werden kategorisiert (z.B. "Frontend", "Backend", "Database", "API", "Component", "Function")
- **Gruppierung**: Ähnliche Stichworte werden gruppiert (z.B. alle React-Komponenten, alle API-Endpoints)
- **Vector-Embeddings**: Stichworte werden als Embeddings gespeichert für semantische Suche
- **Hierarchische Struktur**: Informationen werden hierarchisch strukturiert (Kategorie → Gruppe → Stichwort → Code-Location)

**Context-Window-Optimierung:**
- **Selektive Datenladung**: Nur relevante Code-Teile werden geladen (basierend auf Stichwort-Suche)
- **Minimaler Memory-Footprint**: Kleine Context-Fenster durch effiziente Datenstrukturen
- **Schnelle Suche**: Stichwort-basierte Suche ist schnell und effizient
- **Projekt-Überblick**: Jede Valkyrie hat immer einen guten Überblick über das Projekt, ohne alle Daten zu laden

**RAG-Index-Updates:**
- **Incremental Updates**: Bei Code-Änderungen wird Index inkrementell aktualisiert
- **Real-time Indexing**: Code-Änderungen werden sofort indexiert
- **Stichwort-Refresh**: Stichworte werden bei Code-Änderungen aktualisiert

## Features

### 1. Claude Code Features (Übernommen)
- **Alle Features von Claude Code**: Alle bewährten Features von Claude Code werden übernommen
- **Ralph-ähnliche Funktionalität**: Ähnlich wie das neue Anthropic Plugin "Ralph" für Claude Code
- **Persistent Execution**: Arbeitet nicht aufhören bis Task vollständig erledigt ist
- **Iterative Improvement**: Verbessert Code iterativ bis zur Vollständigkeit
- **Context-Aware**: Nutzt Codebase-Kontext intelligent

### 2. Multi-Instance Orchestration (via Kanban-Board)
- **Mehrere Instanzen einer Valkyrie**: Brünhild kann mehrere Instanzen einer Valkyrie starten
- **Beispiel**: Mehrere Frontend-Agents für verschiedene Komponenten parallel
- **Resource Management**: Verwaltung von Ressourcen für mehrere Instanzen
- **Coordination**: Koordination zwischen Instanzen derselben Valkyrie

### 3. Kanban-Workflow

**Kanban-Board-System:**
- **Board-Erstellung**: Brünnhilde erstellt Kanban-Board mit allen Tasks und Iterationen
- **Task-Management**: Tasks werden in Backlog, In Progress, Review, Done verwaltet
- **Task-Auswahl**: Valkyries wählen Tasks aus Board basierend auf Eignung und Verfügbarkeit
- **Parallele Bearbeitung**: Mehrere Valkyries können parallel arbeiten (basierend auf User-Konfiguration)
- **Dependency-Management**: Tasks mit Dependencies werden erst gestartet, wenn Dependencies erfüllt sind
- **Iteration-Tracking**: Iterationen werden als separate Spalten oder Tags verwaltet

### 4. Statement System
- **Task Completion Statements**: Jede Valkyrie gibt Statement ab nach Abschluss
- **Statement Format**: Kurz, prägnant (z.B. "Frontend-Komponenten X, Y, Z erstellt")
- **Statement Collection**: Brünhild sammelt alle Statements
- **Vollständigkeitsprüfung**: Brünhild prüft basierend auf Statements, ob Task vollständig ist

### 4. Git Integration
- **Repository-Detection**: 
  - **Automatische Erkennung**: Automatische Erkennung von Git-Repositories (`.git`-Ordner)
  - **Multi-Repository-Support**: Unterstützung für mehrere Repositories (Monorepo, Multi-Repo)
  - **Repository-Fehler**: Bei Repository-Fehlern wird Fehler geloggt, Task wird ohne Git-Integration fortgesetzt
- **Branch-Management**: 
  - **Branch-Strategien**: Branch-Strategien (Feature-Branches, Main-Branch, etc.)
  - **Branch-Konflikte**: Bei Branch-Konflikten wird Merge durchgeführt oder User wird benachrichtigt
  - **Branch-Verwaltung**: Automatische Branch-Verwaltung für Feature-Entwicklung
- **Change-Tracking**: 
  - **Change-Tracking**: Automatisches Tracking von Änderungen (Git-Diff)
  - **Change-Validation**: Change-Validation (Syntax-Check, Linter, etc.)
  - **Change-Konflikte**: Bei Change-Konflikten wird User benachrichtigt oder automatischer Merge
- **Repository Detection**: Automatische Erkennung von Git-Repositories
- **Branch Management**: Branch-Verwaltung
- **Change Tracking**: Tracking von Änderungen
- **Commit Management**: Commit-Verwaltung

### 5. Dynamic Parallelization
- **Parallel Execution**: Parallele Ausführung von Sub-Agents (inkl. mehrerer Instanzen)
- **Git-based Coordination**: Koordination basierend auf Git
- **Resource-based Scaling**: Skalierung basierend auf System-Ressourcen
- **Conflict Resolution**: Auflösung von Konflikten

### 6. Context Management
- **Isolated Contexts**: Isolierte Contexts für Sub-Agents (jede Valkyrie hat eigene Aufgabe)
- **Kleine Context-Fenster**: Jede Valkyrie arbeitet nur mit relevantem Code, um Context-Fenster klein zu halten
- **Minimal Context Sharing**: Minimales Context-Sharing zwischen Valkyries
- **Context Cleanup**: Context-Bereinigung nach Task

### 7. Quality Assurance & Completion Verification
- **Code Review**: Automatische Code-Review
- **Testing**: Automatisches Testing
- **Validation**: Validierung von Änderungen
- **Error Detection**: Erkennung von Fehlern
- **Vollständigkeitsprüfung**: Brünhild stellt sicher, dass Task korrekt und vollständig ausgeführt wurde
- **Iteration**: Falls unvollständig, werden fehlende Teile identifiziert und delegiert

## Spezialisierte Workflows & Interaktionen

### Workflow-Beispiele

#### Beispiel 1: Vollständige Feature-Implementierung
**Task**: "Füge User-Authentication mit Login-Formular hinzu"

**Brünnhilde's Decomposition:**
1. API Design: API-Design für Login/Register (Róta - API Design Agent)
2. Database: User-Schema und Migrations (Geirskögul - Database Agent)
3. Backend: API-Endpoints für Login/Register (Þrúðr - Backend Agent)
4. Frontend: Login-Komponente mit Formular (Göll - Frontend Agent)
5. Security: Security-Review für Authentication (Hildr - Security Agent)
6. Tests: Unit-Tests für Backend + Frontend (Gunnr - Test Agent)
7. Configuration: Environment-Variablen für Secrets (Skeggjöld - Configuration Agent)
8. Dokumentation: API-Docs + README-Update (Hlökk - Documentation Agent)

**Parallele Ausführung:**
- API Design Agent erstellt API-Contract zuerst
- Database Agent und Backend Agent arbeiten parallel (mit API-Contract)
- Frontend Agent wartet auf API-Contract, dann parallele Implementierung
- Security Agent wartet auf Backend/Frontend, dann Security-Review
- Test Agent wartet auf Backend/Frontend, dann parallele Test-Generierung
- Configuration Agent arbeitet parallel (unabhängig)
- Documentation Agent wartet auf alle, dann Dokumentations-Generierung

**Brünnhilde's Verification:**
- Prüft, dass API-Contract existiert
- Prüft, dass Database-Schema existiert
- Prüft, dass API-Endpoints existieren
- Prüft, dass Frontend-Komponente existiert und API integriert
- Prüft, dass Security-Review durchgeführt wurde
- Prüft, dass Tests existieren und erfolgreich sind
- Prüft, dass Konfiguration vollständig ist
- Prüft, dass Dokumentation vollständig ist

#### Beispiel 2: Refactoring mit Tests
**Task**: "Refactoriere User-Service und füge Tests hinzu"

**Brünnhilde's Decomposition:**
1. Refactoring: Code-Qualität-Analyse und Refactoring-Planung (Hrist - Refactoring Agent)
2. Backend: Refactoring des User-Service (Þrúðr - Backend Agent)
3. Performance: Performance-Analyse und Optimierung (Skögul - Performance Agent)
4. Tests: Unit-Tests für refactorierten Service (Gunnr - Test Agent)
5. Security: Security-Review für refactorierten Code (Hildr - Security Agent)
6. Dokumentation: Update der Service-Dokumentation (Hlökk - Documentation Agent)

**Sequenzielle Ausführung:**
- Refactoring Agent analysiert Code und plant Refactoring
- Backend Agent refactoriert basierend auf Plan
- Performance Agent analysiert refactorierten Code
- Test Agent generiert Tests basierend auf refactoriertem Code
- Security Agent führt Security-Review durch
- Documentation Agent aktualisiert Dokumentation

#### Beispiel 3: Multi-Component Frontend-Task
**Task**: "Erstelle Dashboard mit 5 verschiedenen Widgets"

**Brünnhilde's Decomposition:**
- 5x Göll (Frontend Agent) Instanzen (eine pro Widget)
- 1x Göll (Frontend Agent) für Dashboard-Container
- 1x Skögul (Performance Agent) für Bundle-Optimierung
- 1x Gunnr (Test Agent) für Widget-Tests
- 1x Hlökk (Documentation Agent) für Dashboard-Dokumentation

**Parallele Ausführung:**
- Alle 5 Widget-Instanzen arbeiten parallel
- Dashboard-Container wartet auf Widgets
- Performance Agent analysiert Bundle-Size nach Widget-Erstellung
- Test Agent wartet auf alle Widgets, dann Test-Generierung

### Inter-Valkyrie-Kommunikation

**API-Contract-Flow:**
1. Róta (API Design Agent) erstellt API-Contract zuerst
2. Þrúðr (Backend Agent) implementiert basierend auf Contract
3. Göll (Frontend Agent) integriert basierend auf Contract
4. Brünnhilde koordiniert diese Abstimmung über Kanban-Board

**Database-Integration:**
- Geirskögul (Database Agent) erstellt Schema und Migrations
- Þrúðr (Backend Agent) verwendet Schema für ORM-Mappings
- Róta (API Design Agent) berücksichtigt Datenbank-Struktur für API-Design

**Security-Integration:**
- Hildr (Security Agent) prüft Code von Backend und Frontend Agent
- Hildr scannt Dependencies (Skeggjöld - Configuration Agent)
- Hildr empfiehlt Security-Verbesserungen

**Performance-Integration:**
- Skögul (Performance Agent) analysiert Code von Frontend und Backend Agent
- Skögul optimiert Database-Queries (Geirskögul - Database Agent)
- Skögul empfiehlt Caching-Strategien

**Test-Integration:**
- Gunnr (Test Agent) benötigt Zugriff auf generierten Code
- Brünnhilde stellt Code-Kontext für Gunnr bereit
- Gunnr generiert Tests basierend auf Code-Struktur

**DevOps-Integration:**
- Mist (DevOps Agent) konfiguriert Deployment für Backend/Frontend
- Mist verwendet Konfiguration von Skeggjöld (Configuration Agent)
- Mist erstellt CI/CD-Pipelines für Tests

**Refactoring-Integration:**
- Hrist (Refactoring Agent) analysiert Code von allen Code-Generierenden Agents
- Hrist empfiehlt Verbesserungen
- Þrúðr/Göll implementieren Refactoring-Vorschläge

**Dokumentations-Integration:**
- Hlökk (Documentation Agent) analysiert generierten Code
- Hlökk verwendet API-Contracts von Róta (API Design Agent)
- Brünnhilde stellt Code-Kontext für Hlökk bereit

### Statement-System Details

**Statement-Struktur:**
```typescript
interface ValkyrieStatement {
  valkyrie: 'brünnhilde' | 'gunnr' | 'hildr' | 'skögul' | 'hrist' | 'mist' | 
            'skeggjöld' | 'göll' | 'geirskögul' | 'thrúd' | 'hlökk' | 'róta' | 'sigrún';
  instanceId: string;  // Eindeutige Instanz-ID (für Multi-Instance-Szenarien)
  taskId: string;      // Task-ID zur Zuordnung
  completed: boolean;  // Ob Task erfolgreich abgeschlossen wurde
  summary: string;      // Kurze Zusammenfassung (für User)
  details: {
    filesCreated: string[];      // Liste der erstellten Dateien
    filesModified: string[];     // Liste der geänderten Dateien
    filesDeleted?: string[];     // Liste der gelöschten Dateien (optional)
    metrics?: Record<string, number>;  // Metriken (z.B. { components: 3, tests: 10, coverage: 85 })
  };
  errors?: string[];    // Fehler (falls vorhanden)
  warnings?: string[];  // Warnungen (falls vorhanden)
  timestamp: number;    // Timestamp der Statement-Erstellung
  executionTimeMs?: number;  // Ausführungszeit in Millisekunden (optional)
}
```

**Statement-Format-Beispiele:**

**Gunnr (Test Agent):**
```typescript
{
  valkyrie: 'gunnr',
  instanceId: 'gunnr-1',
  taskId: 'task-123',
  completed: true,
  summary: 'Tests: 15 Unit-Tests, 5 Integration-Tests, 2 E2E-Tests erstellt, Coverage: 85%',
  details: {
    filesCreated: ['tests/unit/user.test.ts', 'tests/integration/api.test.ts'],
    filesModified: [],
    metrics: { unitTests: 15, integrationTests: 5, e2eTests: 2, coverage: 85 }
  },
  timestamp: 1234567890
}
```

**Göll (Frontend Agent):**
```typescript
{
  valkyrie: 'göll',
  instanceId: 'göll-1',
  taskId: 'task-123',
  completed: true,
  summary: 'Frontend: 3 Komponenten erstellt, 2 Stylesheets generiert, Routing implementiert',
  details: {
    filesCreated: ['src/components/UserProfile.tsx', 'src/components/UserList.tsx'],
    filesModified: ['src/App.tsx'],
    metrics: { components: 3, stylesheets: 2 }
  },
  timestamp: 1234567890
}
```

**Hlökk (Documentation Agent):**
```typescript
{
  valkyrie: 'hlökk',
  instanceId: 'hlökk-1',
  taskId: 'task-123',
  completed: true,
  summary: 'Dokumentation: 2 README-Dateien aktualisiert, API-Dokumentation generiert, 15 Code-Kommentare hinzugefügt',
  details: {
    filesCreated: ['docs/API.md'],
    filesModified: ['README.md', 'docs/ARCHITECTURE.md'],
    metrics: { readmeFiles: 2, apiDocs: 1, codeComments: 15 }
  },
  timestamp: 1234567890
}
```

**Brünnhilde's Statement-Analyse:**

**Vollständigkeitsprüfung:**
- **Erwartete Statements**: Prüft, ob alle erwarteten Valkyries Statements abgegeben haben
- **Fehlende Statements**: Identifiziert fehlende Statements (z.B. wenn eine Valkyrie fehlgeschlagen ist)
- **Statement-Validierung**: Validiert, ob Statements vollständig und korrekt sind

**Konsistenzprüfung:**
- **Erwartete vs. Tatsächliche Ergebnisse**: Vergleicht erwartete Ergebnisse mit tatsächlichen Statements
- **Datei-Konsistenz**: Prüft, ob alle erwarteten Dateien erstellt wurden
- **Metriken-Konsistenz**: Prüft, ob Metriken den Erwartungen entsprechen (z.B. Test-Coverage)

**Lücken-Identifikation:**
- **Fehlende Dateien**: Identifiziert fehlende Dateien basierend auf Task-Anforderungen
- **Fehlende Features**: Erkennt fehlende Features oder Funktionalitäten
- **Unvollständige Implementierungen**: Erkennt unvollständige Implementierungen

**Iterations-Planung:**
- **Zusätzliche Tasks**: Plant zusätzliche Tasks für identifizierte Lücken
- **Priorisierung**: Priorisiert zusätzliche Tasks basierend auf Wichtigkeit
- **Dependency-Management**: Berücksichtigt Dependencies bei Iterations-Planung

**Statement-Sammlung-Workflow:**
1. **Valkyrie sendet Statement**: Jede Valkyrie sendet Statement nach Abschluss an Brünnhilde
2. **Brünnhilde sammelt Statements**: Brünnhilde sammelt alle Statements in einer Liste
3. **Statement-Analyse**: Brünnhilde analysiert alle Statements auf Vollständigkeit und Konsistenz
4. **Lücken-Erkennung**: Falls Lücken erkannt werden, plant Brünnhilde zusätzliche Tasks
5. **ValkyrieResult-Erstellung**: Brünnhilde erstellt `ValkyrieResult` mit allen Statements und Metadaten
6. **Weiterleitung an Odin**: `ValkyrieResult` wird an Odin weitergeleitet (via Event-Dispatcher)

**ValkyrieResult-Struktur (an Thor):**
```typescript
interface ValkyrieResult {
  taskId: string;
  completed: boolean;
  summary: string;  // Zusammenfassung für Odin
  changes: {
    filesCreated: FileChange[];
    filesModified: FileChange[];
    filesDeleted: string[];
  };
  metadata: {
    statements: ValkyrieStatement[];
    qualityMetrics: QualityMetrics;
    warnings: string[];
    errors: string[];
  };
  // Thor analysiert dies und entscheidet über Actions
}

interface FileChange {
  path: string;
  content: string;  // Vollständiger Datei-Inhalt
  operation: 'create' | 'modify' | 'delete';
}
```

**Odin's Analyse von ValkyrieResult:**
- **Odin analysiert Ergebnis**: Odin analysiert `ValkyrieResult` und erkennt, ob Actions benötigt werden
- **Weiterleitung an Thor (falls nötig)**: Falls Actions benötigt werden, leitet Odin die strukturierten Ergebnisse an Thor zur Action-Execution weiter
- **Thor's Analyse**: Thor erkennt alle `filesCreated`, `filesModified`, `filesDeleted` und erstellt entsprechende Actions
- **Action-Ausführung**: Thor führt alle Actions aus (via Mjölnir)
- **Ergebnis-Rückgabe**: Thor gibt `ThorResult` an Odin zurück

## Integration mit Odin und Thor

### Event-basierte Kommunikation

**Wichtig**: Odin und Plugins kommunizieren über Event-Dispatcher. Plugins kommunizieren nicht direkt miteinander, sondern über Events.

**Event-Dispatcher-System:**
- **Pro Platform ein Event-Dispatcher**: Jede Platform hat einen zentralen Event-Dispatcher
- **Event-Registrierung**: Odin und alle Plugins registrieren sich für Events, die für sie relevant sind
- **Event-Publishing**: Services und Plugins können Events publizieren
- **Event-Subscription**: Odin und Plugins abonnieren Events basierend auf ihren Interessen

**Plugin-interne Event-Verarbeitung:**
- **FIFO-Queue**: Events werden plugin-intern auf eine FIFO-Queue gestellt
- **Sequenzielle Abarbeitung**: Events werden sequenziell abgearbeitet (FIFO)
- **Folge-Events**: Plugin kann Folge-Events erstellen und publizieren
- **Return-Events**: Plugin kann Return-Events erstellen, die zurück an Odin/andere Services geroutet werden

**Odin → Valkyries Kommunikation:**
- **Event-basiert**: Odin publiziert `CodingTaskRequest`-Event
- **Valkyries empfängt Event**: Brünnhilde empfängt Event und stellt es auf interne FIFO-Queue
- **Task-Verarbeitung**: Brünnhilde verarbeitet Task aus Queue
- **Zuständigkeits-Management**: Valkyries übernimmt Zuständigkeit via Responsibility Service (gRPC)

**Valkyries → Odin Kommunikation:**
- **Return-Event**: Brünnhilde publiziert `ValkyrieResult`-Event
- **Ergebnis-Format**: Event enthält strukturiertes `ValkyrieResult` mit:
  - Code-Änderungen (Dateien, die geändert/erstellt/gelöscht wurden)
  - Dokumentation
  - Tests
  - Metadaten (Statements, Quality-Metriken)
- **Odin empfängt Event**: Odin empfängt Event und verarbeitet es

### Action-Execution über Thor (via Events)

**Valkyries → Thor Kommunikation (via Events):**
- **Platform-Capabilities-Abfrage**: Brünnhilde fragt Platform-Capabilities: "Welche Services sind verfügbar?"
- **Service-Discovery**: Platform antwortet: "Thor verfügbar, kann `FILE_OPERATION` Actions ausführen"
- **Event-Erstellung**: Brünnhilde erstellt `ThorActionRequest`-Event (direkt zu Thor)
- **Event-Publishing**: Event wird via Event-Dispatcher an Thor geroutet
- **Thor verarbeitet Event**: Thor empfängt Event, stellt es auf interne FIFO-Queue, verarbeitet es
- **Action-Ausführung**: Thor führt Actions aus (via Mjölnir)
- **Return-Event**: Thor publiziert `ThorActionResult`-Event
- **Valkyries empfängt Ergebnis**: Brünnhilde empfängt Event und verarbeitet es

**Workflow: Odin → Valkyries → Thor (via Events) → Valkyries → Odin**

1. **Odin erkennt Coding-Aufgabe**: Odin erkennt via Einherjar Protocol, dass es sich um eine Coding-Aufgabe handelt
2. **Event-Publishing**: Odin publiziert `CodingTaskRequest`-Event
3. **Valkyries empfängt Event**: Brünnhilde empfängt Event, stellt es auf interne FIFO-Queue
4. **Brünnhilde verarbeitet Task**: Brünnhilde analysiert Task, erstellt Kanban-Board, delegiert an Sub-Valkyries
5. **Valkyries arbeiten**: Sub-Valkyries arbeiten an ihren Aufgaben (wählen Tasks aus Kanban-Board), Brünnhilde koordiniert
6. **Brünnhilde sammelt Ergebnisse**: Brünnhilde sammelt alle Statements und Ergebnisse von Valkyries
7. **Brünnhilde prüft und strukturiert**: Brünnhilde prüft Vollständigkeit und erstellt strukturiertes `ValkyrieResult`
8. **Actions benötigt**: Brünnhilde erkennt, dass Actions benötigt werden
9. **Platform-Capabilities-Abfrage**: Brünnhilde fragt Platform-Capabilities nach verfügbaren Services
10. **ThorActionRequest-Event**: Brünnhilde erstellt `ThorActionRequest`-Event (direkt zu Thor)
11. **Thor verarbeitet Event**: Thor empfängt Event, verarbeitet es, führt Actions aus
12. **ThorActionResult-Event**: Thor publiziert `ThorActionResult`-Event
13. **Brünnhilde empfängt Ergebnis**: Brünnhilde empfängt Event, integriert es in `ValkyrieResult`
14. **ValkyrieResult-Event**: Brünnhilde publiziert `ValkyrieResult`-Event
15. **Odin empfängt Ergebnis**: Odin empfängt Event und gibt Response an User zurück

## CLI Interface

### Commands
- `valkyries init` - Initialize Valkyries in project
- `valkyries task <description>` - Execute task
- `valkyries status` - Check task status
- `valkyries history` - View task history
- `valkyries config` - Configure Valkyries

### Example Usage
```bash
# Initialize in project
valkyries init

# Execute task
valkyries task "Add user authentication with JWT"

# Check status
valkyries status

# View history
valkyries history
```

## Deployment

### Als Extension zu Asgard
- **Optional**: Valkyries kann als Extension zu Asgard hinzugefügt werden
- **Installation**: User kann Valkyries separat installieren
- **Integration**: Nach Installation integriert sich Valkyries mit Asgard
- **Thor-Integration**: Thor erkennt, ob Valkyries verfügbar ist

### Automatisch bei Yggdrasil
- **Standard**: Valkyries ist automatisch bei Yggdrasil vorhanden
- **Keine Installation nötig**: User muss nichts installieren
- **Immer verfügbar**: Coding-Aufgaben können immer über Yggdrasil verarbeitet werden
- **Thor-Integration**: Thor erkennt automatisch, dass Valkyries verfügbar ist

## LLM-Konfiguration

### Standard-Konfiguration
- **Per Default**: Alle Valkyries nutzen dasselbe LLM (konfigurierbar über Geri)
- **Einheitliche Model-Auswahl**: Brünhild und alle Sub-Valkyries verwenden standardmäßig das gleiche Model
- **Konsistente Ergebnisse**: Einheitliche Model-Auswahl sorgt für konsistente Code-Qualität
- **Geri-Integration**: Model-Auswahl erfolgt über Geri (LLM Service)

### Individuelle Konfiguration
- **Konfigurierbar**: Jede Valkyrie kann ein eigenes LLM konfiguriert bekommen
- **Use-Case-spezifisch**: Verschiedene Valkyries können verschiedene Models nutzen (z.B. spezialisierte Coding-Models)
- **Konfiguration**: Über `valkyries config` oder Konfigurationsdatei (`valkyries.json`)
- **Gilt auch außerhalb von Ragnarok**: Diese Konfigurationsmöglichkeit gilt für alle Valkyries-Installationen
- **Hot-Reload**: Konfigurationsänderungen können zur Laufzeit neu geladen werden

### Konfigurationsdatei-Format

**Datei**: `valkyries.json` (im Projekt-Root oder in `~/.config/valkyries/`)

**Vollständiges Konfigurationsformat:**
```json
{
  "defaultLLM": {
    "model": "llama-3.1-8b",
    "provider": "local",
    "temperature": 0.2,
    "max_tokens": 4096,
    "top_p": 0.9,
    "frequency_penalty": 0.0,
    "presence_penalty": 0.0
  },
  "valkyries": {
    "brünnhilde": {
      "model": "llama-3.1-8b",
      "provider": "local",
      "temperature": 0.1,
      "max_tokens": 8192
    },
    "gunnr": {
      "model": "llama-3.1-8b",
      "provider": "local",
      "temperature": 0.2,
      "max_tokens": 4096
    },
    "hildr": {
      "model": "llama-3.1-8b",
      "provider": "local",
      "temperature": 0.1,
      "max_tokens": 4096
    },
    "skögul": {
      "model": "llama-3.1-8b",
      "provider": "local",
      "temperature": 0.2,
      "max_tokens": 4096
    },
    "hrist": {
      "model": "deepseek-coder-7b",
      "provider": "local",
      "temperature": 0.2,
      "max_tokens": 4096
    },
    "mist": {
      "model": "llama-3.1-8b",
      "provider": "local",
      "temperature": 0.2,
      "max_tokens": 4096
    },
    "skeggjöld": {
      "model": "llama-3.1-8b",
      "provider": "local",
      "temperature": 0.2,
      "max_tokens": 2048
    },
    "göll": {
      "model": "llama-3.1-8b",
      "provider": "local",
      "temperature": 0.3,
      "max_tokens": 4096
    },
    "geirskögul": {
      "model": "deepseek-coder-7b",
      "provider": "local",
      "temperature": 0.2,
      "max_tokens": 4096
    },
    "thrúd": {
      "model": "deepseek-coder-7b",
      "provider": "local",
      "temperature": 0.2,
      "max_tokens": 4096
    },
    "hlökk": {
      "model": "llama-3.1-8b",
      "provider": "local",
      "temperature": 0.3,
      "max_tokens": 4096
    },
    "róta": {
      "model": "llama-3.1-8b",
      "provider": "local",
      "temperature": 0.2,
      "max_tokens": 4096
    },
    "sigrún": {
      "model": "llama-3.1-8b",
      "provider": "local",
      "temperature": 0.1,
      "max_tokens": 4096
    }
  },
  "resourceLimits": {
    "maxConcurrentTasks": 5,
    "maxMemoryMB": 4096,
    "maxExecutionTimeSeconds": 300
  },
  "qualitySettings": {
    "minTestCoverage": 80,
    "requireCodeReview": true,
    "autoFormat": true,
    "lintOnSave": true
  },
  "gitIntegration": {
    "autoCommit": false,
    "commitMessageTemplate": "feat: {task_description}",
    "branchNaming": "valkyries/{task_id}"
  }
}
```

**Vereinfachtes Format (nur Model-Namen):**
```json
{
  "defaultLLM": "llama-3.1-8b",
  "valkyries": {
    "brünnhilde": "llama-3.1-8b",
    "gunnr": "llama-3.1-8b",
    "hildr": "llama-3.1-8b",
    "skögul": "llama-3.1-8b",
    "hrist": "deepseek-coder-7b",
    "mist": "llama-3.1-8b",
    "skeggjöld": "llama-3.1-8b",
    "göll": "llama-3.1-8b",
    "geirskögul": "deepseek-coder-7b",
    "thrúd": "deepseek-coder-7b",
    "hlökk": "llama-3.1-8b",
    "róta": "llama-3.1-8b",
    "sigrún": "llama-3.1-8b"
  }
}
```

**Konfigurationsfelder:**

**LLM-Konfiguration:**
- `model`: Model-Name (z.B. "llama-3.1-8b", "deepseek-coder-7b", "gpt-4")
- `provider`: Provider-Typ ("local", "cloud", "yggdrasil")
- `temperature`: Temperature für LLM (0.0-2.0, Standard: 0.2)
- `max_tokens`: Maximale Token-Anzahl (Standard: 4096)
- `top_p`: Top-P Sampling (0.0-1.0, Standard: 0.9)
- `frequency_penalty`: Frequency Penalty (Standard: 0.0)
- `presence_penalty`: Presence Penalty (Standard: 0.0)

**Resource-Limits:**
- `maxConcurrentTasks`: Maximale Anzahl gleichzeitiger Tasks
- `maxMemoryMB`: Maximale Memory-Nutzung in MB
- `maxExecutionTimeSeconds`: Maximale Ausführungszeit in Sekunden

**Quality-Settings:**
- `minTestCoverage`: Minimale Test-Coverage in Prozent
- `requireCodeReview`: Erfordert Code-Review vor Commit
- `autoFormat`: Automatisches Formatieren von Code
- `lintOnSave`: Linting beim Speichern

**Git-Integration:**
- `autoCommit`: Automatisches Committen von Änderungen
- `commitMessageTemplate`: Template für Commit-Messages
- `branchNaming`: Branch-Naming-Pattern

**Konfigurations-Validierung:**
- **Schema-Validierung**: Konfiguration wird beim Laden validiert
- **Model-Verfügbarkeit**: System prüft, ob konfigurierte Models verfügbar sind
- **Fallback**: Bei fehlenden Models wird auf `defaultLLM` zurückgegriffen
- **Fehlerbehandlung**: Ungültige Konfigurationen werden mit klaren Fehlermeldungen zurückgewiesen

## Event-Dispatcher Communication

**Event-basierte Kommunikation (Primär):**
- **Odin ↔ Valkyries**: Event-basierte Kommunikation über Event-Dispatcher
- **Plugin-interne FIFO-Queue**: Events werden plugin-intern auf FIFO-Queue gestellt und abgearbeitet
- **Event-Publishing**: Plugins publizieren Events für Folge-Events oder Return-Events
- **Platform-Capabilities**: Plugins nutzen Platform-Capabilities für Service-Discovery

**gRPC Service Communication (Fallback/Support):**
- **Einherjar Protocol**: gRPC für Funktions-Offenlegung
- **Responsibility Service**: gRPC für Zuständigkeits-Management
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

## Abhängigkeiten

### Keine Core Library

- **WICHTIG**: Es gibt keine Edda Core Library
- **Separate Projekte**: Wenn gemeinsame Komponenten benötigt werden (DTOs, Protocols, Utils), sollte ein separates Projekt erstellt werden
- **Selektive Nutzung**: Dies hält Apps klein, da genau gewählt werden kann, was benötigt wird
- **Keine Abhängigkeit**: Valkyries sollte nicht auf Dateien/Protocols/Utils aus dem `edda` Verzeichnis verweisen (KEIN PROJEKT - nur Metadaten-Sammlung)

### Service-Abhängigkeiten

- **Odin**: Für Plugin-Orchestrierung (Valkyries ist ein Plugin für Odin)
- **Thor**: Für Action-Execution, wenn Valkyries-Ergebnisse Actions benötigen (wenn Thor verfügbar)
- **Geri**: LLM Service für Code-Generierung (konfigurierbar pro Valkyrie)

### Technische Abhängigkeiten

- Git Library
- File System APIs
- Execution Environment

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

### Valkyries-spezifische Settings

**Settings-Inhalt (wird während Implementierung definiert)**
- LLM-Konfiguration pro Valkyrie (siehe Abschnitt "LLM-Konfiguration")
- Resource-Limits
- Quality-Einstellungen
- Git-Integration-Einstellungen

## Integration

- **Odin**: 
  - Empfängt `ValkyrieTask` von Odin (via Thor), sendet `ValkyrieResult` zurück
  - Empfängt `ResponsibilityRequest` von Odin, sendet `ResponsibilityResponse` zurück
  - Sendet `ResponsibilityReturn` an Odin, wenn Aufgabe nicht mehr Coding-Aufgaben sind
  - Sendet `ResponsibilityRejection` an Odin, wenn Request nicht in Valkyries Bereich ist
  - Implementiert Einherjar Protocol für Funktions-Offenlegung
- **Thor**: Erkennt Coding-Aufgaben und vermittelt zwischen Odin und Brünhild
- **Brünhild**: Verarbeitet Coding-Tasks und orchestriert Valkyries
- **Valkyries**: Führen Sub-Tasks aus
- **Event-Dispatcher**: Kommunikation über Event-Dispatcher (pro Platform)
- **Interne FIFO-Queue**: Events werden plugin-intern auf FIFO-Queue gestellt und abgearbeitet
- **Platform-Capabilities**: Nutzt Platform-Capabilities für Service-Discovery

## Performance

### Performance-Optimierungen
- **Parallele Execution**: Parallele Ausführung von Sub-Agents für schnellere Completion
- **Context-Isolation**: Isolierte Contexts reduzieren Memory-Usage und verbessern Performance
- **Effiziente Git-Operations**: Optimierte Git-Operations für schnelles Change-Tracking
- **Resource Management**: Intelligentes Resource-Management für optimale Performance
- **Caching**: Caching von häufig verwendeten Code-Patterns und LLM-Responses
- **Streaming**: Streaming von LLM-Responses für bessere UX

### Performance-Metriken
- Schnelle Task-Decomposition (< 5s für komplexe Tasks)
- Effiziente parallele Execution (mehrere Valkyries gleichzeitig)
- Optimierte Context-Management (minimaler Memory-Overhead)

## Datenschutz

### Datenschutz-Features
- **Lokale Verarbeitung**: Code wird lokal verarbeitet, keine unnötige Cloud-Übertragung
- **Minimale Datensammlung**: Nur notwendige Daten werden gespeichert
- **Keine Tracking-Daten**: Keine User-Tracking oder Analytics-Daten
- **User Control**: User hat volle Kontrolle über seine Daten
- **Code-Privacy**: Code bleibt lokal, wird nicht an Dritte weitergegeben

### Compliance
- **GDPR-konform**: Unterstützung für GDPR-Anforderungen
- **Data Minimization**: Nur notwendige Daten werden gespeichert
- **Right to Deletion**: User kann alle Daten löschen
- **Transparency**: User wird über Datenverarbeitung informiert

## Sicherheit

### Security-Features
- **Sandboxing**: Sandboxing für Code-Execution zum Schutz vor schädlichem Code
- **Input Validation**: Umfassende Validierung aller Inputs
- **Code Review**: Automatische Code-Review für Security-Issues
- **Secure Git-Operations**: Sichere Git-Operations ohne Credential-Exposure
- **Permission Checking**: Prüfung von Permissions für File-Operations
- **Audit Logging**: Logging aller Code-Änderungen für Security-Audits

### Security-Best-Practices
- **Keine Hardcoded Secrets**: Keine Hardcoded Passwords oder API-Keys im generierten Code
- **Secure Defaults**: Sichere Standard-Konfigurationen
- **Vulnerability Scanning**: Automatisches Scanning für bekannte Vulnerabilities im Code
- **Dependency Checking**: Prüfung von Dependencies auf Security-Issues
- **Code Signing**: Optional Code-Signing für generierte Artefakte

## Implementierungs-Notizen

- Sollte als CLI-Tool implementiert werden
- **Muss alle Claude Code Features übernehmen**: Besonders bewährte Features von Claude Code
- **Ralph als Referenz**: Das neue Anthropic Plugin "Ralph" für Claude Code dient als Beispiel
- **Brünhild als Orchestrator**: Brünhild delegiert und orchestriert alle anderen Valkyries
- **Multi-Instance Support**: Muss mehrere Instanzen einer Valkyrie unterstützen können
- **Statement System**: Jede Valkyrie muss Statement-System implementieren
- **Kleine Context-Fenster**: Jede Valkyrie arbeitet nur mit relevantem Code (eigene Aufgabe)
- Muss Git-Integration haben
- Sollte parallele Execution unterstützen
- Muss Context-Isolation haben
- Sollte Quality-Assurance haben
- **Vollständigkeitsprüfung**: Brünhild muss sicherstellen, dass Task korrekt und vollständig ist
- **Iterative Completion**: Falls unvollständig, werden fehlende Teile identifiziert und delegiert
- Muss Error-Handling haben
- Sollte User-Feedback geben
- **Persistent Execution**: Muss ähnlich wie Claude Code/Ralph funktionieren (nicht aufhören bis fertig)
- **LLM-Konfiguration**: Per Default nutzen alle Valkyries dasselbe LLM, aber jede Valkyrie kann individuell konfiguriert werden (gilt für alle Installationen)
- **Muss Einherjar Protocol implementieren**: Für Funktions-Offenlegung und Zuständigkeits-Domains
- **Muss Responsibility Service implementieren**: Für Zuständigkeits-Management (TakeResponsibility, ReturnResponsibility, RejectResponsibility)
- **Muss Zuständigkeits-Rückgabe haben**: Wenn Aufgabe nicht mehr Coding-Aufgaben sind
- **Muss Rückweisungs-Mechanismus haben**: Kann Requests zurückweisen, wenn nicht in Valkyries Bereich
- **Performance**: Muss optimiert sein für schnelle Code-Generierung und parallele Execution
- **Datenschutz**: Muss Privacy-by-Design implementieren und Code-Privacy gewährleisten
- **Sicherheit**: Muss robuste Security-Mechanismen haben, insbesondere für Code-Execution

