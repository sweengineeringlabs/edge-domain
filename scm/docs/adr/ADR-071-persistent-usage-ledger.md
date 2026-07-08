# ADR-071: Persistent `UsageLedger` Backend + `SpendLimitPolicy`

**Status:** Proposed
**Date:** 2026-07-08
**Author:** Senior Agentic Engine Engineer
**Relates to:** ADR-042 (plugin boundary), ADR-046 (`edge-llm-tools` — origin of the "cross-cutting rule = ordinary `Policy` impl" pattern `SpendLimitPolicy` follows), ADR-054 (Cost/Usage Tracking, sibling — the exact `UsageLedger` contract this ADR implements, reused verbatim, not redesigned), ADR-057 (LLM End-to-End Dataflow Trace — this ADR's enforcement point is an *addition* to that trace, tracked as a follow-up below, not silently assumed)
**GitHub Issues:** TBD

---

## Context

ADR-054 built `edge-llm-usage` with two ports of deliberately different maturity: `UsageRecorder` (built, `ObserverContext`-metric-only) and `UsageLedger` (contract only). Its own Tracking section named exactly the two gaps this ADR closes, verbatim:

> "Follow-up (separate ADR/issue, explicitly out of scope here): a real persistent `UsageLedger` backend, per ADR-042's plugin boundary"
> "Follow-up (not designed here — needs a real ledger first): `SpendLimitPolicy` via `edge_domain_policy::Policy`, composed the same way ADR-046 composed `CapabilityGatePolicy`/`RiskCeilingPolicy`"

Per explicit user direction, nothing in this audit series stays merely "deferred" — every gap an ADR names as a follow-up gets its own full ADR (the same move ADR-072 already made for ADR-053's "real moderation backend" deferral). This is that ADR for ADR-054's ledger/spend-limit deferral.

Like `edge-llm-guardrails` was for ADR-072 at the time it was written, `edge-llm-usage` (ADR-054) is itself still **Proposed, not yet built** — this ADR designs against its contract as specified, the same forward-reference relationship ADR-072 already had to ADR-053.

### The exact port this ADR must satisfy — reused verbatim, not re-derived

Per ADR-054's own text (`domain/scm/docs/adr/ADR-054-cost-usage-tracking.md`):

```rust
pub trait UsageLedger: Send + Sync {
    /// Persist one usage observation. Real backends aggregate; `NoopUsageLedger` discards.
    fn append(&self, req: UsageAppendRequest) -> Result<UsageAppendResponse, UsageError>;
    /// Read back an aggregate — `NoopUsageLedger` always answers zero.
    fn totals(&self, req: UsageTotalsRequest) -> Result<UsageTotalsResponse, UsageError>;
}

pub struct UsageAppendRequest  { pub usage: TokenUsage, pub model: String, pub tenant_id: Option<String>, pub recorded_at_ms: u64 }
pub struct UsageAppendResponse; // ack marker
pub struct UsageTotalsRequest   { pub tenant_id: Option<String>, pub model: Option<String> }
pub struct UsageTotalsResponse  { pub total_tokens: u64, pub call_count: u64 }
pub enum UsageError { RecorderUnavailable(String), LedgerUnavailable(String) }
```

Nothing about this shape changes here. `LedgerUnavailable(String)` already exists in `UsageError` — a real, fallible backend has a variant to report through without this ADR adding a new error type (see Decision, Part 2).

### Where should the real backend live? — evidence gathered before deciding

Three existing patterns in `edge/` bear directly on this, and the honest answer is "reuse the established convention," not invent a fourth:

1. **`edge-domain`'s own `Repository<T, Id>`/`QueryableRepository<T, Id>`** (`domain/scm/domain-repository/`) is contract-only by explicit, stated rule: its trait doc says implementations *"live in infrastructure crates — never in `edge-domain`."* The reference consumer, `justobserv` (per `data/egress/database/docs/3-architecture/adr/ADR-001-egress-database-domain-boundary.md`), depends on **both** the domain crate (for `Repository`) and a separate datasource crate (for a real pool), and writes `impl Repository<MetricRecord, MetricId> for PgMetricRepo` itself — the domain crate never gains a database dependency.
2. **`edge-data-egress-database`** (`data/egress/database/scm/`, package `edge-data-egress-database`, formerly `swe-edge-egress-database-migration`) is the real, already-implemented, already-`cargo-audit`-clean generic database port in this workspace: `DatabaseConfig`, `DriverKind`, and `connect_and_migrate(&DatabaseConfig) -> Result<DbPool, MigrationError>`, returning a configured, migrated `deadpool` pool over `refinery` drivers, behind `postgres`/`sqlite` feature flags. `sqlx` is hard-banned workspace-wide (RUSTSEC-2023-0071); this crate is the reason nothing needs to reach for it again.
3. **`edge-runtime-db`** (`edge-runtime-*` family, ADR-047, top-level `docs/3-architecture/adr/ADR-047-edge-runtime-primitive-family.md`) is the *proposed*, contracts-only mirror of `edge-domain-*` for runtime concerns — `DatabaseGateway`/`TransactionGateway` — explicitly scheduled as **Phase 3, not started**, and stated to eventually "lift contracts out of egress crates" including `edge-data-egress-database`. It does not exist today; nothing in `edge/` can depend on it yet.

A fourth, tempting shortcut was checked and rejected: **`kgraph`** (`kgraph/scm/main/src/spi/sqlite.rs`) embeds `rusqlite` directly inside its own `spi/`, as a private, in-process store. This is real precedent for an *embedded*-DB dependency existing somewhere in `edge/` — but it is not a precedent for *this* crate's shape. `kgraph` is a self-contained leaf application that owns its persistence privately and exposes no swappable domain port to a consumer; `edge-llm-usage` is a `domain/scm/domain/llm/*` crate exporting a `UsageLedger` trait *specifically so a real backend can be swapped in without touching call sites* (`StdProvider`, `SpendLimitPolicy`) — precisely the polymorphic-consumer shape `Repository<T, Id>` has, not `kgraph`'s single-owner shape. Copying `kgraph`'s pattern into `edge-llm-usage/spi/` would put a concrete `rusqlite`/`sled` dependency inside a domain-tier crate, which is exactly the anti-pattern ADR-054's own Alternatives Considered already rejected ("shipping even an in-memory version risks it being mistaken for 'the' backend and never replaced") — just with a real DB instead of a `HashMap`. The risk is identical; only the storage technology changed.

## Decision

### Part 1 — the real backend is an external plugin, `edge-plugin-llm-usage-ledger`, depending on `edge-data-egress-database`

`edge-llm-usage` gains **no new dependency**. A new standalone plugin repo, `edge-plugin-llm-usage-ledger`, implements `edge_llm_usage::UsageLedger` by holding a `DbPool` obtained from `edge-data-egress-database::connect_and_migrate`, following the exact `edge-domain` ⇄ plugin dependency inversion ADR-042 already established for `edge-plugin-llm-provider`:

```
edge-plugin-llm-usage-ledger (plugin)
  └─ depends on → edge-llm-usage           (UsageLedger trait + Usage*Request/Response types, ADR-054)
  └─ depends on → edge-data-egress-database (DatabaseConfig, connect_and_migrate, DbPool)

edge-llm-usage (domain, framework)
  └─ MUST NOT depend on → edge-plugin-llm-usage-ledger
```

This is the `justobserv`/`Repository` pattern (#1 above) applied at the crate-dependency level, not the `edge-runtime-db` mirror pattern (#3) — because #3 doesn't exist to depend on yet. Once `edge-runtime-db` ships (ADR-047 Phase 3), this plugin can be re-pointed at it instead of `edge-data-egress-database` directly, the same migration ADR-047 §5 already names for its other Phase-3 consumers ("lift contracts out of egress crates"). That re-pointing is future work, tracked below, not performed here.

```
domain/scm/domain/llm/usage/          (edge-llm-usage — NO changes to its dependency graph)
├── api/   UsageRecorder, UsageLedger, Usage*Request/Response, UsageError   (unchanged, ADR-054)
├── core/  DefaultUsageRecorder                                            (unchanged, ADR-054)
└── spi/   NoopUsageLedger                                                 (unchanged, ADR-054 — stays the only in-tree impl, forever)

edge-plugin-llm-usage-ledger/                        (new standalone repo, mirrors edge-plugin-llm-moderation-openai's scm/ layout)
└── scm/
    └── main/src/
        ├── api/
        │   └── types/ledger_backend_config.rs        (LedgerBackendConfig: DatabaseConfig passthrough +
        │                                                table name override, for multi-tenant schema isolation)
        ├── core/
        │   └── sql_usage_ledger.rs                    (SqlUsageLedger: impl UsageLedger — one struct,
        │                                                works over either DbPool variant; see below)
        ├── spi/
        │   └── migrations/V1__create_usage_ledger.sql (tenant_id, model, total_tokens, call_count, updated_at_ms)
        └── saf/
            └── sql_usage_ledger_svc.rs                 (sql_usage_ledger(pool: DbPool) -> impl UsageLedger)
```

`SqlUsageLedger::append` upserts `(tenant_id, model)` → `total_tokens += req.usage.total_tokens, call_count += 1`; `SqlUsageLedger::totals` reads the same row (or aggregates across `model` when `req.model` is `None`). Because `DbPool` (from `edge-data-egress-database`) already covers both `postgres` and `sqlite` behind one enum, one `SqlUsageLedger` struct serves both a production Postgres deployment and a local/dev SQLite one — no separate `PostgresUsageLedger`/`SqliteUsageLedger` split is needed; the `DriverKind` distinction is already resolved one layer down, at `connect_and_migrate` time. Any pool/query failure (connection refused, pool exhausted, malformed row) maps to `UsageError::LedgerUnavailable(String)` — the exact variant ADR-054 already shipped for this, no new error type required.

### Part 2 — `SpendLimitPolicy`

`SpendLimitPolicy` reuses `UsageTotalsRequest` *directly* as its `Policy::Input` — no new request type. The ceiling it checks against is not part of `UsageTotalsRequest` (that type is a pure read-key: tenant + optional model), so it lives on the policy struct itself, via a new, small, purpose-built config type:

```rust
// edge-llm-usage/api/types/spend_limit_config.rs — NEW
pub struct SpendLimitConfig {
    pub default_ceiling_tokens: u64,
    pub tenant_ceiling_tokens: std::collections::HashMap<String, u64>,
    pub fail_mode: SpendLimitFailMode,
}
pub enum SpendLimitFailMode { FailOpen, FailClosed }

// edge-llm-usage/core/spend_limit_policy.rs — NEW
pub(crate) struct SpendLimitPolicy {
    ledger: Arc<dyn UsageLedger>,
    config: SpendLimitConfig,
}

impl Policy for SpendLimitPolicy {
    type Input = UsageTotalsRequest;

    fn name(&self, _req: PolicyNameRequest) -> Result<PolicyNameResponse, PolicyError> {
        Ok(PolicyNameResponse { name: "spend-limit" })
    }

    fn evaluate(&self, req: PolicyEvaluateRequest<'_, UsageTotalsRequest>) -> Result<(), PolicyError> {
        let lookup = UsageTotalsRequest {
            tenant_id: req.input.tenant_id.clone(),
            model: req.input.model.clone(),
        };
        let ceiling = req.input.tenant_id.as_deref()
            .and_then(|t| self.config.tenant_ceiling_tokens.get(t).copied())
            .unwrap_or(self.config.default_ceiling_tokens);

        match self.ledger.totals(lookup) {
            Ok(totals) if totals.total_tokens >= ceiling =>
                Err(PolicyError::new("spend-limit", format!(
                    "tenant {:?} used {} tokens, ceiling is {}",
                    req.input.tenant_id, totals.total_tokens, ceiling,
                ))),
            Ok(_) => Ok(()),
            Err(_) if self.config.fail_mode == SpendLimitFailMode::FailOpen => Ok(()),
            Err(e) => Err(PolicyError::new("spend-limit", format!("ledger unavailable: {e}"))),
        }
    }
}
```

**Why a config type, not a `ProviderConfig` field.** `ProviderConfig` (per ADR-042's `saf/provider_svc.rs`) is a per-vendor completion-execution config — model, API key, `max_tokens`, `system_prompt` — owned by the *provider plugin*, not by `edge-llm-usage`. A per-tenant ceiling is a usage-governance concern, the same category as ADR-046's `CapabilityFlags`/`RiskLevel`, which also got their own types rather than riding on `ProviderConfig`. Putting it there would also force every non-usage-aware `Provider` implementation to carry a field it has no reason to know about. `SpendLimitConfig` sits beside `UsageLedger`/`SpendLimitPolicy` in the one crate that actually consumes it.

**Why `fail_mode`, reusing ADR-072's exact rationale.** Unlike ADR-046's `CapabilityGatePolicy`/`RiskCeilingPolicy` (pure local computation, cannot itself fail), `SpendLimitPolicy` consults a `UsageLedger` that — once backed by `SqlUsageLedger` (Part 1) — is a real, fallible I/O dependency. ADR-072 already faced this exact shape of problem (a real backend consulted by a `Policy` that can fail independently of the enforcement question) and made the choice explicit rather than hardcoded; this ADR reuses that reasoning rather than re-deriving it: hardcoded fail-closed turns a ledger outage into a 100%-of-completions denial; hardcoded fail-open silently disables spend enforcement during exactly the incidents where runaway spend is most likely. `SpendLimitConfig.fail_mode` makes the choice a required, explicit, deployment-time value — no default is silently assumed.

**Naming honesty callout.** "Spend" here means *token count*, not currency. ADR-054 explicitly named "no token→currency conversion" as out of scope, and that has not changed — `SpendLimitPolicy` enforces a raw-token ceiling per tenant, which is a proxy for spend, not spend itself. The name is carried over verbatim from ADR-054's own tracking-item text (`SpendLimitPolicy`), not renamed here, but this ADR states the gap explicitly rather than let the name imply a currency budget that does not exist. Once a pricing catalog exists (a distinct, still-unscoped follow-on), the ceiling could become currency-denominated without changing `Policy`'s shape — only `SpendLimitConfig`'s field types and the comparison in `evaluate`.

### Enforcement point — before the call, inside `StdProvider::complete()`'s existing `CompositePolicy` field

Per ADR-057's own trace, `StdProvider::complete()` already runs a `CompositePolicy` field before `self.completer.complete(...)` — today holding `ContextWindowPolicy` (ADR-050). `SpendLimitPolicy` is added to that same composite, immediately after it:

```
Provider::complete()
  a. retrieval already composed into ContextManager before render (ADR-052 — upstream)
  b. ContextWindowPolicy check (ADR-050)   — cheapest: integer comparison, no I/O
  c. SpendLimitPolicy check (THIS ADR)     — a ledger read: local/DB round-trip, but never a vendor network call
  d. self.completer.complete(...)          — GuardrailPhase::PreCall / real vendor call / PostCall (ADR-053/048)
  e. UsageRecorder::record(...)            — per attempt (ADR-057 #4), unchanged
```

**Before the call is the only sensible position, and here is why, explicitly:** the entire purpose of a spend ceiling is to avoid *paying for* a call that's already over budget. Checking it after `self.completer.complete(...)` returns (the same point `UsageRecorder::record` already runs) would let exactly one more over-ceiling call through on every single evaluation — the vendor charge already happened by the time the check could fire. A pre-call gate is the only position that can actually deny before cost is incurred.

**Ordering relative to `ContextWindowPolicy` and the guardrail's `PreCall` phase, using ADR-057's own stated rationale for the analogous case:** ADR-057 already ordered `ContextWindowPolicy` before `GuardrailPhase::PreCall` because it is "the cheaper failure to detect... and independent of content." `SpendLimitPolicy` is placed after `ContextWindowPolicy` (a pure in-memory integer comparison — strictly cheaper) but before the guardrail's `PreCall` content flattening — a ledger read is local/DB-round-trip cost, still categorically cheaper than flattening + evaluating message content, and, like `ContextWindowPolicy`, entirely content-independent. So the same "cheapest and content-independent checks run first" rule ADR-057 already established places `SpendLimitPolicy` at position (c), not before `ContextWindowPolicy` and not after the guardrail.

**Retry interaction — checked per attempt, same reasoning ADR-057 already used for usage recording (#4).** `RetryHandler` wraps outside `StdProvider` (per ADR-057's resolved ordering, `GovernedHandler(RetryHandler(inner))`); each retry attempt is a fresh `StdProvider::complete()` call, so `SpendLimitPolicy` re-evaluates on every attempt, not just the first. This is the correct behavior, not an oversight to fix later: usage recorded by a prior attempt (including a failed one, per ADR-057 #4) is already reflected in the ledger by the time a retry re-checks, so a request that crosses the ceiling mid-retry-loop (its own prior attempts, or a concurrent tenant request) is correctly denied on the next attempt rather than let through on stale information.

**Update (post-review): this has now landed in ADR-057 itself.** At the time this ADR was drafted, ADR-057's trace diagram and step-`b` prose didn't mention `SpendLimitPolicy` (it didn't exist yet), and this section originally named the addition as a tracked-but-unperformed follow-up. `ADR-057-llm-end-to-end-dataflow-trace.md` has since been amended (a header note, an updated trace diagram with step "c. SpendLimitPolicy check (ADR-071)", and matching Tracking-section language) — the addition described below is done, not pending.

## What this ADR explicitly does NOT solve

- **Token→currency conversion / a pricing catalog.** Still not built anywhere in `edge/`, still ADR-054's own named gap. `SpendLimitPolicy` ceilings are raw token counts, not dollars — see the naming-honesty callout above.
- **Migrating `edge-plugin-llm-usage-ledger` from `edge-data-egress-database` to `edge-runtime-db`.** `edge-runtime-db` (ADR-047 Phase 3) is proposed, not built. Once it exists, re-pointing this plugin at it is a plugin-internal change (swap which crate provides the pool), not a `UsageLedger`/`SpendLimitPolicy` redesign — named as a tracked follow-up, not performed here.
- **Multi-region or sharded ledger aggregation.** `SqlUsageLedger` assumes one pool, one logical ledger table. A globally-distributed deployment aggregating across regions is out of scope.
- **Wiring `SpendLimitPolicy` (or `SqlUsageLedger`) into a live `CompositePolicy` in any composition root.** This ADR produces the policy and the plugin; registering them into `StdProvider`'s actual `CompositePolicy` field in `swe-edge-bootstrap`/`edge-llm-runtime` is separate follow-on work — the same carve-out ADR-053/ADR-072 already used for `GuardrailedCompleter`/`OpenAiModerationPolicy`.
- **Retry/backoff on ledger DB errors specifically.** `SpendLimitConfig.fail_mode` governs what happens on a single failed `totals()` call; nothing retries a transient DB error before falling into fail-open/fail-closed. The same inert-retry gap ADR-048/072 already named for their own vendor calls recurs here.
- **Editing ADR-057's own trace/ordering document.** Named as a required addition above and tracked below, not performed as a silent edit to a different, already-Proposed ADR.
- **Dynamic, runtime-reloadable ceilings.** `SpendLimitConfig` is constructed once at composition-root wiring time; changing a tenant's ceiling requires a redeploy/restart with new config, not a live API. A config-reload mechanism is out of scope.
- **A currency-denominated, cross-tenant global spend cap** (as opposed to a per-tenant ceiling) — not designed here; `tenant_ceiling_tokens` is keyed per tenant only.

## Consequences

**What this enables**
- A real, swappable `UsageLedger` implementation exists for the first time, without `edge-llm-usage` gaining a single new dependency — `NoopUsageLedger` remains the crate's only in-tree impl, exactly as ADR-054 intended.
- Actual spend/budget enforcement, closing ADR-054's second named gap, composed into `StdProvider`'s existing `CompositePolicy` field with zero new composition machinery (same `Policy`/`CompositePolicy` port ADR-046 already proved out).
- A second concrete data point (after `edge-plugin-llm-moderation-openai`, ADR-072) validating that a real, fallible backend behind a `Policy`-consulted port composes the same way regardless of whether the backend is a vendor HTTP API or a local database.
- `edge-data-egress-database` gets a second real consumer beyond its original `justobserv` motivation, without either crate depending on the other — confirms the pattern generalizes.

**What this requires**
- New crate additions inside `edge-llm-usage` (once ADR-054 itself is built): `SpendLimitConfig`/`SpendLimitFailMode` (api/types), `SpendLimitPolicy` (core/), a `saf/spend_limit_policy_svc.rs` factory.
- New standalone plugin repo `edge-plugin-llm-usage-ledger`, depending on `edge-llm-usage` and `edge-data-egress-database`, shipping `SqlUsageLedger` + one migration file.
- A composition root that constructs `SqlUsageLedger` (or keeps `NoopUsageLedger` for environments with no database configured), constructs `SpendLimitPolicy` with it, and registers the policy into `StdProvider`'s `CompositePolicy` immediately after `ContextWindowPolicy` — none of this is automatic.
- No changes to `edge_domain_policy`'s `Policy`/`PolicyError`/`CompositePolicy`, no changes to `UsageLedger`/`UsageRecorder`/`UsageError`/`Usage*Request`/`Usage*Response` as specified in ADR-054 — this ADR is purely additive at the seam ADR-054 already left open for it.
- A tracked addendum to ADR-057's trace (see Tracking) — not a code change, a documentation debt this ADR creates and names rather than leaves silent.

## Alternatives Considered

**Ship the real backend inside `edge-llm-usage/spi/` (embedded `rusqlite`/`sled`, `kgraph`-style)**
Rejected. `edge-llm-usage` is a `domain/scm/domain/llm/*` crate; giving it a concrete database dependency repeats, with a real DB instead of a `HashMap`, the exact risk ADR-054's own Alternatives Considered already rejected for an in-memory ledger — "shipping even [a placeholder] backend risks it being mistaken for 'the' backend and never replaced." `kgraph`'s embedded sqlite is real precedent for an embedded DB existing *somewhere* in `edge/`, but it is a self-contained leaf crate with no swappable domain port, not a `domain/`-tier crate exporting a trait a plugin is meant to implement — the wrong shape to copy here.

**Depend on `edge-runtime-db` now instead of `edge-data-egress-database`**
Rejected for this ADR — `edge-runtime-db` (ADR-047 Phase 3) does not exist yet; nothing can depend on an unbuilt crate. `edge-data-egress-database` is the real, already-implemented, already-audited generic DB port in `edge/` today, and is exactly the crate ADR-047 itself names as the eventual source `edge-runtime-db`'s contracts will be lifted out of — depending on it now and migrating later (tracked above) is more honest than pretending the Phase-3 crate already exists.

**Widen `UsageError` with a new `SpendExceeded` variant, or invent a `SpendLimitError` type**
Rejected, for the same reason ADR-072 rejected widening `PolicyError` for moderation categories: `Policy::evaluate` is fixed to `Result<(), PolicyError>` by the trait itself — a distinct error type cannot ride in the same `CompositePolicy<UsageTotalsRequest>` `Vec` without an adapter that re-erases it back to `PolicyError` anyway, buying nothing. `UsageError::LedgerUnavailable(String)` already exists and already fits the one new failure mode (`totals()` I/O failure) `SqlUsageLedger` introduces; `PolicyError::new("spend-limit", ...)` already fits the enforcement outcome. No new error surface is needed in either direction.

**Put the ceiling on `ProviderConfig` instead of a new `SpendLimitConfig`**
Rejected. `ProviderConfig` is owned by the provider plugin (per-vendor completion settings); a per-tenant spend ceiling is a usage-governance concern with a different owner and a different lifecycle, the same distinction ADR-046 already drew between `ProviderConfig`-shaped concerns and `CapabilityFlags`/`RiskLevel`-shaped ones. Reusing `ProviderConfig` would also force every `Provider` implementation with no interest in spend limiting to carry the field.

**Enforce `SpendLimitPolicy` as a `Handler`-wrapping decorator (`GovernedHandler`-style) instead of a `StdProvider` `CompositePolicy` entry**
Rejected for consistency, not because the decorator shape is wrong in general (ADR-046 uses it correctly for tool governance, which needs `SecurityContext`/capabilities not available inside `Provider`). `ContextWindowPolicy` — the policy `SpendLimitPolicy` is ordered directly against — already lives inside `StdProvider`'s own `CompositePolicy` field per ADR-050/057, and `SpendLimitPolicy` needs exactly the same inputs (a request-scoped `tenant_id`, no `SecurityContext`-specific data). Placing it at a different layer than the policy it's explicitly ordered relative to would make the ordering claim in this ADR unverifiable by inspection — one policy would live in `StdProvider`, the other above `Handler`, with no single place the actual sequence is enforced.

**Enforce the ceiling after the call, alongside `UsageRecorder::record`**
Rejected — see "Enforcement point" above. This ordering would always let one additional over-ceiling call through per evaluation, since the vendor charge is already incurred by the time `UsageRecorder::record` runs; it defeats the stated purpose of a spend *limit*.

## Tracking

- New: `SpendLimitConfig`/`SpendLimitFailMode` (`edge-llm-usage/api/types/`), `SpendLimitPolicy` (`edge-llm-usage/core/`), `saf/spend_limit_policy_svc.rs` — blocked on ADR-054's `edge-llm-usage` actually existing first.
- New standalone plugin repo: `sweengineeringlabs/edge-plugin-llm-usage-ledger` (`SqlUsageLedger`, one migration file, depends on `edge-llm-usage` + `edge-data-egress-database`).
- Follow-up (separate ADR/issue, mirrors ADR-048/072's own tracking): wire `SpendLimitPolicy` into a live `CompositePolicy` on `StdProvider`, and wire `SqlUsageLedger` (vs. `NoopUsageLedger`) into a real composition root.
- **Done (post-review):** ADR-057 (`domain/scm/docs/adr/ADR-057-llm-end-to-end-dataflow-trace.md`) has been amended with the `SpendLimitPolicy` step in its trace diagram and step-list, between `ContextWindowPolicy` and the guardrail `PreCall` step, plus the retry-interaction note. No longer an open follow-up.
- Follow-up (not blocking, tracked once `edge-runtime-db` ships per ADR-047 Phase 3): re-point `edge-plugin-llm-usage-ledger` at `edge-runtime-db` instead of depending on `edge-data-egress-database` directly.
- Not blocking this ADR: retry/backoff on `SqlUsageLedger`'s own DB errors (same inert-gap pattern ADR-048/072 already named for their vendor calls).
- Still open, restated from ADR-054, unchanged by this ADR: token→currency conversion / a pricing catalog, which would let `SpendLimitConfig` become currency-denominated without a `Policy`-shape change.
