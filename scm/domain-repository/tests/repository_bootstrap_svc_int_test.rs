//! Rule-222 coverage for [`RepositoryBootstrap`] trait fn `bootstrap_name`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_repository::RepositoryBootstrap;

struct Repos;
impl RepositoryBootstrap for Repos {}

/// @covers: RepositoryBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_returns_nonempty_string_happy() {
    let f = Repos;
    assert!(!f.bootstrap_name().is_empty(), "bootstrap_name must return a non-empty identifier");
}

/// @covers: RepositoryBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_idempotent_error() {
    let f = Repos;
    assert_eq!(
        f.bootstrap_name(),
        f.bootstrap_name(),
        "bootstrap_name must return the same value on repeated calls"
    );
}

/// @covers: RepositoryBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_callable_via_trait_object_edge() {
    let f: &dyn RepositoryBootstrap = &Repos;
    let _ = f.bootstrap_name();
}
