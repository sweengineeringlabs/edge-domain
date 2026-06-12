//! [`RequestContextBuilder`] — fluent builder for [`RequestContext`].

use std::collections::HashMap;

use crate::api::handler::types::request_context::RequestContext;

/// Fluent builder for assembling a [`RequestContext`].
#[derive(Debug, Default)]
pub struct RequestContextBuilder {
    subject: Option<String>,
    issuer: Option<String>,
    tenant_id: Option<String>,
    trace_id: Option<String>,
    authenticated: bool,
    claims: HashMap<String, String>,
}

impl RequestContextBuilder {
    /// Create a new builder with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the subject claim.
    pub fn with_subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    /// Set the issuer claim.
    pub fn with_issuer(mut self, issuer: impl Into<String>) -> Self {
        self.issuer = Some(issuer.into());
        self
    }

    /// Set the tenant id.
    pub fn with_tenant_id(mut self, tenant_id: impl Into<String>) -> Self {
        self.tenant_id = Some(tenant_id.into());
        self
    }

    /// Set the trace id.
    pub fn with_trace_id(mut self, trace_id: impl Into<String>) -> Self {
        self.trace_id = Some(trace_id.into());
        self
    }

    /// Mark the context as authenticated.
    pub fn authenticated(mut self) -> Self {
        self.authenticated = true;
        self
    }

    /// Merge additional claims.
    pub fn with_claims(mut self, claims: HashMap<String, String>) -> Self {
        self.claims.extend(claims);
        self
    }

    /// Consume the builder and produce a [`RequestContext`].
    pub fn build(self) -> RequestContext {
        RequestContext {
            subject: self.subject,
            issuer: self.issuer,
            tenant_id: self.tenant_id,
            trace_id: self.trace_id.unwrap_or_default(),
            authenticated: self.authenticated,
            claims: self.claims,
        }
    }
}
