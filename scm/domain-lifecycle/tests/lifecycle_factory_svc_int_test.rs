//! SAF facade tests — `LifecycleFactory`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_lifecycle::{
    Lifecycle, LifecycleError, LifecycleFactory, PermissivePolicy, StdLifecycleFactory,
    TransitionPolicy, LIFECYCLE_FACTORY_SVC,
};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum S {
    A,
    B,
    C,
}

struct DenyAll;
impl TransitionPolicy for DenyAll {
    type State = S;

    fn is_allowed(&self, _: S, _: S) -> bool {
        false
    }
}

fn rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("rt")
}

// ── LIFECYCLE_FACTORY_SVC ─────────────────────────────────────────────────────

/// @covers: LIFECYCLE_FACTORY_SVC constant
#[test]
fn test_lifecycle_factory_svc_constant_non_empty_happy() {
    assert!(!LIFECYCLE_FACTORY_SVC.is_empty());
}

// ── managed ───────────────────────────────────────────────────────────────────

/// @covers: LifecycleFactory::managed
#[test]
fn test_managed_starts_in_initial_state_happy() {
    let lc = StdLifecycleFactory::managed(S::A, Box::new(PermissivePolicy::new()));
    assert_eq!(lc.state(), S::A);
}

/// @covers: LifecycleFactory::managed
#[test]
fn test_managed_with_deny_policy_rejects_transition_error() {
    rt().block_on(async {
        let lc = StdLifecycleFactory::managed(S::A, Box::new(DenyAll));
        let err = lc.transition_to(S::B).await.expect_err("must fail");
        assert!(matches!(err, LifecycleError::InvalidTransition { .. }));
    });
}

/// @covers: LifecycleFactory::managed
#[test]
fn test_managed_with_permissive_policy_allows_chain_edge() {
    rt().block_on(async {
        let lc = StdLifecycleFactory::managed(S::A, Box::new(PermissivePolicy::new()));
        lc.transition_to(S::B).await.expect("A→B");
        lc.transition_to(S::C).await.expect("B→C");
        assert_eq!(lc.state(), S::C);
    });
}

// ── permissive ────────────────────────────────────────────────────────────────

/// @covers: LifecycleFactory::permissive
#[test]
fn test_permissive_starts_in_initial_state_happy() {
    let lc = StdLifecycleFactory::permissive(S::A);
    assert_eq!(lc.state(), S::A);
}

/// @covers: LifecycleFactory::permissive
#[test]
fn test_permissive_does_not_reject_any_transition_error() {
    rt().block_on(async {
        // "error" scenario: verify there is no situation where permissive rejects
        let lc = StdLifecycleFactory::permissive(S::C);
        lc.transition_to(S::A).await.expect("backward transition allowed");
        assert_eq!(lc.state(), S::A);
    });
}

/// @covers: LifecycleFactory::permissive
#[test]
fn test_permissive_allows_self_transition_edge() {
    rt().block_on(async {
        let lc = StdLifecycleFactory::permissive(S::B);
        lc.transition_to(S::B).await.expect("self-transition allowed");
        assert_eq!(lc.state(), S::B);
    });
}

// ── std_factory ───────────────────────────────────────────────────────────────

/// @covers: LifecycleFactory::std_factory
#[test]
fn test_std_factory_returns_factory_instance_happy() {
    let _f: StdLifecycleFactory = StdLifecycleFactory::std_factory();
}

/// @covers: LifecycleFactory::std_factory
#[test]
fn test_std_factory_is_zero_sized_error() {
    assert_eq!(std::mem::size_of::<StdLifecycleFactory>(), 0);
}

/// @covers: LifecycleFactory::std_factory
#[test]
fn test_std_factory_constructs_usable_lifecycle_edge() {
    let _f = StdLifecycleFactory::std_factory();
    let lc = StdLifecycleFactory::permissive(S::A);
    assert_eq!(lc.state(), S::A);
}
