//! Coverage for api/handler/errors/handler_error.rs
use edge_domain::HandlerError;

#[test]
fn test_handler_error_internal_wraps_as_execution_failed() {
    let e = HandlerError::internal("db unavailable");
    assert!(matches!(e, HandlerError::ExecutionFailed(_)));
    assert!(e.to_string().contains("db unavailable"));
}

#[test]
fn test_handler_error_invalid_wraps_as_invalid_request() {
    let e = HandlerError::invalid("bad uuid");
    assert!(matches!(e, HandlerError::InvalidRequest(_)));
}

#[test]
fn test_handler_error_not_found_display_includes_message() {
    let e = HandlerError::NotFound("order-99".into());
    assert!(e.to_string().contains("order-99"));
}

#[test]
fn test_handler_error_unauthorized_display_includes_reason() {
    let e = HandlerError::Unauthorized("JWT expired".into());
    assert!(e.to_string().contains("JWT expired"));
}
