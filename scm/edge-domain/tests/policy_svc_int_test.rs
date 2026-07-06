//! SAF facade integration tests — the `Policy` trait is exported from the crate
//! root and implementable by downstream consumers.
//!
//! The fixture models a spending-limit rule so the `_error` scenarios exercise a
//! real business violation rather than a contrived assertion.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::{Policy, PolicyError, PolicyEvaluateRequest, PolicyNameRequest, PolicyNameResponse};

/// A policy that rejects transfers exceeding a daily limit.
struct SpendingLimitPolicy {
    limit: u64,
}

impl Policy for SpendingLimitPolicy {
    type Input = u64;

    fn name(&self, _req: PolicyNameRequest) -> Result<PolicyNameResponse, PolicyError> {
        Ok(PolicyNameResponse { name: "spending-limit" })
    }

    fn evaluate(&self, req: PolicyEvaluateRequest<'_, u64>) -> Result<(), PolicyError> {
        if *req.input <= self.limit {
            Ok(())
        } else {
            Err(PolicyError::new(
                "spending-limit",
                format!("amount {} exceeds daily limit of {}", req.input, self.limit),
            ))
        }
    }
}

fn eval(input: &u64) -> PolicyEvaluateRequest<'_, u64> {
    PolicyEvaluateRequest { input }
}

/// @covers: Policy::name
#[test]
fn test_name_returns_policy_label_happy() {
    let p = SpendingLimitPolicy { limit: 100 };
    assert_eq!(p.name(PolicyNameRequest).unwrap().name, "spending-limit");
}

/// @covers: Policy::name
#[test]
fn test_name_is_static_str_edge() {
    let p = SpendingLimitPolicy { limit: 0 };
    let _: &'static str = p.name(PolicyNameRequest).unwrap().name;
}

/// @covers: Policy::evaluate
#[test]
fn test_evaluate_within_limit_returns_ok_happy() {
    let p = SpendingLimitPolicy { limit: 200 };
    assert!(p.evaluate(eval(&150)).is_ok());
}

/// @covers: Policy::evaluate
#[test]
fn test_evaluate_exactly_at_limit_returns_ok_edge() {
    let p = SpendingLimitPolicy { limit: 100 };
    assert!(p.evaluate(eval(&100)).is_ok());
}

/// @covers: Policy::evaluate
#[test]
fn test_evaluate_exceeds_limit_returns_violation_error() {
    let p = SpendingLimitPolicy { limit: 100 };
    let err = p.evaluate(eval(&101)).unwrap_err();
    assert_eq!(err.policy, "spending-limit");
    assert!(err.reason.contains("101"));
    assert!(err.reason.contains("100"));
}
