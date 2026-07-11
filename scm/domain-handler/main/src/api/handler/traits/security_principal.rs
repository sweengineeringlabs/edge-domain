//! `SecurityPrincipal` — local decoupling boundary for the authenticated request principal.

/// Opaque handle to the authenticated (or unauthenticated) principal for a request.
///
/// Declared locally so `api/` never references `edge_security_runtime::SecurityContext`
/// directly in a type position (SEA `no_foreign_type`). Any real `SecurityContext` value
/// satisfies this automatically via the `impl` in `core/`.
pub trait SecurityPrincipal: Send + Sync {}
