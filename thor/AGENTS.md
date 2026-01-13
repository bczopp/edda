# AGENTS.md - Thor Development Guidelines

> **Hinweis**: Für Hintergrundinformationen zum Projekt, Features, Architektur und technische Details siehe `README.md`.

Diese Datei enthält ausschließlich Richtlinien für das Verhalten des Agents bei der Entwicklung.

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: Red (failing test) → Green (passing test) → Refactor
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests for action execution, integration tests for resource management, end-to-end tests for complete action workflows
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is critical for action execution.

- **Security principles**: Follow security principles as outlined in README.md
- **No hardcoded secrets**: Never commit secrets or credentials

### 3. Performance

**Performance from the Start**: Thor must execute actions efficiently.

- **Performance principles**: Follow performance principles as outlined in README.md
- **Minimal memory footprint**: Minimize memory usage and RAM consumption
- **Code brevity**: Code should be as short as possible while maintaining readability
- **Failsafe**: Code must be failsafe - no errors that could crash the system
- **User experience**: Ensure pleasant user experience - do not slow down user devices

### 4. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality.

- **Check existing actions**: Before implementing new action types, check if similar functionality exists
- **Reuse action handlers**: Reuse common action handling logic
- **Shared utilities**: Wenn gemeinsame Utils benötigt werden, sollte ein separates Projekt erstellt werden
- **Avoid duplication**: Don't duplicate action execution logic

### 5. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple action execution logic.

- **Simple action handlers**: Keep action handlers simple and focused
- **Clear action types**: Maintain clear separation between action types
- **Readable code**: Code should be self-documenting
- **Avoid over-engineering**: Don't add unnecessary complexity

### 6. CQRS (Command Query Responsibility Segregation)

**Software-Level Separation**: Separate read and write operations.

- **Action execution (commands)**: Separate handlers for action execution
- **Status queries (queries)**: Separate handlers for action status queries
- **Resource queries**: Separate handlers for resource status queries
- **Optimization**: Optimize execution and queries independently

### 7. Single Responsibility Principle

**One Responsibility Per Component**: Each component should have a single responsibility.

- **Mjölnir**: Action execution only
- **Tanngrisnir & Tanngnjóstr**: Resource management only
- **Chariot**: Task scheduling only
- **Tool-calling**: Action recognition and delegation only

### 8. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected.

- **Resource managers**: Inject resource managers
- **Action handlers**: Inject action handlers
- **Queue system**: Inject queue system for Valkyries/Frigg
- **Heimdall client**: Inject Heimdall client for permission checking

## Development Workflow

Follow this workflow for all coding tasks:

### 1. Analyze and Understand the Problem

- **Understand requirements**: Read and understand the requirements thoroughly
- **Identify constraints**: Identify technical, performance, and security constraints
- **Research existing solutions**: Check if similar action types or functionality exists
- **Action types**: Identify which action types are involved
- **Document assumptions**: Document any assumptions you make

### 2. Plan Your Steps, Create Todos

#### 2.1. List All Classes/Structures/Functions That Need to Be Implemented

- **Identify components**: List all classes, structs, functions, and modules needed
- **Define interfaces**: Define interfaces for action handlers, resource managers, schedulers
- **Plan dependencies**: Identify dependencies between components
- **Create structure**: Plan the file and directory structure
- **Document design**: Document the design decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for each component
- **Test edge cases**: Include tests for edge cases and error conditions
- **Test action execution**: Write tests for action execution scenarios
- **Test resource management**: Write tests for resource allocation and monitoring
- **Test conflict resolution**: Write tests for conflict resolution logic
- **Mock dependencies**: Use mocks for external dependencies (Heimdall, Valkyries, Frigg)
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
- **Check integration**: Verify integration with Odin, Heimdall, Valkyries, Frigg
- **Performance check**: Verify performance requirements (parallel execution, resource efficiency)
- **Security review**: Review security implications (sandboxing, permissions)
- **If not complete**: If the goal is not achieved, go back to Step 1

### 5. Give a Note to the User

- **Document changes**: Document all changes made
- **Explain usage**: Explain how to use the new code
- **Provide examples**: Provide code examples if helpful
- **List dependencies**: List any new dependencies
- **Migration notes**: Include migration notes if breaking changes

## Service-Specific Guidelines

### Action Execution

- **Action types**: Support all action types (DEVICE_CONTROL, FILE_OPERATION, NETWORK_OPERATION, APPLICATION_CONTROL, SYSTEM_COMMAND, CODING_TASK, HEALTHCARE_TASK)
- **Action lifecycle**: Manage complete action lifecycle (queued, executing, completed, failed)
- **Error handling**: Comprehensive error handling with retry mechanisms
- **Timeout handling**: Adaptive timeouts with minimum/maximum

### Resource Management

- **Resource monitoring**: Monitor system resources (CPU, RAM, disk, network)
- **Resource allocation**: Allocate resources for actions
- **Resource limits**: Prevent resource exhaustion
- **Resource pooling**: Efficient resource pooling

### Task Scheduling

- **Priority-based**: Schedule actions based on priority
- **Parallel execution**: Handle parallel execution of independent actions
- **Conflict resolution**: Resolve conflicts using priority + locking
- **Queue management**: Manage action queue efficiently

### Conflict Resolution

- **Priority + Locking**: Combination of priority and locking for conflict resolution
- **Hybrid locking**: Local locking for local resources, distributed locking for shared resources
- **Pessimistic locking**: Lock before access to prevent race conditions
- **Deadlock detection**: Detect and resolve deadlocks (timeout + detection)
- **System priority**: System determines priority, user can override

### Tool-Calling Agent

- **Coding task recognition**: Automatically recognize coding tasks
- **Healthcare task recognition**: Automatically recognize healthcare tasks
- **Delegation**: Delegate tasks to Valkyries (coding) or Frigg (healthcare)
- **Result analysis**: Analyze structured results from agents
- **Action recognition**: Recognize actions from agent results (file changes, commands, etc.)
- **Action execution**: Execute recognized actions
- **Response generation**: Generate text responses for Odin

### Error Recovery and Resilience

- **Retry mechanisms**: Retry with exponential backoff for network errors
- **Fallback mechanisms**: Fallback to alternative routes/providers
- **Rollback**: Rollback when possible for partial failures
- **Compensation**: Compensation when rollback not possible
- **Replay mechanism**: Replay failed actions for data recovery
- **State synchronization**: State sync as fallback for data recovery

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

- **Action handlers**: Test individual action handlers
- **Resource managers**: Test resource allocation and monitoring
- **Schedulers**: Test task scheduling logic
- **Conflict resolution**: Test conflict resolution algorithms
- **Avoid redundancy**: Avoid redundant test logic - reuse test utilities and helpers
- **Container-based**: All unit tests must run in container environment

### Integration Tests

- **Action execution**: Test complete action execution workflows
- **Resource management**: Test resource allocation and monitoring
- **Conflict resolution**: Test conflict resolution scenarios
- **Agent delegation**: Test delegation to Valkyries/Frigg
- **Container-based**: All integration tests must run in container environment

### Test Coverage

- **Minimum coverage**: Maintain minimum 80% test coverage
- **Critical paths**: 100% coverage for critical paths (action execution, conflict resolution)
- **Error handling**: High coverage for error handling code

## Security Considerations

### Sandboxing

- **Unsafe actions**: Sandbox unsafe actions
- **Permission checking**: Check permissions for all actions via Heimdall
- **Input validation**: Validate all action inputs
- **Output sanitization**: Sanitize action outputs
- **Unauthorized access prevention**: Strict access control to ensure unauthorized users cannot execute actions

### Secure Execution

- **Secure defaults**: Secure default configurations
- **No hardcoded secrets**: Never hardcode secrets or credentials
- **Audit logging**: Log all action executions for security audits

### GDPR Compliance (EU/German Data Protection)

- **Data minimization**: Only process necessary data in actions
- **Data encryption**: Encrypt sensitive data processed by actions
- **Access control**: Strict access control to prevent unauthorized data access
- **Audit logging**: Log all data access for compliance auditing
- **Privacy by design**: Consider privacy from the design phase

## Performance Requirements

### Execution Performance

- **Parallel execution**: Efficient parallel execution of independent actions
- **Resource efficiency**: Efficient resource usage
- **Low latency**: Low latency for action execution
- **High throughput**: High throughput for multiple concurrent actions

### Resource Management

- **Resource monitoring**: Efficient resource monitoring
- **Resource allocation**: Fast resource allocation
- **Resource cleanup**: Efficient resource cleanup after action completion
- **Memory efficiency**: Minimize RAM consumption - only use as much as necessary
- **Code efficiency**: Write efficient, short code that uses minimal resources
- **User experience**: Ensure pleasant user experience - do not slow down user devices

## Additional Resources

- See `thor/README.md` for detailed service documentation
- Das `edda` Verzeichnis ist **KEIN PROJEKT**. Es dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte. Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden.
- Note: For general development principles, see `AGENTS.md` in the parent directory (informational reference only)

