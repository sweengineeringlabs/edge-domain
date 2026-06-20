//! Integration tests for `StdLifecycleFactory` — covers the types/ file directly.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_lifecycle::{Lifecycle, LifecycleBootstrap, PermissivePolicy, StdLifecycleFactory};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum S {
    A,
    B,
}

fn rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("rt")
}

/// @covers: StdLifecycleFactory — constructs via literal and `std_factory`
#[test]
fn test_std_lifecycle_factory_constructs_happy() {
    let _f = StdLifecycleFactory;
    let _g = StdLifecycleFactory::std_factory();
}

/// @covers: StdLifecycleFactory — zero-sized marker type
#[test]
fn test_std_lifecycle_factory_zero_sized_error() {
    assert_eq!(std::mem::size_of::<StdLifecycleFactory>(), 0);
}

/// @covers: StdLifecycleFactory — `managed` returns a working lifecycle
#[test]
fn test_std_lifecycle_factory_managed_returns_lifecycle_edge() {
    rt().block_on(async {
        let lc = StdLifecycleFactory::managed(S::A, Box::new(PermissivePolicy::new()));
        lc.transition_to(S::B).await.expect("transition");
        assert_eq!(lc.state(), S::B);
    });
}

/// @covers: StdLifecycleFactory — `permissive` convenience constructor
#[test]
fn test_std_lifecycle_factory_permissive_allows_all_transitions_happy() {
    rt().block_on(async {
        let lc = StdLifecycleFactory::permissive(S::A);
        lc.transition_to(S::B).await.expect("transition");
        assert_eq!(lc.state(), S::B);
    });
}
