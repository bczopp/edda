# AGENTS.md - Jotunheim Development Guidelines

> **Hinweis**: Für Hintergrundinformationen zum Projekt, Features, Architektur und technische Details siehe `README.md`.

Diese Datei enthält ausschließlich Richtlinien für das Verhalten des Agents bei der Entwicklung.

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: Red (failing test) → Green (passing test) → Refactor
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests for protocol implementation, integration tests for capability negotiation, end-to-end tests for remote control workflows
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is important, but constrained by hardware limitations.

- **Optional encryption**: Encryption for sensitive data (if supported)
- **TLS encryption**: TLS encryption for network connections (if supported)
- **Authentication**: Device authentication via Heimdall (optional)
- **Input validation**: Validate all incoming commands
- **Secure key storage**: Secure storage for keys (if supported)
- **No hardcoded secrets**: Never commit secrets or keys

### 3. Performance

**Performance from the Start**: Jotunheim must be optimized for resource constraints.

- **Minimal footprint**: Optimized for minimal RAM and flash usage
- **Efficient serialization**: MessagePack for minimal data transmission
- **Low CPU usage**: Minimal CPU usage for battery life
- **Optimized network protocols**: Minimal overhead for network communication
- **Streaming support**: Efficient streaming for large data

### 4. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality.

- **Check existing protocols**: Before implementing new protocol features, check if a separate protocol project already exists
- **Reuse protocol components**: Reuse common protocol components from separate projects
- **Shared utilities**: Wenn gemeinsame Utils benötigt werden, sollte ein separates Projekt erstellt werden (minimal subset für IoT)
- **Avoid duplication**: Don't duplicate protocol implementation

### 5. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple implementations for resource-constrained devices.

- **Simple protocol**: Keep protocol simple and efficient
- **Clear capability definition**: Maintain clear capability definitions
- **Readable code**: Code should be self-documenting
- **Avoid over-engineering**: Don't add unnecessary complexity

### 6. CQRS (Command Query Responsibility Segregation)

**Software-Level Separation**: Separate read and write operations.

- **Tool calls (commands)**: Separate handlers for tool/function calls
- **Capability queries (queries)**: Separate handlers for capability queries
- **Status queries**: Separate handlers for status queries
- **Optimization**: Optimize calls and queries independently

### 7. Single Responsibility Principle

**One Responsibility Per Component**: Each component should have a single responsibility.

- **Protocol handling**: Toolcalling protocol only
- **Capability management**: Capability management only
- **Tool execution**: Tool execution only
- **Remote control**: Remote control only

### 8. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected where possible.

- **Network stack**: Inject network stack (WiFi, TCP/IP)
- **MessagePack library**: Inject MessagePack library
- **Tool handlers**: Inject tool handlers
- **Configuration**: Inject configuration

## Development Workflow

Follow this workflow for all coding tasks:

### 1. Analyze and Understand the Problem

- **Understand requirements**: Read and understand the requirements thoroughly
- **Identify constraints**: Identify hardware constraints (memory, CPU, network)
- **Research existing solutions**: Check if similar protocol features exist
- **Hardware limitations**: Understand hardware limitations (ESP32, etc.)
- **Document assumptions**: Document any assumptions you make

### 2. Plan Your Steps, Create Todos

#### 2.1. List All Classes/Structures/Functions That Need to Be Implemented

- **Identify components**: List all classes, structs, functions, and modules needed
- **Define interfaces**: Define interfaces for protocol, capability management, tool execution
- **Plan dependencies**: Identify dependencies between components
- **Create structure**: Plan the file and directory structure
- **Document design**: Document the design decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for each component
- **Test edge cases**: Include tests for edge cases and error conditions
- **Test protocol**: Write tests for toolcalling protocol implementation
- **Test capability negotiation**: Write tests for capability negotiation
- **Test tool execution**: Write tests for tool execution
- **Test resource constraints**: Write tests for resource constraint scenarios
- **Mock dependencies**: Use mocks for external dependencies (network, hardware)
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
- **Check integration**: Verify integration with controllers (Midgard/Asgard)
- **Performance check**: Verify performance requirements (minimal footprint, low CPU usage)
- **Security review**: Review security implications (encryption, authentication - if supported)
- **Resource constraints**: Verify resource constraints are met (memory, CPU, network)
- **If not complete**: If the goal is not achieved, go back to Step 1

### 5. Give a Note to the User

- **Document changes**: Document all changes made
- **Explain usage**: Explain how to use the new code
- **Provide examples**: Provide code examples if helpful
- **List dependencies**: List any new dependencies
- **Migration notes**: Include migration notes if breaking changes

## Platform-Development Guidelines

### Platform-Konzept

**Jotunheim als Platform:**
- **Platform-Rolle**: Jotunheim ist eine Platform (wie Midgard, Alfheim, Asgard, Ragnarok)
- **Connections**: Platform kümmert sich um Connections (Netzwerk, etc.)
- **Konvertierung**: Konvertiert Connections zu Anfragen an Services (Odin/Loki)
- **Platformspezifisch**: Komplett platformspezifische Implementierung (ESP32, ESP8266, etc.)
- **Service-Aufrufe**: Ruft Services (Odin, Loki) via gRPC auf

**Service-Unabhängigkeit:**
- **Services in Rust**: Alle Services (Odin, Loki, Thor, etc.) sind unabhängig von Platformen
- **gRPC-Kommunikation**: Platform kommuniziert mit Services via gRPC
- **Wiederverwendbar**: Services können von verschiedenen Platformen genutzt werden

### Loki Service Development

**Loki als unabhängiger Service:**
- **Unabhängig von Jotunheim**: Loki ist ein unabhängiger Service, nicht Teil von Jotunheim-Platform
- **Script-Execution**: User-generierte Scripte per gRPC zugänglich machen
- **Dynamische gRPC-Funktionen**: Jedes Script wird zu einer aufrufbaren gRPC-Funktion
- **Direkte Ausführung**: Funktion führt Script direkt auf Device aus (nichts anderes)
- **Koordination der 3 Kinder**: Fenrir, Jörmungandr, Hel

**Script-Execution-Guidelines:**
- **Nur performante Sprachen**: Lua (primär), ggf. leichtere Rust-Version wenn signifikante Verbesserungen
- **Direkte Ausführung**: Keine Sandbox für Performance
- **Device-abhängig**: Script-Sprache abhängig vom Device (ESP32 = Lua, größere Devices = mehr Optionen)
- **Resource-Management**: Scripts müssen Resource-Limits beachten
- **gRPC-Funktion pro Script**: Jedes Script wird zu einer aufrufbaren gRPC-Funktion

**Die 3 Kinder von Loki:**
- **Fenrir**: Aggressive Tasks, Hardware-Control (GPIO, Sensors, Actuators, Low-level Hardware-Access)
- **Jörmungandr**: Network/Communication (HTTP/HTTPS, WebSocket, MQTT, Network-Protocol-Handling)
- **Hel**: Data/Storage (File-System, Data-Processing, Cache-Management, Data-Aggregation)

**Bifrost-Verbindung:**
- **Optional**: Bifrost-Verbindung nur bedingt nötig
- **gRPC-Streams**: Wenn gRPC-Streams möglich sind und verschlüsselt (TLS) oder in abgesichertem Netzwerk, brauchen wir keine Bifrost-Verbindung
- **Verschlüsselung**: Streams müssen verschlüsselt übertragen werden (TLS) oder in abgesichertem Netzwerk
- **Fallback**: Bifrost als Fallback wenn gRPC-Streams nicht möglich

## Service-Specific Guidelines

### Toolcalling Protocol

- **gRPC-based**: Type-safe, effizient, HTTP/2
- **Protobuf**: Binary, kompakt, automatische Code-Generierung
- **Token-efficient**: Significantly less overhead than MCP
- **Streaming support**: Built-in gRPC-Streaming für große Responses
- **Error recovery**: Robust error handling with Status-Codes
- **Type-Safe**: Protobuf garantiert korrekte Tool-Definitionen

### Capability Negotiation

- **Variable configuration**: Jotunheim devices have variable configurations (sensors, actuators, interfaces)
- **Capability structure**: Device information, available tools/functions, hardware capabilities, resource limits, protocol features
- **Negotiation flow**: Connection establishment, capability request, capability processing, tool discovery & registration, dynamic capability updates
- **Automatic propagation**: Capabilities automatically propagated on connection
- **On-demand updates**: Capabilities can be requested on demand
- **Event-based updates**: Automatic capability updates for important changes

### Capability Synchronization

- **Peer-to-peer (standard)**: Direct synchronization between devices
- **Asgard as hub**: Asgard takes over capability synchronization when present
- **Leading server**: Oldest server becomes leading server when multiple Asgard servers in same network
- **Event-based cache invalidation**: Cache invalidated on capability update events
- **Timeout as fallback**: Timeout-based cache invalidation as fallback

### Remote Control

- **Receive commands**: Receive commands from other devices
- **Execute commands**: Execute commands locally
- **Send results**: Send results back to controller
- **Tool exposure**: Expose available tools/functions
- **Capability negotiation**: Negotiate capabilities with controller

### OTA Updates

- **Local updates via Asgard**: Asgard as local update server
- **Global updates via Yggdrasil**: Yggdrasil as global update server
- **Update verification**: Digital signatures + checksum verification
- **Automatic rollback**: Automatic rollback when possible on failed updates
- **Manual rollback**: Manual rollback if automatic not possible
- **Capability updates**: Send new capabilities after update

### Implementation Constraints

- **Memory constraints**: Must work with minimal RAM
- **Stack management**: Efficient stack management
- **Memory pooling**: Memory pooling for better performance
- **CPU constraints**: Minimal CPU usage
- **Async processing**: Async processing
- **Task scheduling**: Efficient task scheduling
- **Network constraints**: Optimized for low bandwidth
- **Connection resilience**: Robust error handling
- **Reconnection**: Automatic reconnection

## Testing Requirements

### Container-Based Testing (MANDATORY)

- **All tests in containers**: ALL tests MUST run in containers - nothing on local system
- **No local installation**: No dependencies, tools, or services installed on local development machine
- **Docker/Container setup**: Project must provide Docker/container setup for testing
- **Isolated test environment**: Tests run in completely isolated container environment
- **Reproducible**: Test environment must be reproducible across all development machines
- **CI/CD ready**: Container setup must work in CI/CD pipelines

### Test-Driven Development (MANDATORY)

- **Strict TDD**: Test-Driven Development is MANDATORY - no exceptions
- **Tests first**: ALWAYS write tests before implementation code
- **No implementation without tests**: Never write implementation code without corresponding tests first
- **Red-Green-Refactor**: Strictly follow Red-Green-Refactor cycle

### Unit Tests

- **Protocol implementation**: Test toolcalling protocol implementation
- **Capability management**: Test capability negotiation and management
- **Tool execution**: Test tool execution logic
- **Resource constraints**: Test resource constraint handling
- **Container-based**: All unit tests must run in container environment

### Integration Tests

- **Remote control workflows**: Test complete remote control workflows
- **Capability synchronization**: Test capability synchronization
- **OTA updates**: Test OTA update workflows
- **Error handling**: Test error handling and recovery
- **Container-based**: All integration tests must run in container environment

### Test Coverage

- **Minimum coverage**: Maintain minimum 80% test coverage
- **Critical paths**: 100% coverage for critical paths (protocol, capability negotiation)
- **Resource code**: High coverage for resource management code
- **Avoid redundancy**: Avoid redundant test logic - reuse test utilities and helpers

## Security Considerations

### IoT Security

- **Optional encryption**: Encryption for sensitive data (if hardware supports)
- **TLS encryption**: TLS encryption for network connections (if hardware supports)
- **Authentication**: Device authentication via Heimdall (optional)
- **Input validation**: Validate all incoming commands
- **Secure key storage**: Secure storage for keys (if hardware supports)
- **Minimal attack surface**: Minimal attack surface through lightweight design

### Update Security

- **Digital signatures**: All updates must be digitally signed
- **Checksum verification**: Checksum verification (MD5/SHA256)
- **Update verification**: Verify signature and checksum before installation
- **Trusted sources**: Only signed updates from trusted sources

### GDPR Compliance (EU/German Data Protection)

- **Data minimization**: Only transmit necessary data
- **Data encryption**: Encrypt all personal data in transit (if hardware supports)
- **Access control**: Strict access control to prevent unauthorized data access
- **Privacy by design**: Consider privacy from the design phase (within hardware constraints)

## Performance Requirements

### Resource Performance

- **Minimal RAM usage**: Minimal RAM usage (< 10KB for base implementation)
- **Fast tool execution**: Fast tool execution (< 100ms for simple tools)
- **Efficient network communication**: Efficient network communication (minimal overhead)
- **Low CPU usage**: Low CPU usage for battery life

### Resource Management

- **Memory management**: Efficient memory management for resource constraints - minimize RAM consumption
- **CPU management**: Optimize CPU usage
- **Network management**: Optimize network usage
- **Code efficiency**: Write efficient, short code that uses minimal resources
- **Code brevity**: Code should be as short as possible while maintaining readability
- **Failsafe**: Code must be failsafe - no errors that could crash the device
- **User experience**: Ensure pleasant user experience - do not slow down user devices

## Additional Resources

- See `jotunheim/README.md` for detailed service documentation
- Das `edda` Verzeichnis ist **KEIN PROJEKT**. Es dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte. Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden (minimal subset für IoT).
- Note: For general development principles, see `AGENTS.md` in the parent directory (informational reference only)

