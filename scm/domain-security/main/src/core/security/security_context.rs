//! `SecurityContext` inherent methods and trait impls.

use std::collections::HashMap;
use std::fmt;

use crate::{Principal, SecurityContext};

impl SecurityContext {
    /// Construct an unauthenticated context with no principal or claims.
    pub(crate) fn unauthenticated() -> Self {
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
    pub(crate) fn authenticated_with(principal: Box<dyn Principal>) -> Self {
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
    pub(crate) fn with_tenant(mut self, tenant_id: impl Into<String>) -> Self {
        self.tenant_id = Some(tenant_id.into());
        self
    }

    /// Set the trace identifier; returns `self` for chaining.
    pub(crate) fn with_trace_id(mut self, trace_id: impl Into<String>) -> Self {
        self.trace_id = Some(trace_id.into());
        self
    }

    /// Insert a claim; returns `self` for chaining.
    pub(crate) fn with_claim(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.claims.insert(key.into(), value.into());
        self
    }

    /// Retrieve a claim value by key.
    pub(crate) fn claim(&self, key: &str) -> Option<&str> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unauthenticated() {
        let ctx = SecurityContext::unauthenticated();
        assert!(!ctx.authenticated);
        assert!(ctx.principal.is_none());
        assert!(ctx.tenant_id.is_none());
        assert!(ctx.claims.is_empty());
    }

    #[test]
    fn test_authenticated_with() {
        use crate::AnonymousPrincipal;
        let principal: Box<dyn Principal> = Box::new(AnonymousPrincipal);
        let ctx = SecurityContext::authenticated_with(principal);
        assert!(ctx.authenticated);
        assert!(ctx.principal.is_some());
    }

    #[test]
    fn test_with_tenant() {
        let ctx = SecurityContext::unauthenticated().with_tenant("tenant-123");
        assert_eq!(ctx.tenant_id, Some("tenant-123".to_string()));
    }

    #[test]
    fn test_with_trace_id() {
        let ctx = SecurityContext::unauthenticated().with_trace_id("trace-456");
        assert_eq!(ctx.trace_id, Some("trace-456".to_string()));
    }

    #[test]
    fn test_with_claim() {
        let ctx = SecurityContext::unauthenticated().with_claim("role", "admin");
        assert_eq!(ctx.claim("role"), Some("admin"));
    }

    #[test]
    fn test_claim() {
        let ctx = SecurityContext::unauthenticated();
        assert_eq!(ctx.claim("nonexistent"), None);
    }
}
