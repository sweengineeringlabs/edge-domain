//! Integration tests for `InMemoryRepository` api type and `new_in_memory_repository` factory.

use edge_domain::{
    new_in_memory_queryable_repository, new_in_memory_repository, QueryableRepository, Repository,
    Spec,
};
use std::sync::Arc;

/// @covers: new_in_memory_repository
#[test]
fn test_new_in_memory_repository_factory_returns_arc() {
    let _: Arc<dyn Repository<String, u32>> = Domain::new_in_memory_repository();
}

/// @covers: new_in_memory_queryable_repository
#[test]
fn test_new_in_memory_queryable_repository_factory_returns_arc() {
    let _: Arc<dyn QueryableRepository<String, u32>> = Domain::new_in_memory_queryable_repository();
}

/// @covers: new_in_memory_repository
#[test]
fn test_new_in_memory_repository() {
    let _: Arc<dyn Repository<String, u32>> = Domain::new_in_memory_repository();
}

/// @covers: new_in_memory_queryable_repository
#[test]
fn test_new_in_memory_queryable_repository() {
    let _: Arc<dyn QueryableRepository<String, u32>> = Domain::new_in_memory_queryable_repository();
}

/// @covers: new_in_memory_repository
#[tokio::test]
async fn test_new_in_memory_repository_save_find_round_trip() {
    let repo: Arc<dyn Repository<String, u32>> = Domain::new_in_memory_repository();
    repo.save(1u32, "hello".to_string()).await.unwrap();
    assert_eq!(repo.find(&1u32).await.unwrap().as_deref(), Some("hello"));
}

/// @covers: new_in_memory_queryable_repository
#[tokio::test]
async fn test_new_in_memory_queryable_repository_find_by_spec() {
    struct LongStr;
    impl Spec<String> for LongStr {
        fn matches(&self, s: &String) -> bool {
            s.len() > 3
        }
    }
    let repo: Arc<dyn QueryableRepository<String, u32>> =
        Domain::new_in_memory_queryable_repository();
    repo.save(1u32, "hi".to_string()).await.unwrap();
    repo.save(2u32, "hello".to_string()).await.unwrap();
    let results = repo.find_by(&LongStr).await.unwrap();
    assert_eq!(results.len(), 1);
}
