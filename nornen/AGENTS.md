# AGENTS.md - Nornen Development Guidelines

> **Hinweis**: Für Hintergrundinformationen zum Projekt, Features, Architektur und technische Details siehe `README.md`.

Diese Datei enthält ausschließlich Richtlinien für das Verhalten des Agents bei der Entwicklung.

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: Red (failing test) → Green (passing test) → Refactor
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests for decision logic, integration tests for provider management, end-to-end tests for analytics workflows
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is important for decision service.

- **Input validation**: Validate all decision requests
- **Access control**: Control access to admin API
- **Secure defaults**: Secure default configurations
- **No hardcoded secrets**: Never commit secrets or credentials
- **Audit logging**: Log all decision-making processes

### 3. Performance

**Performance from the Start**: Nornen must provide fast decisions.

- **Fast decisions**: Fast decision-making (< 100ms for standard decisions)
- **Efficient analytics**: Efficient analytics queries (< 200ms for standard analytics)
- **Caching**: Intelligent caching for frequently accessed data
- **Async processing**: Async processing of requests
- **Database optimization**: Optimized database queries via Mimir
- **Batch processing**: Batch processing for analytics requests

### 4. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality.

- **Check existing decision logic**: Before implementing new decision logic, check if similar exists
- **Reuse analytics components**: Reuse common analytics components
- **Shared utilities**: Wenn gemeinsame Utils benötigt werden, sollte ein separates Projekt erstellt werden
- **Avoid duplication**: Don't duplicate decision or analytics logic

### 5. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple decision and analytics logic.

- **Simple decisions**: Keep decision logic simple and effective
- **Clear analytics**: Maintain clear analytics logic
- **Readable code**: Code should be self-documenting
- **Avoid over-engineering**: Don't add unnecessary complexity

### 6. CQRS (Command Query Responsibility Segregation)

**Software-Level Separation**: Separate read and write operations.

- **Decision making (commands)**: Separate handlers for decision operations
- **Analytics queries (queries)**: Separate handlers for analytics queries
- **Configuration queries**: Separate handlers for configuration queries
- **Optimization**: Optimize decisions and queries independently

### 7. Single Responsibility Principle

**One Responsibility Per Component**: Each component should have a single responsibility.

- **Urd (Past)**: History and past statistics only
- **Verdandi (Present)**: Current statistics and real-time analytics only
- **Provider management**: Provider registration management only
- **Admin API**: Admin API only

### 8. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected.

- **Mimir client**: Inject Mimir client for database access
- **Analytics components**: Inject analytics components
- **Configuration**: Inject configuration
- **Service clients**: Inject service clients (Heidrun, Eikthyrnir, etc.)

## Development Workflow

Follow this workflow for all coding tasks:

### 1. Analyze and Understand the Problem

- **Understand requirements**: Read and understand the requirements thoroughly
- **Identify constraints**: Identify technical, performance, and security constraints
- **Research existing solutions**: Check if similar decision or analytics functionality exists
- **Business logic**: Understand business logic requirements
- **Document assumptions**: Document any assumptions you make

### 2. Plan Your Steps, Create Todos

#### 2.1. List All Classes/Structures/Functions That Need to Be Implemented

- **Identify components**: List all classes, structs, functions, and modules needed
- **Define interfaces**: Define interfaces for decision making, analytics, provider management
- **Plan dependencies**: Identify dependencies between components
- **Create structure**: Plan the file and directory structure
- **Document design**: Document the design decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for each component
- **Test edge cases**: Include tests for edge cases and error conditions
- **Test decision logic**: Write tests for decision-making scenarios
- **Test provider management**: Write tests for provider registration workflows
- **Test analytics**: Write tests for analytics calculations
- **Test admin API**: Write tests for admin API endpoints
- **Mock dependencies**: Use mocks for external dependencies (Mimir, services)
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
- **Check integration**: Verify integration with Nidhöggr, Mimir, Heidrun, Eikthyrnir
- **Performance check**: Verify performance requirements (fast decisions, efficient analytics)
- **Security review**: Review security implications (access control, audit logging)
- **If not complete**: If the goal is not achieved, go back to Step 1

### 5. Give a Note to the User

- **Document changes**: Document all changes made
- **Explain usage**: Explain how to use the new code
- **Provide examples**: Provide code examples if helpful
- **List dependencies**: List any new dependencies
- **Migration notes**: Include migration notes if breaking changes

## Service-Specific Guidelines

### Decision Making

- **Request decisions**: Make decisions about incoming/outgoing requests
- **Request validation**: Validate requests based on business logic
- **Request routing**: Decide on request routing
- **Request prioritization**: Prioritize requests based on various factors
- **Provider registration**: Approve or reject provider registrations

### Provider Management

- **Provider approval**: Approve or reject provider registrations
- **Provider validation**: Validate provider capabilities and requirements
- **Provider monitoring**: Monitor provider performance
- **Provider administration**: Manage provider registrations

### User Configuration

- **Configuration storage**: Store user configuration for marketplace
- **Configuration management**: Manage user configurations
- **Configuration validation**: Validate user configurations
- **Configuration synchronization**: Synchronize configurations between devices

### Analytics (Urd & Verdandi)

- **Urd (Past)**: History, request history, historical statistics
- **Verdandi (Present)**: Current statistics, real-time analytics, live metrics
- **Provider analytics**: Request statistics, earnings, quality metrics, usage patterns per provider
- **Requester analytics**: Request history, cost analysis, quality metrics, usage patterns per requester
- **Aggregation**: Time-based aggregation, trend analysis

### Admin API

- **Health check**: Health check endpoints for monitoring
- **Dashboard information**: Provide dashboard data
- **Monitoring data**: Provide monitoring data
- **Admin information**: All information admins need

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

- **Decision logic**: Test decision-making logic
- **Provider management**: Test provider registration workflows
- **Analytics**: Test analytics calculations
- **Admin API**: Test admin API endpoints
- **Avoid redundancy**: Avoid redundant test logic - reuse test utilities and helpers
- **Container-based**: All unit tests must run in container environment

### Integration Tests

- **Decision workflows**: Test complete decision workflows
- **Provider registration**: Test provider registration workflows
- **Analytics workflows**: Test analytics workflows
- **Mimir integration**: Test integration with Mimir
- **API tests**: API tests are critical - test all API endpoints for security and functionality
- **Container-based**: All integration tests must run in container environment

### Critical Test Areas

- **API security**: API tests are critical test areas - everything must work and be secured
- **Unauthorized access prevention**: All API endpoints must be tested to ensure unauthorized users cannot access data
- **Admin API security**: Admin API must be thoroughly tested for security

### Test Coverage

- **Minimum coverage**: Maintain minimum 80% test coverage
- **Critical paths**: 100% coverage for critical paths (decision making, provider management)
- **Analytics code**: High coverage for analytics code
- **API security**: 100% coverage for API security tests - ensure unauthorized access is prevented

## Security Considerations

### Access Control

- **Admin API access**: Control access to admin API
- **Decision authorization**: Authorize decision requests
- **Provider management**: Control access to provider management
- **Unauthorized access prevention**: Strict access control to ensure unauthorized users cannot access data

### Audit Logging

- **Decision logging**: Log all decision-making processes
- **Provider management logging**: Log all provider management operations
- **Admin API logging**: Log all admin API access

### GDPR Compliance (EU/German Data Protection)

- **Data minimization**: Only collect and process necessary data
- **Data encryption**: Encrypt all personal data at rest and in transit
- **Access control**: Strict access control to prevent unauthorized data access
- **Audit logging**: Log all data access for compliance auditing
- **Right to erasure**: Support user right to delete their data
- **Privacy by design**: Consider privacy from the design phase
- **BDSG compliance**: Comply with Bundesdatenschutzgesetz (BDSG) requirements

## Performance Requirements

### Decision Performance

- **Fast decisions**: Fast decision-making (< 100ms for standard decisions)
- **Efficient analytics**: Efficient analytics queries (< 200ms for standard analytics)
- **High throughput**: High throughput (100+ requests/second per instance)

### Resource Management

- **Memory usage**: Efficient memory usage for analytics - minimize RAM consumption
- **CPU usage**: Optimize CPU usage for decision calculations
- **Database access**: Efficient database access via Mimir
- **Code efficiency**: Write efficient, short code that uses minimal resources
- **Code brevity**: Code should be as short as possible while maintaining readability
- **Failsafe**: Code must be failsafe - no errors that could crash the system
- **User experience**: Ensure pleasant user experience - do not slow down user devices

## Additional Resources

- See `nornen/README.md` for detailed service documentation
- Das `edda` Verzeichnis ist **KEIN PROJEKT**. Es dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte. Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden.
- Note: For general development principles, see `AGENTS.md` in the parent directory (informational reference only)

