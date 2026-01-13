# AGENTS.md - Ratatoskr Protocol Development Guidelines

> **Hinweis**: Für Hintergrundinformationen zum Projekt, Features, Architektur und technische Details siehe `README.md`.

Diese Datei enthält ausschließlich Richtlinien für das Verhalten des Agents bei der Entwicklung.

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: Red (failing test) → Green (passing test) → Refactor
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests for protocol implementation, integration tests for message flow, end-to-end tests for business transactions
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is the primary focus of Ratatoskr protocol.

- **TLS encryption**: TLS 1.3 for all connections
- **Message signing**: Digital signatures for all messages
- **Nonce-based authentication**: Nonce-based authentication to prevent replay attacks
- **Audit logging**: Complete audit logging of all business transactions
- **Rate limiting**: Rate limiting for business requests
- **No hardcoded secrets**: Never commit secrets or keys

### 3. Performance

**Performance from the Start**: Ratatoskr must provide efficient message transmission.

- **Binary format**: Efficient binary message format
- **Efficient serialization**: Optimized message serialization
- **Connection pooling**: Efficient connection pooling
- **Message batching**: Batching of messages for better performance
- **Async processing**: Async processing of messages

### 4. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality.

- **Check existing protocols**: Before implementing new protocol features, check if a separate protocol project already exists
- **Reuse message handling**: Reuse common message handling code from separate projects
- **Shared utilities**: Wenn gemeinsame Utils benötigt werden, sollte ein separates Projekt erstellt werden
- **Avoid duplication**: Don't duplicate protocol implementation

### 5. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple protocol design.

- **Simple message format**: Keep message format simple and efficient
- **Clear protocol flow**: Maintain clear protocol flow
- **Readable code**: Code should be self-documenting
- **Avoid over-engineering**: Don't add unnecessary complexity

### 6. CQRS (Command Query Responsibility Segregation)

**Software-Level Separation**: Separate read and write operations.

- **Business requests (commands)**: Separate handlers for business transactions
- **Status queries (queries)**: Separate handlers for status queries
- **Optimization**: Optimize requests and queries independently

### 7. Single Responsibility Principle

**One Responsibility Per Component**: Each component should have a single responsibility.

- **Protocol definition**: Protocol specification only
- **Message serialization**: Message serialization only
- **Security layer**: Security features only
- **Message types**: Message type definitions only

### 8. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected where applicable.

- **Serializers**: Inject serializers for different formats
- **Signers**: Inject message signers
- **Validators**: Inject message validators
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
- **Define interfaces**: Define interfaces for protocol, message types, security
- **Plan dependencies**: Identify dependencies between components
- **Create structure**: Plan the file and directory structure
- **Document design**: Document the design decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for each component
- **Test edge cases**: Include tests for edge cases and error conditions
- **Test protocol**: Write tests for protocol implementation
- **Test message types**: Write tests for all message types
- **Test security**: Write tests for security features (signing, nonce, validation)
- **Test serialization**: Write tests for message serialization
- **Mock dependencies**: Use mocks for external dependencies
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
- **Check integration**: Verify integration with Vedrfolnir and Nidhöggr
- **Performance check**: Verify performance requirements (efficient message transmission)
- **Security review**: Review security implications (TLS, signing, nonce, audit logging)
- **If not complete**: If the goal is not achieved, go back to Step 1

### 5. Give a Note to the User

- **Document changes**: Document all changes made
- **Explain usage**: Explain how to use the new code
- **Provide examples**: Provide code examples if helpful
- **List dependencies**: List any new dependencies
- **Migration notes**: Include migration notes if breaking changes

## Service-Specific Guidelines

### Protocol Definition

- **WebSocket-based**: WebSocket-based protocol (like Bifrost, but with extra security)
- **TLS 1.3**: TLS 1.3 encryption for all connections
- **Binary format**: Efficient binary message format
- **Message types**: Define all message types (CONNECTION_REQUEST, BUSINESS_REQUEST, MARKETPLACE_REQUEST, PAYMENT_REQUEST, PROVIDER_REGISTRATION, HEARTBEAT, DISCONNECT, ERROR)

### Security Features

- **Message signing**: Digital signatures for all messages
- **Audit logging**: Complete audit logging of all business transactions
- **Rate limiting**: Rate limiting for business requests (per device, per user, per request type)
- **Request validation**: Comprehensive validation of all requests
- **Nonce-based authentication**: Nonce-based authentication to prevent replay attacks

### Message Flow

- **Connection establishment**: Connection request/response flow
- **Message sending**: Message sending with signing and nonce
- **Message receiving**: Message receiving with validation
- **Error handling**: Error message handling
- **Heartbeat**: Keep-alive mechanism

### Protocol Implementation

- **Cross-language**: Protocol must work across languages (Rust, Elixir, TypeScript)
- **Versioning**: Support versioning for backward compatibility
- **Serialization**: Efficient binary serialization (Protobuf or MessagePack)
- **Documentation**: Document all message types and flows

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

- **Protocol implementation**: Test protocol implementation
- **Message types**: Test all message types
- **Security features**: Test security features (signing, nonce, validation)
- **Serialization**: Test message serialization/deserialization
- **Avoid redundancy**: Avoid redundant test logic - reuse test utilities and helpers
- **Container-based**: All unit tests must run in container environment

### Integration Tests

- **Message flow**: Test complete message flow (Vedrfolnir → Nidhöggr)
- **Connection establishment**: Test connection establishment flow
- **Error handling**: Test error handling scenarios
- **Security**: Test security features end-to-end
- **WebSocket tests**: WebSocket tests are critical - test all WebSocket connections for security and functionality
- **Container-based**: All integration tests must run in container environment

### Critical Test Areas

- **WebSocket security**: WebSocket tests are critical test areas - everything must work and be secured
- **Unauthorized access prevention**: All WebSocket connections must be tested to ensure unauthorized users cannot access data
- **Business transaction security**: Test all business transactions for security
- **Message security**: Test message validation, signing, and encryption

### Test Coverage

- **Minimum coverage**: Maintain minimum 80% test coverage
- **Critical paths**: 100% coverage for critical paths (protocol implementation, security)
- **Message types**: High coverage for all message types
- **WebSocket security**: 100% coverage for WebSocket security tests - ensure unauthorized access is prevented

## Security Considerations

### Protocol Security

- **TLS encryption**: TLS 1.3 for all connections
- **Message signing**: Digital signatures for all messages
- **Nonce-based authentication**: Nonce-based authentication to prevent replay attacks
- **Request validation**: Comprehensive validation of all requests
- **Unauthorized access prevention**: Strict access control to ensure unauthorized users cannot access data via WebSocket

### Audit Logging

- **Complete logging**: Log all business transactions
- **Immutable logs**: Logs cannot be modified
- **Compliance**: Meet compliance requirements (GDPR, PCI-DSS, etc.)

### Rate Limiting

- **Per-device**: Rate limiting per device
- **Per-user**: Rate limiting per user
- **Per-request-type**: Different limits for different request types

### GDPR Compliance (EU/German Data Protection)

- **Data minimization**: Only transmit necessary data over WebSocket
- **Data encryption**: Encrypt all personal data in transit
- **Access control**: Strict access control to prevent unauthorized data access
- **Audit logging**: Log all data access for compliance auditing
- **Right to erasure**: Support user right to delete their data
- **Privacy by design**: Consider privacy from the design phase
- **BDSG compliance**: Comply with Bundesdatenschutzgesetz (BDSG) requirements

## Performance Requirements

### Message Transmission Performance

- **Efficient serialization**: Efficient binary serialization
- **Low latency**: Low latency for message transmission
- **High throughput**: High throughput for parallel messages
- **Minimal overhead**: Minimal protocol overhead

### Resource Management

- **Memory usage**: Efficient memory usage for message handling - minimize RAM consumption
- **CPU usage**: Optimize CPU usage for serialization and signing
- **Code efficiency**: Write efficient, short code that uses minimal resources
- **Code brevity**: Code should be as short as possible while maintaining readability
- **Failsafe**: Code must be failsafe - no errors that could crash the system
- **User experience**: Ensure pleasant user experience - do not slow down user devices

## Additional Resources

- See `ratatoskr/README.md` for detailed service documentation
- Das `edda` Verzeichnis ist **KEIN PROJEKT**. Es dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte. Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden.
- Note: For general development principles, see `AGENTS.md` in the parent directory (informational reference only)

