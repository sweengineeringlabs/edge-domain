# ADR-004: edge-service — Service-to-Handler Bridge

**Status:** Accepted  
**Date:** 2026-06-13  
**Governing ADR:** [ADR-020](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-020-edge-service-bridge.md) — edge-service Service-to-Handler Bridge

---

## Mandate

`edge-domain-service` and `edge-domain-handler` are independent domain contracts. Neither imports from the other. The bridge between them lives in the infrastructure repo `edge-service` — this workspace provides only the contracts.

---

## What this workspace owns

| Crate | Role |
|---|---|
| `edge-domain-service` | `Service<Req, Resp>` trait — named async domain operation |
| `edge-domain-handler` | `Handler<Req, Resp>` trait — infrastructure execution contract |

Both are stable. Neither is modified by ADR-020.

---

## Invariant (I1 from ADR-020)

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

---

## `ServiceRegistry`

`edge-domain-service` also provides `ServiceRegistry` — a runtime map of service name → boxed `Service` impl. It is the domain-level counterpart to `HandlerRegistry` in `edge-domain-handler`.

`ServiceRegistry` is independent of `HandlerRegistry`. Consumers may use either or both. When both are present, `edge-service`'s `service_handler()` factory bridges a `Service` into a `Handler` for registration in `HandlerRegistry`.

---

## Cascade position

`edge-domain-service` is complete and stable. Unblocks: `edge-service` repo creation (ADR-020 implementation).

No cascade dependencies within this workspace — both contract crates ship independently.
