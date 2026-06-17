# edge-llm-reasoning

LLM Reasoning domain primitive: multi-strategy reasoning patterns
(chain-of-thought, tree-of-thought, reflection, …) for complex problem solving.

## What it provides

- `Reasoning` — primary contract: execute a pattern over a problem, inspect
  supported patterns, validate problems, and assemble reasoning chains.
- `ReasoningFactory` — constructors for the reference reasoner and fluent
  builders for every value type.
- A rich type vocabulary: `ReasoningPattern`, `ReasoningStep`, `StepResult`,
  `ThinkingProcess`, `PatternMetadata`, `ReasoningChain`, `ReasoningError`.

## Layout (SEA)

```
main/src/
├── api/reasoning/    # Public contracts: traits/, types/, errors/
├── core/reasoning/   # Concrete implementations (impl <Trait> for <Type>)
└── saf/reasoning/    # Service Abstraction Framework — the only public surface
```

Consumers depend only on the `saf/` re-exports surfaced from `lib.rs`.

## Usage

```rust
use edge_llm_reasoning::{ReasoningFactory, ReasoningPattern, StdReasoningFactory};

let reasoner = StdReasoningFactory::reasoning(ReasoningPattern::ChainOfThought);
assert!(reasoner.pattern() == ReasoningPattern::ChainOfThought);
```

See `examples/reasoning.rs` for a runnable example.
