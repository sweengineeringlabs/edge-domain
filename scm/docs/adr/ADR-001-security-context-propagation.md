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

#[derive(Debug)]
pub struct SecurityContext {
    pub principal: Option<Box<dyn Principal>>,
    pub tenant_id: Option<String>,
    pub claims: HashMap<String, String>,
    pub trace_id: Option<String>,
    pub authenticated: bool,
}

impl SecurityContext {
    /// Zero-cost anonymous context. All identity fields are None / empty.
    pub fn unauthenticated() -> Self { ... }

    /// Authenticated context with a verified principal. Chain builder methods
    /// to populate remaining fields.
    pub fn authenticated_with(principal: Box<dyn Principal>) -> Self { ... }

    /// Set the tenant scope; returns `self` for chaining.
    pub fn with_tenant(mut self, tenant_id: impl Into<String>) -> Self { ... }

    /// Set the trace identifier; returns `self` for chaining.
    pub fn with_trace_id(mut self, id: impl Into<String>) -> Self { ... }

    /// Insert a single claim key/value pair; returns `self` for chaining.
    pub fn with_claim(mut self, key: impl Into<String>, value: impl Into<String>) -> Self { ... }
}
```

## Feature gate in `edge-domain`

`edge-domain-security` is a **required** (always-on) dependency — `Handler::execute_with_context` directly imports `SecurityContext`. The `security` feature is a **SAF re-export gate** only: enabling it exposes `Principal` and `SecurityContext` in the public `saf/` surface.

```toml
[features]
default     = ["entity", "valueobject"]
security    = []                             # SAF re-export gate — not a dep gate
# edge-domain-security is always present as a required dep (Handler coupling)
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
