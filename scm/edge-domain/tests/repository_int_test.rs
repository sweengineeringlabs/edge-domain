//! Integration tests for the `Repository` trait contract and `Domain::new_in_memory_repository()`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::{Domain, Repository, RepositoryError};
use futures::future::BoxFuture;
use std::sync::Arc;

// ── custom implementation ────────────────────────────────────────────────────

struct ReadOnlyRepo;

impl Repository for ReadOnlyRepo {
    type Entity = String;
    type Id = u64;
    fn find<'a>(&'a self, _id: &'a u64) -> BoxFuture<'a, Result<Option<String>, RepositoryError>> {
        Box::pin(async { Ok(Some("fixed".into())) })
    }
    fn save(&self, _id: u64, _entity: String) -> BoxFuture<'_, Result<(), RepositoryError>> {
        Box::pin(async { Err(RepositoryError::Internal("read-only".into())) })
    }
    fn delete<'a>(&'a self, _id: &'a u64) -> BoxFuture<'a, Result<bool, RepositoryError>> {
        Box::pin(async { Ok(false) })
    }
    fn list(&self) -> BoxFuture<'_, Result<Vec<String>, RepositoryError>> {
        Box::pin(async { Ok(vec!["fixed".into()]) })
    }
}

// ── trait contract tests ─────────────────────────────────────────────────────

/// @covers: Repository::find
#[tokio::test]
async fn test_repository_trait_find_returns_value_for_known_id() {
    let repo: Arc<dyn Repository<Entity = String, Id = u64>> = Arc::new(ReadOnlyRepo);
    assert_eq!(repo.find(&0).await.unwrap().as_deref(), Some("fixed"));
}

/// @covers: Repository::save
#[tokio::test]
async fn test_repository_trait_save_propagates_error_from_impl() {
    let repo: Arc<dyn Repository<Entity = String, Id = u64>> = Arc::new(ReadOnlyRepo);
    assert!(repo.save(0, "x".into()).await.is_err());
}

// ── new_in_memory_repository factory ─────────────────────────────────────────────

/// @covers: new_in_memory_repository
#[tokio::test]
async fn test_new_in_memory_repository_save_and_find_round_trips_entity() {
    let repo: Arc<dyn Repository<Entity = String, Id = u64>> = Domain::new_in_memory_repository();
    repo.save(1, "hello".into()).await.unwrap();
    assert_eq!(repo.find(&1).await.unwrap().as_deref(), Some("hello"));
}

/// @covers: new_in_memory_repository
#[tokio::test]
async fn test_new_in_memory_repository_find_returns_none_for_missing_id() {
    let repo: Arc<dyn Repository<Entity = String, Id = u64>> = Domain::new_in_memory_repository();
    assert!(repo.find(&99).await.unwrap().is_none());
}

/// @covers: new_in_memory_repository
#[tokio::test]
async fn test_new_in_memory_repository_delete_removes_entity() {
    let repo: Arc<dyn Repository<Entity = String, Id = u64>> = Domain::new_in_memory_repository();
    repo.save(1, "bye".into()).await.unwrap();
    assert!(repo.delete(&1).await.unwrap());
    assert!(repo.find(&1).await.unwrap().is_none());
}

/// @covers: new_in_memory_repository
#[tokio::test]
async fn test_new_in_memory_repository_list_returns_all_saved_entities() {
    let repo: Arc<dyn Repository<Entity = String, Id = u64>> = Domain::new_in_memory_repository();
    repo.save(1, "a".into()).await.unwrap();
    repo.save(2, "b".into()).await.unwrap();
    let mut items = repo.list().await.unwrap();
    items.sort();
    assert_eq!(items, vec!["a", "b"]);
}
