//! Integration tests — `REGISTRY_BRIDGE_SVC` constant.

use edge_domain_handler::REGISTRY_BRIDGE_SVC;

/// @covers: REGISTRY_BRIDGE_SVC — correct service identity value
#[test]
fn test_registry_bridge_svc_constant_value_happy() {
    assert_eq!(REGISTRY_BRIDGE_SVC, "registry_bridge");
}

/// @covers: REGISTRY_BRIDGE_SVC — constant is non-empty
#[test]
fn test_registry_bridge_svc_constant_not_empty_error() {
    assert!(!REGISTRY_BRIDGE_SVC.is_empty());
    assert_eq!(REGISTRY_BRIDGE_SVC.len(), "registry_bridge".len());
}

/// @covers: REGISTRY_BRIDGE_SVC — constant contains no whitespace
#[test]
fn test_registry_bridge_svc_constant_no_whitespace_edge() {
    assert!(!REGISTRY_BRIDGE_SVC.contains(' '));
    assert!(!REGISTRY_BRIDGE_SVC.contains('\t'));
    assert_eq!(REGISTRY_BRIDGE_SVC, REGISTRY_BRIDGE_SVC.trim());
}
