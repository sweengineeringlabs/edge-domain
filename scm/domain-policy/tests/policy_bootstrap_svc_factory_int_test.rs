use edge_domain_policy::{POLICY_BOOTSTRAP_SVC, POLICY_BOOTSTRAP_SVC_FACTORY};

#[test]
fn test_policy_bootstrap_svc_anchor_is_unit_happy() {
    assert_eq!(
        POLICY_BOOTSTRAP_SVC,
        (),
        "POLICY_BOOTSTRAP_SVC is a Rule-221 anchor and must be the unit value"
    );
}

#[test]
fn test_policy_bootstrap_svc_factory_constant_value_happy() {
    assert_eq!(POLICY_BOOTSTRAP_SVC_FACTORY, "policy_bootstrap_factory");
}

#[test]
fn test_policy_bootstrap_svc_factory_constant_not_empty_error() {
    assert!(
        !POLICY_BOOTSTRAP_SVC_FACTORY.is_empty(),
        "POLICY_BOOTSTRAP_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_policy_bootstrap_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !POLICY_BOOTSTRAP_SVC_FACTORY.contains(char::is_whitespace),
        "POLICY_BOOTSTRAP_SVC_FACTORY must not contain whitespace"
    );
}
