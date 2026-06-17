# edge-llm-prompt

LLM Prompt domain primitive: template rendering and dynamic context management
for composable prompts. Decouples prompt construction from rendering logic
(handlebars, Jinja2, custom).

## What it provides

- `Prompt` — template contract: render, metadata, validate, variable typing, caching.
- `ContextManager` — variable registration and render-context building.
- `TokenCounter` — exact or approximate prompt tokenization.
- A type vocabulary: `Variable`, `VariableType`, `PromptMetadata`, `RenderContext`,
  `PromptCache`, and `PromptError`.

## Layout (SEA)

```
main/src/
├── api/prompt/    # Public contracts: traits/, types/, errors/
├── core/prompt/   # Concrete implementations (impl <Trait> for <Type>)
└── saf/prompt/    # Service Abstraction Framework — the only public surface
```

Consumers depend only on the `saf/` re-exports surfaced from `lib.rs`.

## Usage

```rust
use edge_llm_prompt::{PromptFactory, StdPromptFactory, PromptMetadata, RenderContext};

let metadata = PromptMetadata::new(
    "greet".to_string(),
    "Greeting".to_string(),
    "1".to_string(),
    vec![],
);
let prompt = StdPromptFactory::prompt("Hello there".to_string(), metadata);
```

See `examples/prompt.rs` for a runnable example.
