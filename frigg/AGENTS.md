# AGENTS.md - Frigg Development Guidelines

> **Hinweis**: Für Hintergrundinformationen zum Projekt, Features, Architektur und technische Details siehe `README.md`.

Diese Datei enthält ausschließlich Richtlinien für das Verhalten des Agents bei der Entwicklung.

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: Red (failing test) → Green (passing test) → Refactor
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests for course management, integration tests for insurance integration, end-to-end tests for healthcare workflows
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is critical for healthcare data.

- **Data encryption**: Encrypt all health data (at rest and in transit)
- **Access control**: Strict access control for health data
- **Authentication**: Secure authentication via Heimdall
- **Authorization**: Granular permission system for health data
- **Audit logging**: Complete logging of all data access
- **No hardcoded secrets**: Never commit secrets or credentials

### 3. Performance

**Performance from the Start**: Frigg must provide fast access to health data.

- **Fast data access**: Fast user data access (< 50ms for standard queries)
- **Efficient database queries**: Optimized database queries with indexes
- **Caching**: Intelligent caching for frequently accessed health data
- **Connection pooling**: Efficient connection pooling for database connections
- **Query optimization**: Special queries for frequent access
- **Partitioning**: Optional partitioning by user or time period

### 4. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality.

- **Check existing healthcare utilities**: Before implementing new healthcare functionality, check if similar exists
- **Reuse course components**: Reuse common course management components
- **Shared utilities**: Wenn gemeinsame Utils benötigt werden, sollte ein separates Projekt erstellt werden
- **Avoid duplication**: Don't duplicate healthcare logic

### 5. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple healthcare logic.

- **Simple course management**: Keep course management simple and effective
- **Clear certification logic**: Maintain clear certification logic
- **Readable code**: Code should be self-documenting
- **Avoid over-engineering**: Don't add unnecessary complexity

### 6. CQRS (Command Query Responsibility Segregation)

**Software-Level Separation**: Separate read and write operations.

- **Course management (commands)**: Separate handlers for course operations
- **Progress queries (queries)**: Separate handlers for progress queries
- **Health data queries**: Separate handlers for health data queries
- **Optimization**: Optimize operations and queries independently

### 7. Single Responsibility Principle

**One Responsibility Per Component**: Each component should have a single responsibility.

- **Course management**: Course creation and management only
- **Certification engine**: Certification generation and validation only
- **Insurance integration**: Insurance provider integration only
- **Progress tracking**: User progress monitoring only

### 8. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected.

- **Database client**: Inject database client for health data
- **Insurance provider client**: Inject insurance provider client
- **Certification service**: Inject certification service
- **Configuration**: Inject configuration

## Development Workflow

Follow this workflow for all coding tasks:

### 1. Analyze and Understand the Problem

- **Understand requirements**: Read and understand the requirements thoroughly
- **Identify constraints**: Identify technical, performance, and security constraints
- **Research existing solutions**: Check if similar healthcare functionality exists
- **Healthcare compliance**: Understand healthcare compliance requirements (GDPR, etc.)
- **Document assumptions**: Document any assumptions you make

### 2. Plan Your Steps, Create Todos

#### 2.1. List All Classes/Structures/Functions That Need to Be Implemented

- **Identify components**: List all classes, structs, functions, and modules needed
- **Define interfaces**: Define interfaces for course management, certification, insurance integration
- **Plan dependencies**: Identify dependencies between components
- **Create structure**: Plan the file and directory structure
- **Document design**: Document the design decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for each component
- **Test edge cases**: Include tests for edge cases and error conditions
- **Test course management**: Write tests for course management workflows
- **Test certification**: Write tests for certification generation and validation
- **Test insurance integration**: Write tests for insurance integration (with mocks)
- **Test progress tracking**: Write tests for progress tracking
- **Test GDPR compliance**: Write tests for GDPR compliance features
- **Mock dependencies**: Use mocks for external dependencies (database, insurance providers)
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
- **Check integration**: Verify integration with Thor and Yggdrasil
- **Performance check**: Verify performance requirements (fast data access, efficient queries)
- **Security review**: Review security implications (encryption, access control, audit logging)
- **GDPR compliance**: Verify GDPR and healthcare compliance
- **If not complete**: If the goal is not achieved, go back to Step 1

### 5. Give a Note to the User

- **Document changes**: Document all changes made
- **Explain usage**: Explain how to use the new code
- **Provide examples**: Provide code examples if helpful
- **List dependencies**: List any new dependencies
- **Migration notes**: Include migration notes if breaking changes

## Service-Specific Guidelines

### Course Management

- **Course creation**: Create and manage courses
- **Module management**: Manage course modules
- **Content management**: Manage course content
- **Quiz/assessment system**: Quiz and assessment system
- **Delegation to Fulla**: Use Fulla for data/treatment plans if needed

### Certification Engine

- **Certification generation**: Generate certifications
- **Certification validation**: Validate certifications
- **Certification tracking**: Track certifications
- **Compliance verification**: Verify compliance
- **Central storage**: Certifications stored on Yggdrasil (not in Frigg database)

### Insurance Integration

- **Activation code system**: User needs activation code from insurance
- **Yggdrasil as proxy**: Integration via Yggdrasil (Yggdrasil as proxy)
- **Activation workflow**: User enters code, Yggdrasil validates with insurance provider
- **Plan activation**: Only activated plans can be started
- **Plan activation**: Plans activated after validation

### Progress Tracking

- **User progress monitoring**: Monitor user progress
- **Completion tracking**: Track course completion
- **Certification tracking**: Track certifications
- **Analytics**: Provide analytics

### Isolated Database

- **Separate database**: Completely separate database for health data
- **Data privacy**: Health data strictly separated from other system data
- **Fast access**: Fast and easy access to user health data
- **Compliance**: Meet privacy requirements (GDPR, etc.)
- **Isolation**: No mixing with other data

### Database Features

- **Encryption**: Encrypt all health data
- **Access control**: Strict access control based on user identity
- **Audit logging**: Complete audit logging of all data access
- **Data minimization**: Only store necessary data
- **Optimized indexes**: Fast queries for user-specific data
- **Caching**: Intelligent caching for frequently accessed data

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

- **Course management**: Test course management logic
- **Certification**: Test certification generation and validation
- **Insurance integration**: Test insurance integration (mocked)
- **Progress tracking**: Test progress tracking logic
- **Avoid redundancy**: Avoid redundant test logic - reuse test utilities and helpers
- **Container-based**: All unit tests must run in container environment

### Integration Tests

- **Healthcare workflows**: Test complete healthcare workflows
- **Insurance integration**: Test insurance integration workflows
- **Database operations**: Test database operations
- **GDPR compliance**: Test GDPR compliance features
- **API tests**: API tests are critical - test all API endpoints for security and functionality
- **Container-based**: All integration tests must run in container environment

### Critical Test Areas

- **API security**: API tests are critical test areas - everything must work and be secured
- **Unauthorized access prevention**: All API endpoints must be tested to ensure unauthorized users cannot access health data
- **Health data security**: Health data access must be thoroughly tested for security

### Test Coverage

- **Minimum coverage**: Maintain minimum 80% test coverage
- **Critical paths**: 100% coverage for critical paths (course management, certification, data access)
- **Healthcare code**: High coverage for all healthcare-related code
- **API security**: 100% coverage for API security tests - ensure unauthorized access is prevented

## Security Considerations

### Data Security

- **Encryption**: Encrypt all health data (at rest and in transit)
- **Access control**: Strict access control for health data
- **Authentication**: Secure authentication via Heimdall
- **Authorization**: Granular permission system
- **Audit logging**: Complete logging of all data access
- **Secure backups**: Encrypted backups for disaster recovery
- **Unauthorized access prevention**: Strict access control to ensure unauthorized users cannot access health data

### GDPR Compliance (EU/German Data Protection)

- **Data minimization**: Only collect and process necessary health data
- **Purpose limitation**: Health data may only be used for specified purpose
- **Storage limitation**: Health data must not be stored longer than necessary
- **Data accuracy**: Ensure health data accuracy and allow corrections
- **Integrity and confidentiality**: Ensure health data security and confidentiality
- **Right to access**: Support user right to access their health data
- **Right to rectification**: Support user right to correct their health data
- **Right to erasure**: Support user right to delete their health data ("Right to be forgotten")
- **Right to data portability**: Support user right to export their health data
- **Right to object**: Support user right to object to health data processing
- **Privacy by design**: Privacy must be considered from the design phase
- **Privacy by default**: Default settings must be privacy-friendly
- **BDSG compliance**: Comply with Bundesdatenschutzgesetz (BDSG) requirements
- **Healthcare compliance**: Meet healthcare-specific privacy requirements
- **Data processing agreements**: Ensure proper data processing agreements are in place
- **Breach notification**: Implement breach notification procedures as required by GDPR

## Performance Requirements

### Data Access Performance

- **Fast user data access**: Fast user data access (< 50ms for standard queries)
- **Efficient database queries**: Optimized database queries with indexes
- **High throughput**: High throughput for parallel requests
- **Scalability**: Design for horizontal scaling

### Resource Management

- **Memory usage**: Efficient memory usage for database operations - minimize RAM consumption
- **CPU usage**: Optimize CPU usage for encryption/decryption
- **Connection management**: Efficient connection pooling
- **Code efficiency**: Write efficient, short code that uses minimal resources
- **Code brevity**: Code should be as short as possible while maintaining readability
- **Failsafe**: Code must be failsafe - no errors that could crash the system
- **User experience**: Ensure pleasant user experience - do not slow down user devices

## Additional Resources

- See `frigg/README.md` for detailed service documentation
- Das `edda` Verzeichnis ist **KEIN PROJEKT**. Es dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte. Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden.
- Note: For general development principles, see `AGENTS.md` in the parent directory (informational reference only)

