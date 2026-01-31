# AGENTS.md - Forseti Development Guidelines

> **Hinweis**: Für Hintergrundinformationen zum Projekt, Features, Architektur und technische Details siehe `README.md`.

Diese Datei enthält ausschließlich Richtlinien für das Verhalten des Agents bei der Entwicklung von Forseti (ML/DL/RL Service).

## Core Development Principles

### 1. Test-Driven Development (TDD)

**Primary Focus**: All code MUST be developed using Test-Driven Development methodology. This is MANDATORY and must be followed strictly.

- **Write tests first**: Tests are ALWAYS written before implementation code - no exceptions
- **Red-Green-Refactor cycle**: Red (failing test) → Green (passing test) → Refactor
- **Test coverage**: Aim for high test coverage (minimum 80% for critical paths)
- **Test types**: Unit tests for training logic, integration tests for Python-FFI, end-to-end tests for complete training workflows
- **ML-specific tests**: Model validation tests, training convergence tests, RL-environment tests
- **Strict adherence**: TDD is not optional - it is the foundation of all development work
- **No implementation without tests**: Never write implementation code without corresponding tests first

### 2. Security

**Security-First Mindset**: Security is critical for ML/DL/RL service handling sensitive training data and models.

- **Training-data security**: All training data must be encrypted at rest
- **Model security**: Model weights must be encrypted in storage
- **Python-FFI security**: Sandboxed Python runtime with resource limits
- **No arbitrary code execution**: Only predefined operations allowed
- **Input validation**: All inputs (data, configs, models) must be validated
- **No hardcoded secrets**: Never commit API keys or credentials
- **Audit logging**: Log all training operations for compliance

### 3. Performance

**Performance from the Start**: Forseti must provide fast training, inference, and RL operations.

- **Training throughput**: > 1000 samples/sec (GPU)
- **RL inference**: < 10ms latency for real-time decisions
- **Model export**: < 5min for 7B-model GGUF conversion
- **FFI overhead**: < 5% vs. native Python
- **Async operations**: Use async for I/O-bound operations
- **Batch processing**: Minimize FFI calls through batching
- **Memory efficiency**: Optimize memory usage for large models

### 4. DRY (Don't Repeat Yourself)

**Reuse Before Creating**: Always check for existing functionality.

- **Check existing trainers**: Before implementing new trainer, check if similar exists
- **Reuse training abstractions**: Reuse common training logic across frameworks
- **Shared utilities**: Wenn gemeinsame Utils benötigt werden, sollte ein separates Projekt erstellt werden
- **Avoid duplication**: Don't duplicate framework-specific logic
- **Framework abstraction**: Use unified training API across PyTorch/TensorFlow/JAX

### 5. KISS (Keep It Simple, Stupid)

**Simplicity Over Complexity**: Prefer simple training pipelines.

- **Simple trainer abstraction**: Keep trainer abstraction simple
- **Clear training flow**: Maintain clear training workflow
- **Readable code**: Code should be self-documenting
- **Avoid over-engineering**: Don't add unnecessary complexity
- **Progressive enhancement**: Start simple, add complexity only when needed

### 6. CQRS (Command Query Responsibility Segregation)

**Software-Level Separation**: Separate read and write operations.

- **Training commands**: Separate handlers for training operations
- **RL commands**: Separate handlers for RL operations
- **Model queries**: Separate handlers for model information queries
- **Status queries**: Separate handlers for training status queries
- **Optimization**: Optimize commands and queries independently

### 7. Single Responsibility Principle

**One Responsibility Per Component**: Each component should have a single responsibility.

- **Training orchestrator**: Training orchestration only
- **RL engine**: RL operations only
- **Model manager**: Model registry and storage only
- **Inference engine**: Inference only
- **Export handler**: Model export only
- **Python-FFI bridge**: FFI communication only

### 8. Dependency Injection

**Inject, Don't Create**: Dependencies should be injected.

- **Framework trainers**: Inject PyTorch/TensorFlow/JAX trainers
- **Model registry**: Inject model registry
- **Configuration**: Inject training configuration
- **Python runtime**: Inject Python runtime
- **Database connection**: Inject database pool

## Development Workflow

Follow this workflow for all coding tasks:

### 1. Analyze and Understand the Problem

- **Understand ML requirements**: Understand training/RL/inference requirements
- **Identify constraints**: Identify performance, memory, and hardware constraints
- **Research existing solutions**: Check if similar training logic exists
- **Ask questions**: Clarify any ML-specific ambiguities
- **Document assumptions**: Document training assumptions

### 2. Plan Your Steps, Create Todos

#### 2.1. List All Components

- **Identify ML components**: List trainers, models, environments, policies
- **Define interfaces**: Define training/RL/inference interfaces
- **Plan dependencies**: Identify dependencies between components
- **Create structure**: Plan file and directory structure
- **Document design**: Document ML architecture decisions

#### 2.2. Create the Tests

- **Write test cases**: Write comprehensive test cases for training/RL logic
- **Test edge cases**: Include tests for overfitting, underfitting, convergence
- **Test RL scenarios**: Test RL environments, policies, rewards
- **Mock dependencies**: Use mocks for Python-FFI and external dependencies
- **Test data**: Prepare synthetic test datasets
- **Container setup**: Ensure all tests can run in GPU-enabled containers
- **No local dependencies**: Tests must not require local GPU/CUDA installation

#### 2.3. Run the Tests and Expect Them to Fail

- **Run test suite**: Execute all tests
- **Verify failures**: Confirm that tests fail as expected (Red phase)
- **Document failures**: Note what failures are expected
- **Check coverage**: Ensure test coverage is comprehensive

#### 2.4. Create the Code and Make the Tests Run Successfully

- **Implement minimally**: Write minimal code to make tests pass (Green phase)
- **Run tests frequently**: Run tests after each small change
- **Fix all tests**: Always try to correct all tests and let them run again
- **Or focus on component**: Or just run tests of the component you currently work on
- **Iterate**: Continue until all tests pass

### 3. Step by Step Work on the Todos

- **Work systematically**: Complete todos one by one
- **Run tests**: Run tests after each todo completion
- **Refactor**: Refactor code while keeping tests green
- **Document**: Document ML-specific code
- **Review**: Review your code before moving to the next todo

### 4. Check Again if the Task's Goal is Achieved

- **Verify ML requirements**: Ensure training/RL/inference requirements are met
- **Test thoroughly**: Run the full test suite including ML validation
- **Check integration**: Verify integration with Odin/Geri/Thor
- **Performance check**: Verify performance requirements are met
- **Security review**: Review security implications
- **If not complete**: If the goal is not achieved, go back to Step 1

### 5. Give a Note to the User

- **Document changes**: Document all ML-related changes
- **Explain training process**: Explain training workflow
- **Provide examples**: Provide training/RL code examples
- **List dependencies**: List any new ML dependencies (PyTorch, TensorFlow, etc.)
- **Migration notes**: Include migration notes if breaking changes

## ML/DL Best Practices

### Data Handling

- **Data validation**: Validate all training data before use
- **Data preprocessing**: Normalize, standardize, augment data as needed
- **Data splitting**: Use proper train/val/test splits (e.g., 70/15/15)
- **Data versioning**: Version training datasets
- **Data privacy**: Ensure training data is encrypted and handled securely

### Model Training

- **Reproducibility**: Set random seeds for reproducible training
- **Checkpointing**: Save model checkpoints regularly during training
- **Early stopping**: Implement early stopping to prevent overfitting
- **Learning rate scheduling**: Use learning rate scheduling (e.g., ReduceLROnPlateau)
- **Gradient clipping**: Use gradient clipping to prevent exploding gradients
- **Monitoring**: Monitor training metrics (loss, accuracy, etc.) continuously
- **Logging**: Log all training hyperparameters and results

### Model Evaluation

- **Validation**: Use separate validation set for model evaluation
- **Multiple metrics**: Evaluate using multiple metrics (accuracy, F1, precision, recall)
- **Cross-validation**: Use cross-validation for robust evaluation
- **Statistical significance**: Test for statistical significance
- **Error analysis**: Analyze model errors to identify failure modes

### Hyperparameter Optimization

- **Grid search**: Use grid search for small hyperparameter spaces
- **Random search**: Use random search for large hyperparameter spaces
- **Bayesian optimization**: Use Bayesian optimization for efficient search
- **Early stopping**: Stop unpromising runs early
- **Document results**: Document all hyperparameter experiments

## RL-Specific Guidelines

### Environment Design

- **Clear observation space**: Define clear observation space
- **Clear action space**: Define clear action space
- **Reward function**: Design reward function carefully (avoid sparse rewards)
- **Episode termination**: Define clear episode termination conditions
- **Environment validation**: Validate environment before training

### Reward Engineering

- **Reward shaping**: Use reward shaping to guide learning
- **Avoid reward hacking**: Design rewards to prevent reward hacking
- **Normalize rewards**: Normalize rewards for stable training
- **Intrinsic motivation**: Consider intrinsic motivation for exploration
- **Reward clipping**: Use reward clipping for stability

### Policy Training

- **Exploration strategy**: Use appropriate exploration strategy (ε-greedy, Boltzmann)
- **Experience replay**: Use experience replay for sample efficiency (DQN, SAC)
- **Target networks**: Use target networks for stability (DQN)
- **Policy gradient variance**: Reduce policy gradient variance (baseline, advantage)
- **PPO clipping**: Use PPO clipping for stable policy updates
- **Monitor convergence**: Monitor policy convergence (reward, policy entropy)

### Multi-Agent RL

- **Communication**: Define agent communication protocol
- **Coordination**: Implement coordination mechanisms
- **Credit assignment**: Handle credit assignment problem
- **Scalability**: Ensure scalability to many agents
- **Decentralized execution**: Support decentralized execution

## Python-FFI Guidelines

### pyo3 Best Practices

- **Minimize FFI calls**: Batch operations to minimize FFI overhead
- **Zero-copy**: Use zero-copy for large tensors (> 1MB)
- **Async bridge**: Use pyo3-asyncio for async Python calls
- **Error handling**: Proper error handling across FFI boundary
- **Type conversion**: Efficient type conversion (Rust ↔ Python)
- **Memory management**: Careful memory management (ownership, lifetimes)

### Performance Optimization

- **Batch processing**: Process data in batches to reduce FFI calls
- **Shared memory**: Use shared memory for large data transfers
- **Async operations**: Use async for I/O-bound Python operations
- **Caching**: Cache frequently used Python objects
- **Profiling**: Profile FFI overhead and optimize hotspots

### Memory Management

- **Resource cleanup**: Proper cleanup of Python resources
- **Memory limits**: Enforce memory limits for Python runtime
- **Leak detection**: Detect and fix memory leaks
- **Reference counting**: Proper reference counting across FFI boundary
- **GIL handling**: Handle Python GIL correctly

## Testing Strategies for ML/RL

### Unit Tests

- **Training logic**: Test training loops, optimizers, loss functions
- **Data preprocessing**: Test data loaders, transformations, augmentations
- **Model operations**: Test forward pass, backward pass, parameter updates
- **RL components**: Test environments, policies, reward functions
- **FFI operations**: Test Python-FFI bridge operations

### Integration Tests

- **Training pipelines**: Test complete training workflows (PyTorch, TensorFlow, JAX)
- **RL pipelines**: Test complete RL training workflows
- **Model export**: Test model export to GGUF/ONNX/SafeTensors
- **Service integration**: Test integration with Odin/Geri/Thor
- **Database operations**: Test model registry CRUD operations

### Model Validation Tests

- **Convergence tests**: Test that training converges on toy datasets
- **Overfitting tests**: Test that model can overfit on small datasets
- **Invariance tests**: Test model invariance properties
- **Adversarial tests**: Test model robustness to adversarial examples
- **Performance tests**: Test that model meets performance requirements

### RL-Environment Tests

- **Environment consistency**: Test environment state transitions
- **Reward consistency**: Test reward function correctness
- **Episode termination**: Test termination conditions
- **Observation space**: Test observation space bounds
- **Action space**: Test action space bounds

### Performance Tests

- **Training throughput**: Test training samples/sec
- **RL inference latency**: Test RL decision latency
- **Model export time**: Test export time for different model sizes
- **FFI overhead**: Test FFI overhead vs. native Python
- **Memory usage**: Test peak memory usage during training

### Container-Based Testing

- **GPU containers**: All tests run in GPU-enabled Docker containers
- **CUDA support**: Tests support NVIDIA CUDA
- **Isolated environment**: Each test run in isolated container
- **Reproducible**: Test environment reproducible across machines
- **CI/CD ready**: Container setup works in CI/CD pipelines

## Code Quality Standards

### Rust Code Quality

- **Linting**: Code must pass `cargo clippy` checks
- **Formatting**: Code must be formatted with `cargo fmt`
- **Documentation**: Public APIs must have doc comments
- **Error handling**: Use `Result<T, E>` for error handling
- **Type safety**: Leverage Rust type system for safety
- **Async/await**: Use async/await for I/O-bound operations

### Python Code Quality

- **Type hints**: Use type hints for all Python functions
- **Linting**: Code must pass `pylint`/`flake8` checks
- **Formatting**: Code must be formatted with `black`
- **Documentation**: Functions must have docstrings
- **Error handling**: Use proper exception handling
- **Testing**: All Python code must have unit tests

### ML Code Quality

- **Reproducibility**: Set random seeds for reproducibility
- **Versioning**: Version datasets, models, and experiments
- **Documentation**: Document training hyperparameters and results
- **Experiment tracking**: Track experiments (e.g., with MLflow, TensorBoard)
- **Model cards**: Create model cards documenting model details

## Performance Standards

### Training Performance

- **Training throughput**: > 1000 samples/sec (GPU)
- **Multi-GPU scaling**: Linear scaling up to 8 GPUs
- **Memory efficiency**: < 8GB VRAM for 1B-model training
- **Checkpoint speed**: < 10s for checkpointing 1B-model

### RL Performance

- **RL inference latency**: < 10ms for policy inference
- **Episode throughput**: > 100 episodes/sec
- **Policy update speed**: < 100ms per policy update
- **Environment parallelization**: Support up to 32 parallel environments

### Inference Performance

- **Inference latency**: < 5ms (Rust-native)
- **Batch throughput**: > 10000 samples/sec
- **Model loading**: < 1s for < 1B models
- **Memory overhead**: < 2GB for inference engine

### Export Performance

- **GGUF export**: < 5min for 7B-model
- **ONNX export**: < 2min for 1B-model
- **SafeTensors export**: < 1min for 1B-model

### FFI Performance

- **FFI overhead**: < 5% vs. native Python
- **Tensor transfer**: Zero-copy for > 1MB tensors
- **Async call overhead**: < 1ms for async Python calls

## Security Standards

### Training Data Security

- **Encryption at rest**: Encrypt all training data at rest
- **Access control**: Strict access control for training data
- **Data validation**: Validate all training data inputs
- **No data leakage**: Ensure no training data leakage in logs/errors
- **GDPR compliance**: Handle training data according to GDPR

### Model Security

- **Model encryption**: Encrypt model weights at rest
- **Model validation**: Validate model files before loading
- **SafeTensors**: Use SafeTensors format (no arbitrary code execution)
- **Model versioning**: Version all models for audit trail
- **Access control**: Strict access control for models

### Runtime Security

- **Sandboxed Python**: Python runtime runs in sandbox
- **Resource limits**: Enforce CPU/memory/GPU limits
- **No arbitrary code**: Only predefined operations allowed
- **Input validation**: Validate all runtime inputs
- **Error handling**: Proper error handling without information leakage

## Monitoring & Logging

### Training Monitoring

- **Loss tracking**: Monitor training/validation loss
- **Metrics tracking**: Monitor accuracy, F1, precision, recall
- **Gradient monitoring**: Monitor gradient norms
- **Learning rate**: Monitor learning rate changes
- **Resource usage**: Monitor GPU/CPU/memory usage

### RL Monitoring

- **Reward tracking**: Monitor episodic reward
- **Policy entropy**: Monitor policy entropy
- **Value estimates**: Monitor value function estimates
- **Exploration**: Monitor exploration rate
- **Success rate**: Monitor task success rate

### System Monitoring

- **Request throughput**: Monitor training/inference requests
- **Latency**: Monitor request latency
- **Error rate**: Monitor error rate
- **Resource usage**: Monitor system resource usage
- **FFI overhead**: Monitor Python-FFI overhead

### Logging Standards

- **Structured logging**: Use structured logging (tracing)
- **Log levels**: Use appropriate log levels (debug, info, warn, error)
- **Request tracing**: Include request_id in all logs
- **Performance logging**: Log training/inference times
- **Error logging**: Log errors with full context
- **No sensitive data**: Never log sensitive data (API keys, training data)

## Documentation Standards

### Code Documentation

- **Public APIs**: Document all public APIs with doc comments
- **Complex logic**: Comment complex training/RL logic
- **Hyperparameters**: Document all hyperparameters
- **Algorithms**: Document ML/RL algorithms used
- **Performance notes**: Document performance considerations

### Training Documentation

- **Training recipes**: Document training recipes for different tasks
- **Hyperparameter guides**: Document hyperparameter tuning guides
- **Best practices**: Document ML/RL best practices
- **Troubleshooting**: Document common training issues and solutions
- **Examples**: Provide training examples for different use cases

### RL Documentation

- **Environment specs**: Document environment specifications
- **Reward functions**: Document reward function design
- **Policy architectures**: Document policy network architectures
- **Training procedures**: Document RL training procedures
- **Deployment**: Document policy deployment procedures

## Getting Started

1. **Read the README**: Read `forseti/README.md` for context
2. **Read AGENTS.md**: Read this file for development guidelines
3. **Set up environment**: Set up Rust + Python development environment
4. **Install dependencies**: Install Rust and Python dependencies
5. **Run tests**: Run existing tests to ensure setup is correct
6. **Follow workflow**: Follow the development workflow for all tasks
7. **GPU setup**: Ensure GPU access for ML/RL development (CUDA, Docker)

## Data Protection and Privacy (GDPR Compliance)

**EU/German Data Protection**: All ML/RL operations must comply with GDPR.

### GDPR Requirements

- **Data minimization**: Only collect training data that is strictly necessary
- **Purpose limitation**: Training data may only be used for specified purpose
- **Storage limitation**: Delete training data after model training
- **Data accuracy**: Ensure training data accuracy
- **Integrity and confidentiality**: Encrypt training data at rest and in transit
- **Right to access**: Users have right to access their training data
- **Right to rectification**: Users have right to correct training data
- **Right to erasure**: Users have right to delete their training data
- **Right to data portability**: Users have right to export training data
- **Privacy by design**: Privacy must be considered from design phase

### Implementation Requirements

- **Encryption**: Encrypt all training data and models at rest
- **Access control**: Strict access control for training data and models
- **Audit logging**: Log all data access for compliance auditing
- **Data deletion**: Implement secure deletion of training data
- **Data export**: Provide mechanisms for training data export
- **Consent management**: Implement consent for data usage
- **Anonymization**: Anonymize training data where possible

## Additional Resources

- Forseti README: `forseti/README.md`
- Main AGENTS.md: `AGENTS.md` (root)
- Technology Decisions: `docs/TECHNOLOGY_DECISIONS.md`
- PyTorch Documentation: https://pytorch.org/docs/
- TensorFlow Documentation: https://www.tensorflow.org/api_docs/
- JAX Documentation: https://jax.readthedocs.io/
- Stable-Baselines3 Documentation: https://stable-baselines3.readthedocs.io/
- pyo3 Documentation: https://pyo3.rs/
