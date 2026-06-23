//! SAF facade tests — `PolicyBootstrap` constructors.

use edge_domain_policy::{Policy, PolicyBootstrap, PolicyViolation};

struct TestPolicies;
impl PolicyBootstrap for TestPolicies {}

struct AlwaysFails;
impl Policy for AlwaysFails {
    type Input = String;
    fn name(&self) -> &'static str {
        "always-fails"
    }
    fn evaluate(&self, _input: &String) -> Result<(), PolicyViolation> {
        Err(PolicyViolation::new("always-fails", "denied"))
    }
}

/// @covers: PolicyBootstrap::composite — empty composite always passes
#[test]
fn test_composite_empty_always_passes_happy() {
    let policy = TestPolicies::composite::<String>();
    assert_eq!(policy.evaluate(&"anything".to_string()), Ok(()), "empty composite should pass");
}

/// @covers: PolicyBootstrap::composite — first failing rule rejects input
#[test]
fn test_composite_with_failing_rule_rejects_input_error() {
    let policy = TestPolicies::composite::<String>().with(Box::new(AlwaysFails));
    assert!(policy.evaluate(&"input".to_string()).is_err());
}

/// @covers: PolicyBootstrap::composite — generic over input type
#[test]
fn test_composite_generic_over_input_type_edge() {
    let policy = TestPolicies::composite::<u64>();
    assert!(policy.evaluate(&42).is_ok());
}
