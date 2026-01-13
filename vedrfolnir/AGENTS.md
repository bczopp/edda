# AGENTS.md - Vedrfolnir Development Guidelines

> **Hinweis**: Für Hintergrundinformationen zum Projekt, Features, Architektur und technische Details siehe `README.md`.

Diese Datei enthält ausschließlich Richtlinien für das Verhalten des Agents bei der Entwicklung.

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: Red (failing test) → Green (passing test) → Refactor
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests for connection management, integration tests for message handling, end-to-end tests for Yggdrasil communication
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is critical for Yggdrasil communication.

- **TLS encryption**: TLS 1.3 for all connections
- **Message signing**: Digital signatures for all messages
- **Nonce management**: Nonce-based authentication to prevent replay attacks
- **Token management**: Secure management of authentication tokens
- **No hardcoded secrets**: Never commit secrets or keys

### 3. Performance

**Performance from the Start**: Vedrfolnir must provide fast message transmission.

- **Fast connection**: Fast connection establishment (< 500ms)
- **Low latency**: Low message latency (< 100ms for standard requests)
- **Connection pooling**: Efficient connection pooling
- **Message batching**: Batching of messages for better performance
- **Async processing**: Async processing of messages
- **Efficient serialization**: Efficient message serialization

### 4. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality.

- **Check existing protocols**: Before implementing new protocol features, check if a separate protocol project already exists
- **Reuse connection logic**: Reuse common connection management logic
- **Shared message handling**: Reuse common message handling code
- **Avoid duplication**: Don't duplicate protocol implementation

### 5. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple connection and message handling.

- **Simple connection management**: Keep connection management simple
- **Clear message flow**: Maintain clear message flow
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
- **Message handling**: Message sending and receiving only
- **Protocol handling**: Ratatoskr protocol implementation only
- **Error recovery**: Error recovery and reconnection only

### 8. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected.

- **Heimdall client**: Inject Heimdall client for token management
- **WebSocket client**: Inject WebSocket client
- **Message handlers**: Inject message handlers
- **Configuration**: Inject configuration

## Development Workflow

Follow this workflow for all coding tasks:

### 1. Analyze and Understand the Problem

- **Understand requirements**: Read and understand the requirements thoroughly
- **Identify constraints**: Identify technical, performance, and security constraints
- **Research existing solutions**: Check if similar protocol features exist
- **Protocol design**: Understand Ratatoskr protocol requirements
- **Document assumptions**: Document any assumptions you make

### 2. Plan Your Steps, Create Todos

#### 2.1. List All Classes/Structures/Functions That Need to Be Implemented

- **Identify components**: List all classes, structs, functions, and modules needed
- **Define interfaces**: Define interfaces for connection management, message handling, protocol
- **Plan dependencies**: Identify dependencies between components
- **Create structure**: Plan the file and directory structure
- **Document design**: Document the design decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for each component
- **Test edge cases**: Include tests for edge cases and error conditions
- **Test connection management**: Write tests for connection establishment and management
- **Test message handling**: Write tests for message sending and receiving
- **Test protocol**: Write tests for Ratatoskr protocol implementation
- **Test error recovery**: Write tests for error recovery and reconnection
- **Mock dependencies**: Use mocks for external dependencies (Heimdall, network, Yggdrasil)
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
- **Check integration**: Verify integration with Odin and Yggdrasil (Nidhöggr)
- **Performance check**: Verify performance requirements (fast connection, low latency)
- **Security review**: Review security implications (TLS, message signing, nonce)
- **If not complete**: If the goal is not achieved, go back to Step 1

### 5. Give a Note to the User

- **Document changes**: Document all changes made
- **Explain usage**: Explain how to use the new code
- **Provide examples**: Provide code examples if helpful
- **List dependencies**: List any new dependencies
- **Migration notes**: Include migration notes if breaking changes

## Service-Specific Guidelines

### Connection Management

- **Connection establishment**: Establish connections to Yggdrasil (Nidhöggr) over Ratatoskr protocol
- **Connection pooling**: Efficient connection pooling
- **Automatic reconnection**: Automatic reconnection (immediate + exponential backoff)
- **Connection monitoring**: Monitor connection status
- **Heartbeat**: Regular heartbeat for keep-alive
- **Connection termination**: Graceful connection termination

### Message Handling

- **Message sending**: Send messages from Odin to Yggdrasil
- **Message receiving**: Receive responses from Yggdrasil
- **Message queuing**: Queue messages when connection unavailable
- **Message retry**: Automatic retry on errors
- **Message validation**: Validate all incoming messages (signature, nonce)
- **Message signing**: Sign all outgoing messages

### Protocol Handling

- **Ratatoskr protocol**: Implement Ratatoskr protocol
- **TLS handshake**: Handle TLS 1.3 encryption
- **Message signing**: Digital signatures for all messages
- **Nonce management**: Manage nonces for replay protection
- **Request validation**: Validate all requests before sending

### Error Recovery

- **Automatic reconnection**: Immediate attempt, then exponential backoff
- **Message queuing**: Queue messages when connection unavailable
- **Queue limits**: Queue limits to prevent memory overflow
- **Automatic sending**: Send queued messages when connection restored
- **Error handling**: Robust error handling for connection errors

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
- **Message handling**: Test message sending and receiving
- **Protocol**: Test Ratatoskr protocol implementation
- **Error recovery**: Test error recovery and reconnection
- **Container-based**: All unit tests must run in container environment

### Integration Tests

- **Yggdrasil communication**: Test complete communication with Yggdrasil (mocked)
- **Odin integration**: Test integration with Odin
- **Error scenarios**: Test error handling and recovery
- **Reconnection**: Test automatic reconnection scenarios
- **Container-based**: All integration tests must run in container environment

### Test Coverage

- **Minimum coverage**: Maintain minimum 80% test coverage
- **Critical paths**: 100% coverage for critical paths (connection management, message handling)
- **Protocol code**: High coverage for protocol implementation code
- **Avoid redundancy**: Avoid redundant test logic - reuse test utilities and helpers
- **WebSocket tests**: WebSocket tests are critical - test all WebSocket connections for security and functionality

## Security Considerations

### Connection Security

- **TLS encryption**: TLS 1.3 for all connections
- **Certificate validation**: Validate TLS certificates
- **Connection authentication**: Authenticate connections via Heimdall

### Message Security

- **Message signing**: Digital signatures for all messages
- **Nonce-based authentication**: Nonce-based authentication to prevent replay attacks
- **Message validation**: Validate all incoming messages
- **Token management**: Secure management of authentication tokens
- **Unauthorized access prevention**: Strict access control to ensure unauthorized users cannot access data via WebSocket

### GDPR Compliance (EU/German Data Protection)

- **Data minimization**: Only transmit necessary data over WebSocket
- **Data encryption**: Encrypt all personal data in transit
- **Access control**: Strict access control to prevent unauthorized data access
- **Privacy by design**: Consider privacy from the design phase
- **BDSG compliance**: Comply with Bundesdatenschutzgesetz (BDSG) requirements

## Performance Requirements

### Connection Performance

- **Fast connection**: Fast connection establishment (< 500ms)
- **Low latency**: Low message latency (< 100ms for standard requests)
- **Efficient transmission**: Efficient message transmission (minimal overhead)
- **High throughput**: High throughput for parallel messages

### Resource Management

- **Memory usage**: Efficient memory usage for message queuing - minimize RAM consumption
- **CPU usage**: Optimize CPU usage for message processing
- **Connection management**: Efficient connection pooling
- **Code efficiency**: Write efficient, short code that uses minimal resources
- **Code brevity**: Code should be as short as possible while maintaining readability
- **Failsafe**: Code must be failsafe - no errors that could crash the system
- **User experience**: Ensure pleasant user experience - do not slow down user devices

## Additional Resources

- See `vedrfolnir/README.md` for detailed service documentation
- Das `edda` Verzeichnis ist **KEIN PROJEKT**. Es dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte. Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden.
- Note: For general development principles, see `AGENTS.md` in the parent directory (informational reference only)

