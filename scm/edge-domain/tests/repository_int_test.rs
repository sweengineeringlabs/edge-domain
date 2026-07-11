//! Integration tests for the `Repository` trait contract and `Domain.new_in_memory_repository()`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::{
    Domain, Repository, RepositoryDeleteResponse, RepositoryError, RepositoryFindResponse,
    RepositoryIdRequest, RepositoryListRequest, RepositoryListResponse, RepositorySaveRequest,
};
use futures::future::BoxFuture;
use std::sync::Arc;

// ── custom implementation ────────────────────────────────────────────────────

struct ReadOnlyRepo;

impl Repository for ReadOnlyRepo {
    type Entity = String;
    type Id = u64;
    fn find<'a>(
        &'a self,
        _req: RepositoryIdRequest<'a, u64>,
    ) -> BoxFuture<'a, Result<RepositoryFindResponse<String>, RepositoryError>> {
        Box::pin(async {
            Ok(RepositoryFindResponse {
                entity: Some("fixed".into()),
            })
        })
    }
    fn save(
        &self,
        _req: RepositorySaveRequest<u64, String>,
    ) -> BoxFuture<'_, Result<(), RepositoryError>> {
        Box::pin(async { Err(RepositoryError::Internal("read-only".into())) })
    }
    fn delete<'a>(
        &'a self,
        _req: RepositoryIdRequest<'a, u64>,
    ) -> BoxFuture<'a, Result<RepositoryDeleteResponse, RepositoryError>> {
        Box::pin(async { Ok(RepositoryDeleteResponse { removed: false }) })
    }
    fn list(
        &self,
        _req: RepositoryListRequest,
    ) -> BoxFuture<'_, Result<RepositoryListResponse<String>, RepositoryError>> {
        Box::pin(async {
            Ok(RepositoryListResponse {
                items: vec!["fixed".into()],
            })
        })
    }
}

// ── trait contract tests ─────────────────────────────────────────────────────

/// @covers: Repository::find
#[tokio::test]
async fn test_repository_trait_find_returns_value_for_known_id() {
    let repo: Arc<dyn Repository<Entity = String, Id = u64>> = Arc::new(ReadOnlyRepo);
    let found = repo.find(RepositoryIdRequest { id: &0 }).await.unwrap();
    assert_eq!(found.entity.as_deref(), Some("fixed"));
}

/// @covers: Repository::save
#[tokio::test]
async fn test_repository_trait_save_propagates_error_from_impl() {
    let repo: Arc<dyn Repository<Entity = String, Id = u64>> = Arc::new(ReadOnlyRepo);
    assert!(repo
        .save(RepositorySaveRequest {
            id: 0,
            entity: "x".into()
        })
        .await
        .is_err());
}

// ── new_in_memory_repository factory ─────────────────────────────────────────────

/// @covers: new_in_memory_repository
#[tokio::test]
async fn test_new_in_memory_repository_save_and_find_round_trips_entity() {
    let repo: Arc<dyn Repository<Entity = String, Id = u64>> = Domain.new_in_memory_repository();
    repo.save(RepositorySaveRequest {
        id: 1,
        entity: "hello".into(),
    })
    .await
    .unwrap();
    let found = repo.find(RepositoryIdRequest { id: &1 }).await.unwrap();
    assert_eq!(found.entity.as_deref(), Some("hello"));
}

/// @covers: new_in_memory_repository
#[tokio::test]
async fn test_new_in_memory_repository_find_returns_none_for_missing_id() {
    let repo: Arc<dyn Repository<Entity = String, Id = u64>> = Domain.new_in_memory_repository();
    let found = repo.find(RepositoryIdRequest { id: &99 }).await.unwrap();
    assert!(found.entity.is_none());
}

/// @covers: new_in_memory_repository
#[tokio::test]
async fn test_new_in_memory_repository_delete_removes_entity() {
    let repo: Arc<dyn Repository<Entity = String, Id = u64>> = Domain.new_in_memory_repository();
    repo.save(RepositorySaveRequest {
        id: 1,
        entity: "bye".into(),
    })
    .await
    .unwrap();
    assert!(
        repo.delete(RepositoryIdRequest { id: &1 })
            .await
            .unwrap()
            .removed
    );
    assert!(repo
        .find(RepositoryIdRequest { id: &1 })
        .await
        .unwrap()
        .entity
        .is_none());
}

/// @covers: new_in_memory_repository
#[tokio::test]
async fn test_new_in_memory_repository_list_returns_all_saved_entities() {
    let repo: Arc<dyn Repository<Entity = String, Id = u64>> = Domain.new_in_memory_repository();
    repo.save(RepositorySaveRequest {
        id: 1,
        entity: "a".into(),
    })
    .await
    .unwrap();
    repo.save(RepositorySaveRequest {
        id: 2,
        entity: "b".into(),
    })
    .await
    .unwrap();
    let mut items = repo.list(RepositoryListRequest).await.unwrap().items;
    items.sort();
    assert_eq!(items, vec!["a", "b"]);
}
