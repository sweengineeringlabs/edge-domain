//! Coverage for api/handler/types/request/request_context_builder.rs
use edge_domain::RequestContextBuilder;

#[test]
fn test_request_context_builder_new_builds_unauthenticated_context() {
    let ctx = RequestContextBuilder::new().build();
    assert!(!ctx.authenticated);
    assert!(ctx.subject.is_none());
}

#[test]
fn test_request_context_builder_with_subject_sets_subject() {
    let ctx = RequestContextBuilder::new()
        .authenticated()
        .with_subject("user-123")
        .build();
    assert_eq!(ctx.subject.as_deref(), Some("user-123"));
    assert!(ctx.authenticated);
}

#[test]
fn test_request_context_builder_with_trace_id_sets_trace_id() {
    let ctx = RequestContextBuilder::new()
        .with_trace_id("trace-xyz")
        .build();
    assert_eq!(ctx.trace_id, "trace-xyz");
}
