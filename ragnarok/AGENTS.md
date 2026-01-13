# AGENTS.md - Ragnarok Development Guidelines

> **Hinweis**: Für Hintergrundinformationen zum Projekt, Features, Architektur und technische Details siehe `README.md`.

Diese Datei enthält ausschließlich Richtlinien für das Verhalten des Agents bei der Entwicklung.

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: Red (failing test) → Green (passing test) → Refactor
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests for TUI components, integration tests for service integration, end-to-end tests for terminal workflows
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is important for terminal applications.

- **Sandboxing**: Sandbox code execution to protect against malicious code
- **Input validation**: Comprehensive validation of all inputs
- **Code review**: Automatic code review for security issues
- **Secure key storage**: Secure storage for API keys
- **Permission checking**: Check permissions for device operations
- **No hardcoded secrets**: Never commit secrets or API keys

### 3. Performance

**Performance from the Start**: Ragnarok must be lightweight and fast.

- **Lightweight design**: Optimized to not slow down the computer
- **Direct model binding**: Direct model binding via llama.cpp for minimal overhead
- **Resource management**: Intelligent resource management for optimal performance
- **Lazy loading**: Model loaded only when needed
- **Efficient CLI**: Fast CLI performance

### 4. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality.

- **Check existing services**: Before implementing new functionality, check if a service already provides it
- **Reuse separate projects**: Wenn gemeinsame Komponenten (DTOs, Protocols, Utils) benötigt werden, sollten separate Projekte erstellt werden
- **Service discovery**: Know what other services provide and use them
- **Avoid duplication**: Don't duplicate service integration logic

### 5. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple terminal implementations.

- **Simple TUI**: Keep TUI simple and efficient
- **Clear service integration**: Maintain clear service integration
- **Readable code**: Code should be self-documenting
- **Avoid over-engineering**: Don't add unnecessary complexity

### 6. CQRS (Command Query Responsibility Segregation)

**Software-Level Separation**: Separate read and write operations.

- **Command execution (commands)**: Separate handlers for command execution
- **Status queries (queries)**: Separate handlers for status queries
- **Optimization**: Optimize execution and queries independently

### 7. Single Responsibility Principle

**One Responsibility Per Component**: Each component should have a single responsibility.

- **TUI components**: TUI rendering only
- **Input handling**: Input handling only
- **Service integration**: Service integration only
- **Model management**: Model management only

### 8. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected.

- **Service clients**: Inject service clients (Odin, Thor, Geri, etc.)
- **TUI library**: Inject TUI library (ratatui, crossterm, etc.)
- **LLM service**: Inject LLM service (llama.cpp bindings)
- **Configuration**: Inject configuration

## Development Workflow

Follow this workflow for all coding tasks:

### 1. Analyze and Understand the Problem

- **Understand requirements**: Read and understand the requirements thoroughly
- **Identify constraints**: Identify technical, performance, and security constraints
- **Research existing solutions**: Check if similar TUI or terminal functionality exists
- **TUI library**: Understand TUI library requirements
- **Document assumptions**: Document any assumptions you make

### 2. Plan Your Steps, Create Todos

#### 2.1. List All Classes/Structures/Functions That Need to Be Implemented

- **Identify components**: List all classes, structs, functions, and modules needed
- **Define interfaces**: Define interfaces for TUI components, service integration, model management
- **Plan dependencies**: Identify dependencies between components
- **Create structure**: Plan the file and directory structure
- **Document design**: Document the design decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for each component
- **Test edge cases**: Include tests for edge cases and error conditions
- **Test TUI components**: Write tests for TUI components (with mocks)
- **Test service integration**: Write tests for service integration
- **Test model management**: Write tests for model management
- **Mock dependencies**: Use mocks for external dependencies (services, TUI library, llama.cpp)
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
- **Performance check**: Verify performance requirements (lightweight, fast CLI)
- **Security review**: Review security implications (sandboxing, code review)
- **If not complete**: If the goal is not achieved, go back to Step 1

### 5. Give a Note to the User

- **Document changes**: Document all changes made
- **Explain usage**: Explain how to use the new code
- **Provide examples**: Provide code examples if helpful
- **List dependencies**: List any new dependencies
- **Migration notes**: Include migration notes if breaking changes

## Service-Specific Guidelines

### TUI (Terminal User Interface)

- **Chat interface**: Interactive chat interface for commands and responses
- **Status display**: Live status of running tasks and services
- **History view**: Display command history
- **Config view**: Configuration interface
- **Input handling**: Text input and keyboard navigation
- **Renderer**: TUI rendering engine (ratatui, crossterm, etc.)
- **Multi-panel**: Multiple panels for chat, status, history simultaneously
- **Responsive**: Adapts to terminal size

### Service Integration

- **Same architecture**: Same architecture as Midgard/Alfheim/Asgard
- **Odin**: Main process service coordinates all services
- **Thor**: Action executor and tool-calling agent
- **Brünnhilde (Valkyries)**: Coding agent (via Thor)
- **Geri**: LLM service for prompt processing
- **Freki**: RAG service for context enrichment
- **Huginn & Muninn**: STT/TTS service (optional, for voice commands)
- **Bifrost**: Optional for home network connection
- **Heimdall**: Optional for security (if home network connected)

### Model Management

- **Bundled model**: Very good, free tool-calling model included (e.g., Llama 3.1 8B)
- **Direct binding**: Direct model binding via llama.cpp for minimal overhead
- **Alternative configuration**: User can route to other local model or use API keys/URL for cloud models
- **Resource optimization**: Bundled model not loaded if external model configured
- **LLM configuration**: Per default all Valkyries use same LLM, but each can be individually configured

### Optional Home Network Connection

- **Optional connection**: User can optionally connect to home network
- **Explicit command**: Must be explicitly called as `/` command
- **Bifrost**: For device-to-device communication (if connected)
- **Heimdall**: For security and authentication (if connected)

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

- **TUI components**: Test TUI components (with mocks)
- **Service integration**: Test service integration logic
- **Model management**: Test model management
- **Input handling**: Test input handling
- **Container-based**: All unit tests must run in container environment

### Integration Tests

- **Terminal workflows**: Test complete terminal workflows
- **Service integration**: Test integration with services
- **Error handling**: Test error handling and recovery
- **Performance**: Test performance (lightweight, fast)
- **Container-based**: All integration tests must run in container environment

### Test Coverage

- **Minimum coverage**: Maintain minimum 80% test coverage
- **Critical paths**: 100% coverage for critical paths (TUI, service integration)
- **Terminal code**: High coverage for terminal-specific code
- **Avoid redundancy**: Avoid redundant test logic - reuse test utilities and helpers

## Security Considerations

### Code Execution Security

- **Sandboxing**: Sandbox code execution to protect against malicious code
- **Code review**: Automatic code review for security issues
- **Permission checking**: Check permissions for device operations
- **Secure key storage**: Secure storage for API keys

### Input Validation

- **Command validation**: Validate all command inputs
- **Code validation**: Validate generated code
- **File operation validation**: Validate file operations

### GDPR Compliance (EU/German Data Protection)

- **Data minimization**: Only process necessary data in commands
- **Code privacy**: Ensure code remains private and is not shared with unauthorized parties
- **Access control**: Strict access control to prevent unauthorized data access
- **Privacy by design**: Consider privacy from the design phase

## Performance Requirements

### CLI Performance

- **Fast response times**: Fast response times (< 1s for simple commands)
- **Efficient model inference**: Efficient model inference (minimal memory overhead)
- **Optimized CLI**: Optimized CLI performance (fast command processing)
- **Lightweight**: Lightweight design (doesn't slow down computer)

### Resource Management

- **Memory usage**: Efficient memory usage for model management - minimize RAM consumption
- **CPU usage**: Optimize CPU usage for CLI operations
- **Model loading**: Lazy loading of model (only when needed)
- **Code efficiency**: Write efficient, short code that uses minimal resources
- **Code brevity**: Code should be as short as possible while maintaining readability
- **Failsafe**: Code must be failsafe - no errors that could crash the system
- **User experience**: Ensure pleasant user experience - do not slow down user devices
- **Minimalist design**: Beautiful but simple to minimalist TUI design - avoid unnecessary complexity

## Additional Resources

- See `ragnarok/README.md` for detailed service documentation
- Das `edda` Verzeichnis ist **KEIN PROJEKT**. Es dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte. Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden.
- Note: For general development principles, see `AGENTS.md` in the parent directory (informational reference only)

