//! SAF facade tests — `Span` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_handler::{
    HandlerError, Span, SpanAnnotationRequest, SpanAnnotationResponse, SpanFinishRequest,
    SpanFinishResponse,
};

struct OkSpan;
impl Span for OkSpan {
    fn record(&self, _req: SpanAnnotationRequest) -> Result<SpanAnnotationResponse, HandlerError> {
        Ok(SpanAnnotationResponse)
    }
    fn finish(&self, _req: SpanFinishRequest) -> Result<SpanFinishResponse, HandlerError> {
        Ok(SpanFinishResponse)
    }
}

struct FailingSpan;
impl Span for FailingSpan {
    fn record(&self, _req: SpanAnnotationRequest) -> Result<SpanAnnotationResponse, HandlerError> {
        Err(HandlerError::ExecutionFailed("span closed".into()))
    }
    fn finish(&self, _req: SpanFinishRequest) -> Result<SpanFinishResponse, HandlerError> {
        Err(HandlerError::ExecutionFailed("span closed".into()))
    }
}

/// @covers: Span::record — success
#[test]
fn test_record_ok_span_returns_ok_happy() {
    assert_eq!(
        OkSpan.record(SpanAnnotationRequest {
            key: "k".into(),
            value: "v".into(),
        }),
        Ok(SpanAnnotationResponse)
    );
}

/// @covers: Span::record — failure propagates
#[test]
fn test_record_failing_span_returns_err_error() {
    assert!(FailingSpan
        .record(SpanAnnotationRequest {
            key: "k".into(),
            value: "v".into(),
        })
        .is_err());
}

/// @covers: Span::finish — success
#[test]
fn test_finish_ok_span_returns_ok_happy() {
    assert_eq!(OkSpan.finish(SpanFinishRequest), Ok(SpanFinishResponse));
}

/// @covers: Span::finish — failure propagates
#[test]
fn test_finish_failing_span_returns_err_error() {
    assert!(FailingSpan.finish(SpanFinishRequest).is_err());
}

/// @covers: Span::record — empty key/value accepted
#[test]
fn test_record_empty_key_value_returns_ok_edge() {
    assert_eq!(
        OkSpan.record(SpanAnnotationRequest {
            key: "".into(),
            value: "".into(),
        }),
        Ok(SpanAnnotationResponse)
    );
}

/// @covers: Span::finish — idempotent across repeated calls
#[test]
fn test_finish_called_twice_both_succeed_edge() {
    let s = OkSpan;
    assert_eq!(s.finish(SpanFinishRequest), Ok(SpanFinishResponse));
    assert_eq!(s.finish(SpanFinishRequest), Ok(SpanFinishResponse));
}
