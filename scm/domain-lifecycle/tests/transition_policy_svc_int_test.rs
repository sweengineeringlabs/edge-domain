//! SAF facade tests — `TransitionPolicy` service (`transition_policy_svc`).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_lifecycle::{TransitionPolicy, TRANSITION_POLICY_SVC};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum State {
    Off,
    On,
    Fault,
}

/// Policy: On→Fault and Fault→Off are the only allowed transitions.
struct RecoveryOnlyPolicy;

impl TransitionPolicy for RecoveryOnlyPolicy {
    type State = State;

    fn is_allowed(&self, from: State, to: State) -> bool {
        matches!((from, to), (State::On, State::Fault) | (State::Fault, State::Off))
    }
}

// ── TRANSITION_POLICY_SVC ─────────────────────────────────────────────────────

/// @covers: TRANSITION_POLICY_SVC — constant is non-empty
#[test]
fn test_transition_policy_svc_constant_is_non_empty_happy() {
    assert!(!TRANSITION_POLICY_SVC.is_empty());
}

// ── TransitionPolicy::is_allowed ──────────────────────────────────────────────

/// @covers: TransitionPolicy::is_allowed — permitted transition returns true
#[test]
fn test_is_allowed_permitted_transition_returns_true_happy() {
    let p = RecoveryOnlyPolicy;
    assert!(p.is_allowed(State::On, State::Fault));
    assert!(p.is_allowed(State::Fault, State::Off));
}

/// @covers: TransitionPolicy::is_allowed — forbidden transition returns false
#[test]
fn test_is_allowed_forbidden_transition_returns_false_error() {
    let p = RecoveryOnlyPolicy;
    assert!(!p.is_allowed(State::Off, State::On));
    assert!(!p.is_allowed(State::On, State::Off));
}

/// @covers: TransitionPolicy::is_allowed — self-transition obeys policy
#[test]
fn test_is_allowed_self_transition_obeys_policy_edge() {
    let p = RecoveryOnlyPolicy;
    assert!(!p.is_allowed(State::On, State::On));
    assert!(!p.is_allowed(State::Off, State::Off));
}
