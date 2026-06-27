use edge_domain_command::COMMAND_BOOTSTRAP_SVC_FACTORY;

#[test]
fn test_command_bootstrap_svc_factory_constant_value_happy() {
    assert_eq!(COMMAND_BOOTSTRAP_SVC_FACTORY, "command_bootstrap_factory");
}

#[test]
fn test_command_bootstrap_svc_factory_constant_not_empty_error() {
    assert!(!COMMAND_BOOTSTRAP_SVC_FACTORY.is_empty(), "COMMAND_BOOTSTRAP_SVC_FACTORY must not be empty");
}

#[test]
fn test_command_bootstrap_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !COMMAND_BOOTSTRAP_SVC_FACTORY.contains(char::is_whitespace),
        "COMMAND_BOOTSTRAP_SVC_FACTORY must not contain whitespace"
    );
}
