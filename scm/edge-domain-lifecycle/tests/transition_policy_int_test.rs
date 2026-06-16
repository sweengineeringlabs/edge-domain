//! Integration tests for `TransitionPolicy` — covers the traits/ file directly.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_lifecycle::TransitionPolicy;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Gate {
    Open,
    Closed,
    Locked,
}

/// A custom policy: Locked is terminal — nothing transitions out of it.
struct LockFinalPolicy;

impl TransitionPolicy for LockFinalPolicy {
    type State = Gate;

    fn is_allowed(&self, _from: Gate, to: Gate) -> bool {
        to != Gate::Locked || _from == Gate::Open
    }
}

/// @covers: TransitionPolicy::is_allowed — custom policy allows a permitted transition
#[test]
fn test_is_allowed_permitted_transition_returns_true_happy() {
    let p = LockFinalPolicy;
    assert!(p.is_allowed(Gate::Open, Gate::Closed));
    assert!(p.is_allowed(Gate::Closed, Gate::Open));
}

/// @covers: TransitionPolicy::is_allowed — custom policy rejects a forbidden transition
#[test]
fn test_is_allowed_forbidden_transition_returns_false_error() {
    let p = LockFinalPolicy;
    assert!(!p.is_allowed(Gate::Closed, Gate::Locked));
}

/// @covers: TransitionPolicy::is_allowed — self-transition semantics depend on policy
#[test]
fn test_is_allowed_self_transition_depends_on_policy_edge() {
    let p = LockFinalPolicy;
    // Open→Open: `to` is Open, not Locked, so allowed
    assert!(p.is_allowed(Gate::Open, Gate::Open));
    // Locked→Locked: `to` is Locked and `from` is not Open, so forbidden
    assert!(!p.is_allowed(Gate::Locked, Gate::Locked));
}
