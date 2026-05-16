//! Integration tests for `ServiceError`.

use edge_domain::ServiceError;

/// @covers: ServiceError
#[test]
fn test_service_error_invalid_request_display_contains_message() {
    let e = ServiceError::InvalidRequest("missing customer_id".into());
    assert!(e.to_string().contains("missing customer_id"));
}

/// @covers: ServiceError
#[test]
fn test_service_error_rule_violation_display_contains_message() {
    let e = ServiceError::RuleViolation("order limit reached".into());
    assert!(e.to_string().contains("order limit reached"));
}

/// @covers: ServiceError
#[test]
fn test_service_error_not_found_display_contains_message() {
    let e = ServiceError::NotFound("customer-42".into());
    assert!(e.to_string().contains("customer-42"));
}

/// @covers: ServiceError
#[test]
fn test_service_error_unavailable_display_contains_message() {
    let e = ServiceError::Unavailable("payment gateway down".into());
    assert!(e.to_string().contains("payment gateway down"));
}

/// @covers: ServiceError
#[test]
fn test_service_error_internal_display_contains_message() {
    let e = ServiceError::Internal("unexpected panic".into());
    assert!(e.to_string().contains("unexpected panic"));
}

/// @covers: ServiceError
#[test]
fn test_service_error_variants_are_distinct() {
    assert!(matches!(ServiceError::InvalidRequest("x".into()), ServiceError::InvalidRequest(_)));
    assert!(matches!(ServiceError::RuleViolation("x".into()), ServiceError::RuleViolation(_)));
    assert!(matches!(ServiceError::NotFound("x".into()), ServiceError::NotFound(_)));
    assert!(matches!(ServiceError::Unavailable("x".into()), ServiceError::Unavailable(_)));
    assert!(matches!(ServiceError::Internal("x".into()), ServiceError::Internal(_)));
}
