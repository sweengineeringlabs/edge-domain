# edge-llm-provider

LLM Provider domain primitive (ADR-033): a pluggable execution-backend abstraction
that decouples agent orchestration from specific LLM backends (OpenAI, Claude, local models).

## What it provides

- `LLMProvider` — backend contract: identity, config, model metadata, completion, and streaming.
- `ExecutionModel` — single-step reasoning-execution contract with budget/throttle checks.
- A rich type vocabulary: `TokenUsage`, `ExecutionError`, `StreamChunk`, `ModelInfo`, and more.

## Layout (SEA)

```
main/src/
├── api/provider/    # Public contracts: traits/, types/, errors/
├── core/provider/   # Concrete implementations (impl <Trait> for <Type>)
└── saf/provider/    # Service Abstraction Framework — the only public surface
```

Consumers depend only on the `saf/` re-exports surfaced from `lib.rs`.

## Usage

```rust
use edge_llm_provider::{ProviderFactory, ExecutionMode};

let mode = ExecutionMode::Streaming;
assert!(mode.is_streaming());
```

See `examples/provider.rs` for a runnable example.
