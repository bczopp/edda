# AGENTS.md - Skuld Development Guidelines

> **Hinweis**: Für Hintergrundinformationen zum Projekt, Features, Architektur und technische Details siehe `README.md`.

Diese Datei enthält ausschließlich Richtlinien für das Verhalten des Agents bei der Entwicklung.

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: Red (failing test) → Green (passing test) → Refactor
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests for scoring algorithms, integration tests for network plan analysis, end-to-end tests for selection workflows
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is important for selection service.

- **Input validation**: Validate all network plans and selection requests
- **Secure defaults**: Secure default configurations
- **No hardcoded secrets**: Never commit secrets or credentials
- **Data privacy**: Protect user preferences and network information

### 3. Performance

**Performance from the Start**: Skuld must provide fast selection decisions.

- **Fast selection**: Fast selection (< 50ms for standard selection)
- **Efficient algorithms**: Optimized scoring algorithms
- **Async processing**: Async processing of selection requests
- **Parallel analysis**: Parallel analysis of multiple options
- **Caching**: Cache network plan analyses

### 4. Concurrency & Parallelism

**Critical for Model Selection**: Skuld implements concurrency/parallelism as required by root AGENTS.md principle 4.

- **Parallel model evaluation**: `ModelSelector` evaluates multiple model candidates in parallel (tokio::spawn per model, then join)
- **Concurrent selection requests**: Async tokio runtime handles multiple selection requests concurrently
- **Fast scoring**: Parallel scoring reduces latency when many models are available
- **Testing**: Performance tests verify parallel evaluation is faster than sequential

### 5. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality.

- **Check existing algorithms**: Before implementing new scoring logic, check if similar exists
- **Reuse scoring components**: Reuse common scoring components
- **Shared utilities**: Wenn gemeinsame Utils benötigt werden, sollte ein separates Projekt erstellt werden
- **Avoid duplication**: Don't duplicate scoring logic

### 6. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple scoring and selection logic.

- **Simple scoring**: Keep scoring algorithms simple and effective
- **Clear selection logic**: Maintain clear selection decision logic
- **Readable code**: Code should be self-documenting
- **Avoid over-engineering**: Don't add unnecessary complexity

### 7. CQRS (Command Query Responsibility Segregation)

**Software-Level Separation**: Separate read and write operations.

- **Selection (commands)**: Separate handlers for selection decisions
- **Network plan queries (queries)**: Separate handlers for network plan queries
- **Status queries**: Separate handlers for selection status queries
- **Optimization**: Optimize selection and queries independently

### 8. Single Responsibility Principle

**One Responsibility Per Component**: Each component should have a single responsibility.

- **Network plan analysis**: Network plan analysis only
- **Scoring**: Scoring calculation only
- **Selection**: Selection decision only
- **Cache management**: Cache management only

### 9. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected.

- **Scoring components**: Inject scoring components
- **Cache**: Inject cache for network plans
- **Configuration**: Inject configuration
- **Network stack**: Inject network stack for latency measurements (optional)

## Development Workflow

Follow this workflow for all coding tasks:

### 1. Analyze and Understand the Problem

- **Understand requirements**: Read and understand the requirements thoroughly
- **Identify constraints**: Identify technical, performance, and security constraints
- **Research existing solutions**: Check if similar selection algorithms exist
- **Scoring factors**: Understand all scoring factors (price, quality, latency, availability, fairness)
- **Document assumptions**: Document any assumptions you make

### 2. Plan Your Steps, Create Todos

#### 2.1. List All Classes/Structures/Functions That Need to Be Implemented

- **Identify components**: List all classes, structs, functions, and modules needed
- **Define interfaces**: Define interfaces for network plan analysis, scoring, selection
- **Plan dependencies**: Identify dependencies between components
- **Create structure**: Plan the file and directory structure
- **Document design**: Document the design decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for each component
- **Test edge cases**: Include tests for edge cases and error conditions
- **Test scoring**: Write tests for scoring algorithm with various scenarios
- **Test selection**: Write tests for selection decision logic
- **Test network plan analysis**: Write tests for network plan analysis
- **Test caching**: Write tests for cache management
- **Mock dependencies**: Use mocks for external dependencies (network stack)
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
- **Check integration**: Verify integration with Odin
- **Performance check**: Verify performance requirements (fast selection decisions)
- **Security review**: Review security implications (data privacy)
- **If not complete**: If the goal is not achieved, go back to Step 1

### 5. Give a Note to the User

- **Document changes**: Document all changes made
- **Explain usage**: Explain how to use the new code
- **Provide examples**: Provide code examples if helpful
- **List dependencies**: List any new dependencies
- **Migration notes**: Include migration notes if breaking changes

## Service-Specific Guidelines

### Network Plan Analysis

- **Plan parsing**: Parse network plans from Odin or Yggdrasil
- **Device evaluation**: Evaluate available devices and models
- **Provider integration**: Consider providers when marketplace active
- **Capability analysis**: Analyze device capabilities
- **Quality metrics**: Consider quality metrics in analysis

### Scoring Algorithm

- **Multi-factor evaluation**: Evaluate based on multiple factors
- **Factor weights**: Configurable weights for factors (price 30%, quality 25%, latency 20%, availability 15%, fairness 10%)
- **Quality weighting**: Quality metrics weighted in scoring
- **User preferences**: Consider user preferences and requirements
- **Provider scoring**: Consider provider quality, pricing, availability, fairness when marketplace active

### Selection Decision

- **Optimal selection**: Select optimal device/model based on score
- **Alternative options**: Provide alternative options in response
- **Justification**: Provide justification for selection
- **Cost/latency estimates**: Provide cost and latency estimates

### Cache Management

- **Network plan cache**: Cache network plan analyses
- **Cache invalidation**: Invalidate cache on network plan updates, device status changes, quality metric updates, timeout
- **Timeout-based**: Timeout-based cache invalidation (e.g., 5 minutes)
- **Efficient caching**: Efficient caching to avoid repeated analyses

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

- **Scoring algorithm**: Test scoring algorithm with various scenarios
- **Network plan analysis**: Test network plan parsing and analysis
- **Selection logic**: Test selection decision logic
- **Cache management**: Test cache operations
- **Container-based**: All unit tests must run in container environment

### Integration Tests

- **Selection workflow**: Test complete selection workflow
- **Network plan integration**: Test integration with network plan format
- **Provider integration**: Test provider integration when marketplace active
- **Container-based**: All integration tests must run in container environment

### Test Coverage

- **Minimum coverage**: Maintain minimum 80% test coverage
- **Critical paths**: 100% coverage for critical paths (scoring, selection)
- **Algorithm code**: High coverage for scoring algorithm code
- **Avoid redundancy**: Avoid redundant test logic - reuse test utilities and helpers

## Security Considerations

### Data Privacy

- **User preferences**: Protect user preferences and requirements
- **Network information**: Protect network plan information
- **Secure defaults**: Secure default configurations

### Input Validation

- **Network plan validation**: Validate network plans
- **Selection request validation**: Validate selection requests

### GDPR Compliance (EU/German Data Protection)

- **Data minimization**: Only process necessary data in selection requests
- **Data encryption**: Encrypt all personal data
- **Access control**: Strict access control to prevent unauthorized data access
- **Privacy by design**: Consider privacy from the design phase

## Performance Requirements

### Selection Performance

- **Fast selection**: Fast selection decisions (< 50ms for standard selection)
- **Efficient algorithms**: Optimized scoring algorithms
- **Low CPU usage**: Low CPU usage through caching
- **Parallel analysis**: Parallel analysis of multiple options

### Resource Management

- **Memory usage**: Efficient memory usage for network plan caching - minimize RAM consumption
- **CPU usage**: Optimize CPU usage for scoring calculations
- **Code efficiency**: Write efficient, short code that uses minimal resources
- **Code brevity**: Code should be as short as possible while maintaining readability
- **Failsafe**: Code must be failsafe - no errors that could crash the system
- **User experience**: Ensure pleasant user experience - do not slow down user devices

## Additional Resources

- See `skuld/README.md` for detailed service documentation
- Das `edda` Verzeichnis ist **KEIN PROJEKT**. Es dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte. Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden.
- Note: For general development principles, see `AGENTS.md` in the parent directory (informational reference only)

