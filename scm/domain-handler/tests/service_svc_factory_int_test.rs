//! Integration tests — `SERVICE_SVC_FACTORY` constant.

use edge_application_handler::SERVICE_SVC_FACTORY;

/// @covers: SERVICE_SVC_FACTORY — correct factory identity value
#[test]
fn test_service_svc_factory_constant_value_happy() {
    assert_eq!(SERVICE_SVC_FACTORY, "service_factory");
}

/// @covers: SERVICE_SVC_FACTORY — constant is non-empty
#[test]
fn test_service_svc_factory_constant_not_empty_error() {
    assert!(!SERVICE_SVC_FACTORY.is_empty());
    assert_eq!(SERVICE_SVC_FACTORY.len(), "service_factory".len());
}

/// @covers: SERVICE_SVC_FACTORY — constant contains no whitespace
#[test]
fn test_service_svc_factory_constant_no_whitespace_edge() {
    assert!(!SERVICE_SVC_FACTORY.contains(' '));
    assert!(!SERVICE_SVC_FACTORY.contains('\t'));
    assert_eq!(SERVICE_SVC_FACTORY, SERVICE_SVC_FACTORY.trim());
}
