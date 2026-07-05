use edge_domain_clock::CLOCK_BOOTSTRAP_SVC_FACTORY;

#[test]
fn test_clock_bootstrap_svc_factory_constant_value_happy() {
    assert_eq!(CLOCK_BOOTSTRAP_SVC_FACTORY, "clock_bootstrap_factory");
}

#[test]
fn test_clock_bootstrap_svc_factory_constant_not_empty_error() {
    assert!(
        !CLOCK_BOOTSTRAP_SVC_FACTORY.is_empty(),
        "CLOCK_BOOTSTRAP_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_clock_bootstrap_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !CLOCK_BOOTSTRAP_SVC_FACTORY.contains(char::is_whitespace),
        "CLOCK_BOOTSTRAP_SVC_FACTORY must not contain whitespace"
    );
}
