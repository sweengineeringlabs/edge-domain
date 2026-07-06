#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — Repository is exported from the crate root.

use edge_domain::{
    Domain, Repository, RepositoryFindResponse, RepositoryIdRequest, RepositorySaveRequest,
};
use std::sync::Arc;

#[derive(Clone)]
struct Record {
    value: i32,
}

#[tokio::test]
async fn test_repository_svc_facade_save_and_find() {
    let repo: Arc<dyn Repository<Entity = Record, Id = String>> =
        Domain::new_in_memory_repository();
    repo.save(RepositorySaveRequest {
        id: "r1".into(),
        entity: Record { value: 42 },
    })
    .await
    .unwrap();
    let id = "r1".to_string();
    let found = repo.find(RepositoryIdRequest { id: &id }).await.unwrap();
    assert_eq!(found.entity.unwrap().value, 42);
}

#[tokio::test]
async fn test_repository_svc_facade_find_missing_returns_none() {
    let repo: Arc<dyn Repository<Entity = Record, Id = String>> =
        Domain::new_in_memory_repository();
    let id = "nope".to_string();
    let found = repo.find(RepositoryIdRequest { id: &id }).await.unwrap();
    assert!(found.entity.is_none());
}
