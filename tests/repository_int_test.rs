//! Integration tests for the `Repository` trait contract.

use std::sync::Arc;
use async_trait::async_trait;
use edge_domain::{Repository, RepositoryError};

struct InMemoryRepo {
    data: std::sync::Mutex<std::collections::HashMap<u64, String>>,
}

impl InMemoryRepo {
    fn new() -> Self {
        Self { data: std::sync::Mutex::new(std::collections::HashMap::new()) }
    }
}

#[async_trait]
impl Repository<String, u64> for InMemoryRepo {
    async fn find(&self, id: &u64) -> Result<Option<String>, RepositoryError> {
        Ok(self.data.lock().unwrap().get(id).cloned())
    }
    async fn save(&self, entity: String) -> Result<(), RepositoryError> {
        let id = self.data.lock().unwrap().len() as u64;
        self.data.lock().unwrap().insert(id, entity);
        Ok(())
    }
    async fn delete(&self, id: &u64) -> Result<bool, RepositoryError> {
        Ok(self.data.lock().unwrap().remove(id).is_some())
    }
    async fn list(&self) -> Result<Vec<String>, RepositoryError> {
        Ok(self.data.lock().unwrap().values().cloned().collect())
    }
}

/// @covers: Repository::save, Repository::find
#[tokio::test]
async fn test_repository_struct_save_and_find_round_trips_entity() {
    let repo: Arc<dyn Repository<String, u64>> = Arc::new(InMemoryRepo::new());
    repo.save("hello".into()).await.unwrap();
    let found = repo.find(&0).await.unwrap();
    assert_eq!(found.as_deref(), Some("hello"));
}

/// @covers: Repository::find
#[tokio::test]
async fn test_repository_struct_find_returns_none_for_missing_id() {
    let repo: Arc<dyn Repository<String, u64>> = Arc::new(InMemoryRepo::new());
    assert!(repo.find(&99).await.unwrap().is_none());
}

/// @covers: Repository::delete
#[tokio::test]
async fn test_repository_struct_delete_removes_entity_and_returns_true() {
    let repo: Arc<dyn Repository<String, u64>> = Arc::new(InMemoryRepo::new());
    repo.save("bye".into()).await.unwrap();
    assert!(repo.delete(&0).await.unwrap());
    assert!(repo.find(&0).await.unwrap().is_none());
}

/// @covers: Repository::delete
#[tokio::test]
async fn test_repository_struct_delete_returns_false_for_missing_id() {
    let repo: Arc<dyn Repository<String, u64>> = Arc::new(InMemoryRepo::new());
    assert!(!repo.delete(&99).await.unwrap());
}

/// @covers: Repository::list
#[tokio::test]
async fn test_repository_struct_list_returns_all_saved_entities() {
    let repo: Arc<dyn Repository<String, u64>> = Arc::new(InMemoryRepo::new());
    repo.save("a".into()).await.unwrap();
    repo.save("b".into()).await.unwrap();
    let mut items = repo.list().await.unwrap();
    items.sort();
    assert_eq!(items, vec!["a", "b"]);
}
