# Changelog

## [0.1.0] — 2026-06-22

### Added

- **Core traits**: Step<Ctx>, Pipeline<Ctx> for composable step-chain execution
- **DefaultPipeline**: Sequential orchestrator with fail-fast error handling
- **PipelineBuilder**: Fluent API for pipeline composition with configuration
- **PipelineConfig**: Timeout, lifecycle events, abort-on-error settings
- **PipelineError**: Structured error types (StepFailed, StepTimeout, ConfigError)
- **Test helpers**: NoopStep, AlwaysPassStep, AlwaysFailStep, MutatingStep
- **Integration**: Wired into edge-domain via `pipeline` feature
- **109 tests**: Unit, integration, and comprehensive scenario coverage
- **Documentation**: README, CHANGELOG, examples

### Features

- Object-safe trait dispatch via `Arc<dyn Step<Ctx>>`
- Nested pipelines (Pipeline implements Step for composability)
- Generic context types (any T: Send)
- Idempotent step invariant
- Async trait execution with error propagation

### Quality

- 100% test scenario coverage (happy/error/edge paths)
- Production-ready error handling
- Type-safe API (no stringly-typed operations)
- Zero unsafe code

## Architecture

- Service Encapsulation Architecture (SEA) L0/L1/L2 layers
- api/ — trait contracts
- core/ — implementations
- spi/ — builders and test doubles
- saf/ — public facade

---

For documentation, see README.md or run `cargo doc --open`.
