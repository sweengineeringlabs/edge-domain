//! Integration tests for the `PolicyBootstrap` SAF facade.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::{
    CompositePolicy, Policy, PolicyBootstrap, PolicyError, PolicyEvaluateRequest,
    PolicyNameRequest, PolicyNameResponse,
};

struct TestPolicies;
impl PolicyBootstrap for TestPolicies {}

/// @covers PolicyBootstrap::composite — happy path: empty composite always passes
#[test]
fn test_composite_empty_always_passes_happy() {
    let p: CompositePolicy<String> = TestPolicies::composite();
    assert!(p
        .evaluate(PolicyEvaluateRequest { input: &"any".to_string() })
        .is_ok());
}

/// @covers PolicyBootstrap::composite — error: composite with a failing rule rejects input
#[test]
fn test_composite_with_failing_rule_rejects_input_error() {
    struct Reject;
    impl Policy for Reject {
        type Input = String;
        fn name(&self, _req: PolicyNameRequest) -> Result<PolicyNameResponse, PolicyError> {
            Ok(PolicyNameResponse { name: "reject-all" })
        }
        fn evaluate(&self, _req: PolicyEvaluateRequest<'_, String>) -> Result<(), PolicyError> {
            Err(PolicyError::new("reject-all", "always fails"))
        }
    }
    let p: CompositePolicy<String> = TestPolicies::composite::<String>().with(Box::new(Reject));
    assert!(p
        .evaluate(PolicyEvaluateRequest { input: &"input".to_string() })
        .is_err());
}

/// @covers PolicyBootstrap::composite — edge: composite is generic over input type
#[test]
fn test_composite_generic_over_input_type_edge() {
    let _p_str: CompositePolicy<String> = TestPolicies::composite();
    let _p_u32: CompositePolicy<u32> = TestPolicies::composite();
}
