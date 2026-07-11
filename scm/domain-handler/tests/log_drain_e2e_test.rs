//! SAF facade tests — `LogDrain` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_handler::{HandlerError, LogDrain, LogEmitRequest, LogEmitResponse};

struct OkDrain;
impl LogDrain for OkDrain {
    fn emit(&self, _req: LogEmitRequest) -> Result<LogEmitResponse, HandlerError> {
        Ok(LogEmitResponse)
    }
}

struct FailingDrain;
impl LogDrain for FailingDrain {
    fn emit(&self, _req: LogEmitRequest) -> Result<LogEmitResponse, HandlerError> {
        Err(HandlerError::ExecutionFailed("drain unavailable".into()))
    }
}

fn req(message: &str) -> LogEmitRequest {
    LogEmitRequest {
        level: "info".into(),
        handler_id: "h".into(),
        message: message.into(),
    }
}

/// @covers: LogDrain::emit — success
#[test]
fn test_emit_ok_drain_returns_ok_happy() {
    assert_eq!(OkDrain.emit(req("hello")), Ok(LogEmitResponse));
}

/// @covers: LogDrain::emit — failure propagates
#[test]
fn test_emit_failing_drain_returns_err_error() {
    assert!(FailingDrain.emit(req("boom")).is_err());
}

/// @covers: LogDrain::emit — empty message accepted
#[test]
fn test_emit_empty_message_returns_ok_edge() {
    assert_eq!(OkDrain.emit(req("")), Ok(LogEmitResponse));
}
