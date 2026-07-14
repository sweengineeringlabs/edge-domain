//! Integration tests for `PolicyError`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::PolicyError;

/// @covers: PolicyError::new
#[test]
fn test_new_sets_policy_field_happy() {
    let v = PolicyError::new("max-length", "input too long");
    assert_eq!(v.policy, "max-length");
}

/// @covers: PolicyError::new
#[test]
fn test_new_sets_reason_field_happy() {
    let v = PolicyError::new("max-length", "input too long");
    assert_eq!(v.reason, "input too long");
}

/// @covers: PolicyError Display
#[test]
fn test_display_includes_policy_and_reason_happy() {
    let v = PolicyError::new("spending-limit", "amount 500 exceeds 200");
    let s = v.to_string();
    assert!(s.contains("spending-limit"));
    assert!(s.contains("amount 500 exceeds 200"));
}

/// @covers: PolicyError PartialEq
#[test]
fn test_two_violations_with_same_fields_are_equal_happy() {
    let a = PolicyError::new("rule", "reason");
    let b = PolicyError::new("rule", "reason");
    assert_eq!(a, b);
}

/// @covers: PolicyError PartialEq
#[test]
fn test_violations_with_different_reason_are_not_equal_error() {
    let a = PolicyError::new("rule", "reason-a");
    let b = PolicyError::new("rule", "reason-b");
    assert_ne!(a, b);
}

/// @covers: PolicyError Clone
#[test]
fn test_clone_produces_equal_violation_edge() {
    let v = PolicyError::new("rule", "reason");
    assert_eq!(v.clone(), v);
}
