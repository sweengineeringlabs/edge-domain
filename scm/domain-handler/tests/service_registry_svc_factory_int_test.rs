//! Integration tests — `SERVICE_REGISTRY_SVC_FACTORY` constant.

use edge_domain_handler::SERVICE_REGISTRY_SVC_FACTORY;

/// @covers: SERVICE_REGISTRY_SVC_FACTORY — correct factory identity value
#[test]
fn test_service_registry_svc_factory_constant_value_happy() {
    assert_eq!(SERVICE_REGISTRY_SVC_FACTORY, "service_registry_factory");
}

/// @covers: SERVICE_REGISTRY_SVC_FACTORY — constant is non-empty
#[test]
fn test_service_registry_svc_factory_constant_not_empty_error() {
    assert!(!SERVICE_REGISTRY_SVC_FACTORY.is_empty());
    assert_eq!(
        SERVICE_REGISTRY_SVC_FACTORY.len(),
        "service_registry_factory".len()
    );
}

/// @covers: SERVICE_REGISTRY_SVC_FACTORY — constant contains no whitespace
#[test]
fn test_service_registry_svc_factory_constant_no_whitespace_edge() {
    assert!(!SERVICE_REGISTRY_SVC_FACTORY.contains(' '));
    assert!(!SERVICE_REGISTRY_SVC_FACTORY.contains('\t'));
    assert_eq!(
        SERVICE_REGISTRY_SVC_FACTORY,
        SERVICE_REGISTRY_SVC_FACTORY.trim()
    );
}
