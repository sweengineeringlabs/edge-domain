#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — Repository is exported from the crate root.

use edge_domain::Domain;
use edge_domain::Repository;
use std::sync::Arc;

#[derive(Clone)]
struct Record {
    value: i32,
}

#[tokio::test]
async fn test_repository_svc_facade_save_and_find() {
    let repo: Arc<dyn Repository<Entity = Record, Id = String>> = Domain::new_in_memory_repository();
    repo.save("r1".into(), Record { value: 42 }).await.unwrap();
    let found = repo.find(&"r1".into()).await.unwrap();
    assert_eq!(found.unwrap().value, 42);
}

#[tokio::test]
async fn test_repository_svc_facade_find_missing_returns_none() {
    let repo: Arc<dyn Repository<Entity = Record, Id = String>> = Domain::new_in_memory_repository();
    let found = repo.find(&"nope".into()).await.unwrap();
    assert!(found.is_none());
}
