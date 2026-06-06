//! `RequestContext` — per-request authentication and routing metadata.

use std::collections::HashMap;

/// Per-request metadata threaded from the edge auth/trace layers into every
/// domain handler invocation.
///
/// Carries identity and routing information extracted by the ingress
/// middleware stack (JWT verification, mTLS peer identity, trace
/// propagation). Stable infrastructure dependencies — egress clients,
/// registries — are injected at handler construction time, not here.
///
/// # Examples
///
/// ```rust
/// use edge_domain::RequestContext;
///
/// // Unauthenticated — default for health checks or anonymous requests.
/// let ctx = RequestContext::unauthenticated();
/// assert!(!ctx.authenticated);
/// assert!(ctx.subject.is_none());
/// assert!(ctx.require_subject().is_none());
///
/// // Authenticated from JWT claims.
/// let ctx = RequestContext::authenticated(
///     "user-123",
///     Some("https://auth.example.com".to_string()),
///     Some("tenant-abc".to_string()),
///     std::collections::HashMap::new(),
/// )
/// .with_trace_id("trace-xyz");
///
/// assert!(ctx.authenticated);
/// assert_eq!(ctx.require_subject(), Some("user-123"));
/// assert_eq!(ctx.trace_id, "trace-xyz");
/// assert_eq!(ctx.tenant_id.as_deref(), Some("tenant-abc"));
/// ```
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
        subject: impl Into<String>,
        issuer: Option<String>,
        tenant_id: Option<String>,
        claims: HashMap<String, String>,
    ) -> Self {
        Self {
            subject: Some(subject.into()),
            issuer,
            tenant_id,
            trace_id: String::new(),
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
        if self.authenticated {
            self.subject.as_deref()
        } else {
            None
        }
    }
}
