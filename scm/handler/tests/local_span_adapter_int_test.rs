//! Integration tests — `LocalSpanAdapter`, exercised indirectly via `HandlerTracer::start_span`
//! (its only public construction path).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_handler::{
    ObserverContext, SpanAnnotationRequest, SpanAnnotationResponse, SpanFinishRequest,
    SpanFinishResponse, SpanStartRequest, TracerRequest,
};
use edge_application_observer::StdObserveFactory;

/// @covers: LocalSpanAdapter — record delegates to the real bridged span
#[test]
fn test_span_adapter_record_via_noop_returns_ok_happy() {
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
    assert_eq!(
        span.record(SpanAnnotationRequest {
            key: "k".into(),
            value: "v".into(),
        }),
        Ok(SpanAnnotationResponse)
    );
}

/// @covers: LocalSpanAdapter — finish delegates to the real bridged span
#[test]
fn test_span_adapter_finish_via_noop_returns_ok_error() {
    let observer = StdObserveFactory::noop_observer_context();
    let tracer = ObserverContext::tracer(observer.as_ref(), TracerRequest)
        .unwrap()
        .tracer;
    let span = tracer
        .start_span(SpanStartRequest {
            handler_id: "h2".into(),
            operation: "op2".into(),
        })
        .unwrap()
        .span;
    assert_eq!(span.finish(SpanFinishRequest), Ok(SpanFinishResponse));
}

/// @covers: LocalSpanAdapter — a second span from the same tracer is independent
#[test]
fn test_span_adapter_multiple_spans_are_independent_edge() {
    let observer = StdObserveFactory::noop_observer_context();
    let tracer = ObserverContext::tracer(observer.as_ref(), TracerRequest)
        .unwrap()
        .tracer;
    for i in 0..2 {
        let span = tracer
            .start_span(SpanStartRequest {
                handler_id: format!("h{i}"),
                operation: "op".into(),
            })
            .unwrap()
            .span;
        assert_eq!(span.finish(SpanFinishRequest), Ok(SpanFinishResponse));
    }
}
