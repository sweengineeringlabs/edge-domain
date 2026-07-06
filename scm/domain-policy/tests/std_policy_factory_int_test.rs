#![allow(clippy::unwrap_used, clippy::expect_used)]
use edge_domain_policy::{
    CompositePolicy, Policy, PolicyEvaluateRequest, PolicyNameRequest, PolicyNameResponse,
    PolicyBootstrap, PolicyError, StdPolicyFactory,
};

fn eval(input: &String) -> PolicyEvaluateRequest<'_, String> {
    PolicyEvaluateRequest { input }
}

#[test]
fn test_std_factory_composite_creates_empty_policy_happy() {
    let p: CompositePolicy<String> = StdPolicyFactory::composite();
    assert_eq!(p.evaluate(eval(&"input".to_string())), Ok(()));
}

#[test]
fn test_std_factory_std_factory_returns_instance_happy() {
    let f = StdPolicyFactory::std_factory();
    // Verify factory is zero-sized marker type
    assert_eq!(std::mem::size_of_val(&f), 0);
}

#[test]
fn test_std_factory_composite_empty_evaluates_ok_happy() {
    let p: CompositePolicy<String> = StdPolicyFactory::composite();
    assert_eq!(p.evaluate(eval(&"input".to_string())), Ok(()));
}

#[test]
fn test_std_factory_composite_with_failing_policy_returns_error() {
    struct AlwaysFail;
    impl Policy for AlwaysFail {
        type Input = String;
        fn name(&self, _req: PolicyNameRequest) -> Result<PolicyNameResponse, PolicyError> {
            Ok(PolicyNameResponse { name: "always-fail" })
        }
        fn evaluate(&self, _req: PolicyEvaluateRequest<'_, String>) -> Result<(), PolicyError> {
            Err(PolicyError::new("always-fail", "always fails"))
        }
    }
    let p = StdPolicyFactory::composite().with(Box::new(AlwaysFail));
    assert!(p.evaluate(eval(&"input".to_string())).is_err());
}

#[test]
fn test_std_factory_is_copy_type_edge() {
    let f = StdPolicyFactory::std_factory();
    let _f2 = f;
    let _f3 = f; // Copy — usable after "move"
    // Verify all copies are identical zero-sized marker types
    assert_eq!(std::mem::size_of_val(&f), std::mem::size_of_val(&_f2));
}
