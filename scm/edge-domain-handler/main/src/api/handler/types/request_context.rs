//! [`RequestContext`] — caller identity and tracing metadata for handler execution.

use std::collections::HashMap;

/// Caller identity and tracing metadata attached to a handler invocation.
#[derive(Debug, Clone, Default)]
pub struct RequestContext {
    /// Subject (user / service account) from the credential token.
    pub subject: Option<String>,
    /// Issuer that signed the credential token.
    pub issuer: Option<String>,
    /// Tenant scope for multi-tenant deployments.
    pub tenant_id: Option<String>,
    /// Distributed trace identifier.
    pub trace_id: String,
    /// Whether the caller has been authenticated.
    pub authenticated: bool,
    /// Arbitrary key/value claims from the credential token.
    pub claims: HashMap<String, String>,
}

impl RequestContext {
    /// Construct an unauthenticated (anonymous) context.
    pub fn unauthenticated() -> Self {
        Self::default()
    }

    /// Construct an authenticated context with the given subject and optional fields.
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

    /// Override the trace id.
    pub fn with_trace_id(mut self, id: impl Into<String>) -> Self {
        self.trace_id = id.into();
        self
    }

    /// Override the tenant id.
    pub fn with_tenant_id(mut self, id: impl Into<String>) -> Self {
        self.tenant_id = Some(id.into());
        self
    }

    /// Return the subject only when the caller is authenticated.
    pub fn require_subject(&self) -> Option<&str> {
        if self.authenticated {
            self.subject.as_deref()
        } else {
            None
        }
    }
}
