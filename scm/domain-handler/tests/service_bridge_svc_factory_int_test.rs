use edge_domain_handler::SERVICE_BRIDGE_SVC_FACTORY;

#[test]
fn test_service_bridge_svc_factory_constant_value_happy() {
    assert_eq!(SERVICE_BRIDGE_SVC_FACTORY, "service_bridge_factory");
}

#[test]
fn test_service_bridge_svc_factory_constant_not_empty_error() {
    assert!(
        !SERVICE_BRIDGE_SVC_FACTORY.is_empty(),
        "SERVICE_BRIDGE_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_service_bridge_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !SERVICE_BRIDGE_SVC_FACTORY.contains(char::is_whitespace),
        "SERVICE_BRIDGE_SVC_FACTORY must not contain whitespace"
    );
}
