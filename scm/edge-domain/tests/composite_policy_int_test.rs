//! Integration tests for `CompositePolicy` — AND-composition of `Policy` rules.
//!
//! Two fixture policies (`NonEmpty`, `MaxLength`) model realistic string
//! validation so each scenario exercises a genuine business rule failure path
//! rather than contrived assertions.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::{
    CompositePolicy, Policy, PolicyError, PolicyEvaluateRequest, PolicyNameRequest,
    PolicyNameResponse,
};

struct NonEmpty;
struct MaxLength(usize);

impl Policy for NonEmpty {
    type Input = String;
    fn name(&self, _req: PolicyNameRequest) -> Result<PolicyNameResponse, PolicyError> {
        Ok(PolicyNameResponse { name: "non-empty" })
    }
    fn evaluate(&self, req: PolicyEvaluateRequest<'_, String>) -> Result<(), PolicyError> {
        if req.input.is_empty() {
            Err(PolicyError::new("non-empty", "value must not be empty"))
        } else {
            Ok(())
        }
    }
}

impl Policy for MaxLength {
    type Input = String;
    fn name(&self, _req: PolicyNameRequest) -> Result<PolicyNameResponse, PolicyError> {
        Ok(PolicyNameResponse { name: "max-length" })
    }
    fn evaluate(&self, req: PolicyEvaluateRequest<'_, String>) -> Result<(), PolicyError> {
        if req.input.len() <= self.0 {
            Ok(())
        } else {
            Err(PolicyError::new(
                "max-length",
                format!("length {} exceeds maximum {}", req.input.len(), self.0),
            ))
        }
    }
}

fn eval(input: &String) -> PolicyEvaluateRequest<'_, String> {
    PolicyEvaluateRequest { input }
}

/// @covers: CompositePolicy::new, Policy::evaluate (composite, all pass)
#[test]
fn test_evaluate_all_policies_pass_returns_ok_happy() {
    let policy = CompositePolicy::new()
        .with(Box::new(NonEmpty))
        .with(Box::new(MaxLength(10)));
    let result = policy.evaluate(eval(&"hello".to_string()));
    assert!(result.is_ok(), "both policies should pass for valid input");
    assert_eq!(result.unwrap(), ());
}

/// @covers: Policy::evaluate (composite, first fails)
#[test]
fn test_evaluate_first_policy_fails_short_circuits_error() {
    let policy = CompositePolicy::new()
        .with(Box::new(NonEmpty))
        .with(Box::new(MaxLength(10)));
    let err = policy.evaluate(eval(&String::new())).unwrap_err();
    assert_eq!(err.policy, "non-empty");
}

/// @covers: Policy::evaluate (composite, second fails)
#[test]
fn test_evaluate_second_policy_fails_returns_its_violation_error() {
    let policy = CompositePolicy::new()
        .with(Box::new(NonEmpty))
        .with(Box::new(MaxLength(3)));
    let err = policy.evaluate(eval(&"toolong".to_string())).unwrap_err();
    assert_eq!(err.policy, "max-length");
    assert!(err.reason.contains("7"));
}

/// @covers: Policy::evaluate (composite, empty)
#[test]
fn test_evaluate_empty_composite_always_returns_ok_edge() {
    let policy: CompositePolicy<String> = CompositePolicy::new();
    let result = policy.evaluate(eval(&"anything".to_string()));
    assert_eq!(result, Ok(()), "empty composite should always pass");
}

/// @covers: Policy::name (composite)
#[test]
fn test_name_returns_composite_happy() {
    let policy: CompositePolicy<String> = CompositePolicy::new();
    assert_eq!(policy.name(PolicyNameRequest).unwrap().name, "composite");
}

/// @covers: CompositePolicy::add (single policy)
#[test]
fn test_add_single_policy_is_evaluated_edge() {
    let policy = CompositePolicy::new().with(Box::new(MaxLength(5)));
    let pass = policy.evaluate(eval(&"hi".to_string()));
    assert_eq!(pass, Ok(()), "short string should pass max length");
    let fail = policy.evaluate(eval(&"exceeds".to_string()));
    assert!(fail.is_err(), "long string should fail max length");
    let err = fail.unwrap_err();
    assert_eq!(err.policy, "max-length");
}
