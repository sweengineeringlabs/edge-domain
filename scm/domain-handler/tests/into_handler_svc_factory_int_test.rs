use edge_domain_handler::INTO_HANDLER_SVC_FACTORY;

#[test]
fn test_into_handler_svc_factory_constant_value_happy() {
    assert_eq!(INTO_HANDLER_SVC_FACTORY, "into_handler_factory");
}

#[test]
fn test_into_handler_svc_factory_constant_not_empty_error() {
    assert!(
        !INTO_HANDLER_SVC_FACTORY.is_empty(),
        "INTO_HANDLER_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_into_handler_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !INTO_HANDLER_SVC_FACTORY.contains(char::is_whitespace),
        "INTO_HANDLER_SVC_FACTORY must not contain whitespace"
    );
}
