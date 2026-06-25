# edge-llm-complete

LLM Complete domain primitive (ADR-043): canonical HTTP-level completion port contract.

## Overview

Defines the `Completer` trait and full supporting type vocabulary for LLM completion.
Provider plugin backends implement `Completer`; agents and reasoning pipelines consume it.

## Usage

```rust
use edge_llm_complete::{Completer, CompletionRequest, Message, StdCompleteFactory};

let req = StdCompleteFactory::request("my-model".into(), vec![
    StdCompleteFactory::user_message("Hello!".into()),
]);
// wire up a real Completer implementation to call req
```

## Architecture

Follows the SEA (Service Encapsulation Architecture) leaf pattern:
- `api/` — public traits and value types
- `core/` — default implementations (`pub(crate)`)
- `saf/` — service abstraction surface (factory functions + SVC constants)
- `spi/` — extension anchor for downstream consumers
