# AGENTS.md - Bifrost Development Guidelines

> **Hinweis**: Für Hintergrundinformationen zum Projekt, Features, Architektur und technische Details siehe `README.md`.

Diese Datei enthält ausschließlich Richtlinien für das Verhalten des Agents bei der Entwicklung.

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: Red (failing test) → Green (passing test) → Refactor
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests for message routing, integration tests for connection management, end-to-end tests for device-to-device communication
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is critical for inter-device communication.

- **TLS encryption**: TLS 1.3 for all connections
- **Connection authentication**: Authenticate all connections via Heimdall
- **Message validation**: Validate all messages
- **Certificate validation**: Validate TLS certificates
- **No hardcoded secrets**: Never commit secrets or keys

### 3. Performance

**Performance from the Start**: Bifrost must provide fast message transmission.

- **Efficient routing**: Optimized message routing for minimal latency
- **Connection pooling**: Efficient connection pooling
- **Message queuing**: Queue messages for offline devices
- **Parallel processing**: Parallel processing for multiple connections
- **Caching**: Cache connection information
- **Minimal memory footprint**: Minimize memory usage and RAM consumption
- **Code brevity**: Code should be as short as possible while maintaining readability
- **Failsafe**: Code must be failsafe - no errors that could crash the system
- **User experience**: Ensure pleasant user experience - do not slow down user devices

### 4. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality.

- **Check existing protocols**: Before implementing new protocol features, check if a separate protocol project already exists
- **Reuse connection logic**: Reuse common connection management logic
- **Shared message handling**: Reuse common message handling code
- **Avoid duplication**: Don't duplicate routing logic

### 5. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple routing and connection logic.

- **Simple routing**: Keep routing logic simple and efficient
- **Clear connection management**: Maintain clear connection lifecycle
- **Readable code**: Code should be self-documenting
- **Avoid over-engineering**: Don't add unnecessary complexity

### 6. CQRS (Command Query Responsibility Segregation)

**Software-Level Separation**: Separate read and write operations.

- **Message sending (commands)**: Separate handlers for sending messages
- **Message queries (queries)**: Separate handlers for message status queries
- **Connection queries**: Separate handlers for connection status queries
- **Optimization**: Optimize sending and queries independently

### 7. Single Responsibility Principle

**One Responsibility Per Component**: Each component should have a single responsibility.

- **Connection management**: Connection establishment and management only
- **Message routing**: Message routing only
- **Device discovery**: Device discovery only
- **Relay functionality**: Relay functionality only

### 8. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected.

- **Heimdall client**: Inject Heimdall client for connection validation
- **Message handlers**: Inject message handlers
- **Connection store**: Inject connection storage
- **Configuration**: Inject configuration

## Development Workflow

Follow this workflow for all coding tasks:

### 1. Analyze and Understand the Problem

- **Understand requirements**: Read and understand the requirements thoroughly
- **Identify constraints**: Identify technical, performance, and security constraints
- **Research existing solutions**: Check if similar protocol features exist
- **Protocol design**: Understand Bifrost protocol requirements
- **Document assumptions**: Document any assumptions you make

### 2. Plan Your Steps, Create Todos

#### 2.1. List All Classes/Structures/Functions That Need to Be Implemented

- **Identify components**: List all classes, structs, functions, and modules needed
- **Define interfaces**: Define interfaces for connection management, message routing, device discovery
- **Plan dependencies**: Identify dependencies between components
- **Create structure**: Plan the file and directory structure
- **Document design**: Document the design decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for each component
- **Test edge cases**: Include tests for edge cases and error conditions
- **Test connection management**: Write tests for connection establishment and management
- **Test message routing**: Write tests for message routing scenarios
- **Test device discovery**: Write tests for device discovery
- **Test error recovery**: Write tests for error recovery and reconnection
- **Mock dependencies**: Use mocks for external dependencies (Heimdall, network)
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
- **Check integration**: Verify integration with Odin, Heimdall, Asgard, Yggdrasil
- **Performance check**: Verify performance requirements (fast message transmission, low latency)
- **Security review**: Review security implications (TLS, authentication, message validation)
- **If not complete**: If the goal is not achieved, go back to Step 1

### 5. Give a Note to the User

- **Document changes**: Document all changes made
- **Explain usage**: Explain how to use the new code
- **Provide examples**: Provide code examples if helpful
- **List dependencies**: List any new dependencies
- **Migration notes**: Include migration notes if breaking changes

## Service-Specific Guidelines

### Connection Establishment

- **Connection/Authentication protocol**: Implement connection/authentication protocol before Bifrost
- **Public/private key management**: Manage public/private keys for device authentication
- **Token management**: Create, validate, and renew tokens
- **Rate limiting**: Rate limiting to prevent brute-force attacks
- **TLS handshake**: TLS 1.3 handshake for encryption
- **Connection validation**: Validate connections via Heimdall

### Message Routing

- **Direct routing**: Direct device-to-device routing when possible
- **Relay routing**: Routing via Asgard/Yggdrasil when direct not possible
- **Broadcast**: Broadcast messages to all devices
- **Multicast**: Multicast to device groups
- **Path optimization**: Optimize routing paths
- **Load balancing**: Load balancing across multiple paths
- **Failover**: Automatic failover on connection failure

### Device Discovery

- **mDNS/Bonjour**: Automatic device discovery in local network
- **Manual discovery**: IP-based connection
- **Service registry**: Centralized device registry (optional)
- **Global discovery**: Global discovery via Yggdrasil

### Connection Management

- **Connection pooling**: Pool of connections per device
- **Connection reuse**: Reuse connections efficiently
- **Connection monitoring**: Monitor connection health
- **Automatic reconnection**: Automatic reconnection (immediate + exponential backoff)
- **Heartbeat mechanism**: Heartbeat for keep-alive
- **Connection termination**: Graceful connection termination

### Error Recovery and Resilience

- **Automatic reconnection**: Immediate attempt, then exponential backoff
- **Retry with backoff**: Retry with exponential backoff for network errors
- **Fallback routing**: Fallback to alternative routes
- **Relay fallback**: Fallback to relay (Asgard/Yggdrasil) when direct connection fails
- **Error handling**: Robust error handling for connection errors

### NAT Traversal

- **Automatic NAT traversal**: Automatic NAT traversal preferred
- **STUN**: STUN protocol for NAT discovery
- **TURN**: TURN server for relay when NAT traversal not possible
- **ICE**: ICE protocol for optimal path
- **Manual configuration**: Fallback to manual port forwarding configuration

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

- **Connection management**: Test connection establishment and management
- **Message routing**: Test message routing logic
- **Device discovery**: Test device discovery
- **Error handling**: Test error handling and recovery
- **Avoid redundancy**: Avoid redundant test logic - reuse test utilities and helpers
- **Container-based**: All unit tests must run in container environment

### Integration Tests

- **Device-to-device communication**: Test complete device-to-device communication workflows
- **Relay functionality**: Test relay functionality via Asgard/Yggdrasil
- **Connection validation**: Test connection validation via Heimdall
- **Error recovery**: Test error recovery and reconnection
- **WebSocket tests**: WebSocket tests are critical - test all WebSocket connections for security and functionality
- **Container-based**: All integration tests must run in container environment

### Critical Test Areas

- **WebSocket security**: WebSocket tests are critical test areas - everything must work and be secured
- **Unauthorized access prevention**: All WebSocket connections must be tested to ensure unauthorized users cannot access data
- **Connection security**: Test connection authentication and authorization thoroughly
- **Message security**: Test message validation and encryption

### Test Coverage

- **Minimum coverage**: Maintain minimum 80% test coverage
- **Critical paths**: 100% coverage for critical paths (connection management, message routing)
- **Error handling**: High coverage for error handling code
- **WebSocket security**: 100% coverage for WebSocket security tests - ensure unauthorized access is prevented

## Security Considerations

### Connection Security

- **TLS encryption**: TLS 1.3 for all connections
- **Certificate validation**: Validate TLS certificates
- **Connection authentication**: Authenticate all connections via Heimdall
- **Message signing**: Digital signatures for message integrity
- **Unauthorized access prevention**: Strict access control to ensure unauthorized users cannot access data via WebSocket

### Message Security

- **Message validation**: Validate all messages
- **Message sanitization**: Sanitize messages to prevent injection attacks
- **End-to-end encryption**: Optional end-to-end encryption for messages
- **Data access control**: Ensure unauthorized users cannot access data through WebSocket messages

### Threat Detection

- **Anomaly detection**: Detect suspicious connection patterns
- **Connection blocking**: Block unauthorized connections
- **Audit logging**: Log all connection events for security audits

### GDPR Compliance (EU/German Data Protection)

- **Data minimization**: Only transmit necessary data over WebSocket
- **Data encryption**: Encrypt all personal data in transit
- **Access control**: Strict access control to prevent unauthorized data access
- **Audit logging**: Log all data access for compliance auditing
- **Right to erasure**: Support user right to delete their data
- **Privacy by design**: Consider privacy from the design phase

## Performance Requirements

### Message Transmission Performance

- **Fast routing**: Fast message routing (< 10ms locally)
- **Low latency**: Low latency for message transmission
- **High throughput**: High throughput for parallel connections
- **Efficient connection management**: Efficient connection pooling and reuse

### Resource Management

- **Memory usage**: Efficient memory usage for connection management - minimize RAM consumption
- **CPU usage**: Optimize CPU usage for message routing
- **Network usage**: Minimize network overhead
- **Code efficiency**: Write efficient, short code that uses minimal resources
- **User experience**: Ensure pleasant user experience - do not slow down user devices

## Additional Resources

- See `bifrost/README.md` for detailed service documentation
- Das `edda` Verzeichnis ist **KEIN PROJEKT**. Es dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte. Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden.
- Note: For general development principles, see `AGENTS.md` in the parent directory (informational reference only)

