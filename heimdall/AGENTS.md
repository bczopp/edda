# AGENTS.md - Heimdall Development Guidelines

> **Hinweis**: Für Hintergrundinformationen zum Projekt, Features, Architektur und technische Details siehe `README.md`.

Diese Datei enthält ausschließlich Richtlinien für das Verhalten des Agents bei der Entwicklung.

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: Red (failing test) → Green (passing test) → Refactor
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests for authentication/authorization, integration tests for token management, end-to-end tests for security workflows
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is the primary responsibility of Heimdall.

- **Input validation**: Validate all authentication and authorization inputs
- **Secure token storage**: Secure storage for tokens
- **Key management**: Secure key management for cryptographic operations
- **No hardcoded secrets**: Never commit secrets, passwords, or keys
- **Fail-safe**: Deny by default, allow only when explicitly authorized

### 3. Performance

**Performance from the Start**: Heimdall must provide fast security checks.

- **Fast token validation**: Optimized token validation (< 10ms for standard tokens)
- **Caching**: Intelligent caching for token validations and permissions
- **Parallel processing**: Parallel processing of multiple security checks
- **Connection pooling**: Efficient connection pooling for database access
- **Optimized algorithms**: Optimized security algorithms for fast checks

### 4. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality.

- **Check existing security utilities**: Before implementing new security functionality, check if a separate project already exists
- **Reuse crypto utilities**: Reuse cryptographic utilities
- **Shared token logic**: Reuse common token management logic
- **Avoid duplication**: Don't duplicate security checking logic

### 5. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple security logic.

- **Simple authentication**: Keep authentication logic simple and clear
- **Clear authorization**: Maintain clear authorization rules
- **Readable code**: Code should be self-documenting
- **Avoid over-engineering**: Don't add unnecessary complexity

### 6. CQRS (Command Query Responsibility Segregation)

**Software-Level Separation**: Separate read and write operations.

- **Authentication (commands)**: Separate handlers for authentication operations
- **Authorization queries**: Separate handlers for permission queries
- **Token management**: Separate handlers for token creation and validation
- **Optimization**: Optimize authentication and queries independently

### 7. Single Responsibility Principle

**One Responsibility Per Component**: Each component should have a single responsibility.

- **Authentication**: Device authentication only
- **Authorization**: Permission checking only
- **Token management**: Token creation and validation only
- **Connection validation**: Bifrost connection validation only

### 8. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected.

- **Key manager**: Inject key manager for cryptographic operations
- **Token store**: Inject token storage
- **Permission store**: Inject permission storage
- **Configuration**: Inject configuration

## Development Workflow

Follow this workflow for all coding tasks:

### 1. Analyze and Understand the Problem

- **Understand requirements**: Read and understand the requirements thoroughly
- **Identify constraints**: Identify technical, performance, and security constraints
- **Research existing solutions**: Check if similar security functionality exists
- **Security implications**: Understand security implications of the change
- **Document assumptions**: Document any assumptions you make

### 2. Plan Your Steps, Create Todos

#### 2.1. List All Classes/Structures/Functions That Need to Be Implemented

- **Identify components**: List all classes, structs, functions, and modules needed
- **Define interfaces**: Define interfaces for authentication, authorization, token management
- **Plan dependencies**: Identify dependencies between components
- **Create structure**: Plan the file and directory structure
- **Document design**: Document the design decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for each component
- **Test edge cases**: Include tests for edge cases and error conditions
- **Test authentication**: Write tests for authentication scenarios
- **Test authorization**: Write tests for authorization scenarios
- **Test token management**: Write tests for token creation, validation, renewal
- **Test security**: Write tests for security attack scenarios
- **Mock dependencies**: Use mocks for external dependencies (database, key storage)
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
- **Check integration**: Verify integration with Odin, Bifrost, Thor
- **Performance check**: Verify performance requirements (fast token validation, low latency)
- **Security review**: Review security implications (authentication, authorization, token security)
- **If not complete**: If the goal is not achieved, go back to Step 1

### 5. Give a Note to the User

- **Document changes**: Document all changes made
- **Explain usage**: Explain how to use the new code
- **Provide examples**: Provide code examples if helpful
- **List dependencies**: List any new dependencies
- **Migration notes**: Include migration notes if breaking changes

## Service-Specific Guidelines

### Authentication

- **Device authentication**: Verify device identities
- **Public/private keys**: Use public/private key pairs for authentication
- **Digital signatures**: Use digital signatures for message validation
- **Token-based**: Token-based authentication after initial authentication
- **Multi-factor**: Optional multi-factor authentication
- **Automatic**: Everything automatic, minimal user disturbance

### Authorization

- **Permission checking**: Check permissions for all actions
- **Role-based access control**: RBAC for granular access control
- **Resource-based permissions**: Permissions based on resources
- **Conditional permissions**: Support conditional permissions
- **Policy enforcement**: Enforce security policies

### Token Management

- **Heimdall tokens**: Long-lived tokens for authentication/authorization (e.g., 24 hours)
- **Session tokens**: Short-lived tokens for Bifrost/Jotunheim connections (e.g., 1 hour)
- **Refresh tokens**: Long-lived refresh tokens for token renewal (e.g., 30 days)
- **Proactive renewal**: Renew tokens before expiration (e.g., 5 minutes before)
- **Automatic renewal**: Automatic renewal without user intervention
- **Token revocation**: Immediate revocation + timeout as fallback
- **Token leak detection**: Anomaly detection + device tracking
- **Token rotation**: Regular rotation + event-based rotation

### Bifrost Connection Validation

- **Connection authentication**: Validate Bifrost connections
- **Message validation**: Validate messages over Bifrost
- **Threat detection**: Detect suspicious activities
- **Connection blocking**: Block unauthorized connections
- **Ongoing monitoring**: Monitor connections for threats

### Guest Network Functionality

- **Guest network (default)**: Automatically create separate guest network for foreign devices
- **Isolation**: Guest network isolated from main network
- **Data transfer permission**: Explicit permission required for data transfer
- **Explicit main network access**: Multiple confirmations required for explicit main network access
- **Audit logging**: Log all guest access for security compliance

### Security Monitoring

- **Audit logging**: Log all security-relevant events
- **Threat detection**: Detect security threats
- **Incident response**: Automatic response to security incidents
- **Security analytics**: Analyze security events

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

- **Authentication**: Test authentication logic
- **Authorization**: Test authorization logic
- **Token management**: Test token creation, validation, renewal, revocation
- **Crypto operations**: Test cryptographic operations
- **Container-based**: All unit tests must run in container environment

### Integration Tests

- **Bifrost validation**: Test Bifrost connection validation
- **Token flow**: Test complete token flow (creation, validation, renewal)
- **Permission checking**: Test permission checking workflows
- **Guest network**: Test guest network functionality
- **Container-based**: All integration tests must run in container environment

### Security Tests

- **Attack scenarios**: Test security attack scenarios
- **Token security**: Test token security (leak detection, revocation)
- **Authentication bypass**: Test authentication bypass attempts
- **Authorization bypass**: Test authorization bypass attempts
- **Container-based**: All security tests must run in container environment

### Test Coverage

- **Minimum coverage**: Maintain minimum 80% test coverage
- **Critical paths**: 100% coverage for critical paths (authentication, authorization, token validation)
- **Security code**: High coverage for all security-related code
- **Avoid redundancy**: Avoid redundant test logic - reuse test utilities and helpers

## Security Considerations

### Authentication Security

- **Public/private keys**: Secure key pairs for authentication
- **Digital signatures**: Digital signatures for message validation
- **Token security**: Secure token generation and validation
- **Multi-factor**: Optional multi-factor authentication

### Authorization Security

- **RBAC**: Granular role-based access control
- **Permission validation**: Validate all permissions
- **Policy enforcement**: Enforce security policies
- **Fail-safe**: Deny by default

### Token Security

- **Secure storage**: Secure storage for tokens
- **Token encryption**: Encrypt tokens if needed
- **Token rotation**: Regular token rotation
- **Token revocation**: Immediate token revocation

### GDPR Compliance (EU/German Data Protection)

- **Data minimization**: Only store necessary authentication data
- **Data encryption**: Encrypt all personal data
- **Access control**: Strict access control to prevent unauthorized data access
- **Audit logging**: Log all authentication and authorization events for compliance auditing
- **Right to erasure**: Support user right to delete authentication data
- **Privacy by design**: Consider privacy from the design phase
- **BDSG compliance**: Comply with Bundesdatenschutzgesetz (BDSG) requirements

### Connection Security

- **TLS encryption**: TLS 1.3 for all connections
- **Certificate validation**: Validate TLS certificates
- **Connection monitoring**: Monitor connections for threats

## Performance Requirements

### Security Check Performance

- **Fast token validation**: Fast token validation (< 10ms for standard tokens)
- **Low latency**: Low latency for permission checks (< 5ms)
- **High throughput**: High throughput for parallel security checks
- **Efficient caching**: Efficient caching for token validations and permissions

### Resource Management

- **Memory usage**: Efficient memory usage for token storage - minimize RAM consumption
- **CPU usage**: Optimize CPU usage for cryptographic operations
- **Connection management**: Efficient connection management
- **Code efficiency**: Write efficient, short code that uses minimal resources
- **Code brevity**: Code should be as short as possible while maintaining readability
- **Failsafe**: Code must be failsafe - no errors that could crash the system
- **User experience**: Ensure pleasant user experience - do not slow down user devices

## Additional Resources

- See `heimdall/README.md` for detailed service documentation
- Das `edda` Verzeichnis ist **KEIN PROJEKT**. Es dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte. Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden.
- Note: For general development principles, see `AGENTS.md` in the parent directory (informational reference only)

