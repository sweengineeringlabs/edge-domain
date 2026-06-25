use edge_domain_handler::HANDLER_BOOTSTRAP_SVC_FACTORY;

#[test]
fn test_handler_bootstrap_svc_factory_constant_value_happy() {
    assert_eq!(HANDLER_BOOTSTRAP_SVC_FACTORY, "handler_bootstrap_factory");
}

#[test]
fn test_handler_bootstrap_svc_factory_constant_not_empty_error() {
    assert!(!HANDLER_BOOTSTRAP_SVC_FACTORY.is_empty(), "HANDLER_BOOTSTRAP_SVC_FACTORY must not be empty");
}

#[test]
fn test_handler_bootstrap_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !HANDLER_BOOTSTRAP_SVC_FACTORY.contains(char::is_whitespace),
        "HANDLER_BOOTSTRAP_SVC_FACTORY must not contain whitespace"
    );
}
