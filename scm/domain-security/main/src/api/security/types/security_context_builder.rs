//! [`SecurityContextBuilder`] — fluent builder for [`SecurityContext`].

use std::collections::HashMap;

use crate::api::security::traits::principal::Principal;
use crate::api::security::types::security_context::SecurityContext;

/// Fluent builder for constructing a [`SecurityContext`].
///
/// Prefer this over direct struct construction for contexts with multiple
/// optional fields.
pub struct SecurityContextBuilder {
    principal: Option<Box<dyn Principal>>,
    tenant_id: Option<String>,
    claims: HashMap<String, String>,
    trace_id: Option<String>,
    authenticated: bool,
    token: Option<String>,
    metadata: HashMap<String, String>,
    is_authorized: bool,
    extensions: std::collections::HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
}

impl SecurityContextBuilder {
    /// Start a new builder with no principal and `authenticated = false`.
    pub fn new() -> Self {
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

    /// Set the principal and mark the context authenticated.
    pub fn principal(mut self, p: Box<dyn Principal>) -> Self {
        self.principal = Some(p);
        self.authenticated = true;
        self
    }

    /// Set the tenant scope.
    pub fn tenant_id(mut self, id: impl Into<String>) -> Self {
        self.tenant_id = Some(id.into());
        self
    }

    /// Set the trace identifier.
    pub fn trace_id(mut self, id: impl Into<String>) -> Self {
        self.trace_id = Some(id.into());
        self
    }

    /// Insert a claim key-value pair.
    pub fn claim(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.claims.insert(key.into(), value.into());
        self
    }

    /// Set the raw token.
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Insert request metadata (e.g. HTTP headers).
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Set authorization result state.
    pub fn is_authorized(mut self, authorized: bool) -> Self {
        self.is_authorized = authorized;
        self
    }

    /// Consume the builder and return the completed [`SecurityContext`].
    pub fn build(self) -> SecurityContext {
        SecurityContext {
            principal: self.principal,
            tenant_id: self.tenant_id,
            claims: self.claims,
            trace_id: self.trace_id,
            authenticated: self.authenticated,
            token: self.token,
            metadata: self.metadata,
            is_authorized: self.is_authorized,
            extensions: self.extensions,
        }
    }
}

impl Default for SecurityContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}
