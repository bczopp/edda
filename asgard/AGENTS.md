# AGENTS.md - Asgard Development Guidelines

> **Hinweis**: Für Hintergrundinformationen zum Projekt, Features, Architektur und technische Details siehe `README.md`.

Diese Datei enthält ausschließlich Richtlinien für das Verhalten des Agents bei der Entwicklung.

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: Red (failing test) → Green (passing test) → Refactor
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests for server components, integration tests for service integration, end-to-end tests for server workflows
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is important for server environments.

- **Secure storage**: Encrypted storage for credentials and device data
- **TLS encryption**: All connections encrypted (TLS 1.3)
- **Authentication**: Secure authentication via Heimdall
- **Authorization**: Granular permission system for device access
- **Network isolation**: Optional network isolation for increased security
- **Input validation**: Comprehensive input validation for all API endpoints
- **No hardcoded secrets**: Never commit secrets or credentials

### 3. Performance

**Performance from the Start**: Asgard must handle server workloads efficiently.

- **Efficient routing**: Optimized message routing for minimal latency
- **Connection pooling**: Efficient connection pooling for WebSocket connections
- **Caching**: Intelligent caching for device registry and frequently used data
- **Database optimization**: Optimized database queries with indexes
- **Load balancing**: Load balancing for parallel requests
- **Async processing**: Async processing for better performance

### 4. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality.

- **Check existing services**: Before implementing new functionality, check if a service already provides it
- **Reuse separate projects**: Wenn gemeinsame Komponenten (DTOs, Protocols, Utils) benötigt werden, sollten separate Projekte erstellt werden
- **Service discovery**: Know what other services provide and use them
- **Avoid duplication**: Don't duplicate server functionality

### 5. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple server implementations.

- **Simple server logic**: Keep server logic simple and clear
- **Clear service boundaries**: Maintain clear service boundaries
- **Readable code**: Code should be self-documenting
- **Avoid over-engineering**: Don't add unnecessary complexity

### 6. CQRS (Command Query Responsibility Segregation)

**Software-Level Separation**: Separate read and write operations.

- **Server operations (commands)**: Separate handlers for write operations
- **Status queries (queries)**: Separate handlers for read operations
- **Optimization**: Optimize reads and writes independently

### 7. Single Responsibility Principle

**One Responsibility Per Component**: Each component should have a single responsibility.

- **Device registry**: Device registry management only
- **Network management**: Network management only
- **Message routing**: Message routing only
- **Connection management**: Connection management only
- **Lock management**: Distributed lock management only

### 8. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected.

- **Service clients**: Inject service clients
- **Database client**: Inject database client
- **Configuration**: Inject configuration
- **Platform APIs**: Inject platform-specific APIs

## Development Workflow

Follow this workflow for all coding tasks:

### 1. Analyze and Understand the Problem

- **Understand requirements**: Read and understand the requirements thoroughly
- **Identify constraints**: Identify technical, performance, and security constraints
- **Research existing solutions**: Check if similar server functionality exists
- **Service dependencies**: Identify which services are needed
- **Document assumptions**: Document any assumptions you make

### 2. Plan Your Steps, Create Todos

#### 2.1. List All Classes/Structures/Functions That Need to Be Implemented

- **Identify components**: List all classes, structs, functions, and modules needed
- **Define interfaces**: Define interfaces for server components and service integration
- **Plan dependencies**: Identify dependencies between components
- **Create structure**: Plan the file and directory structure
- **Document design**: Document the design decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for each component
- **Test edge cases**: Include tests for edge cases and error conditions
- **Test server components**: Write tests for server components
- **Test service integration**: Write integration tests for service interactions
- **Test API endpoints**: Write tests for API endpoints
- **Mock dependencies**: Use mocks for external service dependencies
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
- **Check integration**: Verify integration with Odin and all services
- **Performance check**: Verify performance requirements (efficient routing, high throughput)
- **Security review**: Review security implications (TLS, authentication, authorization)
- **If not complete**: If the goal is not achieved, go back to Step 1

### 5. Give a Note to the User

- **Document changes**: Document all changes made
- **Explain usage**: Explain how to use the new code
- **Provide examples**: Provide code examples if helpful
- **List dependencies**: List any new dependencies
- **Migration notes**: Include migration notes if breaking changes

## Service-Specific Guidelines

### Device Registry Service

- **Device registration**: Register all devices in the network
- **Device metadata**: Manage device metadata
- **Device status tracking**: Track device status
- **Device discovery support**: Support device discovery
- **Capability synchronization**: Synchronize capabilities when Asgard present in network
- **Leading server**: Oldest server becomes leading server when multiple Asgard servers in same network

### Network Manager Service

- **Network ID management**: Manage network IDs
- **Device topology**: Manage device topology
- **Network health monitoring**: Monitor network health
- **Network configuration**: Network configuration management

### Routing Service

- **Message routing**: Route messages between devices
- **Relay functionality**: Relay functionality when direct connection not possible
- **Broadcast/multicast**: Support broadcast and multicast
- **Load balancing**: Load balancing for requests

### Lock Management Service

- **Distributed locking**: Manage locks for shared resources in local network
- **Lock registry**: Central registry for all active locks
- **Lock expiration**: Manage lock expiration (timeout)
- **Deadlock detection**: Detect and resolve deadlocks
- **Priority management**: Manage priorities for lock requests

### Storage Service

- **Persistent storage**: Persistent storage for device data
- **Backup & restore**: Backup and restore functionality
- **Data migration**: Data migration support
- **Query interface**: Query interface for data access

### Web Dashboard (Optional)

- **Optional frontend**: User can optionally use web dashboard to track system activity
- **Text input**: Support text input for manual command entry
- **Voice input**: Support voice input via Huginn (STT)
- **Flexible switching**: User can switch between text and voice
- **Server administration**: Server administration interface
- **Activity monitoring**: Overview of running actions and services

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

- **Server components**: Test individual server components
- **Device registry**: Test device registry logic
- **Message routing**: Test message routing logic
- **Lock management**: Test lock management logic
- **Avoid redundancy**: Avoid redundant test logic - reuse test utilities and helpers
- **Container-based**: All unit tests must run in container environment

### Integration Tests

- **Server workflows**: Test complete server workflows
- **Device integration**: Test device integration
- **Service integration**: Test integration with services
- **Error handling**: Test error handling and recovery
- **API tests**: API tests are critical - test all API endpoints for security and functionality
- **WebSocket tests**: WebSocket tests are critical - test all WebSocket connections for security and functionality
- **Container-based**: All integration tests must run in container environment

### Critical Test Areas

- **API security**: API tests are critical test areas - everything must work and be secured
- **WebSocket security**: WebSocket tests are critical test areas - everything must work and be secured
- **Unauthorized access prevention**: All API and WebSocket endpoints must be tested to ensure unauthorized users cannot access data

### Test Coverage

- **Minimum coverage**: Maintain minimum 80% test coverage
- **Critical paths**: 100% coverage for critical paths (device registry, message routing)
- **Server code**: High coverage for all server-related code
- **API security**: 100% coverage for API security tests - ensure unauthorized access is prevented
- **WebSocket security**: 100% coverage for WebSocket security tests - ensure unauthorized access is prevented

## Security Considerations

### Server Security

- **Secure storage**: Encrypted storage for credentials and device data
- **TLS encryption**: TLS 1.3 for all connections
- **Authentication**: Secure authentication via Heimdall
- **Authorization**: Granular permission system for device access
- **Network isolation**: Optional network isolation
- **Input validation**: Comprehensive input validation
- **Unauthorized access prevention**: Strict access control to ensure unauthorized users cannot access data

### API Security

- **Rate limiting**: Rate limiting for API endpoints
- **DDoS protection**: Protection against DDoS attacks
- **Audit logging**: Complete logging of security-relevant events
- **API endpoint security**: All API endpoints must be secured to prevent unauthorized data access
- **WebSocket security**: All WebSocket connections must be secured to prevent unauthorized data access

### GDPR Compliance (EU/German Data Protection)

- **Data minimization**: Only collect and process necessary data
- **Data encryption**: Encrypt all personal data at rest and in transit
- **Access control**: Strict access control to prevent unauthorized data access
- **Audit logging**: Log all data access for compliance auditing
- **Right to erasure**: Support user right to delete their data
- **Privacy by design**: Consider privacy from the design phase
- **BDSG compliance**: Comply with Bundesdatenschutzgesetz (BDSG) requirements

## Performance Requirements

### Server Performance

- **Efficient routing**: Efficient message routing (< 10ms locally)
- **High throughput**: High throughput for parallel connections
- **Efficient device discovery**: Efficient device discovery (< 1s for local devices)
- **Scalability**: Design for scalability

### Resource Management

- **Memory usage**: Efficient memory usage for server operations - minimize RAM consumption
- **CPU usage**: Optimize CPU usage for message routing
- **Connection management**: Efficient connection pooling
- **Code efficiency**: Write efficient, short code that uses minimal resources
- **Code brevity**: Code should be as short as possible while maintaining readability
- **Failsafe**: Code must be failsafe - no errors that could crash the system
- **User experience**: Ensure pleasant user experience - do not slow down user devices

## Additional Resources

- See `asgard/README.md` for detailed service documentation
- Das `edda` Verzeichnis ist **KEIN PROJEKT**. Es dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte. Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden.
- Note: For general development principles, see `AGENTS.md` in the parent directory (informational reference only)

