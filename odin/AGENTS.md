# AGENTS.md - Odin Development Guidelines

> **Hinweis**: Für Hintergrundinformationen zum Projekt, Features, Architektur und technische Details siehe `README.md`.

Diese Datei enthält ausschließlich Richtlinien für das Verhalten des Agents bei der Entwicklung.

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: Red (failing test) → Green (passing test) → Refactor
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests for individual components, integration tests for service interactions, end-to-end tests for complete workflows
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is fundamental for the orchestrator service.

- **Security principles**: Follow security principles as outlined in README.md
- **Input validation**: All user inputs (text and voice) must be validated and sanitized
- **No hardcoded secrets**: Never commit secrets, passwords, or API keys

### 3. Performance

**Performance from the Start**: Odin must handle multiple concurrent requests efficiently.

- **Performance principles**: Follow performance principles as outlined in README.md
- **Minimal memory footprint**: Minimize memory usage and RAM consumption
- **Code brevity**: Code should be as short as possible while maintaining readability
- **Failsafe**: Code must be failsafe - no errors that could crash the system
- **User experience**: Ensure pleasant user experience - do not slow down user devices

### 4. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality before implementing new code.

- **Check existing services**: Before implementing new functionality, check if a service already provides it
- **Reuse separate projects**: Wenn gemeinsame Komponenten (DTOs, Protocols, Utils) benötigt werden, sollten separate Projekte erstellt werden
- **Service discovery**: Know what other services provide and use them
- **Avoid duplication**: Don't duplicate service coordination logic

### 5. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple, clear solutions.

- **Simple orchestration**: Keep orchestration logic simple and clear
- **Clear service boundaries**: Maintain clear boundaries between services
- **Readable code**: Code should be self-documenting
- **Avoid over-engineering**: Don't add complexity "just in case"

### 6. CQRS (Command Query Responsibility Segregation)

**Software-Level Separation**: Separate read and write operations.

- **Command handlers**: Separate handlers for write operations (actions, state changes)
- **Query handlers**: Separate handlers for read operations (state queries, device info)
- **Different models**: Commands and queries can use different data models
- **Optimization**: Optimize reads and writes independently

### 7. Single Responsibility Principle

**One Responsibility Per Component**: Each component should have a single, well-defined responsibility.

- **Orchestration**: Odin orchestrates, doesn't implement business logic
- **Service coordination**: Clear separation between service coordination and business logic
- **State management**: Separate state management from orchestration
- **Event handling**: Separate event handling from business logic

### 8. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected, not created.

- **Service dependencies**: Inject service clients (Freki, Geri, Thor, etc.)
- **Interface-based**: Depend on interfaces, not concrete implementations
- **Testability**: Makes testing easier with mock dependencies
- **Configuration**: Dependencies can be configured externally

## Development Workflow

Follow this workflow for all coding tasks:

### 1. Analyze and Understand the Problem

- **Understand requirements**: Read and understand the requirements thoroughly
- **Identify constraints**: Identify technical, performance, and security constraints
- **Research existing solutions**: Check if similar functionality exists in Odin or other services
- **Service dependencies**: Identify which services are needed
- **Document assumptions**: Document any assumptions you make

### 2. Plan Your Steps, Create Todos

#### 2.1. List All Classes/Structures/Functions That Need to Be Implemented

- **Identify components**: List all classes, structs, functions, and modules needed
- **Define interfaces**: Define interfaces and contracts between components and services
- **Plan dependencies**: Identify dependencies between components and external services
- **Create structure**: Plan the file and directory structure
- **Document design**: Document the design decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for each component
- **Test edge cases**: Include tests for edge cases and error conditions
- **Test service integration**: Write integration tests for service interactions
- **Mock services**: Use mocks for external service dependencies (Freki, Geri, Thor, etc.)
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
- **Check integration**: Verify integration with other services (Freki, Geri, Thor, Bifrost, Heimdall, Skuld, Vedrfolnir)
- **Performance check**: Verify performance requirements are met (low latency for command processing)
- **Security review**: Review security implications
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

- **Component isolation**: Test components in isolation
- **Mock dependencies**: Mock all external service dependencies
- **Edge cases**: Test edge cases and error conditions
- **State management**: Test state management logic
- **Avoid redundancy**: Avoid redundant test logic - reuse test utilities and helpers
- **Container-based**: All unit tests must run in container environment

### Integration Tests

- **Service integration**: Test integration with other services
- **End-to-end workflows**: Test complete user command workflows
- **Error scenarios**: Test error handling and recovery
- **Performance**: Test performance under load
- **API tests**: API tests are critical - test all API endpoints for security and functionality
- **Container-based**: All integration tests must run in container environment

### Test Coverage

- **Minimum coverage**: Maintain minimum 80% test coverage
- **Critical paths**: 100% coverage for critical paths (command processing, state management)
- **Service interactions**: High coverage for service interaction code
- **API security**: 100% coverage for API security tests - ensure unauthorized access is prevented

## Additional Resources

- Siehe `README.md` für detaillierte Projektinformationen, Features, Architektur und technische Details
- Das `edda` Verzeichnis ist **KEIN PROJEKT**. Es dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte. Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden.
- Hinweis: Für allgemeine Entwicklungsprinzipien siehe `AGENTS.md` im Hauptverzeichnis (nur als Referenz)

