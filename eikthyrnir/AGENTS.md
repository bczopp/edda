# AGENTS.md - Eikthyrnir Development Guidelines

> **Hinweis**: Für Hintergrundinformationen zum Projekt, Features, Architektur und technische Details siehe `README.md`.

Diese Datei enthält ausschließlich Richtlinien für das Verhalten des Agents bei der Entwicklung.

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: Red (failing test) → Green (passing test) → Refactor
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests for quality assessment, integration tests for gRPC service, end-to-end tests for complete workflows
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is fundamental for quality assessment.

- **Input validation**: All inputs must be validated and sanitized
- **Secure calculations**: Secure calculations for quality metrics
- **Audit logging**: Audit logging for all quality assessments
- **No hardcoded secrets**: Never commit secrets, passwords, or API keys
- **Access control**: Access control for quality configurations

### 3. Performance

**Performance from the Start**: Eikthyrnir must handle high request volumes efficiently.

- **Efficient aggregation**: Optimized aggregation algorithms for quality metrics
- **Caching**: Intelligent caching of quality metrics
- **Batch processing**: Batch processing for quality updates
- **Minimal memory footprint**: Minimize memory usage and RAM consumption
- **Code brevity**: Code should be as short as possible while maintaining readability

### 4. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality.

- **Check existing services**: Before implementing new functionality, check if a service already provides it
- **Reuse separate projects**: Wenn gemeinsame Komponenten (DTOs, Protocols, Utils) benötigt werden, sollten separate Projekte erstellt werden
- **Service discovery**: Know what other services provide and use them
- **Avoid duplication**: Don't duplicate quality assessment logic

### 5. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple, clear solutions.

- **Simple calculations**: Keep quality calculations simple and clear
- **Clear metrics**: Maintain clear quality metric definitions
- **Readable code**: Code should be self-documenting
- **Avoid over-engineering**: Don't add unnecessary complexity

### 6. CQRS (Command Query Responsibility Segregation)

**Software-Level Separation**: Separate read and write operations.

- **Quality assessment (commands)**: Separate handlers for quality assessment
- **Quality queries (queries)**: Separate handlers for quality queries
- **Aggregation (commands)**: Separate handlers for quality aggregation
- **Optimization**: Optimize assessment and queries independently

### 7. Single Responsibility Principle

**One Responsibility Per Component**: Each component should have a single responsibility.

- **Quality assessment**: Quality assessment only
- **Quality aggregation**: Quality aggregation only
- **Quality metrics**: Quality metrics calculation only
- **Quality updates**: Quality update handling only

### 8. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected.

- **gRPC server**: Inject gRPC server
- **Aggregation engine**: Inject aggregation engine
- **Configuration**: Inject configuration
- **Cache**: Inject cache (if used)

## Development Workflow

Follow this workflow for all coding tasks:

### 1. Analyze and Understand the Problem

- **Understand requirements**: Read and understand the requirements thoroughly
- **Identify constraints**: Identify performance and accuracy constraints
- **Research existing solutions**: Check if similar quality assessment features exist
- **Document assumptions**: Document any assumptions you make

### 2. Plan Your Steps, Create Todos

#### 2.1. List All Classes/Structures/Functions That Need to Be Implemented

- **Identify components**: List all classes, structs, functions, and modules needed
- **Define interfaces**: Define interfaces for quality assessment, aggregation, metrics
- **Plan dependencies**: Identify dependencies between components
- **Create structure**: Plan the file and directory structure
- **Document design**: Document the design decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for each component
- **Test edge cases**: Include tests for edge cases and error conditions
- **Test quality assessment**: Write tests for quality assessment logic
- **Test aggregation**: Write tests for quality aggregation logic
- **Test metrics**: Write tests for quality metrics calculation
- **Test gRPC service**: Write tests for gRPC service implementation
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
- **Performance check**: Verify performance requirements (high request volumes)
- **Accuracy check**: Verify quality assessment accuracy
- **Security review**: Review security implications
- **If not complete**: If the goal is not achieved, go back to Step 1

### 5. Give a Note to the User

- **Document changes**: Document all changes made
- **Explain usage**: Explain how to use the new code
- **Provide examples**: Provide code examples if helpful
- **List dependencies**: List any new dependencies
- **Migration notes**: Include migration notes if breaking changes

## Service-Specific Guidelines

### Quality Assessment

- **Post-request assessment**: Quality assessment after each request
- **Automatic measurement**: Automatic quality measurement after each request
- **Quality metrics**: Measure response quality, latency, availability
- **Periodic tests**: Regular tests complement continuous assessment

### Quality Aggregation

- **Weighted average**: Weighted average of quality metrics
- **Quality weighting**: Newer requests have higher weight
- **Time-decay**: Older requests have lower weight
- **Batch aggregation**: Batch aggregation for efficiency

### Quality Updates

- **Immediate updates**: Immediate updates for important changes
- **Batch aggregation**: Batch aggregation for normal updates
- **Update strategy**: Balance between immediate updates and efficiency
- **Quality propagation**: Propagate quality updates to relevant services

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

- **Quality assessment logic**: Test quality assessment calculation logic
- **Aggregation logic**: Test quality aggregation logic
- **Metrics calculation**: Test quality metrics calculation
- **Edge cases**: Test edge cases (no requests, single request, etc.)
- **Container-based**: All unit tests must run in container environment

### Integration Tests

- **gRPC service**: Test gRPC service integration
- **Yggdrasil integration**: Test integration with Yggdrasil
- **End-to-end workflows**: Test complete quality assessment workflows
- **Error handling**: Test error handling and recovery
- **Container-based**: All integration tests must run in container environment

### Test Coverage

- **Minimum coverage**: Maintain minimum 80% test coverage
- **Critical paths**: 100% coverage for critical paths (quality assessment, aggregation, gRPC service)
- **Calculation code**: High coverage for all quality calculation code
- **Avoid redundancy**: Avoid redundant test logic - reuse test utilities and helpers

## Security Considerations

### Quality Security

- **Input validation**: Validate all inputs (quality metrics, requests, etc.)
- **Secure calculations**: Secure calculations for quality metrics
- **Audit logging**: Log all quality assessments for audit purposes
- **Data integrity**: Ensure data integrity in quality calculations
- **Access control**: Strict access control for quality configurations

### API Security

- **Authentication**: Proper authentication for all endpoints
- **Authorization**: Role-based access control where appropriate
- **Input sanitization**: Sanitize all inputs
- **Output encoding**: Encode outputs to prevent injection attacks

### GDPR Compliance (EU/German Data Protection)

- **Data minimization**: Only process necessary data
- **No personal data**: Do not store personal data in quality metrics
- **Data encryption**: Encrypt sensitive data at rest and in transit
- **Access control**: Strict access control to prevent unauthorized data access

## Performance Requirements

### Assessment Performance

- **Fast assessment**: Fast quality assessment after each request
- **Efficient aggregation**: Efficient aggregation algorithms
- **Caching**: Cache quality metrics for better performance
- **Batch processing**: Batch processing for quality updates

### Resource Management

- **Memory management**: Efficient memory management - minimize RAM consumption
- **CPU management**: Optimize CPU usage
- **Code efficiency**: Write efficient, short code that uses minimal resources
- **Code brevity**: Code should be as short as possible while maintaining readability
- **Failsafe**: Code must be failsafe - no errors that could crash the system
- **User experience**: Ensure pleasant user experience - do not slow down user devices

## Additional Resources

- See `eikthyrnir/README.md` for detailed service documentation
- Das `edda` Verzeichnis ist **KEIN PROJEKT**. Es dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte. Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden.
- Note: For general development principles, see `AGENTS.md` in the parent directory (informational reference only)

