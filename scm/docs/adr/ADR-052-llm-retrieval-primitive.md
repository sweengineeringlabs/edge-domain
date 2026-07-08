# ADR-052: `edge-llm-retrieval` — Retrieval-Augmentation Primitive

**Status:** Proposed
**Date:** 2026-07-08
**Author:** Senior Agentic Engine Engineer
**Relates to:** ADR-034 (LLM Prompt), ADR-043 (LLM Complete), [edge ADR-042](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-042-llmprovider-reshape-to-edge-plugin.md) (plugin boundary)
**GitHub Issues:** TBD

---

## Context

The 2026-07-08 LLM-landscape audit named "No RAG/embeddings/vector retrieval... anywhere in `edge/`" as one item in ADR-045's "does NOT solve" list (`domain/scm/docs/adr/ADR-045-edge-llm-runtime-standalone-composition-root.md:69`). This ADR verifies that claim independently and designs the primitive that closes it.

**Verification (fresh grep, this session).**

```
cd C:\phd-systems\swelabs\edge
grep -rliE "embedding|vector_store|VectorStore|Embedding|retrieval.?augmented|\bRAG\b" --include=*.rs domain/
```

Zero matches, exit code 1. Widening to the whole `edge/` tree with the four ML-specific terms (`embedding|vector_store|VectorStore|Embedding`, dropping the generic `RAG`/`retrieval` terms which are prone to false positives) returns four files, all outside `domain/`. Inspecting the first hit confirms it is a false positive, not a hidden feature:

```
security/transport/http/egress/scm/oauth/main/src/api/refresh/types/credentials_validation_request.rs:7:
/// (`field_type_purity`) instead of embedding another domain struct.
```

That is the English verb "embed" in a SEA-rule doc comment (the same phrasing convention used across this codebase's `field_type_purity` comments, e.g. `domain/scm/domain/llm/prompt/main/src/api/prompt/types/context_build_response.rs:8-9`), not a reference to vector embeddings. The other three hits are the same pattern (`verification_response.rs`, `resilience_config_resilience_validator.rs`, `default_http_retry_int_test.rs` — "valid**a**tion"/"resol**u**tion"-shaped identifiers, no ML vocabulary). **Correction (post-review):** an earlier draft of this ADR miscounted these as "six files" and cited a nonexistent `identity_resolution_*` filename among them — corrected above to the actual re-verified count and file list. The landscape hole is confirmed: complete, not partial.

**Where a retrieved chunk would have to land.** `edge-llm-prompt`'s `ContextManager` (`domain/scm/domain/llm/prompt/main/src/api/prompt/traits/context_manager.rs:10-33`) is the variable-registration contract — a prior audit this session already established it is *not* a capacity/pruning manager. Its shape:

- `register_variable(req: RegisterVariableRequest<'_>)` (line 14) — `RegisterVariableRequest<'a> { name: String, var: &'a Variable }` (`.../types/register_variable_request.rs:7-12`).
- `build_context(req: ContextBuildRequest)` (line 26) — `ContextBuildRequest` carries no data (`.../types/context_build_request.rs:5`); the response, `ContextBuildResponse { variables: HashMap<String, JsonValue>, metadata: HashMap<String, String>, template_id: Option<String> }` (`.../types/context_build_response.rs:11-18`), is assembled purely from whatever variables were previously registered.

`Variable` (`.../types/variable.rs:6-24`) carries `name`, `var_type: VariableKind`, `value: Option<JsonValue>`, `default`, `required`, `description`. `VariableKind` (`.../types/variable_kind.rs:5-29`) already has a `List` and a `Json` variant — i.e., the existing variable model can already hold an array of retrieved chunks or a structured object per chunk; nothing about `ContextManager`'s trait signature needs to change to accept retrieved content.

**Where a rendered prompt turns into conversation turns.** `edge-llm-complete`'s `Message` (`domain/scm/domain/llm/complete/main/src/api/complete/types/message.rs:7-20`) has `role: Role`, `content: MessageContent`, plus tool-call/cache fields. `Role` (`.../role.rs:6-16`) includes a `System` variant, and `MessageContent` (`.../message_content.rs:8-16`) is `Empty | Text(String) | Parts(Vec<ContentPart>)`. So there are, in principle, two places a retrieved chunk could be injected: as a `ContextManager` variable (pre-render, template-authored) or as a synthesized `Message{role: System, ...}` (post-render, conversation-authored). Decision below explains why only the former composes correctly with the rest of the pipeline.

**Nothing to reuse at the general-domain layer.** Unlike ADR-046 (which found `edge-domain-policy`/`edge-security-runtime`/`edge-domain-observer` already shaped like 3/4 of ADR-036's proposal), this audit found no existing general-purpose similarity-search port to compose from. `domain-repository`'s two traits are the closest candidates and neither fits: `Repository::find/list/count` (`domain/scm/domain-repository/main/src/api/repository/traits/repository.rs:23-95`) is keyed CRUD lookup by `Self::Id`, with no notion of a distance metric or top-k ranking; `Spec::matches` (`.../traits/spec.rs:10-21`) is a boolean predicate over one entity, not a ranked-retrieval query over a corpus. Building `VectorStore` as a `Repository` impl would force a fake `Id`-keyed `find` onto a fundamentally different query shape (nearest-neighbors, not point lookup). There is nothing here to compose from — this crate is new vocabulary, not a wrapper.

## Decision

Build **`edge-llm-retrieval`**, a small, contracts-only crate at `domain/scm/domain/llm/retrieval/` — same subtree as the other five LLM primitives and ADR-046's `edge-llm-tools`.

**Why LLM-scoped, not a general `edge-domain-retrieval`.** `Embedder`/`VectorStore` as abstract shapes (text → vector, vector → ranked neighbors) are not inherently LLM-specific — a non-LLM semantic-search feature could want them too. But two things argue against generalizing now, and for placing this next to the other LLM crates instead of promoting it to a `domain-*` top-level primitive alongside `domain-repository`:

1. **No second consumer exists today.** The only concrete composition point this ADR can point to — `ContextManager::register_variable` — is LLM-specific. Generalizing a crate with a single consumer is exactly the "declare and abandon" anti-pattern (define a general-purpose port, wire it to nothing else, watch it rot). `domain-repository` earned its generality because multiple non-LLM crates already depend on it; `edge-llm-retrieval` has not earned that yet.
2. **The trait signatures stay generalizable regardless of where the crate lives.** As designed below, neither `Embedder` nor `VectorStore` names an LLM type (`Message`, `CompletionRequest`, `Prompt`, etc.) in its signature — only `String`, `Vec<f32>`, and plain metadata maps. If a non-LLM consumer appears later, relocating out from under `domain/llm/` is a crate move, not a trait redesign — the same posture `domain-repository`'s generic `Entity`/`Id` associated types give it today.

Only the composition trait (`RetrievalContextComposer`, below) references an LLM type (`edge_llm_prompt::Variable`) — that dependency is exactly why this crate sits under `domain/llm/` rather than at the `domain-*` top level. This mirrors ADR-046's own precedent of one LLM crate depending on another crate's `api/` types for reuse (`edge-llm-tools` → `edge_llm_agent::{ParameterDocumentation, SkillMetadata}`, ADR-046 line 81) rather than redefining them.

### Shape / workspace layout

```
domain/scm/domain/llm/retrieval/        (edge-llm-retrieval)
├── main/src/api/retrieval/
│   ├── traits/
│   │   ├── embedder.rs                 (Embedder)
│   │   ├── vector_store.rs             (VectorStore)
│   │   └── retrieval_context_composer.rs   (RetrievalContextComposer, default-method logic)
│   ├── types/
│   │   ├── embed_request.rs / embed_response.rs
│   │   ├── vector_record.rs
│   │   ├── vector_upsert_request.rs / vector_upsert_response.rs
│   │   ├── vector_query_request.rs / vector_query_response.rs
│   │   ├── vector_match.rs
│   │   └── retrieval_compose_request.rs / retrieval_compose_response.rs
│   └── errors/
│       └── retrieval_error.rs
└── main/src/saf/retrieval/
    ├── embedder_svc.rs
    ├── vector_store_svc.rs
    └── retrieval_context_composer_svc.rs
```

No `core/`, deliberately (see "contracts-only" below) — the one piece of real logic this ADR needs (composing retrieved matches into a `Variable`) lives as a **default trait method** directly on `RetrievalContextComposer` in `api/`, the same pattern `domain-repository`'s `Spec::matches` already uses for its default `false` predicate (`domain/scm/domain-repository/main/src/api/repository/traits/spec.rs:14-20`) — a trait can carry real, testable default behavior in `api/` without needing a `core/` layer, as long as that behavior touches no vendor/transport concern.

```rust
// api/traits/embedder.rs
pub trait Embedder: Send + Sync {
    fn embed(&self, req: EmbedRequest) -> Result<EmbedResponse, RetrievalError>;
}

// api/traits/vector_store.rs
pub trait VectorStore: Send + Sync {
    fn upsert(&mut self, req: VectorUpsertRequest) -> Result<VectorUpsertResponse, RetrievalError>;
    fn query(&self, req: VectorQueryRequest) -> Result<VectorQueryResponse, RetrievalError>;
}

// api/traits/retrieval_context_composer.rs
pub trait RetrievalContextComposer: Send + Sync {
    /// Convert ranked matches into a prompt `Variable`, ready for
    /// `ContextManager::register_variable`. Default impl below is the
    /// reference behavior; only override for a different serialization shape.
    fn compose(&self, req: RetrievalComposeRequest) -> Result<RetrievalComposeResponse, RetrievalError> {
        let chunks = req.matches.into_iter().map(|m| JsonValue::Object(BTreeMap::from([
            ("id".into(), JsonValue::String(m.id)),
            ("score".into(), JsonValue::Number(m.score as f64)),
            ("text".into(), m.text.map(JsonValue::String).unwrap_or(JsonValue::Null)),
        ]))).collect();
        let var = Variable {
            name: req.variable_name.clone(),
            var_type: VariableKind::List,
            value: Some(JsonValue::Array(chunks)),
            default: None,
            required: false,
            description: Some("Retrieved context chunks, ranked by similarity.".into()),
        };
        Ok(RetrievalComposeResponse { variable: var })
    }
}
```

`EmbedRequest { text: String, model: Option<String> }` / `EmbedResponse { vector: Vec<f32>, dimensions: usize }`; `VectorRecord { id: String, vector: Vec<f32>, text: Option<String>, metadata: HashMap<String, String> }` (the `metadata: HashMap<String,String>` shape mirrors `ContextBuildResponse::metadata`, `.../context_build_response.rs:15`, rather than inventing a new key-value convention); `VectorQueryRequest { vector: Vec<f32>, top_k: usize, filter: HashMap<String, String> }` / `VectorQueryResponse { matches: Vec<VectorMatch> }`; `VectorMatch { id: String, score: f32, text: Option<String>, metadata: HashMap<String, String> }`. `RetrievalError` (`thiserror`, mirrors `RepositoryError`'s shape at `domain/scm/domain-repository/main/src/api/repository/errors/repository_error.rs:7-20`): `EmbeddingFailed(String)`, `DimensionMismatch { expected: usize, actual: usize }`, `StoreUnavailable(String)`, `NotFound(String)`, `InvalidQuery(String)`.

### How a retrieved chunk actually reaches the model

**Decision: inject as a registered prompt variable, before rendering — not as a synthesized `Message` after rendering.**

`edge-llm-prompt`'s pipeline is: register variables → `build_context` → render a `PromptTemplate` against that context → *then* the rendered text becomes (part of) a `Message` for `edge-llm-complete`. Retrieval has to happen before the render step to be useful as *retrieval-augmented generation* in the literal sense — the retrieved chunks need to be available wherever the template author placed `{{retrieved_context}}` (mid-instruction, inside a system preamble, interpolated into a few-shot example, etc.), not bolted on as one more trailing conversation turn the template author had no say over. A raw `Role::System` message appended at the `Message` layer can only ever be prepended/appended to the turn list — it cannot participate in template rendering at all, so it silently forecloses every retrieval use case except "always shove the chunks at the start/end." Composing through `ContextManager::register_variable` keeps retrieval interoperable with the template system that already exists, instead of adding a second, incompatible injection path.

Concretely: `VectorStore::query` → `Vec<VectorMatch>` → `RetrievalContextComposer::compose` → `Variable` → caller (a plugin or the composition root, per ADR-045's pattern) calls `ContextManager::register_variable(RegisterVariableRequest { name: "retrieved_context", var: &variable })` before `build_context`/render. No changes to `ContextManager`'s trait signature are required — `VariableKind::List`/`Json` and `JsonValue::Array`/`Object` already exist and already flow through `build_context` untouched (`ContextBuildResponse.variables: HashMap<String, JsonValue>`).

```
query text
  └─► Embedder::embed(EmbedRequest{text})              → EmbedResponse{vector}
        └─► VectorStore::query(VectorQueryRequest{vector, top_k}) → VectorQueryResponse{matches}
              └─► RetrievalContextComposer::compose(matches)      → Variable
                    └─► ContextManager::register_variable(name, &variable)   (edge-llm-prompt, unchanged)
                          └─► ContextManager::build_context() → PromptTemplate render → Message (edge-llm-complete)
```

### Contracts-only — no `core/`, no vendor backend

Per ADR-042's plugin-boundary rule (also the pattern `edge-llm-complete`'s `Completer` already follows — no real vendor call lives in the domain crate), `edge-llm-retrieval` ships **api/ + saf/ only**:

- No embedding-model implementation (OpenAI/Cohere/local model) — a plugin implements `Embedder`.
- No vector-database implementation (pgvector/Pinecone/Qdrant/in-memory) — a plugin implements `VectorStore`.
- `saf/` re-exports the three traits via `<trait>_svc.rs` files, exactly the `provider_handler_svc.rs`/`context_manager` pattern already used by `edge-llm-provider`/`edge-llm-prompt`.

Depends on: `edge-llm-prompt` (`Variable`, `VariableKind`, `JsonValue` — reused, not redefined, in `RetrievalContextComposer`'s default method).

## What this ADR explicitly does NOT solve

- No real embedding-model or vector-database implementation anywhere — both are plugin work, deferred exactly like `edge-llm-complete`'s vendor `Completer` (ADR-045's "Scope of end-to-end" section).
- No document ingestion/chunking pipeline. `EmbedRequest.text` is assumed already chunked; splitting raw documents into chunks is a separate, unaddressed concern.
- No hybrid search, reranking, or diversity selection (MMR) — `VectorStore::query` is plain top-k similarity, nothing more.
- No token/cost accounting for embedding calls or for the retrieved chunks injected into the prompt — this compounds ADR-045's already-named gap that `TokenCounter` and `ModelInfo.context_window` are never joined (ADR-045 line 70); retrieved context can silently blow the context window exactly like everything else can today.
- No embedding cache/dedup — identical text re-embeds every call.
- No change to `ContextManager`'s trait signature — retrieval is expressed entirely as an ordinary `Variable`, consistent with ADR-046's minimal-invasiveness discipline (no changes to `Skill`/`Agent`) applied here to `ContextManager` instead.

## Consequences

**What this enables**
- A concrete, citable answer to "how would RAG work here at all" — previously a complete landscape hole with no design, not even a stub.
- Retrieval composes with the existing template/render pipeline instead of requiring a parallel injection mechanism, so any `PromptTemplate` can opt in to retrieved context by adding one placeholder.
- Plugins (pgvector, Pinecone, a local embedding model, etc.) have a stable, small port to implement, same shape as every other LLM backend integration in this monorepo.

**What this requires**
- New crate `edge-llm-retrieval` under `domain/scm/domain/llm/retrieval/`, depending on `edge-llm-prompt`.
- Whoever builds the first plugin backend must also decide where `Embedder::embed`/`VectorStore::upsert` get called from (ingestion path) versus `VectorStore::query`/`RetrievalContextComposer::compose` (request path) — this ADR defines the ports, not the ingestion job/scheduler that populates the store.
- No changes to `edge-llm-prompt`, `edge-llm-complete`, or any other existing LLM crate's `api/`.

## Alternatives Considered

**Inject retrieved chunks as a `Message{role: System, ...}` at the `edge-llm-complete` layer instead of a prompt variable**
Rejected. This only works for the "always prepend/append" case and cannot participate in template rendering — a template author who wants retrieved context inline (e.g. `"Given the following facts: {{retrieved_context}}, answer: {{question}}"`) has no way to express that if retrieval only ever produces a trailing `Message`. Registering a variable keeps both use cases (inline-templated and simple-prepend, since a template can trivially be `"{{retrieved_context}}\n{{question}}"`) available through one mechanism.

**Generalize immediately into `edge-domain-retrieval` at the `scm/` top level, alongside `domain-repository`/`domain-policy`**
Rejected for now, not forever. There is no second, non-LLM consumer anywhere in the monorepo today; generalizing a port with exactly one consumer is speculative and risks becoming exactly the kind of declared-and-abandoned surface this codebase's engineering standard rejects. The trait signatures are already consumer-agnostic (`String`/`Vec<f32>`/plain maps, no LLM types in `Embedder`/`VectorStore`), so promoting the crate later, if a non-LLM search feature appears, is a relocation, not a redesign.

**Fold retrieval into `edge-llm-prompt` itself, e.g. a new `ContextManager::register_retrieved(matches)` method**
Rejected. Embedding/vector-store lifecycle (upsert at ingestion time, query at request time) runs on a different cadence and has different dependencies (vector math, a store connection) than prompt-variable bookkeeping. Forcing `edge-llm-prompt` to depend on those concerns would violate the single-responsibility split every other LLM crate in this family already follows (provider/prompt/reasoning/complete/agent are each one concern).

**Fold `Embedder` into `edge-llm-provider`'s `Provider` trait, since providers already make the one vendor HTTP call per request**
Rejected. Embeddings and completions are different API surfaces even on the same vendor (different endpoint, different model, different response shape), and `Provider` (ADR-033) models chat/completion turns, not similarity search. Keeping `Embedder`/`VectorStore` in their own crate keeps `Provider`'s scope exactly what ADR-033 defined it as.

## Tracking

- New crate: `edge-llm-retrieval` (`domain/scm/domain/llm/retrieval/`)
- Follow-up (separate ADR/issue, explicitly out of scope here): first real `Embedder`/`VectorStore` plugin backend
- Follow-up: document ingestion/chunking pipeline that produces `EmbedRequest`s in the first place
- Follow-up: join retrieved-context token accounting with ADR-045's still-open `TokenCounter`/`context_window` gap
- Depends on this ADR being accepted before any composition root (ADR-045's `edge-llm-runtime`) wires a retrieval step ahead of `ContextManager::build_context`
