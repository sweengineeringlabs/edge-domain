# ADR-067: Conversation/Session Persistence Across Process Restarts

**Status:** Proposed
**Date:** 2026-07-08
**Author:** Senior Agentic Engine Engineer
**Relates to:** ADR-030 (Agent as Domain Primitive), ADR-032 (LLM Agent — checkpointing deferral), [edge ADR-037](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-037-native-primitive-connection-contract.md) (Native Primitive Connection Contract — the ADR ADR-032 pointed to for this, superseded before it ever addressed persistence), ADR-045 (`edge-llm-runtime` composition root), ADR-046 (`edge-llm-tools` governance — origin of the `Handler`-wrapping decorator pattern this ADR reuses), ADR-071 (Persistent `UsageLedger` Backend — origin of the "durable backend ships as an external plugin, domain crate stays dependency-free" precedent this ADR follows)
**GitHub Issues:** TBD

---

## Context

`edge-llm-agent`'s `ConversationLoop`/`BoundedConversationLoop` (added 2026-07-04 per edge-domain#119, per `docs/3-architecture/adr/ADR-032-llm-agent.md:378`) drives a bounded multi-turn conversation over a real `edge-pipeline` `Pipeline`. Its state is **pure in-process memory** with no persistence path anywhere in the crate:

- `ConversationState` (`domain/scm/domain/llm/agents/main/src/core/conversation/conversation_state.rs:8-16`) holds exactly three fields: `messages: Vec<Message>` (full history), `turns_taken: u32` (turns executed this run), and `terminated: bool` (set once a turn produces a non-tool-call finish reason). It is `pub(super)` — never surfaced through any public trait signature; the corresponding `api/conversation/conversation_state.rs:1-7` file is a bare marker struct kept only to satisfy `edge-pipeline`'s `Ctx` module-correspondence rule (an accepted `no_orphan_types` tradeoff, tracked in edge-domain#132).
- `BoundedConversationLoop::run` (`core/conversation/bounded_conversation_loop.rs:18-57`) builds `req.max_turns` identical `Arc<dyn Step>` clones up front (line 34: `vec![step; req.max_turns as usize]`), constructs one fresh `ConversationState::new(req.messages)` (line 42, **always** `turns_taken: 0, terminated: false` regardless of what `req.messages` contains), and runs the whole bounded sequence via a single `pipeline.run(...)` call (lines 43-46). There is no external hook between turns — from a caller's perspective, `run()` is one atomic library call.
- Each turn's work happens in `DefaultConversationTurnStep::execute` (`core/conversation/conversation_turn_step.rs:28-88`): it fetches a completion (lines 37-56), appends the assistant message and increments `turns_taken` (lines 58-59), and — if the completion requested tool calls — executes every requested skill **sequentially, in-process** (lines 69-86), appending each tool result to `ctx.messages` as it completes, before returning `Ok(())` for the whole turn.
- `ConversationRunRequest` (`api/types/conversation_run_request.rs:9-16`) already accepts a `messages: Vec<Message>` "conversation history to continue from" and `ConversationRunResponse` (`api/types/conversation_run_response.rs:5-10`) already returns the full updated history — so **multi-run continuation already exists as an API shape** (a caller can feed a prior response's `messages` back into the next request's `messages`). What's missing is (a) anything that actually stores that history keyed by a conversation/session id, and (b) any save point *inside* a single bounded run, so a crash mid-run doesn't lose everything since the last completed `run()` call.
- `Message` already derives `Serialize, Deserialize` (`api/types/message.rs:5`), so the one payload that matters here is already persistence-ready with no type changes.

**The deferral trail, and why it dead-ends today.** `ADR-032-llm-agent.md:379` states plainly: *"Checkpointing deferred... The domain layer has no `Snapshot` trait wired to `AgentLifecycle` today. Pause-and-resume checkpointing is deferred to ADR-037."* `ADR-037-native-primitive-connection-contract.md` was written next — but its entire subject is the `Handler`/`Service` connection contract (superseded by ADR-024 the day after it was drafted, per its own header line 4-10); it never once mentions `Snapshot`, checkpointing, or conversation state. ADR-032's forward reference is a dead pointer: no ADR has actually designed conversation persistence until this one. The 2026-07-08 landscape audit confirms the gap is still live: zero references to `Snapshot`/`SnapshotStore` exist anywhere under `domain/scm/domain/llm/agents/`, and `edge-llm-agent`'s own `Cargo.toml` (`domain/scm/domain/llm/agents/Cargo.toml:20-32`) does not depend on `edge-domain-snapshot`.

### Reuse check — does an existing state-persistence primitive already fit?

Two existing crates in `domain/scm/` solve "persist an aggregate's state, load it back by id" in other domains. Both were read in full before proposing anything new.

**`edge-domain-snapshot` (`Snapshot`/`SnapshotStore`) — a real structural fit, with caveats.**

- `Snapshot` (`domain-snapshot/main/src/api/snapshot/traits/snapshot.rs:12-27`) is deliberately minimal: an associated `AggregateId` type plus two accessor methods, `aggregate_id()` and `version()`. Its own doc comment (line 11) and the crate's `Cargo.toml` description (`domain-snapshot/Cargo.toml:9`: *"point-in-time aggregate state capture for **event replay optimisation**"*) state its intended use is as an `EventStore` companion — but critically, **the trait carries no payload accessor at all**. The actual state lives on whatever concrete fields the implementing struct declares beyond the trait — proven by the crate's own example (`domain-snapshot/examples/snapshot.rs:5-8`: `OrderSnap { id, version }`, no extra state field required) and its own test fixtures (`core/snapshot/in_memory_snapshot_store.rs:90-94`: `InMemorySnapshotStoreOrderFixture { aggregate_id, version }`). Nothing in the trait or its only concrete store forces an `EventStore` pairing — the crate's own tests never construct one.
- `SnapshotStore` (`domain-snapshot/main/src/api/snapshot/traits/snapshot_store.rs:12-30`) is `save(SnapshotSaveRequest<Self::Snap>) -> BoxFuture<Result<(), SnapshotError>>` / `load(SnapshotLoadRequest<'a, Self::AggregateId>) -> BoxFuture<Result<SnapshotLoadResponse<Self::Snap>, SnapshotError>>` — exactly "persist an aggregate's state, load it back by id," with **no dependency on `DomainEvent`/`Command` types anywhere in its signature**.
- The only invariant enforced is `version >= 1` (`errors/snapshot_error.rs:8-15`, `SnapshotError::InvalidVersion`; enforced in `core/snapshot/in_memory_snapshot_store.rs:52-67`) — a natural fit for a monotonically-increasing turn counter, not an obstacle.
- **The one real gap:** the *only* concrete `SnapshotStore` implementation anywhere in the monorepo is `InMemorySnapshotStore` (`domain-snapshot/main/src/core/snapshot/in_memory_snapshot_store.rs:39-83`), and its own doc comment says outright: *"State lives in process memory and is lost when the process stops"* (`api/snapshot/types/in_memory_snapshot_store.rs:11-12`). A grep across all of `edge/` for any other `SnapshotStore` impl (file-based, SQL, Redis) returns nothing. So reusing the **port** is a genuine win; reusing an **already-durable implementation** is not possible today — none exists to reuse, for this domain or any other.

**`edge-domain-saga` (`Saga`/`SagaStore`) — checked, and rejected as a structural mismatch, not a stylistic preference.**

- `Saga` (`domain-saga/main/src/api/saga/traits/saga.rs:16-34`) is generically bounded by `Event: DomainEvent` and `Command: Command` (lines 21, 24) — it is an event-reactive orchestrator with compensation semantics, not a save/load-by-value store. Adopting it would require inventing a `ConversationEvent`/`ConversationCommand` vocabulary purely to satisfy the trait shape — machinery the conversation loop has no other use for.
- `SagaStore` (`domain-saga/main/src/api/saga/traits/saga_store.rs:6-26`) only exposes `register`/`get` — there is no `save` method at all. Mutation happens by taking `&mut self` on the *live* `Saga` instance borrowed out of the store (`Saga::handle(&mut self, ...)`, line 27-30), not by handing the store an updated value. That model fits a long-lived in-memory orchestrator instance, not "serialize the current conversation and hand it to a store," which is exactly what surviving a process restart requires.
- Verdict: `Saga`/`SagaStore` is the wrong shape for this problem. `Snapshot`/`SnapshotStore` is the right one.

## Decision

Reuse `edge_domain_snapshot::{Snapshot, SnapshotStore}` as the persistence port. Add one new type (`ConversationSnapshot`), one new trait + Request/Response pair (`PersistentConversationLoop`), one new `Step`-decorator (`PersistingConversationTurnStep`), and one new `AgentError` variant. No changes to `ConversationLoop`, `BoundedConversationLoop`, `ConversationState`, or `DefaultConversationTurnStep` — this is additive, mirroring ADR-046's "wrap, don't touch the existing trait" approach for `GovernedHandler`.

### 1. `ConversationSnapshot` — the concrete `Snapshot` impl (new, `edge-llm-agent/api/types/`)

```rust
/// A point-in-time capture of a conversation, keyed by `conversation_id` and versioned
/// by cumulative turns taken (across restarts, not just the current `run()` call — see
/// Part 3 on why `version` cannot be `ConversationState::turns_taken` directly).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationSnapshot {
    pub conversation_id: String,
    pub version: u64,
    pub messages: Vec<Message>,   // already Serialize/Deserialize — message.rs:5
    pub terminated: bool,
}

impl edge_domain_snapshot::Snapshot for ConversationSnapshot {
    type AggregateId = String;

    fn aggregate_id(&self, _req: SnapshotAggregateIdRequest)
        -> Result<SnapshotAggregateIdResponse<'_, String>, SnapshotError> {
        Ok(SnapshotAggregateIdResponse { aggregate_id: &self.conversation_id })
    }

    fn version(&self, _req: SnapshotVersionRequest)
        -> Result<SnapshotVersionResponse, SnapshotError> {
        Ok(SnapshotVersionResponse { version: self.version })
    }
}
```

`messages`/`terminated` are plain fields on the concrete type — never routed through the `Snapshot` trait itself (which, per the reuse check above, was never meant to carry payload). Any consumer holding a concrete `Store: SnapshotStore<Snap = ConversationSnapshot>` reads them directly off the loaded value, exactly the pattern `InMemorySnapshotStore<S: Snapshot>`'s own tests already use.

### 2. New port surface for resumable runs (new, `edge-llm-agent/api/`)

`ConversationRunRequest`/`ConversationLoop::run` are untouched — they remain the "single bounded run, caller supplies the starting history" contract they are today. A **separate** trait adds the persistence-aware entry point:

```rust
// api/types/persistent_conversation_run_request.rs — NEW
pub struct PersistentConversationRunRequest {
    /// Stable identifier for the conversation/session (used interchangeably in this ADR).
    pub conversation_id: String,
    /// New message(s) to append to any restored history (may be empty — e.g. a
    /// crash-recovery resume with no new user input yet).
    pub new_messages: Vec<Message>,
    pub max_turns: u32,
    pub handler_context: Box<OwnedHandlerContext>,
}

// api/traits/persistent_conversation_loop.rs — NEW
#[async_trait::async_trait]
pub trait PersistentConversationLoop: Send + Sync {
    /// Load any prior snapshot for `req.conversation_id`, append `req.new_messages`,
    /// run the bounded loop, persisting after every completed turn. Reuses
    /// `ConversationRunResponse` unchanged — same output shape as `ConversationLoop::run`.
    async fn run_persistent(&self, req: PersistentConversationRunRequest)
        -> Result<ConversationRunResponse, AgentError>;
}
```

### 3. `PersistingConversationTurnStep` — the turn-boundary save (new, `edge-llm-agent/core/conversation/`)

The pipeline in `BoundedConversationLoop::run` builds `Arc<dyn Step<Ctx = ConversationState, ExecutionError = AgentError>>` trait objects already (`bounded_conversation_loop.rs:29-34`) — the decorator wraps that trait object, not the private `DefaultConversationTurnStep` concrete type, so it needs no visibility change:

```rust
struct PersistingConversationTurnStep<Store> {
    inner: Arc<dyn Step<Ctx = ConversationState, ExecutionError = AgentError>>,
    store: Arc<Store>,
    conversation_id: String,
    /// Version of the snapshot this run started from (0 if none) — see the
    /// version-monotonicity note below for why this can't just be `ctx.turns_taken`.
    base_version: u64,
}

#[async_trait]
impl<Store> Step for PersistingConversationTurnStep<Store>
where
    Store: SnapshotStore<AggregateId = String, Snap = ConversationSnapshot> + Send + Sync,
{
    type Ctx = ConversationState;
    type ExecutionError = AgentError;

    async fn execute(&self, req: ContextMutationRequest<'_, ConversationState>) -> Result<(), AgentError> {
        let ctx = req.ctx;
        self.inner.execute(ContextMutationRequest { ctx: &mut *ctx }).await?;

        let version = self.base_version + ctx.turns_taken as u64;
        if version == 0 {
            return Ok(()); // no turn has completed yet in this run; nothing new to persist
        }
        self.store
            .save(SnapshotSaveRequest {
                snapshot: ConversationSnapshot {
                    conversation_id: self.conversation_id.clone(),
                    version,
                    messages: ctx.messages.clone(),
                    terminated: ctx.terminated,
                },
            })
            .await
            .map_err(|e| AgentError::PersistenceFailed(e.to_string()))
    }
}
```

**Why `base_version + turns_taken`, not `turns_taken` alone.** `ConversationState::new` always starts `turns_taken` at `0` (`conversation_state.rs:20-26`) — every call to `run_persistent` resets it, even a resumed one. If `version` were `ctx.turns_taken` directly, a conversation resumed after 3 persisted turns would, on its very first new turn, try to save `version = 1` — *lower* than the `3` already on record. `SnapshotStore`'s own version check only rejects `0` (`InvalidVersion`, `snapshot_error.rs:8-15`); it does not reject non-monotonic writes, so this wouldn't error today against `InMemorySnapshotStore` — but it is still the wrong number, and a future durable backend enforcing strict monotonicity (a standard optimistic-concurrency guard) would legitimately reject it. Carrying `base_version` (the version of whatever was loaded at the start of `run_persistent`, `0` for a fresh conversation) forward fixes this at the source.

**Where the save happens, precisely.** After `self.inner.execute(...)` returns — i.e., after the assistant's completion is appended *and every tool call that completion requested has been executed and its result appended* (`conversation_turn_step.rs:69-86` runs to completion before `execute` returns `Ok(())`). This is a turn boundary, not a sub-turn one; see "What v1 restores" below for exactly what that means for mid-flight tool calls. The already-terminated no-op path (`conversation_turn_step.rs:33-35`) still triggers a save of unchanged state on every remaining slack step — redundant I/O, not a correctness issue, and a candidate for a cheap "skip if `ctx.terminated` was already true before this step ran" optimization, left for the implementation, not designed here.

### 4. `PersistentBoundedConversationLoop` — load-then-run (new, `edge-llm-agent/core/conversation/`)

```rust
pub(crate) struct PersistentBoundedConversationLoop<Store> {
    pub(crate) agent: Arc<dyn Agent>,
    pub(crate) store: Arc<Store>,
}

#[async_trait]
impl<Store> PersistentConversationLoop for PersistentBoundedConversationLoop<Store>
where
    Store: SnapshotStore<AggregateId = String, Snap = ConversationSnapshot> + Send + Sync,
{
    async fn run_persistent(&self, req: PersistentConversationRunRequest) -> Result<ConversationRunResponse, AgentError> {
        let loaded = self.store
            .load(SnapshotLoadRequest { id: &req.conversation_id })
            .await
            .map_err(|e| AgentError::PersistenceFailed(e.to_string()))?
            .snapshot;

        let (mut messages, base_version) = match loaded {
            Some(snap) => (snap.messages, snap.version),
            None => (Vec::new(), 0),
        };
        messages.extend(req.new_messages);

        // Build the same bounded pipeline BoundedConversationLoop::run does, but with
        // every turn step wrapped in PersistingConversationTurnStep(conversation_id, base_version).
        // ... (mechanical — reuses BoundedConversationLoop's step-construction, wraps the
        // Arc<dyn Step> it already builds, does not duplicate DefaultConversationTurnStep)
    }
}
```

### 5. Durable backend: an external plugin, not an in-tree implementation

Following ADR-071's precedent exactly (`ADR-071-persistent-usage-ledger.md`, Part 1): `edge-llm-agent` gains **no database dependency**. `InMemorySnapshotStore<ConversationSnapshot>` remains usable for development/tests (already generic, already exists, zero new code), but a real deployment needs a durable `SnapshotStore` — a new standalone plugin, `edge-plugin-llm-conversation-store`, depending on `edge-llm-agent` (for `ConversationSnapshot`) and `edge-data-egress-database` (`data/egress/database/scm/`, the existing, already-audited, `sqlx`-free generic DB port — `DatabaseConfig`/`connect_and_migrate`/`DbPool`), the same shape as `edge-plugin-llm-usage-ledger`:

```
edge-plugin-llm-conversation-store/           (new standalone repo)
└── scm/main/src/
    ├── core/sql_conversation_store.rs        (SqlSnapshotStore: impl SnapshotStore<AggregateId=String, Snap=ConversationSnapshot>)
    ├── spi/migrations/V1__create_conversation_snapshots.sql   (conversation_id PK, version, messages_json, terminated, updated_at_ms)
    └── saf/sql_conversation_store_svc.rs      (sql_conversation_store(pool: DbPool) -> impl SnapshotStore<...>)
```

`messages` persists as a JSON column (`serde_json::to_string(&snap.messages)`), since `Message` is already `Serialize`/`Deserialize` end-to-end. Any pool/query failure maps to `SnapshotError::Unavailable(String)` — the variant already exists (`snapshot_error.rs:16-18`), no new error type needed on the `edge-domain-snapshot` side.

### 6. One new `AgentError` variant

```rust
/// Saving or loading a conversation snapshot via a `SnapshotStore` backend failed.
#[error("conversation persistence failed: {0}")]
PersistenceFailed(String),
```

Grep of every `AgentError` match site in `edge-llm-agent` (`agents/tests/agent_error_svc_int_test.rs:69,79,89`, `core/conversation/bounded_conversation_loop.rs:62`) confirms none matches exhaustively without a wildcard arm — adding this variant is safe within this monorepo today, though it is technically additive-breaking for any external, exhaustive-matching consumer of a non-`#[non_exhaustive]` enum; noted, not hidden.

### What "restored" means for v1 — stated precisely, not implied

- **Restoration granularity is the turn boundary, and only the turn boundary.** A snapshot reflects conversation state exactly as it stood after the last `DefaultConversationTurnStep::execute` call that ran to completion — assistant message appended, and *every* tool call that turn's completion requested already executed and its result appended (`conversation_turn_step.rs:58-86`). On restart, `run_persistent` resumes by handing the loaded `messages` back into a fresh bounded loop and issuing a **new** completion request from there — it does not and cannot replay "turn N+1 was half-done."
- **What is explicitly NOT restorable in v1: mid-tool-call state.** If the process crashes after `ctx.messages.push(assistant message with tool_calls)` (`conversation_turn_step.rs:58`) but partway through the sequential `for call in &completion.tool_calls` loop (lines 69-86) — say, 2 of 3 requested tool calls have executed — that entire turn is lost, not partially recovered. On restart, the loop resumes from the *previous* turn's persisted state and asks the provider for a fresh completion, with no memory that a tool call was ever requested, let alone that some of them already ran.
- **The real risk this creates is side effects, not corrupted history.** If one of those already-executed (but unpersisted) tool calls had a real effect — a webhook already sent, a process already spawned, a write already committed (the exact `CapabilityFlags` ADR-046 defines: `WEBHOOK_SEND`, `PROCESS_SPAWN`, `DATABASE_WRITE`) — that effect already happened and will not be reflected anywhere in the restored conversation, and nothing compensates for it. This ADR does not solve tool-call idempotency or at-least-once/exactly-once execution semantics; it only guarantees the *message history* recovers cleanly to the last completed turn.
- **A conversation is never "done" in a way that blocks resumption.** `ConversationState::terminated` is a per-run flag ("this bounded run's turn sequence reached a terminal finish reason"), not a permanent conversation-closed marker — `ConversationSnapshot.terminated` is carried through for diagnostic/UI purposes (e.g. "was the last turn a final answer"), but `run_persistent` will happily accept `new_messages` (e.g. the user's next reply) against an already-`terminated` snapshot and start a fresh bounded run on top of it, exactly the same way `ConversationRunRequest.messages` already supports continuing a conversation across separate `run()` calls today.

## What this ADR explicitly does NOT solve

- **Mid-tool-call / mid-turn resume.** Stated above — v1's unit of durability is one fully-completed turn, not one tool call.
- **Tool-call idempotency or exactly-once execution.** A tool call that already fired before an unpersisted crash is not tracked, deduplicated, or retried specially on resume — see the side-effects risk above. Would need per-tool-call durable state and idempotency keys, neither of which any `Skill`/`Handler` in `edge-llm-agent` has today.
- **The durable backend itself.** This ADR designs `ConversationSnapshot`/`PersistentConversationLoop`/`PersistingConversationTurnStep` against the `SnapshotStore` port and names the plugin shape (`edge-plugin-llm-conversation-store`); it does not build the plugin. `InMemorySnapshotStore<ConversationSnapshot>` is the only in-tree implementation, and — per its own doc comment — does not survive a restart either. Standing up the plugin is tracked separately, exactly as ADR-071 tracked `edge-plugin-llm-usage-ledger` separately from the `UsageLedger` port design.
- **Multi-instance / distributed coordination.** Two process instances resuming the same `conversation_id` concurrently (e.g. behind a load balancer with no sticky sessions) can race on `save`/`load` — `SnapshotStore` has no locking or leader-election primitive, and this ADR does not add one.
- **Snapshot retention/garbage collection.** Old conversation snapshots accumulate forever; no TTL, archival, or deletion policy is designed here.
- **Schema/version migration of `ConversationSnapshot` itself across deploys.** If a future field is added to `ConversationSnapshot` or `Message`, reading an older persisted row is not addressed.
- **Encryption or PII handling of stored conversation content.** Conversation `messages` may contain arbitrary user content; this ADR persists it as plain JSON with no redaction/encryption story.
- **Wiring `PersistentConversationLoop` into `edge-llm-runtime` (ADR-045) or `AgentManager`.** `AgentManager::conversation_loop` (`api/traits/agent_manager.rs:60-69`) still returns a plain `BoundedConversationLoop`; adding a `conversation_loop_persistent` default method (or similar) that requires a `Store` is a composition-root wiring decision, not designed here.

## Consequences

**What this enables**
- A conversation can survive a process restart for the first time, closing the exact gap ADR-032 named and deferred (`ADR-032-llm-agent.md:379`) and that ADR-037 never actually picked up.
- Reuses a port that already exists, is already tested, and already has an in-memory reference impl — zero new domain contracts invented for "save/load state by id," consistent with the reuse discipline ADR-045/046/071 already established for this audit series.
- The durable-backend-as-external-plugin pattern (ADR-071) gets a second real consumer, further validating it generalizes beyond usage/billing data to arbitrary versioned domain state.
- `Message`'s existing `Serialize`/`Deserialize` derives mean the payload needs zero type changes to become persistence-ready.

**What this requires**
- New dependency edge: `edge-llm-agent` → `edge-domain-snapshot` (currently absent, `Cargo.toml:20-32`).
- New types/traits inside `edge-llm-agent`: `ConversationSnapshot` (api/types), `PersistentConversationRunRequest` (api/types), `PersistentConversationLoop` (api/traits), `PersistingConversationTurnStep` + `PersistentBoundedConversationLoop` (core/conversation), one new `AgentError::PersistenceFailed` variant.
- A new standalone plugin repo, `edge-plugin-llm-conversation-store`, for any deployment that needs to actually survive a restart (not needed for dev/test, which can keep `InMemorySnapshotStore`).
- A composition-root decision (not made here) about how `PersistentConversationLoop` is constructed and exposed — e.g., a new `AgentManager` method, or a decorator applied at `edge-llm-runtime` registration time.
- No changes to `edge_domain_snapshot`'s `Snapshot`/`SnapshotStore`/`SnapshotError`, no changes to `ConversationLoop`/`BoundedConversationLoop`/`ConversationState`/`DefaultConversationTurnStep`, no changes to `ConversationRunRequest`/`ConversationRunResponse`.

## Alternatives Considered

**Invent a bespoke `ConversationStore`/`SessionStore` trait from scratch**
Rejected. Would duplicate `SnapshotStore`'s `save`/`load`-by-id shape field-for-field, plus its own error type, for no behavioral gain — exactly the anti-pattern ADR-046 already rejected when it declined to build a bespoke `GovernancePolicy` instead of reusing `edge_domain_policy::Policy`. The reuse check above found `Snapshot`/`SnapshotStore` fits with only the version-monotonicity detail (Part 3) needing care, not a structural blocker.

**Reuse `domain-saga`'s `Saga`/`SagaStore` instead**
Rejected — see the reuse check above. Requires a `DomainEvent`/`Command` vocabulary this problem has no other use for, and `SagaStore` has no `save` method at all (mutation is via a borrowed live instance, not a value handed to the store). Wrong shape, not a close second choice.

**Embed a durable backend directly in `edge-llm-agent/spi/` (e.g. `rusqlite`, `kgraph`-style)**
Rejected, for the identical reason ADR-071 rejected it for `edge-llm-usage`: it would give a `domain/scm/domain/llm/*` crate a concrete database dependency and risk the embedded version being mistaken for "the" backend and never replaced. The external-plugin-over-`edge-data-egress-database` pattern is proven (ADR-071) and reused verbatim here.

**Save after every message instead of every turn**
Rejected for v1. `edge-pipeline`'s `Step`/`Pipeline` contract only exposes a boundary at `Step::execute` (one call per turn); reaching inside a turn to save after each of the sequential tool-call executions (`conversation_turn_step.rs:69-86`) would mean forking or extending `DefaultConversationTurnStep` itself rather than wrapping it — a materially bigger change for a granularity improvement whose main benefit (surviving a crash between tool calls within one turn) is explicitly out of scope for v1 anyway (see "What v1 restores").

**Snapshot every turn unconditionally, including redundant post-termination no-ops, with no skip check**
Accepted as the v1 default (see Part 3) rather than adding a "was already terminated" guard now — it is a minor I/O inefficiency, not a correctness risk, and premature to optimize before a real durable backend exists to measure it against.

## Tracking

- New types/traits in `edge-llm-agent`: `ConversationSnapshot`, `PersistentConversationRunRequest`, `PersistentConversationLoop`, `PersistingConversationTurnStep`, `PersistentBoundedConversationLoop`, `AgentError::PersistenceFailed`.
- New dependency: `edge-llm-agent` → `edge-domain-snapshot`.
- New standalone plugin repo: `edge-plugin-llm-conversation-store` (`SqlSnapshotStore` over `edge-data-egress-database`, one migration file) — mirrors `edge-plugin-llm-usage-ledger` (ADR-071).
- Follow-up (not designed here): composition-root wiring — how `PersistentConversationLoop` is constructed/exposed from `AgentManager` or `edge-llm-runtime` (ADR-045).
- Follow-up (explicitly out of scope, named not hidden): tool-call idempotency / exactly-once semantics for mid-turn crash recovery.
- Follow-up (explicitly out of scope): multi-instance concurrent-resume coordination, snapshot retention/GC, `ConversationSnapshot` schema migration, encryption/PII handling of persisted message content.
- Not blocking this ADR: the minor "skip redundant post-termination save" optimization named in Alternatives Considered.
