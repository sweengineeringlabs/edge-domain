//! Integration tests for the `PolicyFactory` SAF facade.

use edge_domain::{CompositePolicy, Policy, PolicyFactory, PolicyViolation};

struct TestPolicies;
impl PolicyFactory for TestPolicies {}

/// @covers PolicyFactory::composite — happy path: empty composite always passes
#[test]
fn test_composite_empty_always_passes_happy() {
    let p: CompositePolicy<String> = TestPolicies::composite();
    assert!(p.evaluate(&"any".to_string()).is_ok());
}

/// @covers PolicyFactory::composite — error: composite with a failing rule rejects input
#[test]
fn test_composite_with_failing_rule_rejects_input_error() {
    struct Reject;
    impl Policy for Reject {
        type Input = String;
        fn name(&self) -> &'static str {
            "reject-all"
        }
        fn evaluate(&self, _: &String) -> Result<(), PolicyViolation> {
            Err(PolicyViolation::new("reject-all", "always fails"))
        }
    }
    let p: CompositePolicy<String> = TestPolicies::composite::<String>().with(Box::new(Reject));
    assert!(p.evaluate(&"input".to_string()).is_err());
}

/// @covers PolicyFactory::composite — edge: composite is generic over input type
#[test]
fn test_composite_generic_over_input_type_edge() {
    let _p_str: CompositePolicy<String> = TestPolicies::composite();
    let _p_u32: CompositePolicy<u32> = TestPolicies::composite();
}
