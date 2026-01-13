# AGENTS.md - Midgard Development Guidelines

> **Hinweis**: Für Hintergrundinformationen zum Projekt, Features, Architektur und technische Details siehe `README.md`.

Diese Datei enthält ausschließlich Richtlinien für das Verhalten des Agents bei der Entwicklung.

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: Red (failing test) → Green (passing test) → Refactor
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests for desktop components, integration tests for service integration, end-to-end tests for desktop workflows
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is important for desktop applications.

- **Secure storage**: Encrypted storage for credentials and tokens
- **TLS encryption**: All network connections encrypted (TLS 1.3)
- **Authentication**: Secure authentication via Heimdall
- **Permission system**: Granular permission system for actions
- **Sandboxing**: Sandboxing for unsafe actions
- **Input validation**: Comprehensive input validation
- **No hardcoded secrets**: Never commit secrets or API keys

### 3. Performance

**Performance from the Start**: Midgard must utilize full desktop hardware.

- **Full hardware utilization**: Utilize full desktop hardware capabilities
- **Multi-threading support**: Multi-threading support
- **GPU acceleration**: GPU acceleration support for LLMs
- **Local processing**: Prefer local processing for better performance
- **Caching**: Intelligent caching for frequently used data

### 4. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality.

- **Check existing services**: Before implementing new functionality, check if a service already provides it
- **Reuse separate projects**: Wenn gemeinsame Komponenten (DTOs, Protocols, Utils) benötigt werden, sollten separate Projekte erstellt werden
- **Service discovery**: Know what other services provide and use them
- **Avoid duplication**: Don't duplicate service integration logic

### 5. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple desktop implementations.

- **Simple UI**: Keep desktop UI simple and intuitive
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

- **Odin integration**: Main process orchestration only
- **Service integration**: Service integration only
- **UI components**: UI components only
- **System integration**: System integration only

### 8. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected.

- **Service clients**: Inject service clients (Freki, Geri, Thor, etc.)
- **Audio services**: Inject audio services (Huginn/Muninn)
- **System APIs**: Inject platform-specific system APIs
- **Configuration**: Inject configuration

## Development Workflow

Follow this workflow for all coding tasks:

### 1. Analyze and Understand the Problem

- **Understand requirements**: Read and understand the requirements thoroughly
- **Identify constraints**: Identify technical, performance, and security constraints
- **Research existing solutions**: Check if similar functionality exists in Midgard or other platforms
- **Platform considerations**: Consider Windows, macOS, and Linux platform differences
- **Document assumptions**: Document any assumptions you make

### 2. Plan Your Steps, Create Todos

#### 2.1. List All Classes/Structures/Functions That Need to Be Implemented

- **Identify components**: List all classes, structs, functions, and modules needed
- **Define interfaces**: Define interfaces for desktop components and service integration
- **Plan dependencies**: Identify dependencies between components and external services
- **Create structure**: Plan the file and directory structure for frontend/backend
- **Document design**: Document the design decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for each component
- **Test edge cases**: Include tests for edge cases and error conditions
- **Test desktop components**: Write tests for desktop UI components
- **Test service integration**: Write integration tests for service interactions
- **Test system integration**: Write tests for system integration (file system, applications)
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
- **Performance check**: Verify performance requirements (full hardware utilization, GPU acceleration)
- **Security review**: Review security implications (secure storage, TLS, permissions, sandboxing)
- **Platform testing**: Test on Windows, macOS, and Linux
- **If not complete**: If the goal is not achieved, go back to Step 1

### 5. Give a Note to the User

- **Document changes**: Document all changes made
- **Explain usage**: Explain how to use the new code
- **Provide examples**: Provide code examples if helpful
- **List dependencies**: List any new dependencies
- **Migration notes**: Include migration notes if breaking changes

## Service-Specific Guidelines

### Desktop UI/UX (Optional)

- **Optional frontend**: User can optionally use frontend to track system activity
- **Text input**: Support text input for manual command entry
- **Voice input**: Support voice input via Huginn (STT)
- **Flexible switching**: User can switch between text and voice
- **System tray integration**: System tray integration
- **Notification support**: Notification support
- **Settings UI**: Settings user interface
- **Status dashboard**: Status dashboard
- **Activity monitoring**: Overview of running actions and services

### System Integration

- **File system access**: Full file system access
- **Application control**: Control desktop applications
- **System settings**: Access system settings
- **Clipboard integration**: Clipboard integration
- **Full hardware utilization**: Utilize full desktop hardware

### Service Integration

- **Odin**: Main process orchestration with full event handling
- **Huginn/Muninn**: Microphone input, speaker output, audio device management
- **Freki**: Local vector database, document indexing, context retrieval
- **Geri**: Model selection based on configuration (device or connected server), unified model access
- **Thor**: File operations, application control, system commands, network operations
- **Bifrost**: Device-to-device communication
- **Heimdall**: Security and authentication

### Local LLM Support

- **Ollama**: Local models via Ollama
- **LM Studio**: Local models via LM Studio
- **Custom local models**: Direct integration with custom local models
- **GPU acceleration**: GPU acceleration for local LLMs
- **Model management**: Model management and selection

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

- **Desktop components**: Test desktop UI components
- **Service integration**: Test service integration logic
- **System integration**: Test system integration (file system, applications)
- **Platform features**: Test platform-specific features
- **Container-based**: All unit tests must run in container environment

### Integration Tests

- **Service workflows**: Test complete service workflows
- **Platform integration**: Test platform integration (Windows, macOS, Linux)
- **Error handling**: Test error handling and recovery
- **Performance**: Test performance with full hardware utilization
- **Container-based**: All integration tests must run in container environment

### Test Coverage

- **Minimum coverage**: Maintain minimum 80% test coverage
- **Critical paths**: 100% coverage for critical paths (service integration, system integration)
- **Platform code**: High coverage for platform-specific code
- **Avoid redundancy**: Avoid redundant test logic - reuse test utilities and helpers

## Security Considerations

### Desktop Security

- **Secure storage**: Encrypted storage for credentials and tokens
- **TLS encryption**: TLS 1.3 for all network connections
- **Authentication**: Secure authentication via Heimdall
- **Permission system**: Granular permission system for actions
- **Sandboxing**: Sandboxing for unsafe actions
- **Input validation**: Comprehensive input validation

### System Security

- **File system security**: Secure file system access
- **Application security**: Secure application control
- **System security**: Secure system settings access

### GDPR Compliance (EU/German Data Protection)

- **Data minimization**: Only collect and process necessary data
- **Data encryption**: Encrypt all personal data at rest and in transit
- **Access control**: Strict access control to prevent unauthorized data access
- **Right to erasure**: Support user right to delete their data
- **Privacy by design**: Consider privacy from the design phase
- **BDSG compliance**: Comply with Bundesdatenschutzgesetz (BDSG) requirements

## Performance Requirements

### Desktop Performance

- **Full hardware utilization**: Utilize full desktop hardware
- **GPU acceleration**: GPU acceleration for LLMs
- **Multi-threading**: Multi-threading support
- **Fast response times**: Fast response times for user interactions
- **Efficient local processing**: Efficient local processing

### Resource Management

- **Memory usage**: Efficient memory usage for desktop operations - minimize RAM consumption
- **CPU usage**: Optimize CPU usage for parallel processing
- **GPU usage**: Efficient GPU usage for LLMs
- **Code efficiency**: Write efficient, short code that uses minimal resources
- **Code brevity**: Code should be as short as possible while maintaining readability
- **Failsafe**: Code must be failsafe - no errors that could crash the system
- **User experience**: Ensure pleasant user experience - do not slow down user devices
- **Minimalist design**: Beautiful but simple to minimalist design - avoid unnecessary UI complexity

## Additional Resources

- See `midgard/README.md` for detailed service documentation
- Das `edda` Verzeichnis ist **KEIN PROJEKT**. Es dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte. Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden.
- Note: For general development principles, see `AGENTS.md` in the parent directory (informational reference only)

