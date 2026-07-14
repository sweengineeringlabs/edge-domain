//! Tests for the `ServiceError` error type.

use edge_application_service::ServiceError;

/// @covers: ServiceError::InvalidRequest — Display
#[test]
fn test_invalid_request_display_includes_message_happy() {
    let e = ServiceError::InvalidRequest("bad payload".into());
    assert!(e.to_string().contains("bad payload"));
}

/// @covers: ServiceError::RuleViolation — Display
#[test]
fn test_rule_violation_display_includes_message_error() {
    let e = ServiceError::RuleViolation("limit exceeded".into());
    assert!(e.to_string().contains("limit exceeded"));
}

/// @covers: ServiceError — each variant formats distinctly
#[test]
fn test_variants_format_distinctly_edge() {
    let not_found = ServiceError::NotFound("x".into()).to_string();
    let unavailable = ServiceError::Unavailable("y".into()).to_string();
    let internal = ServiceError::Internal("z".into()).to_string();
    assert!(not_found.contains("not found"));
    assert!(unavailable.contains("unavailable"));
    assert!(internal.contains("internal"));
    assert_ne!(not_found, unavailable);
    assert_ne!(unavailable, internal);
}
