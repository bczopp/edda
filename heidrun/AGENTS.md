# AGENTS.md - Heidrun Development Guidelines

> **Hinweis**: Für Hintergrundinformationen zum Projekt, Features, Architektur und technische Details siehe `README.md`.

Diese Datei enthält ausschließlich Richtlinien für das Verhalten des Agents bei der Entwicklung.

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: Red (failing test) → Green (passing test) → Refactor
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests for calculations, integration tests for gRPC service, end-to-end tests for complete workflows
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is fundamental for financial calculations.

- **Input validation**: All inputs must be validated and sanitized
- **Secure calculations**: Secure calculations without rounding errors
- **Audit logging**: Audit logging for all calculations
- **No hardcoded secrets**: Never commit secrets, passwords, or API keys
- **Access control**: Access control for pricing configurations

### 3. Performance

**Performance from the Start**: Heidrun must handle high request volumes efficiently.

- **Efficient calculations**: Optimized calculations for token counting and pricing
- **Caching**: Intelligent caching of pricing configurations
- **Batch processing**: Batch processing for settlement calculations
- **Minimal memory footprint**: Minimize memory usage and RAM consumption
- **Code brevity**: Code should be as short as possible while maintaining readability

### 4. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality.

- **Check existing services**: Before implementing new functionality, check if a service already provides it
- **Reuse separate projects**: Wenn gemeinsame Komponenten (DTOs, Protocols, Utils) benötigt werden, sollten separate Projekte erstellt werden
- **Service discovery**: Know what other services provide and use them
- **Avoid duplication**: Don't duplicate calculation logic

### 5. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple, clear solutions.

- **Simple calculations**: Keep calculations simple and clear
- **Clear formulas**: Maintain clear calculation formulas
- **Readable code**: Code should be self-documenting
- **Avoid over-engineering**: Don't add unnecessary complexity

### 6. CQRS (Command Query Responsibility Segregation)

**Software-Level Separation**: Separate read and write operations.

- **Token counting (commands)**: Separate handlers for token counting
- **Pricing queries (queries)**: Separate handlers for pricing queries
- **Settlement (commands)**: Separate handlers for settlement calculations
- **Optimization**: Optimize calculations and queries independently

### 7. Single Responsibility Principle

**One Responsibility Per Component**: Each component should have a single responsibility.

- **Token counting**: Token counting only
- **Pricing calculation**: Pricing calculation only
- **Settlement**: Settlement calculation only
- **Pre-authorization**: Pre-authorization handling only

### 8. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected.

- **gRPC server**: Inject gRPC server
- **Calculation engine**: Inject calculation engine
- **Configuration**: Inject configuration
- **Cache**: Inject cache (if used)

## Development Workflow

Follow this workflow for all coding tasks:

### 1. Analyze and Understand the Problem

- **Understand requirements**: Read and understand the requirements thoroughly
- **Identify constraints**: Identify performance and accuracy constraints
- **Research existing solutions**: Check if similar calculation features exist
- **Document assumptions**: Document any assumptions you make

### 2. Plan Your Steps, Create Todos

#### 2.1. List All Classes/Structures/Functions That Need to Be Implemented

- **Identify components**: List all classes, structs, functions, and modules needed
- **Define interfaces**: Define interfaces for token counting, pricing, settlement
- **Plan dependencies**: Identify dependencies between components
- **Create structure**: Plan the file and directory structure
- **Document design**: Document the design decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for each component
- **Test edge cases**: Include tests for edge cases and error conditions
- **Test calculations**: Write tests for calculation logic (token counting, pricing, settlement)
- **Test gRPC service**: Write tests for gRPC service implementation
- **Test accuracy**: Test calculation accuracy (no rounding errors)
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
- **Accuracy check**: Verify calculation accuracy (no rounding errors)
- **Security review**: Review security implications
- **If not complete**: If the goal is not achieved, go back to Step 1

### 5. Give a Note to the User

- **Document changes**: Document all changes made
- **Explain usage**: Explain how to use the new code
- **Provide examples**: Provide code examples if helpful
- **List dependencies**: List any new dependencies
- **Migration notes**: Include migration notes if breaking changes

## Service-Specific Guidelines

### Token Counting

- **Post-request counting**: Token counting after request processing
- **Accurate counting**: Accurate token counting without errors
- **Token tracking**: Track token consumption per request
- **Token aggregation**: Aggregate token statistics

### Pricing Calculation

- **Cent-based pricing**: Cent calculation per 1000 tokens (integer only, no decimals)
- **Calculation formula**: `(tokens / 1000) * pricePerToken` (rounded up)
- **Integer arithmetic**: Use integer arithmetic to avoid rounding errors
- **Price configuration**: Support for price configuration per provider

### Settlement

- **Provider earnings**: Calculate provider earnings (`providerEarnings = totalCost - companyFee`)
- **Company fee**: Calculate company fee (`companyFee = totalCost * commissionRate`)
- **Commission calculation**: Calculate company commission (10-15%)
- **Settlement tracking**: Track all settlements for audit purposes

### Pre-Authorization

- **Estimated costs**: Pre-authorization for estimated costs before request
- **Cost estimation**: Estimate costs based on request parameters
- **Authorization handling**: Manage pre-authorizations
- **Authorization expiration**: Handle authorization expiration

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

- **Calculation logic**: Test token counting, pricing, settlement calculation logic
- **Edge cases**: Test edge cases (zero tokens, very large numbers, etc.)
- **Accuracy**: Test calculation accuracy (no rounding errors)
- **Container-based**: All unit tests must run in container environment

### Integration Tests

- **gRPC service**: Test gRPC service integration
- **Yggdrasil integration**: Test integration with Yggdrasil
- **End-to-end workflows**: Test complete pricing and settlement workflows
- **Error handling**: Test error handling and recovery
- **Container-based**: All integration tests must run in container environment

### Test Coverage

- **Minimum coverage**: Maintain minimum 80% test coverage
- **Critical paths**: 100% coverage for critical paths (calculations, gRPC service)
- **Calculation code**: High coverage for all calculation code
- **Avoid redundancy**: Avoid redundant test logic - reuse test utilities and helpers

## Security Considerations

### Financial Security

- **Input validation**: Validate all inputs (tokens, prices, etc.)
- **Secure calculations**: Secure calculations without rounding errors
- **Audit logging**: Log all calculations for audit purposes
- **Data integrity**: Ensure data integrity in calculations
- **Access control**: Strict access control for pricing configurations

### API Security

- **Authentication**: Proper authentication for all endpoints
- **Authorization**: Role-based access control where appropriate
- **Input sanitization**: Sanitize all inputs
- **Output encoding**: Encode outputs to prevent injection attacks

### GDPR Compliance (EU/German Data Protection)

- **Data minimization**: Only process necessary data
- **No personal data**: Do not store personal data
- **Data encryption**: Encrypt sensitive data at rest and in transit
- **Access control**: Strict access control to prevent unauthorized data access

## Performance Requirements

### Calculation Performance

- **Fast calculations**: Fast token counting and pricing calculations
- **Efficient algorithms**: Use efficient algorithms for calculations
- **Caching**: Cache pricing configurations for better performance
- **Batch processing**: Batch processing for settlement calculations

### Resource Management

- **Memory management**: Efficient memory management - minimize RAM consumption
- **CPU management**: Optimize CPU usage
- **Code efficiency**: Write efficient, short code that uses minimal resources
- **Code brevity**: Code should be as short as possible while maintaining readability
- **Failsafe**: Code must be failsafe - no errors that could crash the system
- **User experience**: Ensure pleasant user experience - do not slow down user devices

## Additional Resources

- See `heidrun/README.md` for detailed service documentation
- Das `edda` Verzeichnis ist **KEIN PROJEKT**. Es dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte. Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden.
- Note: For general development principles, see `AGENTS.md` in the parent directory (informational reference only)

