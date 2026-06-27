//! [`SecurityContext`] — lean carrier for caller identity and request metadata.

use std::collections::HashMap;

use crate::Principal;

/// Carries the security identity and metadata for a single request.
///
/// Construct via [`SecurityBootstrap`](crate::SecurityBootstrap) or directly with
/// [`SecurityContext::unauthenticated`] / [`SecurityContext::authenticated_with`].
/// All methods and trait impls are defined in `core::security::security_context`.
pub struct SecurityContext {
    /// Authenticated principal, if present.
    pub principal: Option<Box<dyn Principal>>,
    /// Tenant scope, for multi-tenant deployments.
    pub tenant_id: Option<String>,
    /// Free-form claims map (e.g. JWT claims, propagated headers).
    pub claims: HashMap<String, String>,
    /// Distributed trace identifier.
    pub trace_id: Option<String>,
    /// Whether this context represents an authenticated caller.
    pub authenticated: bool,
    /// Raw token (e.g. JWT), if available.
    pub token: Option<String>,
    /// Request metadata (e.g. HTTP headers).
    pub metadata: HashMap<String, String>,
    /// Authorization result state.
    pub is_authorized: bool,
    /// Custom extension data for framework interop.
    pub extensions: std::collections::HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
}
