# ADR-004: edge-service — Service-to-Handler Bridge

**Status:** Accepted  
**Date:** 2026-06-13  
**Amended:** 2026-07-15 — corrected against the real, current codebase; the original Mandate and
Invariant I1 below described a design later superseded by `edge`'s own `ADR-020` (whose header
records "Status: Updated — bridge consolidated into `edge-domain-handler`") without this local
mirror ever being updated to match. See "Amendment: the bridge lives in `domain-handler`, not a
separate repo" below for what actually ships today.  
**Governing ADR:** [ADR-020](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-020-edge-service-bridge.md) — edge-service Service-to-Handler Bridge

---

## Amendment (2026-07-15): the bridge lives in `domain-handler`, not a separate repo

The original Mandate and Invariant I1 below claimed `domain-service` (`edge-domain-service`) and
`domain-handler` (`edge-domain-handler`) never import from each other, and that the bridge
between them lives in a standalone `edge-service` repo. Neither is true of the code as it exists
today, verified directly:

- `domain-handler/Cargo.toml` depends on `edge-application-service` directly.
- Four files in `domain-handler/main/src/core/handler/` import `edge_application_service::Service`
  directly: `handler_error.rs`, `service_adapter.rs`, `service_handler.rs`, `std_registry_bridge.rs`.
- The dependency is **one-way** — `domain-service` never imports `domain-handler`, confirmed by
  grep against `domain-service/Cargo.toml`.
- There is no separate `edge-service` crate anywhere in this workspace's `Cargo.toml`.
- The bridge itself — `IntoHandler` (blanket impl for every `S: Service`, wrapping it in
  `DefaultServiceHandler<S>`), and `RegistryBridge`/`StdRegistryBridge` (bulk-transfers a
  `ServiceRegistry`'s entries into a `HandlerRegistry`) — lives directly in
  `domain-handler/main/src/core/handler/service/service_handler.rs` and
  `domain-handler/main/src/core/handler/std_registry_bridge.rs`.

This is consistent with SEA's `no_foreign_type` rule as actually enforced in this repo: the
imports above are confined to `core/` (the implementation layer, where referencing another
domain crate's real types is permitted), never `api/` (the public contract layer, which still
declares its own local, structurally-identical `Service`/`ServiceRegistry` mirror traits so it
never names `edge_application_service` in a type position). So the "never import" framing below
was too strong even for the intent it was trying to protect — the actual, correct invariant is
narrower: **`domain-handler`'s `api/` layer must never reference `domain-service` types directly;
`domain-handler`'s `core/` layer doing so is expected and is where the sanctioned bridge lives.**

Full trace, with file citations, is in `docs/3-design/dataflow.md` §2. See also
[#139](https://github.com/sweengineeringlabs/edge-application/issues/139) (tightening
`Handler`/`Service`'s `Request`/`Response` associated types, which depends on this exact
dependency direction being accurate), [#140](https://github.com/sweengineeringlabs/edge-application/issues/140)
(the bridge silently drops `HandlerContext` at `DefaultServiceHandler::execute`), and
[#141](https://github.com/sweengineeringlabs/edge-application/issues/141) (a related review of
whether extracting concerns to external repos, as `domain-security` was, is the right pattern —
this ADR's original "bridge lives in a standalone repo" framing was the same instinct, and it did
not survive contact with the real implementation).

---

## Mandate (original, 2026-06-13 — superseded by the amendment above)

`edge-domain-service` and `edge-domain-handler` are independent domain contracts. Neither imports from the other. This workspace provides only the contracts.

The bridge between them lives in `edge-service` — a standalone infrastructure repo whose purpose is to enforce the workflow: `service_handler()` is the only sanctioned way to register a consumer's domain logic into the dispatch pipeline. The adapter code is thin by design; the value is the constraint, not the implementation. Consumers implement `Service`, call `service_handler()` in their `_svc.rs`, and the framework controls the rest.

---

## What this workspace owns

| Crate | Role |
|---|---|
| `edge-domain-service` | `Service<Req, Resp>` trait — named async domain operation |
| `edge-domain-handler` | `Handler<Req, Resp>` trait — infrastructure execution contract |

Both are stable. Neither is modified by ADR-020.

---

## Invariant (I1 from ADR-020) — superseded, see amendment above

`edge-domain-service` and `edge-domain-handler` must never import from each other. Any code that references both contracts belongs in `edge-service` or in a consumer `_svc.rs` — never in either domain crate.

---

## `edge-domain-service` contract

```rust
pub trait Service<Request, Response>: Send + Sync
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    fn name(&self) -> &str;
    fn execute(&self, req: Request) -> BoxFuture<'_, Result<Response, ServiceError>>;
}
```

Consumers implement this for their domain operations. The `name()` method returns the stable identifier used for logging and tracing — it is not used for routing.

**Note (2026-07-15):** the real, current trait shape differs slightly from this sample —
`domain-service::Service` uses associated types (`type Request`/`type Response`) rather than
generic trait parameters, and `execute` returns a boxed future via `BoxFuture` from the actual
implementation, not shown fully here. See `domain-service/main/src/api/service/traits/service.rs`
for the exact current definition. This sample is illustrative of the shape, not copy-paste
accurate to the current code.

---

## `ServiceRegistry`

`edge-domain-service` also provides `ServiceRegistry` — a runtime map of service name → boxed `Service` impl. It is the domain-level counterpart to `HandlerRegistry` in `edge-domain-handler`.

`ServiceRegistry` is independent of `HandlerRegistry`. Consumers may use either or both. **(2026-07-15: in the current code, the bridge is `RegistryBridge`/`StdRegistryBridge` and `IntoHandler`, both in `domain-handler`'s `core/` — not a `service_handler()` factory in a separate `edge-service` repo; see the amendment above.)**

---

## Cascade position

`edge-domain-service` is complete and stable. Unblocks: `edge-service` repo creation (ADR-020 implementation).

No cascade dependencies within this workspace — both contract crates ship independently.

**(2026-07-15: the `edge-service` repo creation this line anticipated did happen —
`sweengineeringlabs/edge-service` / package `swe-edge-service` exists — but it is orphaned: zero
real consumers anywhere in the `edge` ecosystem, confirmed by exhaustive grep. The actual,
live bridge is the one inside `domain-handler` described in the amendment above, not that repo.
See [#141](https://github.com/sweengineeringlabs/edge-application/issues/141).)**
