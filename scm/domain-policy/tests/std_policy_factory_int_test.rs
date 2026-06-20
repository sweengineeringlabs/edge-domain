use edge_domain_policy::{CompositePolicy, Policy, PolicyBootstrap, PolicyViolation, StdPolicyFactory};

#[test]
fn test_std_factory_composite_creates_empty_policy_happy() {
    let p: CompositePolicy<String> = StdPolicyFactory::composite();
    assert!(p.evaluate(&"input".to_string()).is_ok());
}

#[test]
fn test_std_factory_std_factory_returns_instance_happy() {
    let _ = StdPolicyFactory::std_factory();
}

#[test]
fn test_std_factory_composite_empty_evaluates_ok_happy() {
    let p: CompositePolicy<String> = StdPolicyFactory::composite();
    assert!(p.evaluate(&"input".to_string()).is_ok());
}

#[test]
fn test_std_factory_composite_with_failing_policy_returns_error() {
    struct AlwaysFail;
    impl Policy for AlwaysFail {
        type Input = String;
        fn name(&self) -> &'static str { "always-fail" }
        fn evaluate(&self, _: &String) -> Result<(), PolicyViolation> {
            Err(PolicyViolation::new("always-fail", "always fails"))
        }
    }
    let p = StdPolicyFactory::composite().with(Box::new(AlwaysFail));
    assert!(p.evaluate(&"input".to_string()).is_err());
}

#[test]
fn test_std_factory_is_copy_type_edge() {
    let f = StdPolicyFactory::std_factory();
    let _f2 = f;
    let _f3 = f; // Copy — usable after "move"
}
