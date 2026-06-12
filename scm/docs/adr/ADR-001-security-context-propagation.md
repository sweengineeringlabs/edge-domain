# ADR-001: Security Context Propagation — domain contract changes

**Status:** Accepted  
**Date:** 2026-06-12  
**Governing ADR:** [ADR-017](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-017-security-context-propagation.md) — Security Context Propagation Pipeline

---

## Mandate

Introduce `edge-domain-security` as the lean contract crate for caller identity and request security context. Remove `RequestContext`. Update the `Handler` execution contract.

---

## New crate: `edge-domain-security`

Added to this workspace. Zero external deps — only `std`, `thiserror`, `async-trait`.

| Type | Purpose |
|------|---------|
| `Principal` trait | `id() -> &str`, `kind() -> &str` — implemented by `TenantId`, `PeerIdentity` in `swe-edge-security` |
| `SecurityContext` | Lean carrier: see fields below |

```rust
pub trait Principal: Send + Sync {
    fn id(&self) -> &str;
    fn kind(&self) -> &str;
}

pub struct SecurityContext {
    pub principal: Option<Box<dyn Principal>>,
    pub tenant_id: Option<String>,
    pub claims: HashMap<String, String>,
    pub trace_id: Option<String>,
    pub authenticated: bool,
}
```

## Feature gate in `edge-domain`

```toml
[features]
default     = ["entity", "valueobject"]
security    = ["dep:edge-domain-security"]   # opt-in — not in default
```

## What is removed

`RequestContext` in `edge-domain-handler` is deleted. All consumers migrate to `SecurityContext`.

## Handler contract change

```rust
// Before
async fn execute_with_context(&self, req: Request, ctx: RequestContext) -> Result<Response, HandlerError>;

// After
async fn execute_with_context(&self, req: Request, ctx: SecurityContext) -> Result<Response, HandlerError>;
```

Default impl: unchanged — delegates to `execute()` with `SecurityContext::unauthenticated()`.

## Invariant (P7 from ADR-017)

`edge-domain-security` must never import from `swe-edge-security` or any ingress/egress crate. It is the contract layer; infrastructure implements it.

---

## Cascade position

Steps **1–3 of 11** in the ADR-017 migration. This repo is the root of the cascade — all other repos are blocked on steps 1–3 completing and a new `edge-domain` tag being cut. Unblocks: dispatch (step 4), swe-edge-security (step 5), proxy (step 8), egress/grpc (step 7).
