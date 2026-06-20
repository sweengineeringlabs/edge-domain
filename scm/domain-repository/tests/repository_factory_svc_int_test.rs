//! SAF facade tests — `RepositoryBootstrap` constructors.
// @allow: no_mocks_in_integration — InMemoryRepository is the production-shipped reference impl, not a test double

use edge_domain_repository::{InMemoryRepository, Repository, RepositoryBootstrap};
use futures::executor::block_on;

struct Repos;
impl RepositoryBootstrap for Repos {}

/// @covers: RepositoryBootstrap::in_memory — returns a usable, empty store
#[test]
fn test_in_memory_returns_empty_store_happy() {
    let repo: InMemoryRepository<String, u32> = Repos::in_memory();
    let items = block_on(repo.list()).unwrap_or_else(|_| vec!["x".into()]);
    assert!(items.is_empty());
}

/// @covers: RepositoryBootstrap::in_memory — two calls return independent instances
#[test]
fn test_in_memory_independent_instances_are_isolated_error() {
    let a: InMemoryRepository<String, u32> = Repos::in_memory();
    let b: InMemoryRepository<String, u32> = Repos::in_memory();
    block_on(a.save(1, "hello".into())).unwrap_or_default();
    let in_b = block_on(b.find(&1)).unwrap_or(Some("x".into()));
    assert!(in_b.is_none());
}

/// @covers: RepositoryBootstrap::in_memory — Default trait also creates an empty store
#[test]
fn test_in_memory_default_creates_empty_store_edge() {
    let repo: InMemoryRepository<u32, u32> = Default::default();
    let items = block_on(repo.list()).unwrap_or_else(|_| vec![1]);
    assert!(items.is_empty());
}
