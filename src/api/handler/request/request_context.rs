//! `RequestContext` — per-request authentication and routing metadata.

use std::collections::HashMap;

/// Per-request metadata threaded from the edge auth/trace layers into every
/// domain handler invocation.
///
/// Carries identity and routing information extracted by the ingress
/// middleware stack (JWT verification, mTLS peer identity, trace
/// propagation). Stable infrastructure dependencies — egress clients,
/// registries — are injected at handler construction time, not here.
#[derive(Debug, Clone, Default)]
pub struct RequestContext {
    /// Authenticated subject from the JWT `sub` claim or API key identity.
    pub subject: Option<String>,
    /// JWT `iss` claim value.
    pub issuer: Option<String>,
    /// Tenant identifier for multi-tenant deployments.  Derived from a
    /// custom `tenant_id` claim or a request header by the auth layer.
    pub tenant_id: Option<String>,
    /// Distributed trace identifier (W3C `traceparent` or custom header).
    pub trace_id: String,
    /// `true` when the request passed the configured auth layer.
    pub authenticated: bool,
    /// All claims from the verified token, serialised to strings for
    /// portability.  Complex (nested) JSON values appear as their JSON
    /// string representation.
    pub claims: HashMap<String, String>,
}

impl RequestContext {
    /// Context for an unauthenticated request — all identity fields empty.
    pub fn unauthenticated() -> Self {
        Self::default()
    }

    /// Build a fully-authenticated context from individual claim values.
    pub fn authenticated(
        subject:   impl Into<String>,
        issuer:    Option<String>,
        tenant_id: Option<String>,
        claims:    HashMap<String, String>,
    ) -> Self {
        Self {
            subject:       Some(subject.into()),
            issuer,
            tenant_id,
            trace_id:      String::new(),
            authenticated: true,
            claims,
        }
    }

    /// Attach a distributed trace identifier.
    pub fn with_trace_id(mut self, id: impl Into<String>) -> Self {
        self.trace_id = id.into();
        self
    }

    /// Attach a tenant identifier.
    pub fn with_tenant_id(mut self, id: impl Into<String>) -> Self {
        self.tenant_id = Some(id.into());
        self
    }

    /// Return the subject, or an error string if the request is unauthenticated.
    pub fn require_subject(&self) -> Option<&str> {
        if self.authenticated { self.subject.as_deref() } else { None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: unauthenticated
    #[test]
    fn test_unauthenticated_returns_default_context() {
        let ctx = RequestContext::unauthenticated();
        assert!(!ctx.authenticated);
        assert!(ctx.subject.is_none());
        assert!(ctx.claims.is_empty());
    }

    /// @covers: authenticated
    #[test]
    fn test_authenticated_sets_subject_and_flag() {
        let ctx = RequestContext::authenticated("alice", Some("auth.example".into()), None, HashMap::new());
        assert!(ctx.authenticated);
        assert_eq!(ctx.subject.as_deref(), Some("alice"));
        assert_eq!(ctx.issuer.as_deref(), Some("auth.example"));
    }

    /// @covers: with_trace_id
    #[test]
    fn test_with_trace_id_sets_trace_id() {
        let ctx = RequestContext::unauthenticated().with_trace_id("abc-123");
        assert_eq!(ctx.trace_id, "abc-123");
    }

    /// @covers: with_tenant_id
    #[test]
    fn test_with_tenant_id_sets_tenant_id() {
        let ctx = RequestContext::unauthenticated().with_tenant_id("acme");
        assert_eq!(ctx.tenant_id.as_deref(), Some("acme"));
    }

    /// @covers: require_subject
    #[test]
    fn test_require_subject_returns_none_for_unauthenticated() {
        assert!(RequestContext::unauthenticated().require_subject().is_none());
    }

    /// @covers: require_subject
    #[test]
    fn test_require_subject_returns_subject_for_authenticated() {
        let ctx = RequestContext::authenticated("bob", None, None, HashMap::new());
        assert_eq!(ctx.require_subject(), Some("bob"));
    }
}
