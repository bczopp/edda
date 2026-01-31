# AGENTS.md - Gladsheim Development Guidelines

> **Hinweis**: Für Hintergrundinformationen zum Projekt, Features, Architektur und technische Details siehe `README.md`.

Diese Datei enthält ausschließlich Richtlinien für das Verhalten des Agents bei der Entwicklung.

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: Red (failing test) → Green (passing test) → Refactor
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests for each servant (Thjalfi, Byggvir, Roskva, Skirnir), integration tests for service lifecycle, end-to-end tests for complete workflows
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is critical for service management - unauthorized service access could compromise entire system.

- **Heimdall-Integration**: Only authorized services may be started (via Heimdall)
- **No hardcoded secrets**: Never commit secrets, passwords, or API keys
- **Process-Isolation**: Services must run in separate processes
- **Resource-Limits**: Enforce strict RAM/CPU limits to prevent resource exhaustion
- **Secure Communication**: All communication via gRPC (type-safe)
- **Audit-Logging**: Log all service starts/stops for security auditing
- **Token-Based-Auth**: Services authenticate via Heimdall-Tokens
- **No Arbitrary Code**: Only registered services can be started (no arbitrary executables)

### 3. Performance

**Performance from the Start**: Gladsheim must manage services efficiently without adding overhead.

- **Low-Latency**: Service-Start < 500ms, Status-Query < 10ms, gRPC-Calls < 5ms
- **Minimal-Overhead**: Gladsheim itself < 50MB RAM, Health-Checks < 1% CPU
- **Efficient-Monitoring**: Resource-Monitoring must be lightweight
- **Async-Operations**: Use tokio for async process management
- **Minimal memory footprint**: Minimize memory usage and RAM consumption
- **Code brevity**: Code should be as short as possible while maintaining readability
- **Failsafe**: Code must be failsafe - service failures must not crash Gladsheim
- **User experience**: Gladsheim should not slow down user devices

### 4. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality.

- **Check existing process-management**: Use existing Rust crates for process management (tokio::process)
- **Reuse monitoring-logic**: Don't duplicate resource-monitoring logic
- **Shared utilities**: Wenn gemeinsame Utils benötigt werden, sollte ein separates Projekt erstellt werden
- **Avoid duplication**: Don't duplicate service-management logic across servants

### 5. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple service-management logic.

- **Simple process-management**: Use standard process management patterns
- **Clear separation**: Maintain clear separation between servants (Thjalfi, Byggvir, Roskva, Skirnir)
- **Readable code**: Code should be self-documenting
- **Avoid over-engineering**: Don't add unnecessary complexity
- **Progressive enhancement**: Start simple, add features as needed

### 6. CQRS (Command Query Responsibility Segregation)

**Software-Level Separation**: Separate read and write operations.

- **Service-Lifecycle (commands)**: Separate handlers for Start/Stop/Restart (Thjalfi)
- **Service-Status (queries)**: Separate handlers for status queries (Skirnir)
- **Health-Monitoring (queries)**: Separate handlers for health queries (Roskva)
- **Resource-Monitoring (queries)**: Separate handlers for resource queries (Byggvir)
- **Optimization**: Optimize commands and queries independently

### 7. Single Responsibility Principle

**One Responsibility Per Component**: Each servant has a single, well-defined responsibility.

- **Thjalfi (Service Loader)**: Service-Start/Stop/Process-Management ONLY
- **Byggvir (Resource Manager)**: Resource-Monitoring/Limits/Enforcement ONLY
- **Roskva (Health Monitor)**: Health-Checks/Crash-Detection/Auto-Restart ONLY
- **Skirnir (Service Registry)**: Registry/Discovery/Status-Tracking ONLY
- **Clear boundaries**: No overlap between servants
- **Focused modules**: Each module does one thing well

### 8. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected.

- **Process-Manager**: Inject process manager (tokio::process)
- **Resource-Monitor**: Inject resource monitor (sysinfo)
- **Health-Checker**: Inject health checker (HTTP/gRPC client)
- **Configuration**: Inject configuration
- **Heimdall-Client**: Inject Heimdall client for authorization

## Development Workflow

Follow this workflow for all coding tasks:

### 1. Analyze and Understand the Problem

- **Understand requirements**: Read and understand the requirements thoroughly
- **Identify constraints**: Identify technical, performance, and security constraints
- **Research existing solutions**: Check if similar service-management functionality exists
- **Identify servant**: Which servant (Thjalfi, Byggvir, Roskva, Skirnir) is responsible?
- **Document assumptions**: Document any assumptions you make

### 2. Plan Your Steps, Create Todos

#### 2.1. List All Classes/Structures/Functions That Need to Be Implemented

- **Identify components**: List all structs, functions, and modules needed
- **Define interfaces**: Define interfaces for servants, gRPC service, configuration
- **Plan dependencies**: Identify dependencies between components
- **Create structure**: Plan the file and directory structure
- **Document design**: Document the design decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for each component
- **Test edge cases**: Include tests for edge cases and error conditions
- **Test servants**: Write tests for Thjalfi, Byggvir, Roskva, Skirnir
- **Test integration**: Write integration tests for service lifecycle workflows
- **Test error-handling**: Test failure scenarios (service crash, timeout, resource exhaustion)
- **Mock dependencies**: Use mocks for external dependencies (processes, system metrics, network)
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
- **Or focus on servant**: Or just run the tests of the servant you currently work on
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
- **Check integration**: Verify integration with Odin and Services
- **Performance check**: Verify performance requirements (< 500ms start, < 10ms query, < 50MB RAM)
- **Security review**: Review security implications (Heimdall-Integration, Process-Isolation)
- **If not complete**: If the goal is not achieved, go back to Step 1

### 5. Give a Note to the User

- **Document changes**: Document all changes made
- **Explain usage**: Explain how to use the new code
- **Provide examples**: Provide code examples if helpful
- **List dependencies**: List any new dependencies
- **Migration notes**: Include migration notes if breaking changes

## Service-Specific Guidelines

### Thjalfi (Service Loader)

**Focus**: Fast and reliable service loading

- **Process-Management**: Use `tokio::process::Command` for async process management
- **Graceful-Shutdown**: Always attempt graceful shutdown (SIGTERM) before force-kill (SIGKILL)
- **Timeout-Handling**: Respect startup/shutdown timeouts
- **Error-Handling**: Handle process spawn errors gracefully
- **Logging**: Log all service starts/stops with timestamps and process IDs

### Byggvir (Resource Manager)

**Focus**: Efficient resource monitoring and enforcement

- **System-Monitoring**: Use `sysinfo` crate for system metrics
- **Real-Time-Monitoring**: Monitor resources in real-time (not just on-demand)
- **Platform-Aware**: Respect platform-specific limits (Mobile: strict, Server: relaxed)
- **Battery-Aware** (Alfheim): Adjust limits based on battery status
- **Enforcement**: Enforce limits but don't be too aggressive (allow short spikes)

### Roskva (Health Monitor)

**Focus**: Reliable health monitoring with minimal overhead

- **Periodic-Checks**: Use configurable check intervals (default: 5s)
- **Multiple-Strategies**: Support HTTP health endpoints, gRPC health protocol, process monitoring
- **Auto-Restart**: Auto-restart failed services (with max-attempts limit)
- **Backoff**: Use exponential backoff for restart attempts
- **Alerting**: Alert Odin on critical failures

### Skirnir (Service Registry)

**Focus**: Fast and accurate service registry

- **In-Memory-Registry**: Use in-memory data structures (HashMap) for O(1) lookups
- **Thread-Safe**: Use Arc<RwLock<>> for thread-safe access
- **Status-Tracking**: Track all state transitions (starting, running, stopping, stopped, crashed)
- **Metadata**: Store comprehensive metadata (process ID, start time, resource usage, health status)
- **API**: Provide gRPC API for Odin

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

- **Thjalfi Tests**: Test service loading, process management, graceful shutdown
- **Byggvir Tests**: Test resource monitoring, limit enforcement, platform-specific limits
- **Roskva Tests**: Test health checking, crash detection, auto-restart
- **Skirnir Tests**: Test registry operations, status tracking, service discovery
- **Mock dependencies**: Mock processes, system metrics, network calls
- **Container-based**: All unit tests must run in container environment

### Integration Tests

- **Service-Lifecycle**: Test complete service lifecycle (start → monitor → stop)
- **Resource-Management**: Test resource limits and enforcement
- **Health-Monitoring**: Test health checks and auto-restart
- **Multiple-Services**: Test managing multiple services concurrently
- **Failure-Scenarios**: Test service crashes, timeouts, resource exhaustion
- **Container-based**: All integration tests must run in container environment

### Security Tests

- **Authorization**: Test that only authorized services can be started
- **Process-Isolation**: Test that services are properly isolated
- **Resource-Limits**: Test that resource limits are enforced
- **Heimdall-Integration**: Test Heimdall authorization flow
- **Container-based**: All security tests must run in container environment

### Performance Tests

- **Start-Latency**: Test service start latency (< 500ms target)
- **Query-Latency**: Test status query latency (< 10ms target)
- **Health-Check-Overhead**: Test health check CPU overhead (< 1% target)
- **Memory-Overhead**: Test Gladsheim memory usage (< 50MB target)
- **Concurrent-Services**: Test managing 10+ services concurrently
- **Load-Testing**: Test under realistic load conditions

### Test Coverage

- **Minimum coverage**: Maintain minimum 80% test coverage
- **Critical paths**: 100% coverage for critical paths (service start/stop, resource enforcement)
- **Error-handling**: High coverage for all error-handling code
- **Edge-cases**: Coverage for all edge cases

## Security Considerations

### Heimdall-Integration

- **Authorization**: All service starts must be authorized via Heimdall
- **Token-Validation**: Validate Heimdall tokens before starting services
- **Service-Whitelist**: Only whitelisted services can be started
- **Audit-Logging**: Log all authorization attempts and failures

### Process-Isolation

- **Separate-Processes**: All services run in separate processes (never threads)
- **No-Shared-Memory**: No shared memory between services
- **IPC-via-gRPC**: All communication via gRPC (type-safe, secure)
- **Process-Boundaries**: Strict process boundaries

### Resource-Limits

- **Enforcement**: Strict enforcement of RAM/CPU limits
- **Cascading-Failure-Prevention**: Prevent one service from exhausting resources for others
- **Priority-Based**: Important services (Odin, Heimdall) have higher priority
- **Graceful-Degradation**: Stop less-important services first when resources are low

### Secure-Communication

- **gRPC-Only**: All communication via gRPC (no raw sockets)
- **Local-Only**: Gladsheim gRPC server binds to localhost only (not exposed externally)
- **Type-Safe**: Protobuf ensures type-safe communication
- **Authentication**: gRPC calls require authentication (Heimdall tokens)

## Performance Requirements

### Latency-Requirements

- **Service-Start**: < 500ms for Gladsheim to start a service (excluding service startup time)
- **Service-Stop**: < 100ms for Gladsheim to stop a service (excluding service shutdown time)
- **Status-Query**: < 10ms for status queries (GetServiceStatus, ListServices)
- **Health-Check**: < 50ms for health checks
- **gRPC-Calls**: < 5ms for local gRPC calls

### Resource-Requirements

- **Memory-Overhead**: < 50MB for Gladsheim itself
- **CPU-Overhead**: < 1% CPU for health monitoring
- **Disk-I/O**: Minimal disk I/O (only for logging)
- **Network-I/O**: Only local gRPC (no external network)

### Scalability-Requirements

- **Max-Services**: Support 25+ services on Asgard, 5+ on Alfheim
- **Concurrent-Operations**: Support 100+ concurrent gRPC requests
- **Parallel-Starts**: Support 10+ parallel service starts
- **Health-Monitoring**: Support 100+ services health monitoring

## Platform-Specific Considerations

### Midgard (Desktop)

- **Resource-Limits**: Moderate limits (512MB default, 25% CPU)
- **Service-Count**: Support 15+ services
- **Health-Checks**: Standard check intervals (5s)

### Alfheim (Mobile)

- **Battery-Aware**: Reduce monitoring when battery < 30%, stop services when < 15%
- **Resource-Limits**: Strict limits (256MB default, 25% CPU)
- **Service-Count**: Support 5+ services
- **Background-Processing**: Consider iOS/Android background limits
- **Network-Aware**: Pause services on poor network

### Asgard (Homeserver)

- **Resource-Limits**: Relaxed limits (2048MB default, 75% CPU)
- **Service-Count**: Support 25+ services
- **24/7-Operation**: Optimized for continuous operation
- **High-Availability**: Prioritize reliability over resource efficiency

### Ragnarok (Terminal)

- **Resource-Limits**: Minimal limits (512MB default, 30% CPU)
- **Service-Count**: Support 8+ services
- **CLI-Optimized**: Text-based status output
- **No-GUI**: No GUI overhead

### Jotunheim (IoT)

- **NO Gladsheim**: Jotunheim does not use Gladsheim (too resource-limited)
- **Manual-Management**: Services are started/stopped manually on IoT devices

## Error-Handling

### Graceful-Failure

- **Service-Crashes**: Handle service crashes gracefully (log, alert, optionally restart)
- **Resource-Exhaustion**: Handle resource exhaustion gracefully (stop less-important services)
- **Timeout-Handling**: Handle timeouts gracefully (log, retry, or fail)
- **Network-Failures**: Handle health-check network failures gracefully

### Recovery-Strategies

- **Auto-Restart**: Automatic restart for crashed services (with max-attempts)
- **Exponential-Backoff**: Use exponential backoff for restart attempts
- **Cascading-Failure-Prevention**: Prevent one failure from causing cascading failures
- **Fallback**: Fallback to alternative services when possible

### Logging

- **Structured-Logging**: Use structured logging (tracing crate)
- **Correlation-IDs**: Use correlation IDs for request tracking
- **Log-Levels**: Use appropriate log levels (DEBUG, INFO, WARN, ERROR)
- **Error-Context**: Include full error context in logs

## Additional Resources

- See `gladsheim/README.md` for detailed project documentation
- Das `edda` Verzeichnis ist **KEIN PROJEKT**. Es dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte. Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden.
- Note: For general development principles, see `AGENTS.md` in the parent directory (informational reference only)
