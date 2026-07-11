//! Integration tests — `SERVICE_REGISTRY_SVC` constant.

use edge_domain_handler::SERVICE_REGISTRY_SVC;

/// @covers: SERVICE_REGISTRY_SVC — correct service identity value
#[test]
fn test_service_registry_svc_constant_value_happy() {
    assert_eq!(SERVICE_REGISTRY_SVC, "service_registry");
}

/// @covers: SERVICE_REGISTRY_SVC — constant is non-empty
#[test]
fn test_service_registry_svc_constant_not_empty_error() {
    assert!(!SERVICE_REGISTRY_SVC.is_empty());
    assert_eq!(SERVICE_REGISTRY_SVC.len(), "service_registry".len());
}

/// @covers: SERVICE_REGISTRY_SVC — constant contains no whitespace
#[test]
fn test_service_registry_svc_constant_no_whitespace_edge() {
    assert!(!SERVICE_REGISTRY_SVC.contains(' '));
    assert!(!SERVICE_REGISTRY_SVC.contains('\t'));
    assert_eq!(SERVICE_REGISTRY_SVC, SERVICE_REGISTRY_SVC.trim());
}
