//! `SecurityContextBuilder` inherent methods and trait impls.

use std::collections::HashMap;

use crate::{Principal, SecurityContext, SecurityContextBuilder};

impl SecurityContextBuilder {
    /// Start a new builder with no principal and `authenticated = false`.
    pub(crate) fn new() -> Self {
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
    pub(crate) fn principal(mut self, p: Box<dyn Principal>) -> Self {
        self.principal = Some(p);
        self.authenticated = true;
        self
    }

    /// Set the tenant scope.
    pub(crate) fn tenant_id(mut self, id: impl Into<String>) -> Self {
        self.tenant_id = Some(id.into());
        self
    }

    /// Set the trace identifier.
    pub(crate) fn trace_id(mut self, id: impl Into<String>) -> Self {
        self.trace_id = Some(id.into());
        self
    }

    /// Insert a claim key-value pair.
    pub(crate) fn claim(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.claims.insert(key.into(), value.into());
        self
    }

    /// Set the raw token.
    pub(crate) fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Insert request metadata (e.g. HTTP headers).
    pub(crate) fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Set authorization result state.
    pub(crate) fn is_authorized(mut self, authorized: bool) -> Self {
        self.is_authorized = authorized;
        self
    }

    /// Consume the builder and return the completed [`SecurityContext`].
    pub(crate) fn build(self) -> SecurityContext {
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
