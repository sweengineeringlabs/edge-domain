//! Integration tests — `SERVICE_SVC` constant.

use edge_domain_handler::SERVICE_SVC;

/// @covers: SERVICE_SVC — correct service identity value
#[test]
fn test_service_svc_constant_value_happy() {
    assert_eq!(SERVICE_SVC, "service");
}

/// @covers: SERVICE_SVC — constant is non-empty
#[test]
fn test_service_svc_constant_not_empty_error() {
    assert!(!SERVICE_SVC.is_empty());
    assert_eq!(SERVICE_SVC.len(), "service".len());
}

/// @covers: SERVICE_SVC — constant contains no whitespace
#[test]
fn test_service_svc_constant_no_whitespace_edge() {
    assert!(!SERVICE_SVC.contains(' '));
    assert!(!SERVICE_SVC.contains('\t'));
    assert_eq!(SERVICE_SVC, SERVICE_SVC.trim());
}
