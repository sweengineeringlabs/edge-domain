//! Integration tests for the `RepositoryBootstrap` SAF facade.
#![allow(clippy::unwrap_used)]

use edge_domain::{Repository, RepositoryBootstrap};

struct TestRepositories;
impl RepositoryBootstrap for TestRepositories {}

/// @covers RepositoryBootstrap::in_memory — happy path: fresh store has no entries
#[tokio::test]
async fn test_in_memory_returns_fresh_store_happy() {
    let r = TestRepositories::in_memory::<String, u32>();
    assert!(r.find(&0u32).await.unwrap().is_none());
}

/// @covers RepositoryBootstrap::in_memory — error: store is non-zero-size (heap-backed)
#[test]
fn test_in_memory_is_nonzero_size_error() {
    assert_ne!(
        std::mem::size_of_val(&TestRepositories::in_memory::<String, u32>()),
        0,
    );
}

/// @covers RepositoryBootstrap::in_memory — edge: store is usable for generic types
#[tokio::test]
async fn test_in_memory_accepts_generic_types_edge() {
    let r = TestRepositories::in_memory::<u64, String>();
    r.save("key".to_string(), 42u64).await.unwrap();
    assert_eq!(r.find(&"key".to_string()).await.unwrap(), Some(42u64));
}
