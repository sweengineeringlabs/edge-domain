//! In-memory `Repository` implementation for development and testing.

use std::collections::HashMap;
use std::hash::Hash;

use async_trait::async_trait;
use parking_lot::RwLock;

use crate::api::queryable_repository::QueryableRepository;
use crate::api::repository::Repository;
use crate::api::repository_error::RepositoryError;

/// Thread-safe in-memory repository backed by a `HashMap`.
///
/// Suitable for development, testing, and services that do not require
/// durable storage. Not suitable for production persistence.
pub(crate) struct InMemoryRepository<T, Id>
where
    Id: Hash + Eq + Clone + Send + Sync + 'static,
    T: Clone + Send + Sync + 'static,
{
    store: RwLock<HashMap<Id, T>>,
}

impl<T, Id> InMemoryRepository<T, Id>
where
    Id: Hash + Eq + Clone + Send + Sync + 'static,
    T: Clone + Send + Sync + 'static,
{
    pub(crate) fn new() -> Self {
        Self { store: RwLock::new(HashMap::new()) }
    }
}

#[async_trait]
impl<T, Id> Repository<T, Id> for InMemoryRepository<T, Id>
where
    Id: Hash + Eq + Clone + Send + Sync + 'static,
    T: Clone + Send + Sync + 'static,
{
    async fn find(&self, id: &Id) -> Result<Option<T>, RepositoryError> {
        Ok(self.store.read().get(id).cloned())
    }

    async fn save(&self, id: Id, entity: T) -> Result<(), RepositoryError> {
        self.store.write().insert(id, entity);
        Ok(())
    }

    async fn delete(&self, id: &Id) -> Result<bool, RepositoryError> {
        Ok(self.store.write().remove(id).is_some())
    }

    async fn list(&self) -> Result<Vec<T>, RepositoryError> {
        Ok(self.store.read().values().cloned().collect())
    }
}

impl<T, Id> QueryableRepository<T, Id> for InMemoryRepository<T, Id>
where
    Id: Hash + Eq + Clone + Send + Sync + 'static,
    T: Clone + Send + Sync + 'static,
{}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo() -> InMemoryRepository<String, u64> {
        InMemoryRepository::new()
    }

    #[test]
    fn test_new_creates_empty_repository() {
        let r: InMemoryRepository<String, u64> = InMemoryRepository::new();
        assert!(r.store.read().is_empty());
    }

    #[tokio::test]
    async fn test_save_and_find_round_trips_entity() {
        let r = repo();
        r.save(1, "hello".into()).await.unwrap();
        assert_eq!(r.find(&1).await.unwrap().as_deref(), Some("hello"));
    }

    #[tokio::test]
    async fn test_find_returns_none_for_missing_id() {
        assert!(repo().find(&99).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_delete_removes_entity_and_returns_true() {
        let r = repo();
        r.save(1, "bye".into()).await.unwrap();
        assert!(r.delete(&1).await.unwrap());
        assert!(r.find(&1).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_delete_returns_false_for_missing_id() {
        assert!(!repo().delete(&99).await.unwrap());
    }

    #[tokio::test]
    async fn test_list_returns_all_saved_entities() {
        let r = repo();
        r.save(1, "a".into()).await.unwrap();
        r.save(2, "b".into()).await.unwrap();
        let mut items = r.list().await.unwrap();
        items.sort();
        assert_eq!(items, vec!["a", "b"]);
    }
}
