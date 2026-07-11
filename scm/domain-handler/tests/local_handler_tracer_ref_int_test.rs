//! Integration tests — `LocalHandlerTracerRef`, exercised indirectly via `ObserverContext::tracer`
//! (its only public construction path).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_handler::{
    ObserverContext, SpanFinishRequest, SpanFinishResponse, SpanStartRequest, TracerRequest,
};
use edge_domain_observer::StdObserveFactory;

/// @covers: LocalHandlerTracerRef::start_span — starts a span through the borrowed real tracer
#[test]
fn test_handler_tracer_ref_start_span_via_noop_returns_ok_happy() {
    let observer = StdObserveFactory::noop_observer_context();
    let tracer = ObserverContext::tracer(observer.as_ref(), TracerRequest)
        .unwrap()
        .tracer;
    let span = tracer
        .start_span(SpanStartRequest {
            handler_id: "h".into(),
            operation: "op".into(),
        })
        .unwrap()
        .span;
    assert_eq!(span.finish(SpanFinishRequest), Ok(SpanFinishResponse));
}

/// @covers: LocalHandlerTracerRef::start_span — empty handler_id/operation do not error
#[test]
fn test_handler_tracer_ref_start_span_empty_fields_returns_ok_error() {
    let observer = StdObserveFactory::noop_observer_context();
    let tracer = ObserverContext::tracer(observer.as_ref(), TracerRequest)
        .unwrap()
        .tracer;
    let result = tracer.start_span(SpanStartRequest {
        handler_id: String::new(),
        operation: String::new(),
    });
    assert!(result.is_ok());
}

/// @covers: LocalHandlerTracerRef::start_span — the same tracer reference is reusable across calls
#[test]
fn test_handler_tracer_ref_reusable_across_multiple_spans_edge() {
    let observer = StdObserveFactory::noop_observer_context();
    let tracer = ObserverContext::tracer(observer.as_ref(), TracerRequest)
        .unwrap()
        .tracer;
    assert!(tracer
        .start_span(SpanStartRequest {
            handler_id: "a".into(),
            operation: "op".into()
        })
        .is_ok());
    assert!(tracer
        .start_span(SpanStartRequest {
            handler_id: "b".into(),
            operation: "op".into()
        })
        .is_ok());
}
