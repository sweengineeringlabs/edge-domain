//! Umbrella-level integration tests that exercise `edge-domain-lifecycle` as a
//! dependency — verifying the sub-crate contract is accessible end-to-end.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_lifecycle::{Lifecycle, ManagedLifecycle, PermissivePolicy, TransitionPolicy};

/// @covers: Lifecycle::state, Lifecycle::transition_to (PermissivePolicy)
#[tokio::test]
async fn test_managed_lifecycle_state_returns_initial_state_happy() {
    let policy = PermissivePolicy::new();
    let lifecycle = ManagedLifecycle::new("initial", Box::new(policy));
    assert_eq!(lifecycle.state(), "initial");
}

/// @covers: Lifecycle::transition_to
#[tokio::test]
async fn test_managed_lifecycle_transition_to_changes_state_happy() {
    let policy = PermissivePolicy::new();
    let lifecycle = ManagedLifecycle::new("initial", Box::new(policy));
    lifecycle
        .transition_to("next")
        .await
        .expect("transition should succeed");
    assert_eq!(lifecycle.state(), "next");
}

/// @covers: Lifecycle::transition_to (multiple transitions)
#[tokio::test]
async fn test_managed_lifecycle_multiple_transitions_happy() {
    let policy = PermissivePolicy::new();
    let lifecycle = ManagedLifecycle::new(1, Box::new(policy));
    lifecycle
        .transition_to(2)
        .await
        .expect("first transition should succeed");
    lifecycle
        .transition_to(3)
        .await
        .expect("second transition should succeed");
    assert_eq!(lifecycle.state(), 3);
}

/// @covers: PermissivePolicy::is_allowed
#[test]
fn test_permissive_policy_allows_all_transitions_happy() {
    let policy = PermissivePolicy::new();
    assert!(policy.is_allowed("A", "B"));
    assert!(policy.is_allowed("B", "A"));
    assert!(policy.is_allowed("A", "A"));
}

/// @covers: PermissivePolicy default construction
#[test]
fn test_permissive_policy_default_creates_policy_happy() {
    let policy = PermissivePolicy::<u32>::default();
    assert!(policy.is_allowed(1, 2));
}

/// @covers: Lifecycle state with zero/boundary values
#[test]
fn test_managed_lifecycle_with_zero_initial_state_edge() {
    let policy = PermissivePolicy::new();
    let lifecycle = ManagedLifecycle::new(0, Box::new(policy));
    assert_eq!(lifecycle.state(), 0);
}

/// @covers: Lifecycle state with empty string
#[test]
fn test_managed_lifecycle_with_empty_string_state_edge() {
    let policy = PermissivePolicy::new();
    let lifecycle = ManagedLifecycle::new("", Box::new(policy));
    assert_eq!(lifecycle.state(), "");
}
