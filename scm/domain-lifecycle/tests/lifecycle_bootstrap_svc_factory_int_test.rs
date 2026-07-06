use edge_domain_lifecycle::{LIFECYCLE_BOOTSTRAP_SVC_FACTORY, LIFECYCLE_FACTORY_SVC};

#[test]
fn test_lifecycle_factory_svc_constant_value_happy() {
    assert_eq!(LIFECYCLE_FACTORY_SVC, "lifecycle_factory");
}

#[test]
fn test_lifecycle_bootstrap_svc_factory_constant_value_happy() {
    assert_eq!(LIFECYCLE_BOOTSTRAP_SVC_FACTORY, "lifecycle_bootstrap_factory");
}

#[test]
fn test_lifecycle_bootstrap_svc_factory_constant_not_empty_error() {
    assert!(
        !LIFECYCLE_BOOTSTRAP_SVC_FACTORY.is_empty(),
        "LIFECYCLE_BOOTSTRAP_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_lifecycle_bootstrap_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !LIFECYCLE_BOOTSTRAP_SVC_FACTORY.contains(char::is_whitespace),
        "LIFECYCLE_BOOTSTRAP_SVC_FACTORY must not contain whitespace"
    );
}
