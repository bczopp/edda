# AGENTS.md - Yggdrasil Development Guidelines

> **Hinweis**: Für Hintergrundinformationen zum Projekt, Features, Architektur und technische Details siehe `README.md`.

Diese Datei enthält ausschließlich Richtlinien für das Verhalten des Agents bei der Entwicklung.

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: Red (failing test) → Green (passing test) → Refactor
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests for Elixir modules, integration tests for service coordination, end-to-end tests for complete workflows
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is the highest priority, especially network isolation.

- **Network isolation**: Strict network isolation between users and networks (highest priority)
- **TLS encryption**: TLS 1.3 for all connections
- **Authentication**: JWT-based authentication with secure tokens
- **Authorization**: Role-based authorization (RBAC) with strict network isolation
- **Rate limiting**: Rate limiting for API protection
- **DDoS protection**: DDoS protection on infrastructure level
- **Data encryption**: Encrypt sensitive data (at rest and in transit)
- **No hardcoded secrets**: Never commit secrets or credentials

### 3. Performance

**Performance from the Start**: Yggdrasil must handle millions of concurrent connections.

- **Horizontal scaling**: Scale across multiple instances for high throughput
- **Load balancing**: Intelligent load balancing for optimal performance
- **Database sharding**: Database sharding for better performance with large datasets
- **Caching**: Redis/Memcached for fast access to frequently used data
- **CDN integration**: CDN for static assets and API responses
- **Database optimization**: Optimized database queries with indexes and query optimization
- **Async processing**: Async processing for better response times

### 4. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality.

- **Check existing services**: Before implementing new functionality, check if a service already provides it
- **Reuse Rust microservices**: Use existing Rust microservices (Mimir, Nornen, Nidhöggr, Heidrun, Eikthyrnir, Læraðr)
- **Shared libraries**: Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden
- **Avoid duplication**: Don't duplicate service coordination logic

### 5. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple service coordination.

- **Simple coordination**: Keep service coordination simple and clear
- **Clear service boundaries**: Maintain clear boundaries between services
- **Readable code**: Code should be self-documenting
- **Avoid over-engineering**: Don't add unnecessary complexity

### 6. CQRS (Command Query Responsibility Segregation)

**Software-Level Separation**: Separate read and write operations.

- **Business operations (commands)**: Separate handlers for write operations
- **Query operations (queries)**: Separate handlers for read operations
- **Different models**: Commands and queries can use different data models
- **Optimization**: Optimize reads and writes independently

### 7. Single Responsibility Principle

**One Responsibility Per Component**: Each component should have a single responsibility.

- **Device registry**: Global device registry only
- **User management**: User management only
- **Subscription management**: Subscription management only
- **Payment processing**: Payment processing only
- **Marketplace management**: Marketplace management only
- **Bifrost relay**: Bifrost relay functionality only

### 8. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected.

- **Rust microservices**: Inject Rust microservice clients (gRPC)
- **Database**: Inject database client (Ecto)
- **Payment providers**: Inject payment provider clients
- **Configuration**: Inject configuration

## Development Workflow

Follow this workflow for all coding tasks:

### 1. Analyze and Understand the Problem

- **Understand requirements**: Read and understand the requirements thoroughly
- **Identify constraints**: Identify technical, performance, and security constraints
- **Research existing solutions**: Check if similar functionality exists in Yggdrasil or Rust microservices
- **Service dependencies**: Identify which services are needed
- **Network isolation**: Understand network isolation requirements
- **Document assumptions**: Document any assumptions you make

### 2. Plan Your Steps, Create Todos

#### 2.1. List All Classes/Structures/Functions That Need to Be Implemented

- **Identify components**: List all modules, functions, and processes needed
- **Define interfaces**: Define interfaces and contracts between components and services
- **Plan dependencies**: Identify dependencies between components and external services
- **Create structure**: Plan the file and directory structure
- **Document design**: Document the design decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for each component
- **Test edge cases**: Include tests for edge cases and error conditions
- **Test service integration**: Write integration tests for service interactions
- **Test network isolation**: Write tests for network isolation
- **Mock services**: Use mocks for external service dependencies (Rust microservices, payment providers)
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
- **Or focus on module**: Or just run the tests of the module you currently work on
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
- **Check integration**: Verify integration with Rust microservices and user devices
- **Performance check**: Verify performance requirements (low API latency, high throughput)
- **Security review**: Review security implications (network isolation, authentication, authorization)
- **Network isolation**: Verify network isolation (users cannot see or access other networks)
- **If not complete**: If the goal is not achieved, go back to Step 1

### 5. Give a Note to the User

- **Document changes**: Document all changes made
- **Explain usage**: Explain how to use the new code
- **Provide examples**: Provide code examples if helpful
- **List dependencies**: List any new dependencies
- **Migration notes**: Include migration notes if breaking changes

## Service-Specific Guidelines

### Elixir Implementation

- **Phoenix Framework**: Use Phoenix for web API and WebSocket connections
- **Ecto**: Use Ecto for database access
- **OTP**: Use OTP for distributed systems and supervision
- **Phoenix Channels**: Use Phoenix Channels for Bifrost WebSocket connections
- **GenServer**: Use GenServer for state management
- **Supervisor**: Use Supervisor for process supervision

### Rust Microservices Coordination

- **gRPC communication**: Communicate with Rust microservices via gRPC
- **Async processing**: Async request processing
- **Service discovery**: Discover available Rust microservices
- **Load balancing**: Load balancing across multiple service instances
- **Error handling**: Handle service failures gracefully

### Network Isolation (Highest Priority)

- **Complete isolation**: Complete network isolation between users and networks
- **No visibility**: Users cannot see other networks or their names
- **Network segmentation**: Separate network segments for each user/network
- **VPC isolation**: Virtual Private Cloud isolation for each tenant
- **Kubernetes network policies**: Strict network policies to prevent cross-network communication
- **RBAC**: Role-Based Access Control ensures users only access their own resources
- **Resource quotas**: Each tenant has own resource limits

### Bifrost Relay System

- **Persistent connections**: Maintain persistent Bifrost WebSocket connections to all registered devices
- **Connection initiation**: Devices connect via Bifrost, not webhooks
- **Message routing**: Route messages between devices over Bifrost
- **Relay function**: Relay messages when direct device-to-device connection not possible
- **Event notifications**: Send events via Bifrost messages (not webhooks)

### Marketplace Management

- **Provider registration**: Manage provider registrations
- **Request routing**: Route compute requests to providers
- **Fair distribution**: Fair distribution algorithm for provider selection
- **Transaction system**: Manage transactions and settlement
- **Quality assessment**: Assess provider quality
- **Analytics**: Provide provider and requester analytics

### Payment Integration

- **Payment methods**: Manage payment methods (add, verify, set default, remove)
- **Pre-authorization**: Pre-authorization for estimated costs
- **Payout processing**: Automatic and manual payouts
- **Netting**: Optional netting (earnings against costs)
- **PCI-DSS compliance**: PCI-DSS compliance for credit card data
- **KYC**: KYC for providers (depending on region)

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

- **Elixir modules**: Test individual Elixir modules
- **Service coordination**: Test service coordination logic
- **Network isolation**: Test network isolation logic
- **Business logic**: Test business logic
- **Avoid redundancy**: Avoid redundant test logic - reuse test utilities and helpers
- **Container-based**: All unit tests must run in container environment

### Integration Tests

- **Service integration**: Test integration with Rust microservices
- **Device communication**: Test device communication workflows
- **Marketplace workflows**: Test marketplace workflows
- **Payment workflows**: Test payment workflows
- **API tests**: API tests are critical - test all API endpoints for security and functionality
- **WebSocket tests**: WebSocket tests are critical - test all WebSocket connections for security and functionality
- **Container-based**: All integration tests must run in container environment

### Critical Test Areas

- **API security**: API tests are critical test areas - everything must work and be secured
- **WebSocket security**: WebSocket tests are critical test areas - everything must work and be secured
- **Unauthorized access prevention**: All API and WebSocket endpoints must be tested to ensure unauthorized users cannot access data
- **Network isolation**: Test network isolation thoroughly to ensure users cannot access other networks

### Test Coverage

- **Minimum coverage**: Maintain minimum 80% test coverage
- **Critical paths**: 100% coverage for critical paths (network isolation, authentication, service coordination)
- **Security code**: High coverage for all security-related code
- **API security**: 100% coverage for API security tests - ensure unauthorized access is prevented
- **WebSocket security**: 100% coverage for WebSocket security tests - ensure unauthorized access is prevented

## Security Considerations

### Network Isolation

- **Complete isolation**: Complete network isolation (highest priority)
- **No visibility**: Users cannot see or access other networks
- **Network segmentation**: Separate network segments
- **VPC isolation**: VPC isolation for each tenant
- **Kubernetes network policies**: Strict network policies
- **RBAC**: Role-Based Access Control
- **Unauthorized access prevention**: Strict access control to ensure unauthorized users cannot access data

### Security Features

- **TLS encryption**: TLS 1.3 for all connections
- **Authentication**: JWT-based authentication
- **Authorization**: Role-based authorization with strict network isolation
- **Rate limiting**: Rate limiting for API protection
- **DDoS protection**: DDoS protection
- **Data encryption**: Encrypt sensitive data
- **Security monitoring**: Continuous security monitoring and threat detection
- **Network monitoring**: Continuous monitoring for unauthorized cross-network access
- **API endpoint security**: All API endpoints must be secured to prevent unauthorized data access
- **WebSocket security**: All WebSocket connections must be secured to prevent unauthorized data access

### GDPR Compliance (EU/German Data Protection)

- **Data minimization**: Only collect and process necessary data
- **Data encryption**: Encrypt all personal data at rest and in transit
- **Access control**: Strict access control to prevent unauthorized data access
- **Audit logging**: Log all data access for compliance auditing
- **Right to access**: Support user right to access their data
- **Right to erasure**: Support user right to delete their data
- **Right to data portability**: Support user right to export their data
- **Privacy by design**: Consider privacy from the design phase
- **Privacy by default**: Default settings must be privacy-friendly
- **BDSG compliance**: Comply with Bundesdatenschutzgesetz (BDSG) requirements
- **Data processing agreements**: Ensure proper data processing agreements are in place
- **Breach notification**: Implement breach notification procedures as required by GDPR

## Performance Requirements

### API Performance

- **Low latency**: Low API latency (< 100ms for standard requests)
- **High throughput**: High throughput (1000+ requests/second per instance)
- **Fast database queries**: Fast database queries (< 50ms for standard queries)
- **Efficient scaling**: Linear scaling with additional instances

### Resource Management

- **Memory usage**: Efficient memory usage (Erlang VM handles this well) - minimize RAM consumption
- **CPU usage**: Optimize CPU usage for service coordination
- **Connection management**: Efficient connection management for millions of connections
- **Code efficiency**: Write efficient, short code that uses minimal resources
- **Code brevity**: Code should be as short as possible while maintaining readability
- **Failsafe**: Code must be failsafe - no errors that could crash the system
- **User experience**: Ensure pleasant user experience - do not slow down user devices

## Additional Resources

- See `yggdrasil/README.md` for detailed service documentation
- Das `edda` Verzeichnis ist **KEIN PROJEKT**. Es dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte. Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden.
- Note: For general development principles, see `AGENTS.md` in the parent directory (informational reference only)

