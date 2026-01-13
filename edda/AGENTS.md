# AGENTS.md - Edda Metadaten-Sammlung

> **WICHTIG**: Dieser Ordner ist **KEIN PROJEKT**! Er dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte.
> **Hinweis**: Für Hintergrundinformationen siehe `README.md`.

Diese Datei enthält Richtlinien für das Verhalten des Agents bei der Arbeit mit diesem Metadaten-Verzeichnis.

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: Red (failing test) → Green (passing test) → Refactor
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests for DTOs and protocols, integration tests for serialization, cross-language tests for compatibility
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is important for core data structures.

- **Input validation**: Validate all DTO inputs
- **Secure serialization**: Secure serialization for sensitive data
- **No hardcoded secrets**: Never commit secrets or credentials
- **Crypto utilities**: Provide secure cryptographic utilities

### 3. Performance

**Performance from the Start**: Services must be lightweight and fast.

- **Lightweight design**: Minimal dependencies for fast load times
- **Efficient serialization**: Optimized JSON/MessagePack serialization
- **Caching**: Intelligent caching where appropriate
- **Lazy loading**: Lazy loading for large data structures
- **Memory management**: Efficient memory management for minimal footprint

### 4. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality.

- **Check existing projects**: Before creating new DTOs/Protocols/Utils, check if a separate project already exists
- **Reuse separate projects**: If DTOs/Protocols/Utils von mehreren Projekten benötigt werden, sollte ein separates Projekt erstellt werden
- **Avoid duplication**: Don't duplicate DTO or protocol definitions across projects

### 5. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple data structures and protocols.

- **Simple DTOs**: Keep DTOs simple and focused
- **Clear protocols**: Maintain clear protocol definitions
- **Readable code**: Code should be self-documenting
- **Avoid over-engineering**: Don't add unnecessary complexity

### 6. CQRS (Command Query Responsibility Segregation)

**Software-Level Separation**: Separate read and write operations in protocols.

- **Command DTOs**: Separate DTOs for commands (write operations)
- **Query DTOs**: Separate DTOs for queries (read operations)
- **Response DTOs**: Separate DTOs for responses
- **Optimization**: Optimize serialization for commands and queries independently

### 7. Single Responsibility Principle

**One Responsibility Per Component**: Each component should have a single responsibility.

- **DTOs**: Data structures only
- **Protocols**: Protocol definitions only
- **Utilities**: Utility functions only
- **Serialization**: Serialization logic only

### 8. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected where applicable.

- **Serializers**: Inject serializers for different formats
- **Validators**: Inject validators
- **Crypto utilities**: Inject crypto utilities
- **Configuration**: Inject configuration

## Development Workflow

Follow this workflow for all coding tasks:

### 1. Analyze and Understand the Problem

- **Understand requirements**: Read and understand the requirements thoroughly
- **Identify constraints**: Identify technical, performance, and security constraints
- **Research existing solutions**: Check if similar DTOs or protocols exist
- **Cross-language compatibility**: Consider cross-language compatibility
- **Document assumptions**: Document any assumptions you make

### 2. Plan Your Steps, Create Todos

#### 2.1. List All Classes/Structures/Functions That Need to Be Implemented

- **Identify components**: List all classes, structs, functions, and modules needed
- **Define interfaces**: Define interfaces for DTOs, protocols, utilities
- **Plan dependencies**: Identify dependencies between components
- **Create structure**: Plan the file and directory structure for all languages
- **Document design**: Document the design decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for each component
- **Test edge cases**: Include tests for edge cases and error conditions
- **Test serialization**: Write tests for serialization/deserialization
- **Test validation**: Write tests for DTO validation
- **Test cross-language**: Write tests for cross-language compatibility
- **Mock dependencies**: Use mocks for external dependencies

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
- **Check compatibility**: Verify cross-language compatibility
- **Performance check**: Verify performance requirements (fast serialization, minimal footprint)
- **Security review**: Review security implications (validation, crypto)
- **If not complete**: If the goal is not achieved, go back to Step 1

### 5. Give a Note to the User

- **Document changes**: Document all changes made
- **Explain usage**: Explain how to use the new code
- **Provide examples**: Provide code examples if helpful
- **List dependencies**: List any new dependencies
- **Migration notes**: Include migration notes if breaking changes

## Service-Specific Guidelines

### Separate Projects für gemeinsame Komponenten

Wenn DTOs, Protocols oder Utils von mehreren Projekten benötigt werden:
- **Separate Projekte erstellen**: Für DTOs, Protocols, Utils, die von mehreren Projekten benötigt werden
- **Selektive Nutzung**: Platformen können selektiv Services und deren Dependencies einbinden
- **Versionierung**: Separate Projekte können ihre eigene Versionierung haben
- **Cross-language**: Separate Projekte können Code für mehrere Sprachen generieren (Rust, Elixir, TypeScript, Go)

### Beispiel-Projekte

- **Bifrost Protocol**: Separate Projekt für Bifrost Protocol Definition
- **Ratatoskr Protocol**: Separate Projekt für Ratatoskr Protocol Definition
- **DTOs**: Separate Projekte für DTOs, die von mehreren Services benötigt werden
- **Utils**: Separate Projekte für Utils, die von mehreren Services benötigt werden

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

- **Service-specific tests**: Test service-specific logic
- **Serialization**: Test serialization/deserialization where applicable
- **Utilities**: Test utility functions where applicable
- **Crypto**: Test cryptographic operations where applicable
- **Container-based**: All unit tests must run in container environment

### Integration Tests

- **Service integration**: Test service integration with other services
- **Protocol compatibility**: Test protocol compatibility where applicable
- **Serialization compatibility**: Test serialization compatibility where applicable
- **Container-based**: All integration tests must run in container environment

### Test Coverage

- **Minimum coverage**: Maintain minimum 80% test coverage
- **Critical paths**: 100% coverage for critical paths (DTO validation, serialization)
- **Cross-language**: High coverage for cross-language compatibility
- **Avoid redundancy**: Avoid redundant test logic - reuse test utilities and helpers

## Security Considerations

### Input Validation

- **DTO validation**: Comprehensive validation for all DTOs
- **Input sanitization**: Sanitize inputs to prevent injection attacks
- **Type checking**: Strong type checking

### Crypto Utilities

- **Secure encryption**: Secure encryption/decryption functions
- **Key management**: Secure key management
- **Digital signatures**: Digital signature support
- **Hash functions**: Secure hash functions

### GDPR Compliance (EU/German Data Protection)

- **Data minimization**: DTOs should only contain necessary data
- **Data encryption**: Provide encryption utilities for personal data
- **Privacy by design**: Consider privacy from the design phase
- **BDSG compliance**: Support compliance with Bundesdatenschutzgesetz (BDSG) requirements

## Performance Requirements

### Serialization Performance

- **Fast serialization**: Fast DTO validation (< 1ms for standard DTOs)
- **Efficient crypto**: Efficient crypto operations (async, non-blocking)
- **Minimal overhead**: Optimized protocol implementations (minimal overhead)
- **Lightweight**: Minimal footprint for fast load times

### Resource Management

- **Memory usage**: Efficient memory usage for DTOs - minimize RAM consumption
- **CPU usage**: Optimize CPU usage for serialization
- **Code efficiency**: Write efficient, short code that uses minimal resources
- **Code brevity**: Code should be as short as possible while maintaining readability
- **Failsafe**: Code must be failsafe - no errors that could crash the system
- **User experience**: Ensure pleasant user experience - do not slow down user devices

## Additional Resources

- See `edda/README.md` for metadata collection documentation
- **WICHTIG**: Dieser Ordner ist **KEIN PROJEKT**. Er dient nur als Metadaten-Sammlung.
- Note: For general development principles, see `AGENTS.md` in the parent directory (informational reference only)

