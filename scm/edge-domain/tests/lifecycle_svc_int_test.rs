//! Umbrella-level integration tests that exercise `edge-domain-lifecycle` as a
//! dependency — verifying the sub-crate contract is accessible end-to-end.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_lifecycle::{
    Lifecycle, LifecycleStateRequest, LifecycleTransitionRequest, ManagedLifecycle,
    PermissivePolicy,
};

fn permissive<S: Copy + Eq + std::fmt::Debug + Send + Sync + 'static>(
    initial: S,
) -> ManagedLifecycle<S> {
    ManagedLifecycle::new(initial, Box::new(PermissivePolicy::new()))
}

/// @covers: Lifecycle::state, Lifecycle::transition_to (PermissivePolicy)
#[tokio::test]
async fn test_managed_lifecycle_state_returns_initial_state_happy() {
    let lifecycle = permissive("initial");
    assert_eq!(
        lifecycle.state(LifecycleStateRequest).expect("state").state,
        "initial"
    );
}

/// @covers: Lifecycle::transition_to
#[tokio::test]
async fn test_managed_lifecycle_transition_to_changes_state_happy() {
    let lifecycle = permissive("initial");
    lifecycle
        .transition_to(LifecycleTransitionRequest { target: "next" })
        .await
        .expect("transition should succeed");
    assert_eq!(
        lifecycle.state(LifecycleStateRequest).expect("state").state,
        "next"
    );
}

/// @covers: Lifecycle::transition_to (multiple transitions)
#[tokio::test]
async fn test_managed_lifecycle_multiple_transitions_happy() {
    let lifecycle = permissive(1);
    lifecycle
        .transition_to(LifecycleTransitionRequest { target: 2 })
        .await
        .expect("first transition should succeed");
    lifecycle
        .transition_to(LifecycleTransitionRequest { target: 3 })
        .await
        .expect("second transition should succeed");
    assert_eq!(
        lifecycle.state(LifecycleStateRequest).expect("state").state,
        3
    );
}

/// @covers: PermissivePolicy::is_allowed (via ManagedLifecycle::new)
#[tokio::test]
async fn test_permissive_policy_allows_all_transitions_happy() {
    let lifecycle = permissive("A");
    assert!(lifecycle
        .transition_to(LifecycleTransitionRequest { target: "B" })
        .await
        .is_ok());
    assert!(lifecycle
        .transition_to(LifecycleTransitionRequest { target: "A" })
        .await
        .is_ok());
    assert!(lifecycle
        .transition_to(LifecycleTransitionRequest { target: "A" })
        .await
        .is_ok());
}

/// @covers: PermissivePolicy default construction (via ManagedLifecycle::new)
#[tokio::test]
async fn test_permissive_policy_default_creates_policy_happy() {
    let lifecycle = permissive(1u32);
    assert!(lifecycle
        .transition_to(LifecycleTransitionRequest { target: 2u32 })
        .await
        .is_ok());
}

/// @covers: Lifecycle state with zero/boundary values
#[tokio::test]
async fn test_managed_lifecycle_with_zero_initial_state_edge() {
    let lifecycle = permissive(0);
    assert_eq!(
        lifecycle.state(LifecycleStateRequest).expect("state").state,
        0
    );
}

/// @covers: Lifecycle state with empty string
#[tokio::test]
async fn test_managed_lifecycle_with_empty_string_state_edge() {
    let lifecycle = permissive("");
    assert_eq!(
        lifecycle.state(LifecycleStateRequest).expect("state").state,
        ""
    );
}
