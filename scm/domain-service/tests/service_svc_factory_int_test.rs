//! Tests for `service_svc_factory`.

use edge_application_service::SERVICE_SVC;

/// @covers: SERVICE_SVC constant
#[test]
fn test_service_svc_constant_happy() {
    assert_eq!(SERVICE_SVC, "service");
}

/// @covers: SERVICE_SVC constant — not empty
#[test]
fn test_service_svc_constant_not_empty_edge() {
    assert!(!SERVICE_SVC.is_empty());
}

/// @covers: SERVICE_SVC constant — matches identifier
#[test]
fn test_service_svc_constant_matches_identifier_edge() {
    assert!(SERVICE_SVC.chars().all(|c| c.is_alphanumeric() || c == '_'));
}
