//! `RequestContextBuilder` — fluent builder for `RequestContext`.

use std::collections::HashMap;

use crate::api::handler::request::RequestContext;

/// Builds a [`RequestContext`] step-by-step.
pub struct RequestContextBuilder {
    subject: Option<String>,
    issuer: Option<String>,
    tenant_id: Option<String>,
    trace_id: String,
    authenticated: bool,
    claims: HashMap<String, String>,
}

impl Default for RequestContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl RequestContextBuilder {
    /// Start with an unauthenticated context.
    pub fn new() -> Self {
        Self {
            subject: None,
            issuer: None,
            tenant_id: None,
            trace_id: String::new(),
            authenticated: false,
            claims: HashMap::new(),
        }
    }

    /// @covers: with_subject
    pub fn with_subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    /// @covers: with_issuer
    pub fn with_issuer(mut self, issuer: impl Into<String>) -> Self {
        self.issuer = Some(issuer.into());
        self
    }

    /// @covers: with_tenant_id
    pub fn with_tenant_id(mut self, id: impl Into<String>) -> Self {
        self.tenant_id = Some(id.into());
        self
    }

    /// @covers: with_trace_id
    pub fn with_trace_id(mut self, id: impl Into<String>) -> Self {
        self.trace_id = id.into();
        self
    }

    /// @covers: authenticated
    pub fn authenticated(mut self) -> Self {
        self.authenticated = true;
        self
    }

    /// @covers: with_claims
    pub fn with_claims(mut self, claims: HashMap<String, String>) -> Self {
        self.claims = claims;
        self
    }

    /// @covers: build
    pub fn build(self) -> RequestContext {
        RequestContext {
            subject: self.subject,
            issuer: self.issuer,
            tenant_id: self.tenant_id,
            trace_id: self.trace_id,
            authenticated: self.authenticated,
            claims: self.claims,
        }
    }
}


