//! Integration tests for `PermissivePolicy` — covers the types/ file directly.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_lifecycle::{PermissivePolicy, TransitionPolicy};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum S {
    A,
    B,
    C,
}

/// @covers: PermissivePolicy — allows any forward transition
#[test]
fn test_is_allowed_forward_transition_returns_true_happy() {
    let p = PermissivePolicy::new();
    assert!(p.is_allowed(S::A, S::B));
    assert!(p.is_allowed(S::B, S::C));
}

/// @covers: PermissivePolicy — allows self-transition (same state → same state)
#[test]
fn test_is_allowed_self_transition_returns_true_error() {
    let p = PermissivePolicy::new();
    assert!(p.is_allowed(S::A, S::A));
}

/// @covers: PermissivePolicy — allows backward transition (no restrictions)
#[test]
fn test_is_allowed_backward_transition_returns_true_edge() {
    let p = PermissivePolicy::new();
    assert!(p.is_allowed(S::C, S::A));
}
