# AGENTS.md - Alfheim Development Guidelines

> **Hinweis**: Für Hintergrundinformationen zum Projekt, Features, Architektur und technische Details siehe `README.md`.

Diese Datei enthält ausschließlich Richtlinien für das Verhalten des Agents bei der Entwicklung.

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: Red (failing test) → Green (passing test) → Refactor
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests for mobile components, integration tests for service integration, end-to-end tests for mobile workflows
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is important for mobile applications.

- **Security principles**: Follow security principles as outlined in README.md
- **Input validation**: Comprehensive input validation
- **No hardcoded secrets**: Never commit secrets or API keys

### 3. Performance

**Performance from the Start**: Alfheim must be optimized for mobile hardware.

- **Performance principles**: Follow performance principles as outlined in README.md
- **Resource efficiency**: Optimize for battery life, memory, CPU, and network usage

### 4. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality.

- **Check existing services**: Before implementing new functionality, check if a service already provides it
- **Reuse separate projects**: Wenn gemeinsame Komponenten (DTOs, Protocols, Utils) benötigt werden, sollten separate Projekte erstellt werden
- **Service discovery**: Know what other services provide and use them
- **Avoid duplication**: Don't duplicate service integration logic

### 5. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple mobile implementations.

- **Simple UI**: Keep mobile UI simple and intuitive
- **Clear service integration**: Maintain clear service integration
- **Readable code**: Code should be self-documenting
- **Avoid over-engineering**: Don't add unnecessary complexity

### 6. CQRS (Command Query Responsibility Segregation)

**Software-Level Separation**: Separate read and write operations.

- **Action execution (commands)**: Separate handlers for write operations
- **Status queries (queries)**: Separate handlers for read operations
- **Optimization**: Optimize reads and writes independently

### 7. Single Responsibility Principle

**One Responsibility Per Component**: Each component should have a single responsibility.

- **Odin integration**: Lightweight main process only
- **Service integration**: Service integration only
- **UI components**: UI components only
- **Mobile optimizations**: Mobile-specific optimizations only

### 8. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected.

- **Service clients**: Inject service clients (Freki, Geri, Thor, etc.)
- **Audio services**: Inject audio services (Huginn/Muninn)
- **Configuration**: Inject configuration
- **Platform APIs**: Inject platform-specific APIs

## Development Workflow

Follow this workflow for all coding tasks:

### 1. Analyze and Understand the Problem

- **Understand requirements**: Read and understand the requirements thoroughly
- **Identify constraints**: Identify mobile constraints (battery, memory, network, CPU)
- **Research existing solutions**: Check if similar functionality exists in Alfheim or other platforms
- **Platform considerations**: Consider iOS and Android platform differences
- **Document assumptions**: Document any assumptions you make

### 2. Plan Your Steps, Create Todos

#### 2.1. List All Classes/Structures/Functions That Need to Be Implemented

- **Identify components**: List all classes, structs, functions, and modules needed
- **Define interfaces**: Define interfaces for mobile components and service integration
- **Plan dependencies**: Identify dependencies between components and external services
- **Create structure**: Plan the file and directory structure for iOS/Android/shared
- **Document design**: Document the design decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for each component
- **Test edge cases**: Include tests for edge cases and error conditions
- **Test mobile components**: Write tests for mobile UI components
- **Test service integration**: Write integration tests for service interactions
- **Test platform features**: Write tests for platform-specific features
- **Mock services**: Use mocks for external service dependencies
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
- **Performance check**: Verify performance requirements (battery optimization, memory efficiency)
- **Security review**: Review security implications (secure storage, TLS, permissions)
- **Platform testing**: Test on both iOS and Android
- **If not complete**: If the goal is not achieved, go back to Step 1

### 5. Give a Note to the User

- **Document changes**: Document all changes made
- **Explain usage**: Explain how to use the new code
- **Provide examples**: Provide code examples if helpful
- **List dependencies**: List any new dependencies
- **Migration notes**: Include migration notes if breaking changes

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

- **Mobile components**: Test mobile UI components
- **Service integration**: Test service integration logic
- **Platform features**: Test platform-specific features
- **Mobile optimizations**: Test mobile optimization logic
- **Container-based**: All unit tests must run in container environment

### Integration Tests

- **Service workflows**: Test complete service workflows
- **Platform integration**: Test platform integration (iOS/Android)
- **Error handling**: Test error handling and recovery
- **Performance**: Test performance under mobile constraints
- **Container-based**: All integration tests must run in container environment

### Test Coverage

- **Minimum coverage**: Maintain minimum 80% test coverage
- **Critical paths**: 100% coverage for critical paths (service integration, user input)
- **Platform code**: High coverage for platform-specific code
- **Avoid redundancy**: Avoid redundant test logic - reuse test utilities and helpers

## Additional Resources

- Siehe `README.md` für detaillierte Projektinformationen, Features, Architektur und technische Details
- Das `edda` Verzeichnis ist **KEIN PROJEKT**. Es dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte. Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden.
- Hinweis: Für allgemeine Entwicklungsprinzipien siehe `AGENTS.md` im Hauptverzeichnis (nur als Referenz)

