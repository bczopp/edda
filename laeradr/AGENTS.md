# AGENTS.md - Læraðr Development Guidelines

> **Hinweis**: Für Hintergrundinformationen zum Projekt, Features, Architektur und technische Details siehe `README.md`.

Diese Datei enthält ausschließlich Richtlinien für das Verhalten des Agents bei der Entwicklung.

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: Red (failing test) → Green (passing test) → Refactor
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests for data management operations, integration tests for gRPC service, end-to-end tests for complete workflows
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is fundamental for data management.

- **Input validation**: All inputs must be validated and sanitized
- **Secure operations**: Secure data management operations
- **Audit logging**: Audit logging for all data management operations
- **No hardcoded secrets**: Never commit secrets, passwords, or API keys
- **Access control**: Access control for data management operations

### 3. Performance

**Performance from the Start**: Læraðr must handle high data volumes efficiently.

- **Efficient indexing**: Optimized indexing algorithms
- **Caching**: Intelligent caching of frequently used data
- **Batch processing**: Batch processing for aggregation and cleanup
- **Minimal memory footprint**: Minimize memory usage and RAM consumption
- **Code brevity**: Code should be as short as possible while maintaining readability

### 4. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality.

- **Check existing services**: Before implementing new functionality, check if a service already provides it
- **Reuse existing code**: Use existing functions, services, and utilities
- **Service discovery**: Know what other services provide and use them
- **Avoid duplication**: Don't duplicate data management logic

### 5. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple, clear solutions.

- **Simple operations**: Keep data management operations simple and clear
- **Clear structure**: Maintain clear data structure definitions
- **Readable code**: Code should be self-documenting
- **Avoid over-engineering**: Don't add unnecessary complexity

### 6. CQRS (Command Query Responsibility Segregation)

**Software-Level Separation**: Separate read and write operations.

- **Data operations (commands)**: Separate handlers for data indexing, validation, aggregation, retention
- **Data queries (queries)**: Separate handlers for data queries
- **Optimization**: Optimize operations and queries independently

### 7. Single Responsibility Principle

**One Responsibility Per Component**: Each component should have a single responsibility.

- **Dáinn (Indexing)**: Data indexing only
- **Dvalinn (Validation)**: Data validation only
- **Duneyrr (Aggregation)**: Data aggregation only
- **Duraþrór (Retention)**: Data retention only

### 8. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected.

- **gRPC server**: Inject gRPC server
- **Indexing engine**: Inject indexing engine
- **Validation engine**: Inject validation engine
- **Aggregation engine**: Inject aggregation engine
- **Retention engine**: Inject retention engine
- **Configuration**: Inject configuration
- **Cache**: Inject cache (if used)

## Development Workflow

Follow this workflow for all coding tasks:

### 1. Analyze and Understand the Problem

- **Understand requirements**: Read and understand the requirements thoroughly
- **Identify constraints**: Identify performance and data volume constraints
- **Research existing solutions**: Check if similar data management features exist
- **Document assumptions**: Document any assumptions you make

### 2. Plan Your Steps, Create Todos

#### 2.1. List All Classes/Structures/Functions That Need to Be Implemented

- **Identify components**: List all classes, structs, functions, and modules needed
- **Define interfaces**: Define interfaces for indexing, validation, aggregation, retention
- **Plan dependencies**: Identify dependencies between components
- **Create structure**: Plan the file and directory structure
- **Document design**: Document the design decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for each component
- **Test edge cases**: Include tests for edge cases and error conditions
- **Test data operations**: Write tests for indexing, validation, aggregation, retention logic
- **Test gRPC service**: Write tests for gRPC service implementation
- **Test data integrity**: Test data integrity in all operations
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
- **Check integration**: Verify integration with Yggdrasil
- **Performance check**: Verify performance requirements (high data volumes)
- **Data integrity check**: Verify data integrity in all operations
- **Security review**: Review security implications
- **If not complete**: If the goal is not achieved, go back to Step 1

### 5. Give a Note to the User

- **Document changes**: Document all changes made
- **Explain usage**: Explain how to use the new code
- **Provide examples**: Provide code examples if helpful
- **List dependencies**: List any new dependencies
- **Migration notes**: Include migration notes if breaking changes

## Service-Specific Guidelines

### Dáinn - Data Indexing

- **Indexing operations**: Index data for fast search
- **Search functionality**: Provide search functionality for indexed data
- **Index management**: Manage indexes
- **Index optimization**: Optimize indexes for performance

### Dvalinn - Data Validation

- **Validation operations**: Validate data against schemas
- **Schema checks**: Perform schema validation for data structures
- **Data integrity**: Ensure data integrity
- **Validation rules**: Manage validation rules

### Duneyrr - Data Aggregation

- **Aggregation operations**: Aggregate data for statistics
- **Statistics calculation**: Calculate statistics over data
- **Data summarization**: Summarize data
- **Aggregation functions**: Provide various aggregation functions

### Duraþrór - Data Retention

- **Retention operations**: Manage data retention
- **Archiving**: Archive old data
- **Cleanup**: Automatically clean up old data
- **Data lifecycle**: Manage data lifecycle

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

- **Data operation logic**: Test indexing, validation, aggregation, retention logic
- **Edge cases**: Test edge cases (empty data, large data volumes, etc.)
- **Data integrity**: Test data integrity in all operations
- **Container-based**: All unit tests must run in container environment

### Integration Tests

- **gRPC service**: Test gRPC service integration
- **Yggdrasil integration**: Test integration with Yggdrasil
- **End-to-end workflows**: Test complete data management workflows
- **Error handling**: Test error handling and recovery
- **Container-based**: All integration tests must run in container environment

### Test Coverage

- **Minimum coverage**: Maintain minimum 80% test coverage
- **Critical paths**: 100% coverage for critical paths (data operations, gRPC service)
- **Operation code**: High coverage for all data operation code
- **Avoid redundancy**: Avoid redundant test logic - reuse test utilities and helpers

## Security Considerations

### Data Security

- **Input validation**: Validate all inputs (data, schemas, etc.)
- **Secure operations**: Secure data management operations
- **Audit logging**: Log all data management operations for audit purposes
- **Data integrity**: Ensure data integrity in all operations
- **Access control**: Strict access control for data management operations

### API Security

- **Authentication**: Proper authentication for all endpoints
- **Authorization**: Role-based access control where appropriate
- **Input sanitization**: Sanitize all inputs
- **Output encoding**: Encode outputs to prevent injection attacks

### GDPR Compliance (EU/German Data Protection)

- **Data minimization**: Only process necessary data
- **Data retention policies**: Implement data retention policies for GDPR compliance
- **Data encryption**: Encrypt sensitive data at rest and in transit
- **Access control**: Strict access control to prevent unauthorized data access

## Performance Requirements

### Data Management Performance

- **Fast indexing**: Fast data indexing operations
- **Efficient validation**: Efficient data validation
- **Efficient aggregation**: Efficient data aggregation algorithms
- **Efficient cleanup**: Efficient data cleanup operations
- **Caching**: Cache frequently used data for better performance

### Resource Management

- **Memory management**: Efficient memory management - minimize RAM consumption
- **CPU management**: Optimize CPU usage
- **Code efficiency**: Write efficient, short code that uses minimal resources
- **Code brevity**: Code should be as short as possible while maintaining readability
- **Failsafe**: Code must be failsafe - no errors that could crash the system
- **User experience**: Ensure pleasant user experience - do not slow down user devices

## Additional Resources

- See `laeradr/README.md` for detailed service documentation
- Note: For general development principles, see `AGENTS.md` in the parent directory (informational reference only)

