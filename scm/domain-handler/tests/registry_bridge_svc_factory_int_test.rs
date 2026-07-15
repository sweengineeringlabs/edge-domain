//! Integration tests — `REGISTRY_BRIDGE_SVC_FACTORY` constant.

use edge_application_handler::REGISTRY_BRIDGE_SVC_FACTORY;

/// @covers: REGISTRY_BRIDGE_SVC_FACTORY — correct factory identity value
#[test]
fn test_registry_bridge_svc_factory_constant_value_happy() {
    assert_eq!(REGISTRY_BRIDGE_SVC_FACTORY, "registry_bridge_factory");
}

/// @covers: REGISTRY_BRIDGE_SVC_FACTORY — constant is non-empty
#[test]
fn test_registry_bridge_svc_factory_constant_not_empty_error() {
    assert!(!REGISTRY_BRIDGE_SVC_FACTORY.is_empty());
    assert_eq!(
        REGISTRY_BRIDGE_SVC_FACTORY.len(),
        "registry_bridge_factory".len()
    );
}

/// @covers: REGISTRY_BRIDGE_SVC_FACTORY — constant contains no whitespace
#[test]
fn test_registry_bridge_svc_factory_constant_no_whitespace_edge() {
    assert!(!REGISTRY_BRIDGE_SVC_FACTORY.contains(' '));
    assert!(!REGISTRY_BRIDGE_SVC_FACTORY.contains('\t'));
    assert_eq!(
        REGISTRY_BRIDGE_SVC_FACTORY,
        REGISTRY_BRIDGE_SVC_FACTORY.trim()
    );
}
