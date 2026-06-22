# edge-domain-pipeline

**A generic, reusable trait for immediate/synchronous step-chain execution.**

## Overview

`edge-domain-pipeline` provides a composable pattern for ordered pipeline execution with sequential steps, shared context mutations, and fail-fast error handling.

Unlike Handler (async with branching) or Saga (event-driven, long-running), Pipeline is for immediate, linear execution: step → step → step with early termination on error.

## Core Concepts

### Step<Ctx>
A single composable operation that mutates shared context and may fail. Steps are idempotent.

```rust
pub trait Step<Ctx>: Send + Sync {
    async fn execute(&self, ctx: &mut Ctx) -> Result<(), PipelineError>;
    fn name(&self) -> &str;
}
```

### Pipeline<Ctx>
Orchestrates sequential step execution with guaranteed fail-fast semantics.

```rust
pub trait Pipeline<Ctx>: Send + Sync {
    async fn execute(&self, ctx: &mut Ctx) -> Result<(), PipelineError>;
    fn step_count(&self) -> usize;
    fn is_empty(&self) -> bool;
}
```

### DefaultPipeline<Ctx>
Standard implementation: executes steps sequentially, stops on first error.

### PipelineBuilder<Ctx>
Fluent API for composing pipelines with configuration (timeouts, lifecycle events, abort-on-error).

## Usage

```rust
use edge_domain::PipelineBuilder;

let pipeline = PipelineBuilder::new()
    .with(ExtractTokenStep)
    .with(VerifyTokenStep)
    .with_timeout(Duration::from_secs(10))
    .with(IdentifyCallerStep)
    .build();

pipeline.execute(&mut context).await?;
```

## Features

- **Composable** — Pipelines implement Step, enabling nesting
- **Type-safe** — Generic over context type; no stringly-typed APIs
- **Error-aware** — Fail-fast with PipelineError variants
- **Configurable** — Timeouts, lifecycle events, abort behavior
- **Testable** — Test doubles: NoopStep, AlwaysPassStep, AlwaysFailStep, MutatingStep

## Test Coverage

109 tests across:
- Unit tests (core methods)
- Integration tests (trait contracts)
- Comprehensive tests (happy/error/edge scenarios)
- 100% scenario coverage for all trait methods

## Architecture

SEA structure (L0/L1/L2 layers):
- **api/** — Trait contracts (Step, Pipeline)
- **core/** — DefaultPipeline orchestrator
- **spi/** — PipelineBuilder, test helpers
- **saf/** — Public facade (re-exports)

## License

MIT / Apache 2.0
