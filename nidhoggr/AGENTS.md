# AGENTS.md - Nidhöggr Development Guidelines

> **Hinweis**: Für Hintergrundinformationen zum Projekt, Features, Architektur und technische Details siehe `README.md`.

Diese Datei enthält ausschließlich Richtlinien für das Verhalten des Agents bei der Entwicklung.

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: Red (failing test) → Green (passing test) → Refactor
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests for connection management, integration tests for message routing, end-to-end tests for service communication
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is critical for connection endpoint.

- **TLS encryption**: TLS 1.3 for all connections
- **Message validation**: Validate all incoming messages (signature, nonce)
- **Rate limiting**: Rate limiting per device/user
- **Audit logging**: Complete audit logging of all business transactions
- **Connection monitoring**: Monitor connections for suspicious activities
- **No hardcoded secrets**: Never commit secrets or keys

### 3. Performance

**Performance from the Start**: Nidhöggr must handle high throughput.

- **Fast connection**: Low connection latency (< 100ms for connection establishment)
- **Fast routing**: Fast message routing (< 10ms for message routing)
- **High throughput**: High throughput (1000+ messages/second per instance)
- **Connection pooling**: Efficient connection pooling
- **Message batching**: Batching of messages for better performance
- **Async processing**: Async processing of messages
- **Load balancing**: Load balancing across multiple service instances

### 4. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality.

- **Check existing protocols**: Before implementing new protocol features, check if a separate protocol project already exists
- **Reuse connection logic**: Reuse common connection management logic
- **Shared message handling**: Reuse common message handling code
- **Avoid duplication**: Don't duplicate routing logic

### 5. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple routing and connection logic.

- **Simple routing**: Keep message routing simple and efficient
- **Clear service mapping**: Maintain clear message type to service mapping
- **Readable code**: Code should be self-documenting
- **Avoid over-engineering**: Don't add unnecessary complexity

### 6. CQRS (Command Query Responsibility Segregation)

**Software-Level Separation**: Separate read and write operations.

- **Message routing (commands)**: Separate handlers for routing messages
- **Connection queries (queries)**: Separate handlers for connection status queries
- **Service queries**: Separate handlers for service availability queries
- **Optimization**: Optimize routing and queries independently

### 7. Single Responsibility Principle

**One Responsibility Per Component**: Each component should have a single responsibility.

- **Connection management**: Connection establishment and management only
- **Message receiving**: Message receiving and validation only
- **Message routing**: Message routing only
- **Service communication**: Service communication only

### 8. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected.

- **Heimdall client**: Inject Heimdall client for connection validation
- **Service clients**: Inject service clients (Nornen, Mimir, Heidrun, etc.)
- **Message handlers**: Inject message handlers
- **Configuration**: Inject configuration

## Development Workflow

Follow this workflow for all coding tasks:

### 1. Analyze and Understand the Problem

- **Understand requirements**: Read and understand the requirements thoroughly
- **Identify constraints**: Identify technical, performance, and security constraints
- **Research existing solutions**: Check if similar routing functionality exists
- **Service mapping**: Understand message type to service mapping
- **Document assumptions**: Document any assumptions you make

### 2. Plan Your Steps, Create Todos

#### 2.1. List All Classes/Structures/Functions That Need to Be Implemented

- **Identify components**: List all classes, structs, functions, and modules needed
- **Define interfaces**: Define interfaces for connection management, message routing, service communication
- **Plan dependencies**: Identify dependencies between components
- **Create structure**: Plan the file and directory structure
- **Document design**: Document the design decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for each component
- **Test edge cases**: Include tests for edge cases and error conditions
- **Test connection management**: Write tests for connection establishment and management
- **Test message routing**: Write tests for message routing scenarios
- **Test service communication**: Write tests for service communication (gRPC)
- **Test error handling**: Write tests for error handling and connection termination
- **Mock dependencies**: Use mocks for external dependencies (Heimdall, services, network)
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
- **Check integration**: Verify integration with Vedrfolnir, Nornen, Mimir, Heidrun, Eikthyrnir
- **Performance check**: Verify performance requirements (fast routing, high throughput)
- **Security review**: Review security implications (TLS, message validation, rate limiting, audit logging)
- **If not complete**: If the goal is not achieved, go back to Step 1

### 5. Give a Note to the User

- **Document changes**: Document all changes made
- **Explain usage**: Explain how to use the new code
- **Provide examples**: Provide code examples if helpful
- **List dependencies**: List any new dependencies
- **Migration notes**: Include migration notes if breaking changes

## Service-Specific Guidelines

### Connection Management

- **Receive connections**: Receive connections from Vedrfolnir clients
- **Connection validation**: Validate all incoming connections via Heimdall
- **Connection monitoring**: Monitor all active connections
- **Connection termination**: Terminate connections on authentication errors, rate limiting violations, security violations, graceful shutdown, timeout
- **TLS handshake**: Handle TLS 1.3 encryption

### Message Receiving

- **Receive messages**: Receive messages over Ratatoskr protocol
- **Message validation**: Validate all incoming messages (signature, nonce, etc.)
- **Rate limiting**: Check rate limits per device/user
- **Audit logging**: Create audit logs for all business transactions

### Message Routing

- **Direct forwarding**: Forward messages directly to appropriate services
- **Service discovery**: Determine which service handles which message type
- **Service mapping**: Map message types to services:
  - MARKETPLACE_REQUEST → Nornen
  - PAYMENT_REQUEST → Heidrun
  - PROVIDER_REGISTRATION → Nornen
  - BUSINESS_REQUEST → Nornen
- **Load balancing**: Load balancing across multiple service instances
- **Error handling**: Handle service failures gracefully

### Service Communication

- **gRPC communication**: Communicate with services via gRPC
- **Async processing**: Async message processing
- **Response handling**: Handle responses from services
- **Error handling**: Handle service errors

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
- **Message validation**: Test message validation logic
- **Message routing**: Test message routing logic
- **Service communication**: Test service communication (gRPC, mocked)
- **Avoid redundancy**: Avoid redundant test logic - reuse test utilities and helpers
- **Container-based**: All unit tests must run in container environment

### Integration Tests

- **Complete message flow**: Test complete message flow (Vedrfolnir → Nidhöggr → Service)
- **Service integration**: Test integration with services (Nornen, Mimir, Heidrun, etc.)
- **Error scenarios**: Test error handling and connection termination
- **Rate limiting**: Test rate limiting scenarios
- **WebSocket tests**: WebSocket tests are critical - test all WebSocket connections for security and functionality
- **Container-based**: All integration tests must run in container environment

### Critical Test Areas

- **WebSocket security**: WebSocket tests are critical test areas - everything must work and be secured
- **Unauthorized access prevention**: All WebSocket connections must be tested to ensure unauthorized users cannot access data
- **Message security**: Test message validation, signing, and encryption thoroughly

### Test Coverage

- **Minimum coverage**: Maintain minimum 80% test coverage
- **Critical paths**: 100% coverage for critical paths (connection management, message routing)
- **Security code**: High coverage for all security-related code
- **WebSocket security**: 100% coverage for WebSocket security tests - ensure unauthorized access is prevented

## Security Considerations

### Connection Security

- **TLS encryption**: TLS 1.3 for all connections
- **Certificate validation**: Validate TLS certificates
- **Connection authentication**: Authenticate connections via Heimdall
- **Connection monitoring**: Monitor connections for suspicious activities
- **Unauthorized access prevention**: Strict access control to ensure unauthorized users cannot access data via WebSocket

### Message Security

- **Message validation**: Validate all incoming messages (signature, nonce)
- **Rate limiting**: Rate limiting per device/user
- **Audit logging**: Complete audit logging
- **Security violations**: Terminate connections on security violations
- **Data access control**: Ensure unauthorized users cannot access data through WebSocket messages

### GDPR Compliance (EU/German Data Protection)

- **Data minimization**: Only transmit necessary data over WebSocket
- **Data encryption**: Encrypt all personal data in transit
- **Access control**: Strict access control to prevent unauthorized data access
- **Audit logging**: Log all data access for compliance auditing
- **Right to erasure**: Support user right to delete their data
- **Privacy by design**: Consider privacy from the design phase
- **BDSG compliance**: Comply with Bundesdatenschutzgesetz (BDSG) requirements

## Performance Requirements

### Connection Performance

- **Fast connection**: Low connection latency (< 100ms for connection establishment)
- **Fast routing**: Fast message routing (< 10ms for message routing)
- **High throughput**: High throughput (1000+ messages/second per instance)
- **Efficient connection management**: Efficient connection pooling

### Resource Management

- **Memory usage**: Efficient memory usage for connection management - minimize RAM consumption
- **CPU usage**: Optimize CPU usage for message routing
- **Connection management**: Efficient connection pooling
- **Code efficiency**: Write efficient, short code that uses minimal resources
- **Code brevity**: Code should be as short as possible while maintaining readability
- **Failsafe**: Code must be failsafe - no errors that could crash the system
- **User experience**: Ensure pleasant user experience - do not slow down user devices

## Additional Resources

- See `nidhoggr/README.md` for detailed service documentation
- Das `edda` Verzeichnis ist **KEIN PROJEKT**. Es dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte. Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden.
- Note: For general development principles, see `AGENTS.md` in the parent directory (informational reference only)

