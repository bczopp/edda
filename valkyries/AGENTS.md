# AGENTS.md - Valkyries Development Guidelines

> **Hinweis**: Für Hintergrundinformationen zum Projekt, Features, Architektur und technische Details siehe `README.md`.

Diese Datei enthält ausschließlich Richtlinien für das Verhalten des Agents bei der Entwicklung.

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: Red (failing test) → Green (passing test) → Refactor
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests for agent logic, integration tests for agent orchestration, end-to-end tests for complete coding workflows
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is critical for code execution.

- **Sandboxing**: Sandbox code execution to protect against malicious code
- **Input validation**: Comprehensive validation of all inputs
- **Code review**: Automatic code review for security issues
- **Secure git operations**: Secure git operations without credential exposure
- **Permission checking**: Check permissions for file operations
- **No hardcoded secrets**: Never commit secrets or API keys in generated code

### 3. Performance

**Performance from the Start**: Valkyries must provide fast code generation.

- **Parallel execution**: Parallel execution of sub-agents for faster completion
- **Context isolation**: Isolated contexts reduce memory usage and improve performance
- **Efficient git operations**: Optimized git operations for fast change tracking
- **Resource management**: Intelligent resource management for optimal performance
- **Caching**: Caching of frequently used code patterns and LLM responses
- **Streaming**: Streaming of LLM responses for better UX

### 4. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality.

- **Check existing agents**: Before implementing new agent functionality, check if similar exists
- **Reuse agent components**: Reuse common agent components
- **Shared utilities**: Wenn gemeinsame Utils benötigt werden, sollte ein separates Projekt erstellt werden
- **Avoid duplication**: Don't duplicate code generation logic

### 5. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple agent logic.

- **Simple agents**: Keep each agent simple and focused
- **Clear responsibilities**: Maintain clear responsibilities for each agent
- **Readable code**: Code should be self-documenting
- **Avoid over-engineering**: Don't add unnecessary complexity

### 6. CQRS (Command Query Responsibility Segregation)

**Software-Level Separation**: Separate read and write operations.

- **Code generation (commands)**: Separate handlers for code generation
- **Code analysis (queries)**: Separate handlers for code analysis
- **Different models**: Generation and analysis can use different data models
- **Optimization**: Optimize generation and analysis independently

### 7. Single Responsibility Principle

**One Responsibility Per Agent**: Each agent should have a single, well-defined responsibility.

- **Brünnhilde**: Orchestration, task decomposition, quality assurance only
- **Gunnr**: Testing only
- **Hildr**: Security only
- **Skögul**: Performance only
- **Hrist**: Refactoring only
- **Mist**: DevOps only
- **Skeggjöld**: Configuration only
- **Göll**: Frontend only
- **Geirskögul**: Database only
- **Þrúðr**: Backend only
- **Hlökk**: Documentation only
- **Róta**: API design only
- **Sigrún**: Optimization only

### 8. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected.

- **LLM service**: Inject LLM service (Geri)
- **Git service**: Inject git service
- **File service**: Inject file service
- **Execution service**: Inject execution service
- **Configuration**: Inject configuration

## Development Workflow

Follow this workflow for all coding tasks:

### 1. Analyze and Understand the Problem

- **Understand requirements**: Read and understand the requirements thoroughly
- **Identify constraints**: Identify technical, performance, and security constraints
- **Research existing solutions**: Check if similar agent functionality exists
- **Agent selection**: Identify which agents are needed
- **Document assumptions**: Document any assumptions you make

### 2. Plan Your Steps, Create Todos

#### 2.1. List All Classes/Structures/Functions That Need to Be Implemented

- **Identify components**: List all classes, structs, functions, and modules needed
- **Define interfaces**: Define interfaces for agents, orchestration, task decomposition
- **Plan dependencies**: Identify dependencies between components
- **Create structure**: Plan the file and directory structure
- **Document design**: Document the design decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for each component
- **Test edge cases**: Include tests for edge cases and error conditions
- **Test agent logic**: Write tests for individual agent logic
- **Test orchestration**: Write tests for Brünnhilde orchestration
- **Test task decomposition**: Write tests for task decomposition
- **Test quality assurance**: Write tests for quality assurance
- **Mock dependencies**: Use mocks for external dependencies (LLM, git, file system)
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
- **Check integration**: Verify integration with Thor
- **Performance check**: Verify performance requirements (fast code generation, parallel execution)
- **Security review**: Review security implications (sandboxing, code review)
- **If not complete**: If the goal is not achieved, go back to Step 1

### 5. Give a Note to the User

- **Document changes**: Document all changes made
- **Explain usage**: Explain how to use the new code
- **Provide examples**: Provide code examples if helpful
- **List dependencies**: List any new dependencies
- **Migration notes**: Include migration notes if breaking changes

## Service-Specific Guidelines

### Brünnhilde (Lead Agent)

- **Task analysis**: Analyze incoming tasks for complexity, dependencies, requirements
- **Task decomposition**: Decompose complex tasks into atomic sub-tasks
- **Sub-agent orchestration**: Select and coordinate sub-agents
- **Multi-instance planning**: Plan multiple instances of same agent when needed
- **Workflow management**: Create detailed workflows for task execution
- **Quality assurance**: Coordinate code reviews and verify completeness
- **Statement collection**: Collect and analyze statements from all agents
- **Iteration planning**: Plan additional iterations for missing parts

### Sub-Agents

- **Focused context**: Each agent works only with relevant code (small context windows)
- **Statement system**: Each agent provides statement after completion
- **Multi-instance support**: Support multiple instances of same agent
- **Context isolation**: Isolated contexts for each agent
- **Specialized tools**: Each agent uses specialized tools and technologies

### Task Decomposition

- **Dependency mapping**: Identify dependencies between sub-tasks
- **Priority determination**: Determine priorities based on dependencies
- **Resource estimation**: Estimate required resources for each sub-task
- **Parallelization strategy**: Determine which tasks can run in parallel

### Quality Assurance

- **Code review coordination**: Coordinate code reviews through specialized agents
- **Test verification**: Verify all tests are successful
- **Integration checking**: Check integration between components
- **Completeness verification**: Verify all requirements are met
- **Quality metrics**: Collect and analyze quality metrics

### Statement System

- **Statement collection**: Collect statements from all agents after completion
- **Statement analysis**: Analyze statements for completeness and consistency
- **Gap detection**: Detect gaps between expected and actual results
- **Iteration planning**: Plan additional iterations for missing parts

### LLM Configuration

- **Default configuration**: All agents use same LLM by default (configurable)
- **Individual configuration**: Each agent can have individual LLM configuration
- **Use-case specific**: Different agents can use different models (e.g., specialized coding models)
- **Configuration management**: Configuration managed via Geri

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

- **Agent logic**: Test individual agent logic
- **Orchestration**: Test Brünnhilde orchestration
- **Task decomposition**: Test task decomposition logic
- **Quality assurance**: Test quality assurance logic
- **Container-based**: All unit tests must run in container environment

### Integration Tests

- **Complete workflows**: Test complete coding workflows
- **Agent coordination**: Test agent coordination
- **Thor integration**: Test integration with Thor
- **Error handling**: Test error handling and recovery
- **Container-based**: All integration tests must run in container environment

### Test Coverage

- **Minimum coverage**: Maintain minimum 80% test coverage
- **Critical paths**: 100% coverage for critical paths (orchestration, task decomposition)
- **Agent code**: High coverage for all agent code
- **Avoid redundancy**: Avoid redundant test logic - reuse test utilities and helpers

## Security Considerations

### Code Execution Security

- **Sandboxing**: Sandbox code execution to protect against malicious code
- **Code review**: Automatic code review for security issues
- **Permission checking**: Check permissions for file operations
- **Secure git operations**: Secure git operations without credential exposure

### Input Validation

- **Task validation**: Validate all task inputs
- **Code validation**: Validate generated code
- **File operation validation**: Validate file operations

### GDPR Compliance (EU/German Data Protection)

- **Data minimization**: Only process necessary data in coding tasks
- **Code privacy**: Ensure code remains private and is not shared with unauthorized parties
- **Access control**: Strict access control to prevent unauthorized data access
- **Privacy by design**: Consider privacy from the design phase

## Performance Requirements

### Code Generation Performance

- **Fast decomposition**: Fast task decomposition (< 5s for complex tasks)
- **Efficient parallel execution**: Efficient parallel execution of multiple agents
- **Optimized context management**: Optimized context management (minimal memory overhead)
- **Fast code generation**: Fast code generation

### Resource Management

- **Memory usage**: Efficient memory usage for context management - minimize RAM consumption
- **CPU usage**: Optimize CPU usage for parallel agent execution
- **Resource allocation**: Intelligent resource allocation for agents
- **Code efficiency**: Write efficient, short code that uses minimal resources
- **Code brevity**: Code should be as short as possible while maintaining readability
- **Failsafe**: Code must be failsafe - no errors that could crash the system
- **User experience**: Ensure pleasant user experience - do not slow down user devices

## Additional Resources

- See `valkyries/README.md` for detailed service documentation
- Das `edda` Verzeichnis ist **KEIN PROJEKT**. Es dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte. Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden.
- Note: For general development principles, see `AGENTS.md` in the parent directory (informational reference only)

