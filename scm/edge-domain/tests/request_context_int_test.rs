//! Coverage for api/handler/types/request/request_context.rs
use edge_domain::RequestContext;
use std::collections::HashMap;

#[test]
fn test_request_context_unauthenticated_has_no_subject() {
    let ctx = RequestContext::unauthenticated();
    assert!(!ctx.authenticated);
    assert!(ctx.subject.is_none());
}

#[test]
fn test_request_context_authenticated_sets_subject() {
    let ctx = RequestContext::authenticated("user-1", None, None, HashMap::new());
    assert!(ctx.authenticated);
    assert_eq!(ctx.require_subject(), Some("user-1"));
}

#[test]
fn test_request_context_require_subject_is_none_when_unauthenticated() {
    let ctx = RequestContext::unauthenticated();
    assert!(ctx.require_subject().is_none());
}

#[test]
fn test_request_context_with_trace_id_sets_trace_id() {
    let ctx = RequestContext::unauthenticated().with_trace_id("trace-abc");
    assert_eq!(ctx.trace_id, "trace-abc");
}
