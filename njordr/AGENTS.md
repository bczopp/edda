# AGENTS.md - Njörðr Development Guidelines

> **Hinweis**: Für Hintergrundinformationen zum Projekt, Features, Architektur und technische Details siehe `README.md`.

Diese Datei enthält ausschließlich Richtlinien für das Verhalten des Agents bei der Entwicklung.

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: Red (failing test) → Green (passing test) → Refactor
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests for marketplace operations, integration tests for gRPC service, end-to-end tests for complete workflows
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is fundamental for marketplace operations.

- **Input validation**: All inputs must be validated and sanitized
- **Provider verification**: Verify provider registrations
- **Transaction security**: Secure transaction management
- **Audit logging**: Audit logging for all marketplace operations
- **No hardcoded secrets**: Never commit secrets, passwords, or API keys
- **Access control**: Access control for marketplace operations

### 3. Performance

**Performance from the Start**: Njörðr must handle high request volumes efficiently.

- **Efficient routing**: Optimized routing algorithms for provider selection
- **Caching**: Intelligent caching of provider information and quality metrics
- **Batch processing**: Batch processing for analytics aggregation
- **Minimal memory footprint**: Minimize memory usage and RAM consumption
- **Code brevity**: Code should be as short as possible while maintaining readability

### 4. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality.

- **Check existing services**: Before implementing new functionality, check if a service already provides it
- **Reuse separate projects**: Wenn gemeinsame Komponenten (DTOs, Protocols, Utils) benötigt werden, sollten separate Projekte erstellt werden
- **Service discovery**: Know what other services provide and use them
- **Avoid duplication**: Don't duplicate marketplace logic

### 5. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple, clear solutions.

- **Simple algorithms**: Keep routing algorithms simple and clear
- **Clear structure**: Maintain clear marketplace structure definitions
- **Readable code**: Code should be self-documenting
- **Avoid over-engineering**: Don't add unnecessary complexity

### 6. CQRS (Command Query Responsibility Segregation)

**Software-Level Separation**: Separate read and write operations.

- **Marketplace operations (commands)**: Separate handlers for provider management, request routing, transaction management
- **Marketplace queries (queries)**: Separate handlers for provider queries, analytics queries
- **Optimization**: Optimize operations and queries independently

### 7. Single Responsibility Principle

**One Responsibility Per Component**: Each component should have a single responsibility.

- **Provider management**: Provider management only
- **Request routing**: Request routing only
- **Transaction management**: Transaction management only
- **Marketplace coordination**: Marketplace coordination only

### 8. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected.

- **gRPC server**: Inject gRPC server
- **Heidrun client**: Inject Heidrun client for pricing
- **Eikthyrnir client**: Inject Eikthyrnir client for quality assessment
- **Nornen client**: Inject Nornen client for decisions
- **Configuration**: Inject configuration
- **Cache**: Inject cache (if used)

## Development Workflow

Follow this workflow for all coding tasks:

### 1. Analyze and Understand the Problem

- **Understand requirements**: Read and understand the requirements thoroughly
- **Identify constraints**: Identify performance and scalability constraints
- **Research existing solutions**: Check if similar marketplace features exist
- **Document assumptions**: Document any assumptions you make

### 2. Plan Your Steps, Create Todos

#### 2.1. List All Classes/Structures/Functions That Need to Be Implemented

- **Identify components**: List all classes, structs, functions, and modules needed
- **Define interfaces**: Define interfaces for provider management, request routing, transaction management
- **Plan dependencies**: Identify dependencies between components
- **Create structure**: Plan the file and directory structure
- **Document design**: Document the design decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for each component
- **Test edge cases**: Include tests for edge cases and error conditions
- **Test marketplace operations**: Write tests for provider management, request routing, transaction management
- **Test gRPC service**: Write tests for gRPC service implementation
- **Test service integration**: Write tests for integration with Heidrun, Eikthyrnir, Nornen
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
- **Check integration**: Verify integration with Yggdrasil, Heidrun, Eikthyrnir, Nornen
- **Performance check**: Verify performance requirements (high request volumes)
- **Security review**: Review security implications
- **If not complete**: If the goal is not achieved, go back to Step 1

### 5. Give a Note to the User

- **Document changes**: Document all changes made
- **Explain usage**: Explain how to use the new code
- **Provide examples**: Provide code examples if helpful
- **List dependencies**: List any new dependencies
- **Migration notes**: Include migration notes if breaking changes

## Service-Specific Guidelines

### Provider Management

- **Provider registration**: Manage provider registrations
- **Provider configuration**: Manage provider configurations (pricing, models, capacity)
- **Provider discovery**: Discover and match providers
- **Provider status**: Track provider status (available, busy, offline)

### Request Routing

- **Request reception**: Receive compute requests
- **Provider matching**: Match requests with suitable providers
- **Route selection**: Select optimal provider based on fair distribution algorithm
- **Request routing**: Route requests to selected providers
- **Load balancing**: Distribute load across providers

### Transaction Management

- **Transaction tracking**: Track all marketplace transactions
- **Transaction lifecycle**: Manage transaction lifecycle (PENDING, PROCESSING, COMPLETED, FAILED, CANCELLED, REFUNDED)
- **Transaction coordination**: Coordinate with Heidrun for settlement
- **Refund handling**: Manage refunds
- **Dispute resolution**: Support dispute resolution

### Fair Distribution Algorithm

- **Fairness score**: Score based on previous usage
- **Round-robin**: Rotation when conditions are equal
- **Quality weighting**: Weighting based on quality metrics (from Eikthyrnir)
- **Cost optimization**: Optimization based on costs (from Heidrun)

### Service Integration

- **Heidrun**: Pricing, settlement, pre-authorization
- **Eikthyrnir**: Quality assessment for provider selection
- **Nornen**: Provider registration approval, request decisions, user configuration

### gRPC Service

- **gRPC-based**: Type-safe, efficient, HTTP/2
- **Protobuf**: Binary, compact, automatic code generation
- **Asynchronous**: Asynchronous request/response handling
- **Error handling**: Robust error handling with status codes
- **Type-Safe**: Protobuf guarantees correct request/response structures

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

- **Marketplace operation logic**: Test provider management, request routing, transaction management logic
- **Edge cases**: Test edge cases (no providers, single provider, etc.)
- **Algorithm logic**: Test fair distribution algorithm logic
- **Container-based**: All unit tests must run in container environment

### Integration Tests

- **gRPC service**: Test gRPC service integration
- **Yggdrasil integration**: Test integration with Yggdrasil
- **Service integration**: Test integration with Heidrun, Eikthyrnir, Nornen
- **End-to-end workflows**: Test complete marketplace workflows
- **Error handling**: Test error handling and recovery
- **Container-based**: All integration tests must run in container environment

### Test Coverage

- **Minimum coverage**: Maintain minimum 80% test coverage
- **Critical paths**: 100% coverage for critical paths (provider management, request routing, transaction management, gRPC service)
- **Operation code**: High coverage for all marketplace operation code
- **Avoid redundancy**: Avoid redundant test logic - reuse test utilities and helpers

## Security Considerations

### Marketplace Security

- **Input validation**: Validate all inputs (requests, provider configurations, etc.)
- **Provider verification**: Verify provider registrations
- **Transaction security**: Secure transaction management
- **Audit logging**: Log all marketplace operations for audit purposes
- **Data integrity**: Ensure data integrity in all operations
- **Access control**: Strict access control for marketplace operations

### API Security

- **Authentication**: Proper authentication for all endpoints
- **Authorization**: Role-based access control where appropriate
- **Input sanitization**: Sanitize all inputs
- **Output encoding**: Encode outputs to prevent injection attacks

### GDPR Compliance (EU/German Data Protection)

- **Data minimization**: Only process necessary data
- **Privacy by design**: Consider privacy from the design phase
- **Data encryption**: Encrypt sensitive data at rest and in transit
- **Access control**: Strict access control to prevent unauthorized data access

## Performance Requirements

### Marketplace Performance

- **Fast routing**: Fast request routing and provider selection
- **Efficient algorithms**: Use efficient algorithms for provider matching and routing
- **Caching**: Cache provider information and quality metrics for better performance
- **Batch processing**: Batch processing for analytics aggregation

### Resource Management

- **Memory management**: Efficient memory management - minimize RAM consumption
- **CPU management**: Optimize CPU usage
- **Code efficiency**: Write efficient, short code that uses minimal resources
- **Code brevity**: Code should be as short as possible while maintaining readability
- **Failsafe**: Code must be failsafe - no errors that could crash the system
- **User experience**: Ensure pleasant user experience - do not slow down user devices

## Additional Resources

- See `njordr/README.md` for detailed service documentation
- Das `edda` Verzeichnis ist **KEIN PROJEKT**. Es dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte. Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden.
- See `docs/planning/phase7-marketplace-plan.md` for marketplace planning
- Note: For general development principles, see `AGENTS.md` in the parent directory (informational reference only)

