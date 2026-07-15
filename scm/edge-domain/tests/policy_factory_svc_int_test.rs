//! Integration tests for the `CompositePolicy` SAF facade.
#![cfg(feature = "policy")]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::{
    CompositePolicy, Policy, PolicyError, PolicyEvaluateRequest, PolicyNameRequest,
    PolicyNameResponse,
};

/// @covers CompositePolicy::new — happy path: empty composite always passes
#[test]
fn test_composite_empty_always_passes_happy() {
    let p: CompositePolicy<String> = CompositePolicy::new();
    assert_eq!(
        p.evaluate(PolicyEvaluateRequest {
            input: &"any".to_string()
        }),
        Ok(())
    );
}

/// @covers CompositePolicy::new — error: composite with a failing rule rejects input
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
    let p: CompositePolicy<String> = CompositePolicy::<String>::new().with(Box::new(Reject));
    assert!(p
        .evaluate(PolicyEvaluateRequest {
            input: &"input".to_string()
        })
        .is_err());
}

/// @covers CompositePolicy::new — edge: composite is generic over input type
#[test]
fn test_composite_generic_over_input_type_edge() {
    let p_str: CompositePolicy<String> = CompositePolicy::new();
    let p_u32: CompositePolicy<u32> = CompositePolicy::new();
    assert_eq!(p_str.name(PolicyNameRequest).unwrap().name, "composite");
    assert_eq!(p_u32.name(PolicyNameRequest).unwrap().name, "composite");
}
