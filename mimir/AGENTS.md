# AGENTS.md - Mimir Development Guidelines

> **Hinweis**: Für Hintergrundinformationen zum Projekt, Features, Architektur und technische Details siehe `README.md`.

Diese Datei enthält ausschließlich Richtlinien für das Verhalten des Agents bei der Entwicklung.

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: Red (failing test) → Green (passing test) → Refactor
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests for database operations, integration tests for query optimization, end-to-end tests for GDPR compliance workflows
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is the primary responsibility of Mimir.

- **Encryption**: Encrypt all personally identifiable data (at rest and in transit)
- **Access control**: Strict access control based on user identity
- **Audit logging**: Complete audit logging of all data access
- **Data minimization**: Only store necessary data
- **No hardcoded secrets**: Never commit secrets or keys

### 3. Performance

**Performance from the Start**: Mimir must provide fast database operations.

- **Query optimization**: Optimized database queries with indexes
- **Connection pooling**: Efficient connection pooling
- **Caching**: Intelligent caching for frequently accessed data
- **Database sharding**: Support for database sharding
- **Async operations**: Async database operations

### 4. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality.

- **Check existing database utilities**: Before implementing new database functionality, check if a separate project already exists
- **Reuse query patterns**: Reuse common query patterns from separate projects
- **Shared utilities**: Wenn gemeinsame Utils benötigt werden, sollte ein separates Projekt erstellt werden
- **Avoid duplication**: Don't duplicate database operation logic

### 5. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple database operations.

- **Simple queries**: Keep queries simple and efficient
- **Clear data model**: Maintain clear data model
- **Readable code**: Code should be self-documenting
- **Avoid over-engineering**: Don't add unnecessary complexity

### 6. CQRS (Command Query Responsibility Segregation)

**Software-Level Separation**: Separate read and write operations.

- **Database writes (commands)**: Separate handlers for write operations
- **Database queries (queries)**: Separate handlers for read operations
- **Different models**: Writes and queries can use different data models
- **Optimization**: Optimize reads and writes independently

### 7. Single Responsibility Principle

**One Responsibility Per Component**: Each component should have a single responsibility.

- **Database management**: Database operations only
- **Encryption**: Data encryption/decryption only
- **Access control**: Access control only
- **Audit logging**: Audit logging only

### 8. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected.

- **Database client**: Inject database client
- **Encryption service**: Inject encryption service
- **Access control**: Inject access control service
- **Configuration**: Inject configuration

## Development Workflow

Follow this workflow for all coding tasks:

### 1. Analyze and Understand the Problem

- **Understand requirements**: Read and understand the requirements thoroughly
- **Identify constraints**: Identify technical, performance, and security constraints
- **Research existing solutions**: Check if similar database functionality exists
- **GDPR requirements**: Understand GDPR compliance requirements
- **Document assumptions**: Document any assumptions you make

### 2. Plan Your Steps, Create Todos

#### 2.1. List All Classes/Structures/Functions That Need to Be Implemented

- **Identify components**: List all classes, structs, functions, and modules needed
- **Define interfaces**: Define interfaces for database operations, encryption, access control
- **Plan dependencies**: Identify dependencies between components
- **Create structure**: Plan the file and directory structure
- **Document design**: Document the design decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for each component
- **Test edge cases**: Include tests for edge cases and error conditions
- **Test database operations**: Write tests for database queries and writes
- **Test encryption**: Write tests for data encryption/decryption
- **Test access control**: Write tests for access control
- **Test GDPR compliance**: Write tests for GDPR compliance features
- **Mock dependencies**: Use mocks for external dependencies (database, encryption)
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
- **Check integration**: Verify integration with Nornen and other Yggdrasil services
- **Performance check**: Verify performance requirements (fast queries, efficient writes)
- **Security review**: Review security implications (encryption, access control, audit logging)
- **GDPR compliance**: Verify GDPR compliance
- **If not complete**: If the goal is not achieved, go back to Step 1

### 5. Give a Note to the User

- **Document changes**: Document all changes made
- **Explain usage**: Explain how to use the new code
- **Provide examples**: Provide code examples if helpful
- **List dependencies**: List any new dependencies
- **Migration notes**: Include migration notes if breaking changes

## Service-Specific Guidelines

### Database Management

- **Isolated database**: Completely separate database from other databases
- **Encrypted storage**: Encrypt all personally identifiable data
- **Query optimization**: Optimized queries with indexes
- **Connection pooling**: Efficient connection pooling
- **Transaction management**: Manage database transactions
- **Database sharding**: Support for database sharding

### Encryption

- **At-rest encryption**: Encrypt all data in database
- **In-transit encryption**: Encrypt all data in transit
- **Key management**: Secure key management
- **Encryption algorithms**: Modern encryption algorithms (AES-256, etc.)

### Access Control

- **Role-based access control**: RBAC for access control
- **User context validation**: Validate user context for every access
- **Permission checking**: Check permissions for every data access
- **Multi-factor authentication**: Support for multi-factor authentication

### Audit Logging

- **Complete logging**: Log all data access
- **Immutable logs**: Logs cannot be modified
- **Compliance logging**: Logging meets compliance requirements
- **Log retention**: Long-term log retention

### GDPR Compliance

- **Right to access**: Users can access their data
- **Right to rectification**: Users can correct their data
- **Right to erasure**: Users can delete their data ("Right to Deletion")
- **Right to data portability**: Users can export their data
- **Right to object**: Users can object to data processing
- **Data minimization**: Only store necessary data
- **Purpose limitation**: Use data only for specific purposes
- **Storage limitation**: Store data only as long as necessary

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

- **Database operations**: Test database queries and writes
- **Encryption**: Test data encryption/decryption
- **Access control**: Test access control logic
- **Audit logging**: Test audit logging
- **Avoid redundancy**: Avoid redundant test logic - reuse test utilities and helpers
- **Container-based**: All unit tests must run in container environment

### Integration Tests

- **Database workflows**: Test complete database workflows
- **GDPR compliance**: Test GDPR compliance features
- **Service integration**: Test integration with Nornen and other services
- **Error handling**: Test error handling and recovery
- **API tests**: API tests are critical - test all API endpoints for security and functionality
- **Container-based**: All integration tests must run in container environment

### Critical Test Areas

- **API security**: API tests are critical test areas - everything must work and be secured
- **Unauthorized access prevention**: All API endpoints must be tested to ensure unauthorized users cannot access data
- **Data access control**: Test data access control thoroughly

### Test Coverage

- **Minimum coverage**: Maintain minimum 80% test coverage
- **Critical paths**: 100% coverage for critical paths (database operations, encryption, access control)
- **GDPR features**: High coverage for GDPR compliance code
- **API security**: 100% coverage for API security tests - ensure unauthorized access is prevented

## Security Considerations

### Data Security

- **Encryption**: Encrypt all personally identifiable data
- **Access control**: Strict access control
- **Audit logging**: Complete audit logging
- **Secure backups**: Encrypted backups for disaster recovery
- **Unauthorized access prevention**: Strict access control to ensure unauthorized users cannot access data

### Key Management

- **Secure key storage**: Secure storage for encryption keys
- **Key rotation**: Support key rotation
- **Key access control**: Control access to encryption keys

### GDPR Compliance (EU/German Data Protection)

- **Data minimization**: Only collect and process necessary data
- **Purpose limitation**: Data may only be used for specified purpose
- **Storage limitation**: Data must not be stored longer than necessary
- **Data accuracy**: Ensure data accuracy and allow corrections
- **Integrity and confidentiality**: Ensure data security and confidentiality
- **Right to access**: Support user right to access their data
- **Right to rectification**: Support user right to correct their data
- **Right to erasure**: Support user right to delete their data ("Right to be forgotten")
- **Right to data portability**: Support user right to export their data
- **Right to object**: Support user right to object to data processing
- **Privacy by design**: Privacy must be considered from the design phase
- **Privacy by default**: Default settings must be privacy-friendly
- **BDSG compliance**: Comply with Bundesdatenschutzgesetz (BDSG) requirements
- **Data processing agreements**: Ensure proper data processing agreements are in place
- **Breach notification**: Implement breach notification procedures as required by GDPR

## Performance Requirements

### Database Performance

- **Fast queries**: Fast queries (< 50ms for standard queries)
- **Efficient writes**: Efficient writes (< 100ms for standard writes)
- **High throughput**: High throughput (1000+ queries/second per instance)
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

- See `mimir/README.md` for detailed service documentation
- Das `edda` Verzeichnis ist **KEIN PROJEKT**. Es dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte. Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden.
- Note: For general development principles, see `AGENTS.md` in the parent directory (informational reference only)

