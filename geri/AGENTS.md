# AGENTS.md - Geri Development Guidelines

> **Hinweis**: Für Hintergrundinformationen zum Projekt, Features, Architektur und technische Details siehe `README.md`.

Diese Datei enthält ausschließlich Richtlinien für das Verhalten des Agents bei der Entwicklung.

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: Red (failing test) → Green (passing test) → Refactor
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests for model management, integration tests for LLM calls, end-to-end tests for complete prompt processing workflows
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is critical for LLM service.

- **Security principles**: Follow security principles as outlined in README.md
- **No hardcoded secrets**: Never commit API keys or credentials

### 3. Performance

**Performance from the Start**: Geri must provide fast LLM responses.

- **Performance principles**: Follow performance principles as outlined in README.md

### 4. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality.

- **Check existing providers**: Before implementing new provider integration, check if similar exists
- **Reuse provider abstractions**: Reuse common provider abstraction code
- **Shared utilities**: Wenn gemeinsame Utils benötigt werden, sollte ein separates Projekt erstellt werden
- **Avoid duplication**: Don't duplicate provider integration logic

### 5. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple provider integration.

- **Simple provider abstraction**: Keep provider abstraction simple
- **Clear model selection**: Maintain clear model selection logic
- **Readable code**: Code should be self-documenting
- **Avoid over-engineering**: Don't add unnecessary complexity

### 6. CQRS (Command Query Responsibility Segregation)

**Software-Level Separation**: Separate read and write operations.

- **Model calls (commands)**: Separate handlers for LLM calls
- **Model queries (queries)**: Separate handlers for model information queries
- **Status queries**: Separate handlers for request status queries
- **Optimization**: Optimize calls and queries independently

### 7. Single Responsibility Principle

**One Responsibility Per Component**: Each component should have a single responsibility.

- **Model management**: Model registry and selection only
- **Prompt processing**: Prompt formatting only
- **Provider integration**: Provider-specific integration only
- **Cost tracking**: Cost calculation only

### 8. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected.

- **Provider clients**: Inject provider clients
- **Model registry**: Inject model registry
- **Configuration**: Inject configuration
- **Cost calculator**: Inject cost calculator

## Development Workflow

Follow this workflow for all coding tasks:

### 1. Analyze and Understand the Problem

- **Understand requirements**: Read and understand the requirements thoroughly
- **Identify constraints**: Identify technical, performance, and security constraints
- **Research existing solutions**: Check if similar provider integration exists
- **Provider APIs**: Understand provider API requirements
- **Document assumptions**: Document any assumptions you make

### 2. Plan Your Steps, Create Todos

#### 2.1. List All Classes/Structures/Functions That Need to Be Implemented

- **Identify components**: List all classes, structs, functions, and modules needed
- **Define interfaces**: Define interfaces for providers, model registry, prompt formatters
- **Plan dependencies**: Identify dependencies between components
- **Create structure**: Plan the file and directory structure
- **Document design**: Document the design decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for each component
- **Test edge cases**: Include tests for edge cases and error conditions
- **Test provider integration**: Write tests for provider integration (with mocks)
- **Test model selection**: Write tests for model selection logic
- **Test prompt formatting**: Write tests for prompt formatting
- **Mock providers**: Use mocks for external LLM providers
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
- **Check integration**: Verify integration with Odin and Freki
- **Performance check**: Verify performance requirements (fast responses, efficient streaming)
- **Security review**: Review security implications (API key storage, TLS)
- **If not complete**: If the goal is not achieved, go back to Step 1

### 5. Give a Note to the User

- **Document changes**: Document all changes made
- **Explain usage**: Explain how to use the new code
- **Provide examples**: Provide code examples if helpful
- **List dependencies**: List any new dependencies
- **Migration notes**: Include migration notes if breaking changes

## Service-Specific Guidelines

### Model Management

- **Model registry**: Maintain registry of available models
- **Model versioning**: Support model versioning
- **Model health monitoring**: Monitor model health and availability
- **Automatic fallback**: Automatic fallback to alternative models on failure
- **Local LLM guarantee**: Every device must have a local LLM available

### Model Selection

- **Configuration-based**: Model selection based on device or connected server configuration
- **User selection**: Support explicit user model selection
- **Automatic selection**: Automatic selection based on best choice for situation
- **Multi-factor evaluation**: Evaluate models based on size, hardware, reliability, ping, distance
- **Load balancing**: Distribute requests across providers (not all to one)
- **No provider preference**: Local and cloud models have equal priority

### Provider Integration

- **Unified API**: Unified API for all providers (local and cloud)
- **Local providers**: Ollama, LM Studio, custom local models
- **Cloud providers**: OpenAI, Anthropic, Google, etc. (only if API keys provided)
- **Plugin architecture**: Plugin architecture for different providers
- **Error handling**: Comprehensive error handling with retry and fallback

### Prompt Processing

- **Prompt formatting**: Format prompts for different model APIs
- **System prompts**: Add system prompts where needed
- **Context window management**: Manage context windows efficiently
- **RAG context integration**: Integrate RAG context from Freki

### Cost Management

- **Token counting**: Count tokens for requests and responses
- **Cost calculation**: Calculate costs for cloud providers
- **Budget limits**: Support budget limits
- **Cost tracking**: Track costs per request

### Cloud LLM Fallback

- **Automatic fallback**: Automatic fallback to local LLM when cloud limit reached
- **Best local LLM**: Identify best available local LLM (multi-factor evaluation)
- **Network search**: Search network for best LLM (e.g., smartphone uses desktop LLM)
- **User notification**: TTS notification with reason for fallback
- **Monthly reset**: Automatic return to cloud LLM after monthly reset

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

- **Model management**: Test model registry and selection
- **Prompt formatting**: Test prompt formatting for different providers
- **Cost calculation**: Test cost calculation logic
- **Provider abstraction**: Test provider abstraction layer
- **Container-based**: All unit tests must run in container environment

### Integration Tests

- **LLM calls**: Test LLM calls with mocked providers
- **Streaming**: Test streaming functionality
- **Error handling**: Test error handling and fallback
- **Cost tracking**: Test cost tracking
- **Container-based**: All integration tests must run in container environment

### Test Coverage

- **Minimum coverage**: Maintain minimum 80% test coverage
- **Critical paths**: 100% coverage for critical paths (model selection, prompt processing)
- **Provider integration**: High coverage for provider integration code
- **Avoid redundancy**: Avoid redundant test logic - reuse test utilities and helpers

## Security Considerations

### API Key Management

- **Secure storage**: Secure storage for API keys (Keychain/Keystore)
- **No hardcoded keys**: Never hardcode API keys
- **Key rotation**: Support key rotation
- **Access control**: Control access to API keys

### Secure Communication

- **TLS encryption**: TLS 1.3 for all cloud connections
- **Certificate validation**: Validate TLS certificates
- **Authentication**: Secure authentication for cloud services

### Input Validation

- **Prompt validation**: Validate all prompt inputs
- **Prompt sanitization**: Sanitize prompts to prevent injection attacks

### GDPR Compliance (EU/German Data Protection)

- **Data minimization**: Only process necessary data in prompts
- **Data encryption**: Encrypt all personal data in prompts
- **Access control**: Strict access control to prevent unauthorized data access
- **Privacy by design**: Consider privacy from the design phase
- **BDSG compliance**: Comply with Bundesdatenschutzgesetz (BDSG) requirements

## Performance Requirements

### Response Performance

- **Fast responses**: Fast LLM responses
- **Streaming**: Efficient streaming for better UX
- **Low latency**: Low latency for model calls
- **High throughput**: High throughput for parallel requests

### Resource Management

- **Memory usage**: Efficient memory usage for model management - minimize RAM consumption
- **Connection management**: Efficient connection pooling
- **Request queuing**: Efficient request queuing
- **Code efficiency**: Write efficient, short code that uses minimal resources
- **Code brevity**: Code should be as short as possible while maintaining readability
- **Failsafe**: Code must be failsafe - no errors that could crash the system
- **User experience**: Ensure pleasant user experience - do not slow down user devices

## Additional Resources

- See `geri/README.md` for detailed service documentation
- Das `edda` Verzeichnis ist **KEIN PROJEKT**. Es dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte. Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden.
- Note: For general development principles, see `AGENTS.md` in the parent directory (informational reference only)

