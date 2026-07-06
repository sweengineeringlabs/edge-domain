//! SAF facade tests — `PolicyBootstrap` constructors.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_policy::{Policy, PolicyBootstrap, PolicyEvaluateRequest, PolicyNameRequest, PolicyNameResponse, PolicyError};

struct TestPolicies;
impl PolicyBootstrap for TestPolicies {}

struct AlwaysFails;
impl Policy for AlwaysFails {
    type Input = String;
    fn name(&self, _req: PolicyNameRequest) -> Result<PolicyNameResponse, PolicyError> {
        Ok(PolicyNameResponse { name: "always-fails" })
    }
    fn evaluate(&self, _req: PolicyEvaluateRequest<'_, String>) -> Result<(), PolicyError> {
        Err(PolicyError::new("always-fails", "denied"))
    }
}

/// @covers: PolicyBootstrap::composite — empty composite always passes
#[test]
fn test_composite_empty_always_passes_happy() {
    let policy = TestPolicies::composite::<String>();
    assert_eq!(
        policy.evaluate(PolicyEvaluateRequest { input: &"anything".to_string() }),
        Ok(()),
        "empty composite should pass"
    );
}

/// @covers: PolicyBootstrap::composite — first failing rule rejects input
#[test]
fn test_composite_with_failing_rule_rejects_input_error() {
    let policy = TestPolicies::composite::<String>().with(Box::new(AlwaysFails));
    let result = policy.evaluate(PolicyEvaluateRequest { input: &"input".to_string() });
    assert!(matches!(result, Err(v) if v.policy == "always-fails"), "should reject with always-fails policy");
}

/// @covers: PolicyBootstrap::composite — generic over input type
#[test]
fn test_composite_generic_over_input_type_edge() {
    let policy = TestPolicies::composite::<u64>();
    assert_eq!(policy.evaluate(PolicyEvaluateRequest { input: &42 }), Ok(()), "empty composite with u64 should pass");
}
