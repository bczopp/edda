# AGENTS.md - Freki Development Guidelines

> **Hinweis**: Für Hintergrundinformationen zum Projekt, Features, Architektur und technische Details siehe `README.md`.

Diese Datei enthält ausschließlich Richtlinien für das Verhalten des Agents bei der Entwicklung.

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: Red (failing test) → Green (passing test) → Refactor
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests for vector operations, integration tests for document indexing, end-to-end tests for retrieval workflows
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is important for document processing.

- **Security principles**: Follow security principles as outlined in README.md
- **No hardcoded secrets**: Never commit secrets or API keys

### 3. Performance

**Performance from the Start**: Freki must provide fast retrieval.

- **Performance principles**: Follow performance principles as outlined in README.md

### 4. Concurrency & Parallelism

**Critical for RAG Service**: Freki implements concurrency/parallelism as required by root AGENTS.md principle 4.

- **Parallel indexing**: `BatchIndexingManager` indexes multiple documents in parallel (tokio::spawn per chunk)
- **Concurrent queries**: Handles multiple retrieval requests concurrently (async runtime)
- **Load tests**: ≥20 concurrent queries, ≤5s total (see `tests/load_test.rs`)
- **Performance optimization**: Parallel indexing is faster than sequential (verified in tests)

### 5. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality.

- **Check existing utilities**: Before implementing new functionality, check if a separate project already exists
- **Reuse embedding models**: Reuse embedding model instances
- **Shared vector operations**: Reuse common vector operations
- **Avoid duplication**: Don't duplicate document processing logic

### 6. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple retrieval logic.

- **Simple chunking**: Keep chunking strategies simple and effective
- **Clear search logic**: Maintain clear semantic search logic
- **Readable code**: Code should be self-documenting
- **Avoid over-engineering**: Don't add unnecessary complexity

### 7. CQRS (Command Query Responsibility Segregation)

**Software-Level Separation**: Separate read and write operations.

- **Indexing (commands)**: Separate handlers for document indexing
- **Retrieval (queries)**: Separate handlers for document retrieval
- **Different models**: Indexing and retrieval can use different data models
- **Optimization**: Optimize indexing and retrieval independently

### 7. Single Responsibility Principle

**One Responsibility Per Component**: Each component should have a single responsibility.

- **Vector database**: Vector database management only
- **Embedding generation**: Embedding generation only
- **Document indexing**: Document indexing only
- **Context retrieval**: Context retrieval only

### 9. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected.

- **Vector database**: Inject vector database client
- **Embedding model**: Inject embedding model
- **Document storage**: Inject document storage
- **Configuration**: Inject configuration

## Development Workflow

Follow this workflow for all coding tasks:

### 1. Analyze and Understand the Problem

- **Understand requirements**: Read and understand the requirements thoroughly
- **Identify constraints**: Identify technical, performance, and security constraints
- **Research existing solutions**: Check if similar functionality exists
- **Vector database**: Identify which vector database to use
- **Document assumptions**: Document any assumptions you make

### 2. Plan Your Steps, Create Todos

#### 2.1. List All Classes/Structures/Functions That Need to Be Implemented

- **Identify components**: List all classes, structs, functions, and modules needed
- **Define interfaces**: Define interfaces for vector database, embedding model, document storage
- **Plan dependencies**: Identify dependencies between components
- **Create structure**: Plan the file and directory structure
- **Document design**: Document the design decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for each component
- **Test edge cases**: Include tests for edge cases and error conditions
- **Test vector operations**: Write tests for vector search and similarity calculations
- **Test document indexing**: Write tests for document indexing and chunking
- **Test retrieval**: Write tests for context retrieval
- **Mock dependencies**: Use mocks for external dependencies (vector database, embedding model)
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
- **Check integration**: Verify integration with Odin and Geri
- **Performance check**: Verify performance requirements (fast vector search, efficient indexing)
- **Security review**: Review security implications (document access, encryption)
- **If not complete**: If the goal is not achieved, go back to Step 1

### 5. Give a Note to the User

- **Document changes**: Document all changes made
- **Explain usage**: Explain how to use the new code
- **Provide examples**: Provide code examples if helpful
- **List dependencies**: List any new dependencies
- **Migration notes**: Include migration notes if breaking changes

## Service-Specific Guidelines

### Document Indexing

- **Watch-folder**: Automatically index new/changed files in watch folders
- **Manual addition**: Support manual document addition (optional)
- **Semantic chunking**: Chunk documents based on semantics with max size
- **Incremental updates**: Update only changed parts when possible
- **Full re-indexing**: Full re-indexing when incremental not possible
- **Batch indexing**: Batch indexing for large document sets

### Embedding Generation

- **Unified model**: Use unified embedding model as standard for all document types
- **Type-specific models**: Support type-specific models as option
- **Model caching**: Cache embeddings for frequently used documents
- **Batch processing**: Batch embedding generation for efficiency

### Vector Search

- **Similarity search**: Implement efficient similarity search (cosine, dot product, euclidean)
- **Hybrid search**: Support hybrid search (vector + keyword)
- **Filtering**: Support filtering by metadata
- **Re-ranking**: Support re-ranking with cross-encoders
- **Top-K with threshold**: Return top K documents above threshold

### Multi-Document Retrieval

- **Top-K retrieval**: Retrieve top K most relevant documents
- **Threshold filtering**: Filter documents below relevance threshold
- **Document ranking**: Rank documents by relevance score
- **Context extraction**: Extract relevant text passages from documents

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

- **Vector operations**: Test vector similarity calculations
- **Chunking**: Test document chunking strategies
- **Embedding generation**: Test embedding generation
- **Search algorithms**: Test search algorithms
- **Container-based**: All unit tests must run in container environment

### Integration Tests

- **Document indexing**: Test complete document indexing workflows
- **Context retrieval**: Test context retrieval workflows
- **Vector database**: Test integration with vector database
- **Embedding model**: Test integration with embedding model
- **Container-based**: All integration tests must run in container environment

### Test Coverage

- **Minimum coverage**: Maintain minimum 80% test coverage
- **Critical paths**: 100% coverage for critical paths (vector search, document indexing)
- **Search algorithms**: High coverage for search algorithm code
- **Avoid redundancy**: Avoid redundant test logic - reuse test utilities and helpers

## Security Considerations

### Document Security

- **Secure storage**: Optional encryption for sensitive documents
- **Access control**: Control access to indexed documents
- **Input validation**: Validate all document inputs
- **Malware scanning**: Optional scanning for malware

### Data Privacy

- **Local processing**: Prefer local document processing
- **Minimal data collection**: Only index necessary data
- **User control**: User has full control over indexed documents

### GDPR Compliance (EU/German Data Protection)

- **Data minimization**: Only index necessary document data
- **Data encryption**: Encrypt all personal data in indexed documents
- **Access control**: Strict access control to prevent unauthorized data access
- **Right to erasure**: Support user right to delete indexed documents
- **Privacy by design**: Consider privacy from the design phase
- **BDSG compliance**: Comply with Bundesdatenschutzgesetz (BDSG) requirements

## Performance Requirements

### Search Performance

- **Fast vector search**: Fast vector search (< 100ms for standard queries)
- **Efficient indexing**: Efficient indexing (< 1s per document)
- **High throughput**: High throughput for parallel queries

### Resource Management

- **Memory usage**: Efficient memory usage for embeddings and indices - minimize RAM consumption
- **CPU usage**: Optimize CPU usage for embedding generation
- **Storage usage**: Efficient storage usage for vector database
- **Code efficiency**: Write efficient, short code that uses minimal resources
- **Code brevity**: Code should be as short as possible while maintaining readability
- **Failsafe**: Code must be failsafe - no errors that could crash the system
- **User experience**: Ensure pleasant user experience - do not slow down user devices

## Additional Resources

- See `freki/README.md` for detailed service documentation
- Das `edda` Verzeichnis ist **KEIN PROJEKT**. Es dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte. Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden.
- Note: For general development principles, see `AGENTS.md` in the parent directory (informational reference only)

