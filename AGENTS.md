# AGENTS.md - Edda Development Guidelines

## Project Overview

Edda is a distributed, privacy-focused AI assistant system built with a microservices architecture. The system is named after Norse mythology, with each service representing a different mythological entity. The system enables users to interact with AI assistants across multiple devices (desktop, mobile, server, terminal, IoT) while maintaining strict privacy, security, and local-first principles.

**WICHTIG**: Das `edda` Verzeichnis ist **KEIN PROJEKT**. Es dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte.

## Architecture Overview

Edda follows a microservices architecture where each service is independently deployable and responsible for a specific domain:

- **Core Services**: Odin (orchestration with vision-model support), Thor (action execution), Freki (RAG), Geri (LLM with vision-model support), Huginn/Muninn (STT/TTS + data forwarding for images/videos), Bifrost (communication), Heimdall (security), Skuld (LLM selection), Loki (script execution), Forseti (ML/DL/RL)
- **Device Platforms**: Midgard (desktop), Alfheim (mobile), Asgard (homeserver), Ragnarok (terminal), Jotunheim (IoT)
- **Plugins**: Valkyries (coding agent), Frigg (healthcare)
- **Infrastructure**: Yggdrasil (cloud server), Mimir (privacy database), Nornen (decision service), and other supporting services

**Platform-Konzept:**
- **Alle Orte (außer Yggdrasil) sind Platformen**: Midgard, Alfheim, Asgard, Ragnarok, Jotunheim
- **Platform-Rolle**: Platformen kümmern sich um Connections (Netzwerk, UI, etc.) und konvertieren diese zu Anfragen an Services
- **Service-Unabhängigkeit**: Services (Odin, Loki, Thor, etc.) sind unabhängig von Platformen (alles in Rust)
- **gRPC-Kommunikation**: Platformen kommunizieren mit Services via gRPC

Services communicate via:
- **gRPC**: For service-to-service communication (on-device and cross-device)
  - **Loki Function Calls**: gRPC for IoT device toolcalling (via Loki Service für Jotunheim-Devices)
  - **Loki Script Execution**: gRPC for Loki service (user-generated scripts)
  - **Cross-Device Actions**: gRPC for ThorAction/ThorResult between devices
  - **On-Device Services**: gRPC for Odin ↔ Thor, Freki, Geri, Skuld
  - **Plugin Communication**: gRPC for Odin ↔ Plugins
  - **Yggdrasil Microservices**: gRPC for Yggdrasil ↔ Rust-Microservices
  - **Yggdrasil API**: gRPC for Request/Response-Patterns with Yggdrasil (Device-Registry, User-Management, etc.)
  - **Einherjar Protocol**: gRPC for function discovery and capability exposure (all gods/services)
  - **Responsibility Service**: gRPC for responsibility management (all gods/services)
  - **Vision Service**: gRPC for image/video analysis (Odin ↔ Geri)
  - **Huginn Data Service**: gRPC for data forwarding (text, images, videos, video streams)
- **Bifrost Protocol**: Device-Mesh (Meshtastic-inspiriert, IP + optional LoRa) + WebSocket/Application-Protocol für device-to-device communication (messaging, events, connection establishment)
- **Ratatoskr Protocol**: WebSocket-based business-logic communication with Yggdrasil (persistent connections for Marketplace, Payments, etc.)

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: 
  - Red: Write failing test
  - Green: Write minimal code to pass
  - Refactor: Improve code while keeping tests green
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests, integration tests, and end-to-end tests as appropriate
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is not an afterthought but a fundamental requirement.

- **Input validation**: All inputs must be validated and sanitized
- **Secure defaults**: Default configurations must be secure
- **No hardcoded secrets**: Never commit secrets, passwords, or API keys
- **Authentication & Authorization**: Proper authentication and authorization for all operations
- **Encryption**: Sensitive data must be encrypted at rest and in transit
- **Security audits**: Regular security reviews and vulnerability scanning

### 3. Performance

**Performance from the Start**: Performance considerations must be part of the initial design.

- **Resource management**: Efficient use of CPU, memory, and network resources
- **Optimization strategies**: Profile before optimizing, measure improvements
- **Async/await**: Use asynchronous operations for I/O-bound tasks
- **Caching**: Implement intelligent caching where appropriate
- **Connection pooling**: Reuse connections efficiently
- **Load testing**: Test under realistic load conditions

### 4. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality before implementing new code.

- **Check existing services**: Before implementing new functionality, check if a service or function already exists
- **Reuse existing code**: Use existing functions, services, and utilities
- **Avoid duplication**: Don't duplicate code; extract common functionality
- **Shared components**: Wenn gemeinsame Komponenten (DTOs, Protocols, Utils) benötigt werden, sollten separate Projekte erstellt werden
- **Service discovery**: Know what other services provide and use them

### 5. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple, clear solutions over complex ones.

- **Simple solutions**: Choose the simplest solution that works
- **Avoid over-engineering**: Don't add complexity "just in case"
- **Clear code**: Code should be self-documenting and easy to understand
- **Readable over clever**: Prefer readable code over clever optimizations
- **Progressive enhancement**: Start simple, add complexity only when needed
- **Code brevity**: Code should be as short as possible while maintaining readability
- **Minimal design**: Beautiful but simple to minimalist design - avoid unnecessary complexity

### 6. CQRS (Command Query Responsibility Segregation)

**Software-Level Separation**: Separate read and write operations at the software level, not just the database.

- **Command handlers**: Separate handlers for write operations (commands)
- **Query handlers**: Separate handlers for read operations (queries)
- **Different models**: Commands and queries can use different data models
- **Optimization**: Optimize reads and writes independently
- **Clear boundaries**: Clear separation between command and query paths

### 7. Single Responsibility Principle

**One Responsibility Per Component**: Each class, function, or module should have a single, well-defined responsibility.

- **Focused classes**: Each class should do one thing well
- **Clear responsibilities**: Responsibilities should be clearly defined
- **Separation of concerns**: Separate different concerns into different modules
- **Maintainability**: Easier to understand, test, and maintain
- **Refactoring**: Easier to refactor when responsibilities are clear

### 8. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected, not created within the component.

- **Constructor injection**: Prefer constructor injection for required dependencies
- **Interface-based**: Depend on interfaces, not concrete implementations
- **Testability**: Makes testing easier with mock dependencies
- **Flexibility**: Allows swapping implementations without code changes
- **Configuration**: Dependencies can be configured externally

## Development Workflow

Follow this workflow for all coding tasks:

### 1. Analyze and Understand the Problem

- **Understand requirements**: Read and understand the requirements thoroughly
- **Identify constraints**: Identify technical, performance, and security constraints
- **Research existing solutions**: Check if similar functionality exists in the codebase
- **Ask questions**: Clarify any ambiguities before starting
- **Document assumptions**: Document any assumptions you make

### 2. Plan Your Steps, Create Todos

#### 2.1. List All Classes/Structures/Functions That Need to Be Implemented

- **Identify components**: List all classes, structs, functions, and modules needed
- **Define interfaces**: Define interfaces and contracts between components
- **Plan dependencies**: Identify dependencies between components
- **Create structure**: Plan the file and directory structure
- **Document design**: Document the design decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for each component
- **Test edge cases**: Include tests for edge cases and error conditions
- **Test integration**: Write integration tests for component interactions
- **Mock dependencies**: Use mocks for external dependencies
- **Test data**: Prepare test data and fixtures
- **Container setup**: Ensure all tests can run in container environment
- **No local dependencies**: Tests must not require any local system installation

#### 2.3. Run the Tests and Expect Them to Fail

- **Run test suite**: Execute all tests
- **Verify failures**: Confirm that tests fail as expected (Red phase)
- **Document failures**: Note what failures are expected
- **Check coverage**: Ensure test coverage is comprehensive

#### 2.4. Create the Code and Make the Tests Run Successfully

- **Implement minimally**: Write minimal code to make tests pass (Green phase)
- **Run tests frequently**: Run tests after each small change
- **Fix all tests**: Always try to correct all tests and let them run again all together to save time
- **Or focus on class**: Or just run the tests of the class you currently work on
- **Iterate**: Continue until all tests pass

### 3. Step by Step Work on the Todos

- **Work systematically**: Complete todos one by one
- **Run tests**: Run tests after each todo completion
- **Refactor**: Refactor code while keeping tests green
- **Document**: Document code as you go
- **Review**: Review your code before moving to the next todo

### 4. Check Again if the Task's Goal is Achieved

- **Verify requirements**: Ensure all requirements are met
- **Test thoroughly**: Run the full test suite
- **Check integration**: Verify integration with other services
- **Performance check**: Verify performance requirements are met
- **Security review**: Review security implications
- **If not complete**: If the goal is not achieved, go back to Step 1

### 5. Give a Note to the User

- **Document changes**: Document all changes made
- **Explain usage**: Explain how to use the new code
- **Provide examples**: Provide code examples if helpful
- **List dependencies**: List any new dependencies
- **Migration notes**: Include migration notes if breaking changes

### Settings und Hot-Reload (Konfiguration)

- **Verpflichtend:** Infra- und Core-Services, die in IMPLEMENTATION_PLAN oder AGENTS.md „Hot-Reload“ oder „Runtime-Konfigurationsänderung“ erwähnen, müssen Hot-Reload unterstützen.
- **Technik:** `notify` + `Arc<RwLock<Settings>>` + Validierung beim Laden; eigener Fehlertyp (z. B. thiserror) für Settings-Fehler.
- **Optional:** Plattformen und reine Protokolle (z. B. Ratatoskr) – „Load once“ reicht.
- **Dokumentation:** In IMPLEMENTATION_PLAN und/oder README pro Projekt kurz „Hot-Reload: ja/nein“ vermerken.

## Project Structure

```
edda/
├── AGENTS.md                    # This file
├── alfheim/                     # Mobile platform
├── asgard/                      # Homeserver platform
├── bifrost/                     # Communication service
├── docs/                        # Documentation
├── edda/                        # Metadaten-Sammlung (KEIN PROJEKT)
├── freki/                       # RAG service
├── frigg/                       # Healthcare plugin
├── geri/                        # LLM service
├── heimdall/                    # Security service
├── huginn-muninn/               # STT/TTS service
├── jotunheim/                      # IoT devices
├── midgard/                     # Desktop platform
├── mimir/                       # Privacy database service
├── nidhoggr/                    # Connection endpoint
├── nornen/                      # Decision service
├── odin/                        # Main orchestrator
├── ragnarok/                    # Terminal platform
├── ratatoskr/                   # Business protocol
├── skuld/                       # LLM selection service
├── thor/                        # Action executor
├── valkyries/                   # Coding agent plugin
├── vedrfolnir/                  # Connection builder client
└── yggdrasil/                   # Cloud server
```

## Technology Stack

- **Rust**: Primary language for all services (performance, memory safety, cross-platform)
- **Elixir**: Yggdrasil cloud server (massive concurrency, fault tolerance)
- **TypeScript**: Frontend applications only (Midgard, Alfheim, Asgard GUIs - nur UI, keine Backend-Logik)
- **Rust**: Alle Backend-Logik (Services und Platform-Logik)
- **gRPC**: Service-to-service communication
- **WebSocket**: Bifrost protocol for device-to-device communication
- **Protobuf**: Message definitions
- **MessagePack**: IoT device communication

## Code Quality Standards

- **Linting**: All code must pass linting checks
- **Formatting**: Code must be formatted according to project standards
- **Documentation**: Public APIs must be documented
- **Error handling**: Comprehensive error handling with proper error types
- **Logging**: Appropriate logging levels and structured logging
- **Monitoring**: Instrument code for monitoring and observability
- **Avoid redundancy**: Avoid redundancy in code, especially in tests - reuse test utilities and helpers
- **Test organization**: Organize tests to avoid duplicate test logic
- **Code brevity**: Code should be as short as possible while maintaining readability
- **Minimal memory footprint**: Minimize memory usage and RAM consumption
- **Resource efficiency**: Only use as much RAM as necessary - do not slow down user devices
- **User experience**: Ensure pleasant user experience - products should not slow down devices

## Security Standards

- **No secrets in code**: Use environment variables or secure secret management
- **Input validation**: Validate and sanitize all inputs
- **Output encoding**: Encode outputs to prevent injection attacks
- **Authentication**: Proper authentication for all endpoints
- **Authorization**: Role-based access control where appropriate
- **Encryption**: Encrypt sensitive data at rest and in transit
- **Audit logging**: Log security-relevant events
- **API security**: All API endpoints must be secured to prevent unauthorized data access
- **WebSocket security**: All WebSocket connections must be secured to prevent unauthorized data access
- **Data access control**: Strict access control to ensure unauthorized users cannot access data

## Performance Standards

- **Response times**: Meet defined response time requirements
- **Resource usage**: Monitor and optimize resource usage
- **Scalability**: Design for horizontal scaling where needed
- **Caching**: Implement caching strategies appropriately
- **Database queries**: Optimize database queries and use indexes
- **Connection management**: Efficient connection pooling and reuse
- **Minimal memory footprint**: Minimize memory usage and RAM consumption
- **Resource efficiency**: Only use as much RAM as necessary - do not slow down user devices
- **Code efficiency**: Write efficient code that uses minimal resources
- **Performance optimization**: Continuously optimize for performance and resource usage
- **User experience**: Ensure pleasant user experience - products should not slow down devices

## Testing Standards

### Container-Based Testing (MANDATORY)

- **All tests in containers**: ALL tests MUST run in containers - nothing on local system
- **No local installation**: No dependencies, tools, or services installed on local development machine
- **Docker/Container setup**: Each project must provide Docker/container setup for testing
- **Docker-Build-Kontext:** Standard (`context: .`, `dockerfile: Dockerfile.test`) und Ausnahme bei Path-Dependencies siehe `docs/test-infrastructure-template.md` (Abschnitt „Docker-Build-Kontext“).
- **Isolated test environment**: Tests run in completely isolated container environment
- **Reproducible**: Test environment must be reproducible across all development machines
- **CI/CD ready**: Container setup must work in CI/CD pipelines

### Test-Driven Development (MANDATORY)

- **Strict TDD**: Test-Driven Development is MANDATORY - no exceptions
- **Tests first**: ALWAYS write tests before implementation code
- **No implementation without tests**: Never write implementation code without corresponding tests first
- **Red-Green-Refactor**: Strictly follow Red-Green-Refactor cycle
- **Test coverage**: Maintain high test coverage (minimum 80% for critical paths)
- **Test performance**: Tests should run quickly
- **Test data**: Use realistic test data
- **Mock external services**: Mock external dependencies in tests
- **Avoid redundancy**: Avoid redundancy in tests, especially duplicate test logic across different test files
- **Critical test areas**: API tests and WebSocket tests are critical test areas - everything must work and be secured
- **API security testing**: All API endpoints must be tested for unauthorized access prevention
- **WebSocket security testing**: All WebSocket connections must be tested for unauthorized data access prevention

### Test Types

**Unit Tests:**
- **Purpose**: Test individual components, functions, and modules in isolation
- **Scope**: Single component or function
- **Dependencies**: Mock all external dependencies (services, databases, network)
- **Speed**: Fast execution (milliseconds per test)
- **Coverage**: High coverage for business logic and critical paths
- **Examples**: Service-specific logic, serialization, utilities, cryptographic operations

**Integration Tests:**
- **Purpose**: Test interactions between multiple components or services
- **Scope**: Multiple components or services working together
- **Dependencies**: Use real or test implementations of dependencies (databases, services)
- **Speed**: Slower execution (seconds per test)
- **Coverage**: Test critical integration paths and workflows
- **Examples**: Service integration, protocol compatibility, API endpoints, WebSocket connections

**End-to-End (E2E) Tests:**
- **Purpose**: Test complete workflows from user perspective
- **Scope**: Entire system or major subsystems
- **Dependencies**: Use real or near-real implementations (test databases, test services)
- **Speed**: Slowest execution (minutes per test)
- **Coverage**: Test critical user workflows and scenarios
- **Examples**: Complete user workflows, cross-service communication, device-to-device communication

### Testing Distributed Systems

**Service Isolation:**
- **Mock Services**: Mock external services for unit tests
- **Test Services**: Use test implementations of services for integration tests
- **Service Containers**: Run services in separate containers for integration tests
- **Network Simulation**: Simulate network conditions (latency, packet loss, disconnections)

**Distributed System Testing Strategies:**
- **Contract Testing**: Test service contracts (gRPC, WebSocket, API)
- **Chaos Testing**: Test system behavior under failure conditions (service failures, network partitions)
- **Load Testing**: Test system behavior under load (concurrent requests, high throughput)
- **Consistency Testing**: Test data consistency across distributed services
- **Eventual Consistency**: Test eventual consistency scenarios (replication, synchronization)

**Test Infrastructure:**
- **Test Containers**: Use test containers for databases, message queues, etc.
- **Service Mesh**: Use service mesh for service-to-service communication testing
- **Mock Servers**: Use mock servers for external API dependencies
- **Test Data Management**: Manage test data across multiple services

### Edge Case Testing

**Error Scenarios:**
- **Network Failures**: Test behavior when network connections fail
- **Service Failures**: Test behavior when services are unavailable
- **Timeout Scenarios**: Test behavior when requests timeout
- **Invalid Input**: Test behavior with invalid or malformed input
- **Resource Exhaustion**: Test behavior when resources are exhausted (memory, connections)

**Concurrency Scenarios:**
- **Race Conditions**: Test for race conditions in concurrent operations
- **Deadlocks**: Test for deadlock scenarios
- **Lock Contention**: Test lock contention and resolution
- **Concurrent Requests**: Test handling of concurrent requests

**Boundary Conditions:**
- **Empty Data**: Test behavior with empty data structures
- **Maximum Values**: Test behavior with maximum values (size limits, count limits)
- **Minimum Values**: Test behavior with minimum values
- **Null/None Values**: Test behavior with null or missing values

**Security Edge Cases:**
- **Unauthorized Access**: Test prevention of unauthorized access
- **Injection Attacks**: Test prevention of injection attacks (SQL, command, etc.)
- **Rate Limiting**: Test rate limiting and throttling
- **Token Expiration**: Test behavior with expired tokens
- **Malformed Requests**: Test handling of malformed or malicious requests

### Test Organization

**Test Structure:**
- **Test Files**: Organize tests by component or feature
- **Test Utilities**: Reuse test utilities and helpers to avoid redundancy
- **Test Fixtures**: Use test fixtures for common test data
- **Test Factories**: Use test factories for creating test objects
- **Verzeichnisstruktur:** Alle Projekte verwenden, wo sinnvoll, `tests/utils` und bei Bedarf `tests/mocks`; die Trennung `tests/integration/` vs. `tests/unit/` (oder Tests direkt unter `tests/`) ist empfohlen. Details siehe `docs/test-infrastructure-template.md`.

**Test Naming:**
- **Descriptive Names**: Use descriptive test names that explain what is being tested
- **Given-When-Then**: Structure test names using Given-When-Then pattern (optional)
- **Test Categories**: Use test categories or tags for test organization

**Test Maintenance:**
- **Keep Tests Simple**: Keep tests simple and focused on one scenario
- **Avoid Test Interdependencies**: Avoid dependencies between tests
- **Regular Refactoring**: Regularly refactor tests to maintain clarity
- **Update Tests with Code**: Update tests when code changes

## Documentation Standards

- **README**: Each service must have a comprehensive README
- **API documentation**: Document all public APIs
- **Code comments**: Comment complex logic and algorithms
- **Architecture decisions**: Document significant architecture decisions
- **Changelog**: Maintain changelog for releases
- **IMPLEMENTATION_PLAN:** Bei Merge/Abnahme: IMPLEMENTATION_PLAN-Status des betroffenen Projekts prüfen und anpassen (Checkboxen, „completed“-Vermerke nach Abschluss einer Phase oder eines Meilensteins).

### API Documentation

**Dokumentationsformat:**
- **OpenAPI/Swagger**: OpenAPI 3.0 Specification für REST-APIs
- **GraphQL Schema**: GraphQL Schema-Dokumentation für GraphQL-APIs
- **gRPC Documentation**: Protocol Buffers mit Kommentaren für gRPC-Services
- **Postman Collections**: Postman Collections für API-Testing (optional)

**API-Dokumentations-Generierung:**
- **Code-First**: API-Dokumentation wird aus Code generiert (Annotations, Decorators)
- **Schema-First**: API-Dokumentation wird aus OpenAPI/GraphQL-Schema generiert
- **Automatische Generierung**: API-Dokumentation wird automatisch bei Code-Änderungen aktualisiert
- **CI/CD Integration**: API-Dokumentation wird in CI/CD-Pipeline generiert

**Dokumentations-Tools:**
- **OpenAPI/Swagger**: Swagger UI, ReDoc, Redocly
- **GraphQL**: GraphQL Playground, GraphiQL
- **gRPC**: gRPC-Web, grpcurl
- **Generatoren**: Docusaurus, MkDocs, Sphinx, GitBook

**Dokumentations-Hosting:**
- **GitHub Pages**: Statische Dokumentation auf GitHub Pages
- **Dedicated Documentation Site**: Separate Dokumentations-Website (z.B. docs.edda.ai)
- **Integrated in Application**: Dokumentation als Teil der Anwendung (z.B. `/docs` Endpoint)
- **Versionierung**: Dokumentation wird versioniert (z.B. `/docs/v1`, `/docs/v2`)

**API-Dokumentations-Inhalt:**
- **Endpoint-Beschreibungen**: Vollständige Beschreibung aller Endpoints
- **Request/Response-Beispiele**: Beispiele für Requests und Responses
- **Authentication**: Authentifizierungs-Workflow und Token-Management
- **Rate-Limits**: Rate-Limit-Informationen
- **Error-Codes**: Vollständige Liste aller Error-Codes
- **SDKs**: SDK-Dokumentation für verschiedene Sprachen (optional)

### User Documentation

**Dokumentationsstruktur:**
- **Getting Started Guide**: Erste Schritte mit Edda
- **User Guides**: Detaillierte Anleitungen für alle Features
- **Tutorials**: Schritt-für-Schritt Tutorials
- **API-Dokumentation**: Für Entwickler
- **Plugin-Entwicklung**: Guides für Plugin-Entwicklung
- **Troubleshooting**: Häufige Probleme und Lösungen
- **FAQ**: Häufig gestellte Fragen

**Dokumentations-Tools:**
- **Markdown**: Markdown für Dokumentations-Inhalt
- **Docusaurus**: Für strukturierte Dokumentations-Website
- **MkDocs**: Alternative zu Docusaurus
- **Sphinx**: Für Python-bezogene Dokumentation
- **GitBook**: Für interaktive Dokumentation

**Tutorial-Struktur:**
- **Schritt-für-Schritt**: Klare Schritt-für-Schritt-Anleitungen
- **Code-Beispiele**: Praktische Code-Beispiele
- **Screenshots**: Screenshots für visuelle Anleitung
- **Prerequisites**: Voraussetzungen für jedes Tutorial
- **Expected Results**: Erwartete Ergebnisse nach jedem Schritt
- **Troubleshooting**: Häufige Probleme und Lösungen pro Tutorial

**Video-Tutorials:**
- **Integration**: Video-Tutorials werden in Dokumentation eingebettet
- **Hosting**: Videos werden auf YouTube, Vimeo oder ähnlichen Plattformen gehostet
- **Embedding**: Videos werden in Dokumentations-Seiten eingebettet
- **Transkripte**: Transkripte für Video-Tutorials (für Barrierefreiheit)
- **Kapitel-Markierungen**: Kapitel-Markierungen für schnelle Navigation

**Dokumentations-Workflow:**
- **Valkyries Integration**: Hlökk (Documentation Agent) generiert und aktualisiert Dokumentation
- **Versionierung**: Dokumentation wird mit Code versioniert
- **Review-Prozess**: Dokumentation wird im Review-Prozess geprüft
- **Automatische Updates**: Dokumentation wird automatisch bei Code-Änderungen aktualisiert

## Getting Started

1. **Read the README**: Read the service-specific README for context
2. **Read AGENTS.md**: Read this file and the service-specific AGENTS.md
3. **Set up environment**: Set up development environment
4. **Run tests**: Run existing tests to ensure environment is set up correctly
5. **Follow workflow**: Follow the development workflow for all tasks

## Data Protection and Privacy (GDPR Compliance)

**EU/German Data Protection**: All projects must comply with EU and German data protection regulations (GDPR).

### GDPR Requirements

- **Data minimization**: Only collect and process data that is strictly necessary
- **Purpose limitation**: Data may only be used for the specified purpose
- **Storage limitation**: Data must not be stored longer than necessary
- **Data accuracy**: Ensure data accuracy and allow corrections
- **Integrity and confidentiality**: Ensure data security and confidentiality
- **Right to access**: Users have the right to access their data
- **Right to rectification**: Users have the right to correct their data
- **Right to erasure**: Users have the right to delete their data ("Right to be forgotten")
- **Right to data portability**: Users have the right to export their data
- **Right to object**: Users have the right to object to data processing
- **Privacy by design**: Privacy must be considered from the design phase
- **Privacy by default**: Default settings must be privacy-friendly

### Implementation Requirements

- **Encryption**: Encrypt all personal data at rest and in transit
- **Access control**: Strict access control to prevent unauthorized data access
- **Audit logging**: Log all data access for compliance auditing
- **Data deletion**: Implement secure data deletion mechanisms
- **Data export**: Provide mechanisms for data export
- **Consent management**: Implement consent management where required
- **Privacy notices**: Provide clear privacy notices to users

### German-Specific Requirements

- **BDSG compliance**: Comply with Bundesdatenschutzgesetz (BDSG) requirements
- **Data processing agreements**: Ensure proper data processing agreements are in place
- **Data protection officer**: Consider requirements for data protection officer
- **Breach notification**: Implement breach notification procedures as required by GDPR

## Design Principles

### Code Efficiency

- **Short code**: Code should be as short as possible while maintaining readability and functionality
- **Minimal memory footprint**: Minimize memory usage and RAM consumption
- **Resource efficiency**: Only use as much RAM as necessary - do not slow down user devices
- **Efficient algorithms**: Use efficient algorithms and data structures
- **Lazy loading**: Load resources only when needed
- **Memory management**: Proper memory management and cleanup

### Performance

- **Failsafe**: Code must be failsafe - no errors that could crash the system
- **Performant**: High performance is essential - optimize for speed and efficiency
- **User experience**: Ensure pleasant user experience - products should not slow down devices
- **Responsive**: Applications should remain responsive even under load
- **Resource monitoring**: Continuously monitor and optimize resource usage

### Design

- **Beautiful but simple**: Beautiful but simple to minimalist design
- **Minimalist UI**: Avoid unnecessary UI complexity
- **Clean interfaces**: Clean and intuitive interfaces
- **User-friendly**: Focus on user experience and ease of use
- **Performance over features**: Prioritize performance over unnecessary features

## Additional Resources

- Service-specific README files provide detailed information about each service
- Service-specific AGENTS.md files provide service-specific development guidelines
- Documentation in `docs/` directory provides architecture and planning information
- Das `edda` Verzeichnis ist **KEIN PROJEKT**. Es dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte. Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden.

