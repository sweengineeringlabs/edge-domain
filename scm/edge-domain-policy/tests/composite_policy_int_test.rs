//! Integration tests for `CompositePolicy` AND-composition.

use edge_domain_policy::{CompositePolicy, Policy, PolicyViolation};

struct Reject(&'static str);
impl Policy for Reject {
    type Input = String;
    fn name(&self) -> &'static str {
        self.0
    }
    fn evaluate(&self, _input: &String) -> Result<(), PolicyViolation> {
        Err(PolicyViolation::new(self.0, "denied"))
    }
}

struct Accept;
impl Policy for Accept {
    type Input = String;
    fn name(&self) -> &'static str {
        "accept"
    }
    fn evaluate(&self, _input: &String) -> Result<(), PolicyViolation> {
        Ok(())
    }
}

/// @covers: CompositePolicy::new + evaluate — empty composite passes
#[test]
fn test_evaluate_empty_composite_passes_happy() {
    let c: CompositePolicy<String> = CompositePolicy::new();
    assert!(c.evaluate(&"x".to_string()).is_ok());
}

/// @covers: CompositePolicy::with + evaluate — failing member short-circuits
#[test]
fn test_evaluate_failing_member_returns_err_error() {
    let c = CompositePolicy::new()
        .with(Box::new(Accept))
        .with(Box::new(Reject("second")));
    let err = c.evaluate(&"x".to_string());
    assert!(err.is_err());
}

/// @covers: CompositePolicy — first violation wins
#[test]
fn test_evaluate_first_violation_short_circuits_edge() {
    let c = CompositePolicy::new()
        .with(Box::new(Reject("first")))
        .with(Box::new(Reject("second")));
    let err = c.evaluate(&"x".to_string()).unwrap_err();
    assert_eq!(err.policy, "first");
}
