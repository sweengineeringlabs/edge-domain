//! Integration tests for `PolicyViolation`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::PolicyViolation;

/// @covers: PolicyViolation::new
#[test]
fn test_new_sets_policy_field_happy() {
    let v = PolicyViolation::new("max-length", "input too long");
    assert_eq!(v.policy, "max-length");
}

/// @covers: PolicyViolation::new
#[test]
fn test_new_sets_reason_field_happy() {
    let v = PolicyViolation::new("max-length", "input too long");
    assert_eq!(v.reason, "input too long");
}

/// @covers: PolicyViolation Display
#[test]
fn test_display_includes_policy_and_reason_happy() {
    let v = PolicyViolation::new("spending-limit", "amount 500 exceeds 200");
    let s = v.to_string();
    assert!(s.contains("spending-limit"));
    assert!(s.contains("amount 500 exceeds 200"));
}

/// @covers: PolicyViolation PartialEq
#[test]
fn test_two_violations_with_same_fields_are_equal_happy() {
    let a = PolicyViolation::new("rule", "reason");
    let b = PolicyViolation::new("rule", "reason");
    assert_eq!(a, b);
}

/// @covers: PolicyViolation PartialEq
#[test]
fn test_violations_with_different_reason_are_not_equal_error() {
    let a = PolicyViolation::new("rule", "reason-a");
    let b = PolicyViolation::new("rule", "reason-b");
    assert_ne!(a, b);
}

/// @covers: PolicyViolation Clone
#[test]
fn test_clone_produces_equal_violation_edge() {
    let v = PolicyViolation::new("rule", "reason");
    assert_eq!(v.clone(), v);
}
