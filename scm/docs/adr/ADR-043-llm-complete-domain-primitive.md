# ADR-043: LLM Complete Domain Primitive — HTTP-level completion port contract

**Status:** Implemented  
**Date:** 2026-06-19  
**Governing ADR:** [ADR-042](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-042-llm-provider-reshape.md) — LLM Provider Reshape  
**Relates to:** [ADR-006](ADR-006-observability-domain-primitive.md) — Observability Domain Primitive, [ADR-033](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-033-llm-provider-domain-primitive.md) — LLM Provider Domain Primitive  
**GitHub Issues:** [#79](https://github.com/sweengineeringlabs/edge-domain/issues/79) — implement edge-llm-complete, [#80](https://github.com/sweengineeringlabs/edge-domain/issues/80) — wire Completer port into edge-llm-provider

---

## Mandate

Introduce `edge-llm-complete` as a new crate in this workspace — the canonical HTTP-level completion port contract for the LLM domain. It defines the boundary that all LLM vendor backends (OpenAI, Anthropic, local models) implement and that all domain consumers (agents, reasoning pipelines, plugins) program against.

`edge-llm-provider` is **not modified** to carry this responsibility. The provider crate models an execution backend for agent-reasoning steps (`ExecutionModel::execute_step`). The complete crate models the raw HTTP-level chat completion interface (`Completer::complete`, `Completer::complete_stream`). These are different abstractions at different levels of the LLM interaction stack.

---

## The boundary problem

Before this ADR, consumers were coupled to `llmprovider`'s internal `LlmComplete` trait — a concrete crate-internal type that leaked vendor-specific design decisions into domain logic. Two consequences:

1. **Domain logic could not be tested without an LLM backend.** No noop or echo completer existed at the domain level.
2. **Vendor swapping required changes in domain code.** The interface was not at a stable port boundary.

`edge-llm-complete` establishes the stable port. Vendors implement `Completer`. Domain logic uses `Completer`. Neither side knows about the other.

---

## New crate: `edge-llm-complete`

Location: `domain/llm/complete/`  
Crate name: `edge-llm-complete`  
`service_type`: `"complete"`

Dependencies: `thiserror`, `async-trait`, `serde`, `serde_json`, `futures` (for `BoxStream`). No vendor deps. No `edge-domain-*` deps — this is a standalone LLM primitive.

---

## What this crate owns

### Primary port contract

| Trait | Role |
|---|---|
| `Completer` | Primary port — `complete()`, `complete_stream()`, `supported_models()`, `model_info()`, `list_models()` |

`Completer` is the single boundary vendors implement and consumers depend on. All other traits in this crate are either composition helpers or secondary contracts for specialised roles.

### Secondary contracts

| Trait | Role |
|---|---|
| `Processor` | SEA processor boundary — `process(request)` — single-method variant of `Completer` for middleware and pipeline stages that only transform; does not expose streaming or model metadata |
| `CompleterHandler` | Marker supertrait (`Completer + Processor + Send + Sync`) — signals the canonical wiring point for `HandlerRegistry` registration; no added methods |
| `CompleteFactory` | Constructor contract — 21 factory methods covering every type in the crate; enables zero-boilerplate request construction in tests and domain code |
| `CompleteOps` | Assembly and inspection — `assemble()`, `extract_usage()`, `extract_finish()`, `create_response()` |
| `CacheableMessage` | Anthropic prompt-caching hint attachment — `with_cache_control()`, `mark_ephemeral()` |
| `ContentFlattener` | Multi-modal to plain-text conversion — `flatten(content: &MessageContent) -> String` |
| `ModelOps` | Model metadata registry — `find_model()`, `create_model_info()` |
| `StreamOps` | Streaming delta accumulation — `apply_delta()`, `into_chunk()` |
| `ToolOps` | Tool-augmented completion execution — `execute()`, `available_tools()`, `tool_choice()`, `merge_delta()` |
| `Validator` | Pre-flight request validation — `validate(request)` |

### Types

| Type | Role |
|---|---|
| `CompletionRequest` | Request to `Completer`: model id, messages, temperature, max_tokens, top_p, stop, tools, tool_choice |
| `CompletionResponse` | Non-streaming result: id, model, content, tool_calls, finish_reason, `Box<TokenUsage>` |
| `CompletionStream` | **Type alias** — `BoxStream<'static, Result<StreamChunk, CompleteError>>` (see §Stream type alias below) |
| `Message` | Single conversation turn: role, content, name, tool_call_id, tool_calls, cache_control |
| `MessageContent` | Message body — `Empty \| Text(String) \| Parts(Vec<ContentPart>)` |
| `ContentPart` | Single multi-modal part — `Text { text } \| ImageUrl { image_url: Box<ImageUrl> } \| ImageBase64 { data, media_type }` |
| `Role` | Message author — `System \| User \| Assistant \| Tool` |
| `ImageUrl` | Image URL with optional detail hint (`"low" \| "high" \| "auto"`) |
| `FinishReason` | Generation end cause — `Stop \| Length \| ToolCalls \| ContentFilter \| Error` |
| `ToolChoice` | Tool calling mode — `Auto \| None \| Required \| Function { name }` |
| `ToolDefinition` | Tool exposed to model: name, description, JSON Schema parameters |
| `ToolCall` | Tool invocation emitted by model: id, name, JSON-encoded arguments |
| `ToolCallDelta` | Streaming fragment of a tool call: index, id, name, partial arguments |
| `StreamChunk` | Single streaming chunk: id, `Box<StreamDelta>`, optional finish_reason |
| `StreamDelta` | Incremental payload: optional text content, optional tool_call fragments |
| `TokenUsage` | Token consumption: prompt, completion, total, cache_read_input, cache_creation_input |
| `ModelInfo` | Model metadata: id, name, provider, context_window, supports_vision, supports_function_calling, supports_streaming |
| `CacheControl` | Anthropic prompt-caching hint: cache_type (JSON key: `"type"`) |
| `EchoCompleter` | Reference `Completer` — echoes last user message; supports model `"echo"` |
| `NoopCompleter` | Reference `Completer` — always returns `CompleteError::ProviderNotFound("noop")` |
| `StdCompleteFactory` | Reference `CompleteFactory` — all methods use trait defaults |

### Error taxonomy

`CompleteError` — 14 variants covering the full HTTP-level failure surface:

| Variant | Retryable |
|---|---|
| `Configuration(String)` | No |
| `ProviderNotFound(String)` | No |
| `ModelNotFound(String)` | No |
| `AuthenticationFailed(String)` | No |
| `RateLimited { retry_after_ms: Option<u64> }` | Yes |
| `ContextLengthExceeded { used: u32, max: u32 }` | No |
| `ContentFiltered(String)` | No |
| `InvalidRequest(String)` | No |
| `NetworkError(String)` | Yes |
| `StreamError(String)` | No |
| `Timeout(u64)` | Yes |
| `ProviderError { provider: String, message: String }` | No |
| `SerializationError(String)` | No |
| `IoError(std::io::Error)` | No |

`is_retryable()` and `retry_after()` are provided for middleware use.

### SAF factories

| Factory fn | Returns | Const anchor |
|---|---|---|
| `noop_completer()` | `NoopCompleter` | `COMPLETER_SVC` |
| `echo_completer()` | `EchoCompleter` | `COMPLETER_SVC` |
| `std_complete_factory()` | `StdCompleteFactory` | `COMPLETE_FACTORY_SVC` |
| *(11 SAF constants total)* | — | `COMPLETER_SVC`, `COMPLETER_HANDLER_SVC`, `PROCESSOR_SVC`, `CACHEABLE_MESSAGE_SVC`, `COMPLETE_FACTORY_SVC`, `COMPLETE_OPS_SVC`, `CONTENT_FLATTENER_SVC`, `MODEL_OPS_SVC`, `STREAM_OPS_SVC`, `TOOL_OPS_SVC`, `VALIDATOR_SVC` |

---

## Stream type alias bypass

`CompletionStream` is a type alias (`pub type CompletionStream = BoxStream<...>`), not a trait or struct. The SEA arch rule `saf_no_trait_reexport` fires on any `pub use ... \w+Stream` in any `saf/` file — the `*Stream` suffix pattern is caught regardless of whether the re-exported item is a trait. Similarly, `pub_types_in_api_only` forbids `pub type` declarations in `saf/` files.

**Resolution:** `CompletionStream` is re-exported directly from `lib.rs` via `pub use crate::api::CompletionStream`, bypassing the `saf/` chain entirely. This is the only item that uses this path. The comment in `lib.rs` documents why.

---

## Relationship to other crates

| Concern | Owner |
|---|---|
| HTTP-level completion port contract (this ADR) | `edge-llm-complete` |
| Agent-reasoning execution backend | `edge-llm-provider` |
| Completer port adapter on top of ExecutionModel | `edge-llm-provider::EchoProviderCompleter` (implements `Completer`) |
| Vendor HTTP backends (OpenAI, Anthropic, …) | `edge-plugin-llm-provider` (assembler layer) |
| Assembler wiring of `CompleteFactory` | `swe-edge-bootstrap` (issue [edge#265](https://github.com/sweengineeringlabs/edge/issues/265)) |

`edge-llm-provider` depends on `edge-llm-complete` — not the reverse. The provider crate's `EchoProviderCompleter` implements `edge_llm_complete::Completer` by adapting `ExecutionModel::execute_step`, establishing the bridge between the two abstractions. Real vendor backends will implement `Completer` directly without going through `ExecutionModel`.

---

## Boundary rules

**B1 — No vendor deps.** `edge-llm-complete` must not import any vendor SDK (openai, anthropic, reqwest). It defines contracts; vendor backends implement them.

**B2 — No dep on `edge-llm-provider`.** The complete port contract must not import from the provider crate. Dependency runs the other way.

**B3 — `Completer` is the only external-facing port.** All other traits (`Processor`, `StreamOps`, `ToolOps`, etc.) are internal composition contracts. External code (agents, plugins, assembler) depends on `Completer` and the value types — not on the auxiliary traits.

**B4 — Noop and echo impls are always available.** `NoopCompleter` and `EchoCompleter` are unconditionally compiled — no feature flag. Any consumer can use them in tests without a live backend.

**B5 — `Box<T>` wrapping for api_field_type_purity.** Fields whose type is a custom domain struct in `api/` must be wrapped in `Box<T>` to satisfy the SEA `api_field_type_purity` rule (`StreamChunk::delta: Box<StreamDelta>`, `CompletionResponse::usage: Box<TokenUsage>`, `ContentPart::ImageUrl { image_url: Box<ImageUrl> }`). Method parameters use `&T` references instead.

---

## Known limitations (deferred)

**L1 — `ToolOps::execute()` is synchronous.** Real tool execution is often async. Implementers that call async runtimes inside `execute()` must use `block_on()`. A future revision should make this `async fn`. Tracked: [#81](https://github.com/sweengineeringlabs/edge-domain/issues/81)

**L2 — No `timeout_ms` on `CompletionRequest`.** `CompleteError::Timeout` exists but no per-request timeout is carried by the request type. Vendors must use their own timeout mechanism. A `timeout_ms: Option<u64>` field on `CompletionRequest` is the correct fix. Tracked: [#82](https://github.com/sweengineeringlabs/edge-domain/issues/82)

**L3 — `ModelInfo` capability model is three booleans.** `supports_vision`, `supports_function_calling`, `supports_streaming` are sufficient for current use. A richer `capabilities: Vec<ModelCapability>` design should be considered if multimodal audio/video or structured output support is needed. Tracked: [#83](https://github.com/sweengineeringlabs/edge-domain/issues/83)

**L4 — System prompt not directly cacheable via `CacheableMessage`.** The trait is implemented for `Message` but `CompletionRequest` has no first-class method to mark the system prompt as cacheable. This is the most impactful Anthropic caching use-case. Callers can work around via `Message::system(...).mark_ephemeral()`, but a `with_cached_system(content)` convenience on `CompletionRequest` would be clearer. Tracked: [#84](https://github.com/sweengineeringlabs/edge-domain/issues/84)

---

## Implementation order (layer-gated TDD)

1. **API layer** — all traits, types, error taxonomy; tests written first (RED → GREEN); gate: `cargo test`, `arch audit --rs`, `cargo clippy -D warnings`
2. **Core/SPI layer** — `EchoCompleter`, `NoopCompleter`, `StdCompleteFactory` impls; SPI anchor (`const _: () = ()`); gate: same
3. **SAF layer** — factory fns, SAF const anchors, integration tests, `examples/complete.rs`; gate: `cargo test`, arch 183/183, clippy, fmt

Final state: **183/183 arch audit** (commit f357770, 2026-06-19).
