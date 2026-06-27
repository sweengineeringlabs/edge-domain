//! [`SecurityContext`] — lean carrier for caller identity and request metadata.

use std::collections::HashMap;
use std::fmt;

use crate::api::security::traits::principal::Principal;

/// Carries the security identity and metadata for a single request.
///
/// Construct via [`SecurityBootstrap`](crate::SecurityBootstrap) or directly with
/// [`SecurityContext::unauthenticated`] / [`SecurityContext::authenticated_with`].
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

impl SecurityContext {
    /// Construct an unauthenticated context with no principal or claims.
    pub fn unauthenticated() -> Self {
        Self {
            principal: None,
            tenant_id: None,
            claims: HashMap::new(),
            trace_id: None,
            authenticated: false,
            token: None,
            metadata: HashMap::new(),
            is_authorized: false,
            extensions: HashMap::new(),
        }
    }

    /// Construct an authenticated context for the given principal.
    pub fn authenticated_with(principal: Box<dyn Principal>) -> Self {
        Self {
            authenticated: true,
            principal: Some(principal),
            tenant_id: None,
            claims: HashMap::new(),
            trace_id: None,
            token: None,
            metadata: HashMap::new(),
            is_authorized: false,
            extensions: HashMap::new(),
        }
    }

    /// Set the tenant scope; returns `self` for chaining.
    pub fn with_tenant(mut self, tenant_id: impl Into<String>) -> Self {
        self.tenant_id = Some(tenant_id.into());
        self
    }

    /// Set the trace identifier; returns `self` for chaining.
    pub fn with_trace_id(mut self, trace_id: impl Into<String>) -> Self {
        self.trace_id = Some(trace_id.into());
        self
    }

    /// Insert a claim; returns `self` for chaining.
    pub fn with_claim(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.claims.insert(key.into(), value.into());
        self
    }

    /// Retrieve a claim value by key.
    pub fn claim(&self, key: &str) -> Option<&str> {
        self.claims.get(key).map(String::as_str)
    }
}

impl Default for SecurityContext {
    fn default() -> Self {
        Self::unauthenticated()
    }
}

impl fmt::Debug for SecurityContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SecurityContext")
            .field("principal_id", &self.principal.as_ref().map(|p| p.id()))
            .field("principal_kind", &self.principal.as_ref().map(|p| p.kind()))
            .field("tenant_id", &self.tenant_id)
            .field("trace_id", &self.trace_id)
            .field("authenticated", &self.authenticated)
            .field("token", &self.token.as_ref().map(|_| "***"))
            .field("metadata_keys", &self.metadata.keys().collect::<Vec<_>>())
            .field("is_authorized", &self.is_authorized)
            .finish()
    }
}
