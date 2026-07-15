# edge-application Architecture

**Audience:** Developers and architects working in this repo, and any agent (human or AI)
picking up domain-crate work here.

This is the entry point for understanding how `edge-application`'s domain crates are structured
and how they connect — both to each other and to the wider `edge` platform. It synthesizes the
ADRs in `docs/adr/` and the verified dataflow in `docs/3-design/dataflow.md`; it does not
duplicate either, it points at them.

---

## What this repo is

`edge-application` is the domain/hexagon layer for the `edge` platform (per `edge`'s own
`docs/3-architecture/architecture.md`): a set of independent, SEA-compliant (`api/` → `core/` →
`saf/`) Rust crates, each declaring one port contract — `Handler`, `Service`, `Command`, `Query`,
`Event`, `Registry`, `Observer`, `Repository`, `Snapshot`, `Lifecycle`, `Policy`, `Validator`,
`Entity`, `Value Object`, `Saga`, `Projection` — plus an umbrella crate (`edge-domain`, package
`edge-application`) that re-exports whichever subset a consumer opts into via Cargo features.

Consumers outside this repo (`edge-proxy`, `edge-dispatcher`, `swe-edge-bootstrap`, `edge-llm-runtime`,
and others) depend on these crates as libraries and wire them into a live dispatch pipeline —
this repo itself contains no ingress/egress/transport code. See `docs/3-design/dataflow.md` for
exactly how far that wiring is confirmed to reach today.

---

## SEA layering

Every domain crate follows the same shape:

```
api/      — public contract: traits, dto/, vo/, errors/, entity/. No implementation.
core/     — concrete implementations. May depend on other domain crates' api/ (see
            no_foreign_type below for the exact rule).
saf/      — Service Abstraction Framework: the only public re-export surface. `_svc.rs` files
            (one `pub const X_SVC: &str` identity marker + `pub use crate::api::X` + optional
            factory) — a naming/discoverability convention, not a runtime lookup mechanism.
spi/      — extension points, where a crate has them.
```

**`no_foreign_type` (the rule most relevant to the ADRs below):** `api/` must never reference
another domain crate's concrete types in a type position. `core/` may — that's the layer where
cross-crate bridges live. This was the exact point ADR-004 got wrong before its 2026-07-15
amendment (see below) — it read `no_foreign_type` as "never import, anywhere," when the actual,
enforced rule is narrower and already satisfied by the real code.

---

## Governing ADRs

Every ADR in `docs/adr/` mirrors a governing decision made in the `edge` repo, scoped to what
this workspace owns. Status reflects this repo's own doc, not necessarily the upstream one's:

| ADR | Title | Status | Governs |
|---|---|---|---|
| [001](../adr/ADR-001-security-context-propagation.md) | Security Context Propagation | Accepted | `SecurityContext` shape referenced by `domain-handler` |
| [002](../adr/ADR-002-event-sourcing-pipeline.md) | Event Sourcing Pipeline | Accepted | `domain-event`, `EventStore`/`EventBus` |
| [003](../adr/ADR-003-repository-pattern.md) | Repository Pattern | Accepted | `domain-repository` |
| [004](../adr/ADR-004-edge-service-bridge.md) | edge-service — Service-to-Handler Bridge | Accepted, amended 2026-07-15 | `domain-service` ↔ `domain-handler` bridge — see `dataflow.md` §2 |
| [005](../adr/ADR-005-command-query-bus-stack.md) | CommandBus/QueryBus Middleware Stack | Accepted | `domain-command`, `domain-query`, `HandlerContext.commands` |
| [006](../adr/ADR-006-observability-domain-primitive.md) | Observability Domain Primitive | Implemented | `domain-observer`, `HandlerContext.observer` — see `dataflow.md` §3 |
| [043](../adr/ADR-043-llm-complete-domain-primitive.md) | LLM Complete Domain Primitive | Implemented | HTTP-level completion port (out of this repo's own domain-crate family; see note below) |
| [044](../adr/ADR-044-observability-llm-integration.md) | Observability↔LLM Integration | Implemented except L4 | Two injection seams between observability and LLM primitives |

**Note on ADR-043/044:** these reference LLM-domain concepts. Confirmed separately (2026-07-15,
exhaustive grep) that `edge-llm`'s own crates have zero Cargo-level dependency on
`edge-application` — these two ADRs record decisions made *about* LLM primitives that were
evaluated in this repo's planning process, not evidence of an actual code dependency in either
direction. Don't infer a live connection from their presence here.

**No ADR governs `domain-registry`.** Its generalization intent (unifying `HandlerRegistry`/
`ServiceRegistry`) is recorded upstream in `edge`'s `ADR-029`, not mirrored locally — see
`dataflow.md` §5 and issue #141.

---

## Confirmed dataflow

```mermaid
graph TB
    subgraph app["edge-application workspace"]
        Handler["domain-handler<br/><i>Handler, HandlerRegistry</i>"]
        Service["domain-service<br/><i>Service, ServiceRegistry</i>"]
        Observer["domain-observer<br/><i>Tracer, LogDrain, MetricRegistry</i>"]
        Command["domain-command<br/><i>CommandBus</i>"]
        Registry["domain-registry<br/><i>Registry&lt;V&gt;</i>"]
    end

    Consumer["edge-dispatcher / swe-edge-bootstrap<br/>(live dispatch, outside this repo)"]

    Service -->|"IntoHandler / RegistryBridge<br/>composition-time only,<br/>HandlerContext dropped — #140"| Handler
    Observer -->|"7 blanket impls + ObserverContextAdapter<br/>per-request, not enforced"| Handler
    Command -->|"HandlerContext.commands<br/>per-request, convention only"| Handler
    Handler -->|"InProcessHandlerRegistry<br/>wrapped, live"| Consumer
    Registry -.->|"no bridge exists — ADR-029 deferred, #141"| Handler
    Registry -.->|"no bridge exists"| Service

    style Registry stroke-dasharray: 5 5
```

`docs/3-design/dataflow.md` is the traced, cited reference for how the pieces above actually
connect at the code level — the live `HandlerRegistry` chain, the `Service`→`Handler` bridge and
its confirmed `HandlerContext`-dropping behavior, the `ObserverContext` blanket-impl bridge,
`CommandBus` injection, and `domain-registry::Registry<V>`'s confirmed lack of any bridge to the
other two registries. Read that document for citations; this document is the map, not the
evidence.

---

## Open design questions (not yet decided — tracked as issues, not ADRs)

These are real, current gaps identified while building the ADR/dataflow picture above. They are
deliberately **not** written as settled architecture — treat them as open until an ADR (or ADR
amendment) closes them:

- **[#139](https://github.com/sweengineeringlabs/edge-application/issues/139)** — `Handler`/`Service`'s
  `Request`/`Response` associated types are currently unconstrained (`Send + 'static` only).
  Proposes a shared `domain-base` crate with `Request`/`Response` marker traits, bound at the
  trait level, so a `Handler`/`Service` implementor can't satisfy the contract with an arbitrary
  type. Explicitly scoped to stay an in-repo crate (see #141 below for why that matters).
- **[#140](https://github.com/sweengineeringlabs/edge-application/issues/140)** — `HandlerContext`
  (`security`, `commands`, `observer`) is silently dropped at the exact point a bridged `Service`
  is invoked (`DefaultServiceHandler::execute` forwards only `req.req`, never `req.ctx`). Not
  necessarily a bug — `Service`'s trait never promised context — but currently undecided whether
  that's the intended final shape or should change.
- **[#141](https://github.com/sweengineeringlabs/edge-application/issues/141)** — review of
  whether retiring `domain-security` in favor of the external `edge-security` repo (2026-07-06,
  `fba9004`) was the right call, given the `no_foreign_type` decoupling cost it produced five
  days later (`bd911de`). Relevant precedent for any future "should this concern live in its own
  repo" decision, including how #139's `domain-base` crate should be scoped.

---

## See also

- `docs/3-design/dataflow.md` — the traced, cited dataflow reference this document summarizes
- `docs/3-design/temp/edge-repo-dataflow-snapshot.md` — temporary mirror of `edge` repo's own
  (partially stale) dataflow docs; delete once that repo's git conflicts are resolved
- `docs/adr/` — this repo's own ADRs, each mirroring an upstream `edge` decision
- `edge`'s own `docs/3-architecture/architecture.md` and `docs/3-architecture/adr/` — the
  platform-wide picture this repo's domain layer fits into
