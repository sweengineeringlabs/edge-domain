//! Integration tests for the `RepositoryFactory` SAF facade.

use edge_domain::RepositoryFactory;

struct TestRepositories;
impl RepositoryFactory for TestRepositories {}

/// @covers RepositoryFactory::in_memory — happy path: returns a default config (capacity 0)
#[test]
fn test_in_memory_returns_default_capacity_happy() {
    let r = TestRepositories::in_memory();
    assert_eq!(r.initial_capacity, 0, "default capacity is 0");
}

/// @covers RepositoryFactory::in_memory — error: zero capacity is valid, no panic
#[test]
fn test_in_memory_zero_capacity_is_valid_error() {
    let r = TestRepositories::in_memory();
    assert_eq!(r.initial_capacity, 0);
}

/// @covers RepositoryFactory::in_memory — edge: config is a plain cloneable struct
#[test]
fn test_in_memory_config_is_plain_struct_edge() {
    let r = TestRepositories::in_memory();
    let r2 = r.clone();
    assert_eq!(r.initial_capacity, r2.initial_capacity);
}
