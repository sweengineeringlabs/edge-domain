//! SAF facade tests — `HandlerTracer` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_handler::{
    HandlerError, HandlerTracer, Span, SpanAnnotationRequest, SpanAnnotationResponse,
    SpanFinishRequest, SpanFinishResponse, SpanStartRequest, SpanStartResponse,
};

struct StubSpan;
impl Span for StubSpan {
    fn record(&self, _req: SpanAnnotationRequest) -> Result<SpanAnnotationResponse, HandlerError> {
        Ok(SpanAnnotationResponse)
    }
    fn finish(&self, _req: SpanFinishRequest) -> Result<SpanFinishResponse, HandlerError> {
        Ok(SpanFinishResponse)
    }
}

struct OkTracer;
impl HandlerTracer for OkTracer {
    fn start_span(&self, _req: SpanStartRequest) -> Result<SpanStartResponse, HandlerError> {
        Ok(SpanStartResponse {
            span: Box::new(StubSpan),
        })
    }
}

struct FailingTracer;
impl HandlerTracer for FailingTracer {
    fn start_span(&self, _req: SpanStartRequest) -> Result<SpanStartResponse, HandlerError> {
        Err(HandlerError::ExecutionFailed("tracer offline".into()))
    }
}

fn req(handler_id: &str) -> SpanStartRequest {
    SpanStartRequest {
        handler_id: handler_id.into(),
        operation: "op".into(),
    }
}

/// @covers: HandlerTracer::start_span — success
#[test]
fn test_start_span_ok_tracer_returns_span_happy() {
    let span = OkTracer
        .start_span(req("h"))
        .expect("start_span should succeed")
        .span;
    assert_eq!(span.finish(SpanFinishRequest), Ok(SpanFinishResponse));
}

/// @covers: HandlerTracer::start_span — failure propagates
#[test]
fn test_start_span_failing_tracer_returns_err_error() {
    assert!(FailingTracer.start_span(req("h")).is_err());
}

/// @covers: HandlerTracer::start_span — empty handler id accepted
#[test]
fn test_start_span_empty_handler_id_returns_span_edge() {
    let span = OkTracer
        .start_span(req(""))
        .expect("start_span should succeed")
        .span;
    assert_eq!(span.finish(SpanFinishRequest), Ok(SpanFinishResponse));
}
