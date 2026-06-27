//! `SecurityContextBuilder` inherent methods and trait impls.

use std::collections::HashMap;

use crate::{Principal, SecurityContext, SecurityContextBuilder};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let builder = SecurityContextBuilder::new();
        assert!(!builder.authenticated);
        assert!(builder.principal.is_none());
        assert!(builder.tenant_id.is_none());
    }

    #[test]
    fn test_principal() {
        use crate::AnonymousPrincipal;
        let principal: Box<dyn Principal> = Box::new(AnonymousPrincipal);
        let builder = SecurityContextBuilder::new().principal(principal);
        assert!(builder.authenticated);
        assert!(builder.principal.is_some());
    }

    #[test]
    fn test_tenant_id() {
        let builder = SecurityContextBuilder::new().tenant_id("tenant-789");
        assert_eq!(builder.tenant_id, Some("tenant-789".to_string()));
    }

    #[test]
    fn test_trace_id() {
        let builder = SecurityContextBuilder::new().trace_id("trace-xyz");
        assert_eq!(builder.trace_id, Some("trace-xyz".to_string()));
    }

    #[test]
    fn test_claim() {
        let builder = SecurityContextBuilder::new().claim("scope", "read");
        assert_eq!(builder.claims.get("scope"), Some(&"read".to_string()));
    }

    #[test]
    fn test_token() {
        let builder = SecurityContextBuilder::new().token("secret-token-123");
        assert_eq!(builder.token, Some("secret-token-123".to_string()));
    }

    #[test]
    fn test_metadata() {
        let builder = SecurityContextBuilder::new().metadata("x-custom", "value");
        assert_eq!(builder.metadata.get("x-custom"), Some(&"value".to_string()));
    }

    #[test]
    fn test_is_authorized() {
        let builder = SecurityContextBuilder::new().is_authorized(true);
        assert!(builder.is_authorized);
    }

    #[test]
    fn test_build() {
        let ctx = SecurityContextBuilder::new()
            .tenant_id("tenant-001")
            .claim("role", "user")
            .build();
        assert_eq!(ctx.tenant_id, Some("tenant-001".to_string()));
        assert_eq!(ctx.claim("role"), Some("user"));
    }
}
