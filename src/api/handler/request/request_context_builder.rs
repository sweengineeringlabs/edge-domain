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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_unauthenticated_context() {
        let ctx = RequestContextBuilder::new().build();
        assert!(!ctx.authenticated);
        assert!(ctx.subject.is_none());
    }

    /// @covers: with_subject
    #[test]
    fn test_with_subject_sets_subject() {
        let ctx = RequestContextBuilder::new().with_subject("alice").build();
        assert_eq!(ctx.subject.as_deref(), Some("alice"));
    }

    /// @covers: with_issuer
    #[test]
    fn test_with_issuer_sets_issuer() {
        let ctx = RequestContextBuilder::new()
            .with_issuer("auth.example.com")
            .build();
        assert_eq!(ctx.issuer.as_deref(), Some("auth.example.com"));
    }

    /// @covers: with_tenant_id
    #[test]
    fn test_with_tenant_id_sets_tenant_id() {
        let ctx = RequestContextBuilder::new().with_tenant_id("acme").build();
        assert_eq!(ctx.tenant_id.as_deref(), Some("acme"));
    }

    /// @covers: with_trace_id
    #[test]
    fn test_with_trace_id_sets_trace_id() {
        let ctx = RequestContextBuilder::new().with_trace_id("t-123").build();
        assert_eq!(ctx.trace_id, "t-123");
    }

    /// @covers: with_claims
    #[test]
    fn test_with_claims_sets_claims() {
        let mut claims = HashMap::new();
        claims.insert("role".into(), "admin".into());
        let ctx = RequestContextBuilder::new()
            .with_claims(claims.clone())
            .build();
        assert_eq!(ctx.claims.get("role").map(String::as_str), Some("admin"));
    }

    /// @covers: authenticated
    #[test]
    fn test_authenticated_sets_flag() {
        let ctx = RequestContextBuilder::new().authenticated().build();
        assert!(ctx.authenticated);
    }

    /// @covers: build
    #[test]
    fn test_build_produces_full_context() {
        let ctx = RequestContextBuilder::new()
            .with_subject("bob")
            .with_issuer("auth.example.com")
            .with_trace_id("trace-1")
            .authenticated()
            .build();
        assert_eq!(ctx.subject.as_deref(), Some("bob"));
        assert_eq!(ctx.trace_id, "trace-1");
        assert!(ctx.authenticated);
    }
}
