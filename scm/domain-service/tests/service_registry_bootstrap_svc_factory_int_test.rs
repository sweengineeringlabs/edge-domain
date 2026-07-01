//! Tests for `service_registry_bootstrap_svc_factory`.

use edge_domain_service::SERVICE_REGISTRY_BOOTSTRAP_SVC;

/// @covers: SERVICE_REGISTRY_BOOTSTRAP_SVC constant
#[test]
fn test_service_registry_bootstrap_svc_constant_happy() {
    assert_eq!(SERVICE_REGISTRY_BOOTSTRAP_SVC, "service_registry_bootstrap");
}

/// @covers: SERVICE_REGISTRY_BOOTSTRAP_SVC constant — not empty
#[test]
fn test_service_registry_bootstrap_svc_constant_not_empty_edge() {
    assert!(!SERVICE_REGISTRY_BOOTSTRAP_SVC.is_empty());
}

/// @covers: SERVICE_REGISTRY_BOOTSTRAP_SVC constant — matches identifier
#[test]
fn test_service_registry_bootstrap_svc_constant_matches_identifier_edge() {
    assert!(SERVICE_REGISTRY_BOOTSTRAP_SVC.chars().all(|c| c.is_alphanumeric() || c == '_'));
}
