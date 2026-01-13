# AGENTS.md - Huginn & Muninn Development Guidelines

> **Hinweis**: Für Hintergrundinformationen zum Projekt, Features, Architektur und technische Details siehe `README.md`.

Diese Datei enthält ausschließlich Richtlinien für das Verhalten des Agents bei der Entwicklung.

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: Red (failing test) → Green (passing test) → Refactor
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests for audio processing, integration tests for STT/TTS services, end-to-end tests for complete voice workflows
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is important for audio processing.

- **Input validation**: Validate all audio inputs
- **Secure storage**: Secure storage for temporary audio data
- **TLS encryption**: TLS 1.3 for all cloud connections
- **Authentication**: Secure authentication for cloud services
- **No hardcoded secrets**: Never commit API keys or credentials

### 3. Performance

**Performance from the Start**: Huginn & Muninn must provide real-time audio processing.

- **Real-time processing**: Optimized for low latency STT/TTS
- **Streaming support**: Efficient streaming for real-time audio processing
- **Caching**: Intelligent caching for frequently used TTS phrases
- **Audio optimization**: Optimized audio processing for fast transcription
- **Connection pooling**: Efficient connection pooling for cloud services
- **Local processing**: Local processing for minimal latency

### 4. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality.

- **Check existing audio utilities**: Before implementing new audio functionality, check if a separate project already exists
- **Reuse audio processing**: Reuse common audio processing code from separate projects
- **Shared utilities**: Wenn gemeinsame Utils benötigt werden, sollte ein separates Projekt erstellt werden
- **Avoid duplication**: Don't duplicate audio processing logic

### 5. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple audio processing logic.

- **Simple audio pipeline**: Keep audio processing pipeline simple
- **Clear service integration**: Maintain clear integration with STT/TTS services
- **Readable code**: Code should be self-documenting
- **Avoid over-engineering**: Don't add unnecessary complexity

### 6. CQRS (Command Query Responsibility Segregation)

**Software-Level Separation**: Separate read and write operations.

- **Audio processing (commands)**: Separate handlers for audio processing
- **Status queries (queries)**: Separate handlers for processing status queries
- **Service queries**: Separate handlers for service availability queries
- **Optimization**: Optimize processing and queries independently

### 7. Single Responsibility Principle

**One Responsibility Per Component**: Each component should have a single responsibility.

- **Huginn (STT)**: Speech-to-text conversion only
- **Muninn (TTS)**: Text-to-speech conversion only
- **Audio processing**: Audio format conversion only
- **Service integration**: Service integration only

### 8. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected.

- **STT service**: Inject STT service (local or cloud)
- **TTS service**: Inject TTS service (local or cloud)
- **Audio device**: Inject audio device interface
- **Configuration**: Inject configuration

## Development Workflow

Follow this workflow for all coding tasks:

### 1. Analyze and Understand the Problem

- **Understand requirements**: Read and understand the requirements thoroughly
- **Identify constraints**: Identify technical, performance, and security constraints
- **Research existing solutions**: Check if similar audio processing exists
- **Service APIs**: Understand STT/TTS service API requirements
- **Document assumptions**: Document any assumptions you make

### 2. Plan Your Steps, Create Todos

#### 2.1. List All Classes/Structures/Functions That Need to Be Implemented

- **Identify components**: List all classes, structs, functions, and modules needed
- **Define interfaces**: Define interfaces for STT/TTS services, audio processing
- **Plan dependencies**: Identify dependencies between components
- **Create structure**: Plan the file and directory structure
- **Document design**: Document the design decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for each component
- **Test edge cases**: Include tests for edge cases and error conditions
- **Test audio processing**: Write tests for audio format conversion
- **Test STT/TTS**: Write tests for STT/TTS integration (with mocks)
- **Test streaming**: Write tests for streaming functionality
- **Mock services**: Use mocks for external STT/TTS services
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
- **Performance check**: Verify performance requirements (low latency, real-time processing)
- **Security review**: Review security implications (audio storage, API keys)
- **If not complete**: If the goal is not achieved, go back to Step 1

### 5. Give a Note to the User

- **Document changes**: Document all changes made
- **Explain usage**: Explain how to use the new code
- **Provide examples**: Provide code examples if helpful
- **List dependencies**: List any new dependencies
- **Migration notes**: Include migration notes if breaking changes

## Service-Specific Guidelines

### Huginn (STT) - Speech-to-Text

- **Audio input**: Receive audio input from device microphone
- **Audio preprocessing**: Noise reduction, normalization
- **STT service**: Call STT service (local or cloud)
- **Text transcription**: Transcribe audio to text
- **Message creation**: Create RavenMessage with transcribed text
- **Multi-language**: Support multiple languages
- **Wake word**: Optional wake word detection
- **Real-time**: Real-time audio processing

### Muninn (TTS) - Text-to-Speech

- **Text input**: Receive RavenMessage with text from Odin
- **Text preprocessing**: SSML parsing, text normalization
- **TTS service**: Call TTS service (local or cloud)
- **Audio generation**: Generate audio from text
- **Audio output**: Send audio to device output
- **Voice selection**: Support different voices
- **SSML support**: Support SSML for advanced features
- **Caching**: Cache frequently used TTS phrases

### Audio Processing

- **Format conversion**: Support various audio formats (WAV, MP3, Opus)
- **Sample rate conversion**: Convert between sample rates
- **Audio quality optimization**: Optimize audio quality
- **Streaming support**: Support streaming for real-time processing
- **Audio buffering**: Buffer audio for streaming

### Service Integration

- **Local services**: Whisper.cpp, Vosk (STT), Coqui TTS, Piper (TTS)
- **Cloud services**: Google Speech-to-Text, Azure Speech, Amazon Polly (optional, with API keys)
- **Unified API**: Unified API for local and cloud services
- **Fallback**: Fallback to alternative services on failure
- **Error handling**: Comprehensive error handling

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

- **Audio processing**: Test audio format conversion
- **STT integration**: Test STT service integration (mocked)
- **TTS integration**: Test TTS service integration (mocked)
- **Streaming**: Test streaming functionality
- **Container-based**: All unit tests must run in container environment

### Integration Tests

- **STT workflow**: Test complete STT workflow
- **TTS workflow**: Test complete TTS workflow
- **Service fallback**: Test service fallback scenarios
- **Error handling**: Test error handling and recovery
- **Container-based**: All integration tests must run in container environment

### Test Coverage

- **Minimum coverage**: Maintain minimum 80% test coverage
- **Critical paths**: 100% coverage for critical paths (audio processing, STT/TTS conversion)
- **Service integration**: High coverage for service integration code
- **Avoid redundancy**: Avoid redundant test logic - reuse test utilities and helpers

## Security Considerations

### Audio Security

- **Secure storage**: Encrypted storage for temporary audio data
- **Audio sanitization**: Sanitize audio inputs to prevent audio injection
- **Access control**: Control access to audio data

### API Key Management

- **Secure storage**: Secure storage for API keys (Keychain/Keystore)
- **No hardcoded keys**: Never hardcode API keys
- **Key rotation**: Support key rotation

### Secure Communication

- **TLS encryption**: TLS 1.3 for all cloud connections
- **Certificate validation**: Validate TLS certificates
- **Authentication**: Secure authentication for cloud services

### GDPR Compliance (EU/German Data Protection)

- **Data minimization**: Only process necessary audio data
- **Audio privacy**: Audio data should not be stored except for temporary processing
- **Data encryption**: Encrypt all personal data in audio processing
- **Access control**: Strict access control to prevent unauthorized data access
- **Right to erasure**: Support user right to delete audio data
- **Privacy by design**: Consider privacy from the design phase
- **BDSG compliance**: Comply with Bundesdatenschutzgesetz (BDSG) requirements

## Performance Requirements

### Processing Performance

- **Low latency STT**: Low latency for STT (< 500ms for short phrases)
- **Fast TTS generation**: Fast TTS generation (< 1s for standard phrases)
- **Efficient audio processing**: Efficient audio processing (minimal CPU overhead)
- **Real-time streaming**: Real-time streaming support

### Resource Management

- **Memory usage**: Efficient memory usage for audio buffering - minimize RAM consumption
- **CPU usage**: Optimize CPU usage for audio processing
- **Network usage**: Minimize network usage for cloud services
- **Code efficiency**: Write efficient, short code that uses minimal resources
- **Code brevity**: Code should be as short as possible while maintaining readability
- **Failsafe**: Code must be failsafe - no errors that could crash the system
- **User experience**: Ensure pleasant user experience - do not slow down user devices

## Additional Resources

- See `huginn-muninn/README.md` for detailed service documentation
- Das `edda` Verzeichnis ist **KEIN PROJEKT**. Es dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte. Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden.
- Note: For general development principles, see `AGENTS.md` in the parent directory (informational reference only)

