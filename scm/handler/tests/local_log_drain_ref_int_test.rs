//! Integration tests — `LocalLogDrainRef`, exercised indirectly via `ObserverContext::drain`
//! (its only public construction path).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_handler::{DrainRequest, LogEmitRequest, LogEmitResponse, ObserverContext};
use edge_application_observer::StdObserveFactory;

/// @covers: LocalLogDrainRef::emit — emits through the borrowed real drain
#[test]
fn test_log_drain_ref_emit_via_noop_returns_ok_happy() {
    let observer = StdObserveFactory::noop_observer_context();
    let drain = ObserverContext::drain(observer.as_ref(), DrainRequest)
        .unwrap()
        .drain;
    assert_eq!(
        drain.emit(LogEmitRequest {
            level: "info".into(),
            handler_id: "h".into(),
            message: "m".into(),
        }),
        Ok(LogEmitResponse)
    );
}

/// @covers: LocalLogDrainRef::emit — an empty message does not error
#[test]
fn test_log_drain_ref_emit_empty_message_returns_ok_error() {
    let observer = StdObserveFactory::noop_observer_context();
    let drain = ObserverContext::drain(observer.as_ref(), DrainRequest)
        .unwrap()
        .drain;
    assert_eq!(
        drain.emit(LogEmitRequest {
            level: "error".into(),
            handler_id: "h".into(),
            message: String::new(),
        }),
        Ok(LogEmitResponse)
    );
}

/// @covers: LocalLogDrainRef::emit — repeated emits on the same reference are independent
#[test]
fn test_log_drain_ref_repeated_emits_are_independent_edge() {
    let observer = StdObserveFactory::noop_observer_context();
    let drain = ObserverContext::drain(observer.as_ref(), DrainRequest)
        .unwrap()
        .drain;
    for level in ["info", "warn", "error"] {
        assert_eq!(
            drain.emit(LogEmitRequest {
                level: level.to_string(),
                handler_id: "h".into(),
                message: "m".into(),
            }),
            Ok(LogEmitResponse)
        );
    }
}
