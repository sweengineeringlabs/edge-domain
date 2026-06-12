//! Integration tests for `CompositePolicy` — AND-composition of `Policy` rules.
//!
//! Two fixture policies (`NonEmpty`, `MaxLength`) model realistic string
//! validation so each scenario exercises a genuine business rule failure path
//! rather than contrived assertions.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::{CompositePolicy, Policy, PolicyViolation};

struct NonEmpty;
struct MaxLength(usize);

impl Policy for NonEmpty {
    type Input = String;
    fn name(&self) -> &'static str {
        "non-empty"
    }
    fn evaluate(&self, input: &String) -> Result<(), PolicyViolation> {
        if input.is_empty() {
            Err(PolicyViolation::new("non-empty", "value must not be empty"))
        } else {
            Ok(())
        }
    }
}

impl Policy for MaxLength {
    type Input = String;
    fn name(&self) -> &'static str {
        "max-length"
    }
    fn evaluate(&self, input: &String) -> Result<(), PolicyViolation> {
        if input.len() <= self.0 {
            Ok(())
        } else {
            Err(PolicyViolation::new(
                "max-length",
                format!("length {} exceeds maximum {}", input.len(), self.0),
            ))
        }
    }
}

/// @covers: CompositePolicy::new, Policy::evaluate (composite, all pass)
#[test]
fn test_evaluate_all_policies_pass_returns_ok_happy() {
    let policy = CompositePolicy::new()
        .with(Box::new(NonEmpty))
        .with(Box::new(MaxLength(10)));
    assert!(policy.evaluate(&"hello".to_string()).is_ok());
}

/// @covers: Policy::evaluate (composite, first fails)
#[test]
fn test_evaluate_first_policy_fails_short_circuits_error() {
    let policy = CompositePolicy::new()
        .with(Box::new(NonEmpty))
        .with(Box::new(MaxLength(10)));
    let err = policy.evaluate(&String::new()).unwrap_err();
    assert_eq!(err.policy, "non-empty");
}

/// @covers: Policy::evaluate (composite, second fails)
#[test]
fn test_evaluate_second_policy_fails_returns_its_violation_error() {
    let policy = CompositePolicy::new()
        .with(Box::new(NonEmpty))
        .with(Box::new(MaxLength(3)));
    let err = policy.evaluate(&"toolong".to_string()).unwrap_err();
    assert_eq!(err.policy, "max-length");
    assert!(err.reason.contains("7"));
}

/// @covers: Policy::evaluate (composite, empty)
#[test]
fn test_evaluate_empty_composite_always_returns_ok_edge() {
    let policy: CompositePolicy<String> = CompositePolicy::new();
    assert!(policy.evaluate(&"anything".to_string()).is_ok());
}

/// @covers: Policy::name (composite)
#[test]
fn test_name_returns_composite_happy() {
    let policy: CompositePolicy<String> = CompositePolicy::new();
    assert_eq!(policy.name(), "composite");
}

/// @covers: CompositePolicy::add (single policy)
#[test]
fn test_add_single_policy_is_evaluated_edge() {
    let policy = CompositePolicy::new().with(Box::new(MaxLength(5)));
    assert!(policy.evaluate(&"hi".to_string()).is_ok());
    assert!(policy.evaluate(&"exceeds".to_string()).is_err());
}
