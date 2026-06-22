//! `Repository` — core async CRUD port contract.

use futures::future::BoxFuture;

use crate::api::repository::errors::RepositoryError;
use crate::api::repository::types::Page;

/// Core async CRUD contract for a typed repository.
///
/// Associated types enforce that each implementor is bound to exactly one
/// entity type and one id type — a `UserRepository` can never accidentally
/// store a `Product`.
///
/// All methods return `BoxFuture` so implementations are not required to use
/// `#[async_trait]`.  Default methods for `exists`, `count`, and `list_page`
/// are provided in terms of the required methods.
pub trait Repository: Send + Sync {
    /// The entity type stored in this repository.
    type Entity: Send + 'static;
    /// The identifier type used to key entities.
    type Id: Send + Sync + 'static;

    /// Returns the entity with the given `id`, or `None` if it does not exist.
    fn find<'a>(
        &'a self,
        id: &'a Self::Id,
    ) -> BoxFuture<'a, Result<Option<Self::Entity>, RepositoryError>>;

    /// Persists `entity` under `id`, replacing any existing entry.
    fn save(
        &self,
        id: Self::Id,
        entity: Self::Entity,
    ) -> BoxFuture<'_, Result<(), RepositoryError>>;

    /// Removes the entity with the given `id`.
    ///
    /// Returns `true` if an entry was removed, `false` if it did not exist.
    fn delete<'a>(&'a self, id: &'a Self::Id) -> BoxFuture<'a, Result<bool, RepositoryError>>;

    /// Returns all entities in the repository.
    fn list(&self) -> BoxFuture<'_, Result<Vec<Self::Entity>, RepositoryError>>;

    /// Returns `true` if an entity with the given `id` exists.
    fn exists<'a>(&'a self, id: &'a Self::Id) -> BoxFuture<'a, Result<bool, RepositoryError>> {
        Box::pin(async move { self.find(id).await.map(|opt| opt.is_some()) })
    }

    /// Returns the total number of entities in the repository.
    fn count(&self) -> BoxFuture<'_, Result<usize, RepositoryError>> {
        Box::pin(async move { self.list().await.map(|v| v.len()) })
    }

    /// Returns a paginated slice of entities.
    fn list_page(
        &self,
        offset: usize,
        limit: usize,
    ) -> BoxFuture<'_, Result<Page<Self::Entity>, RepositoryError>>
    where
        Self::Entity: Clone,
        Self: Sized,
    {
        Box::pin(async move {
            let all = self.list().await?;
            let total = all.len();
            let items = all.into_iter().skip(offset).take(limit).collect();
            Ok(Page::new(items, total, offset, limit))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;

    struct MapRepo {
        store: std::sync::Mutex<std::collections::HashMap<u32, String>>,
    }

    impl MapRepo {
        fn new() -> Self {
            Self {
                store: std::sync::Mutex::new(std::collections::HashMap::new()),
            }
        }
    }

    impl Repository for MapRepo {
        type Entity = String;
        type Id = u32;

        fn find<'a>(
            &'a self,
            id: &'a u32,
        ) -> BoxFuture<'a, Result<Option<String>, RepositoryError>> {
            let val = self
                .store
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .get(id)
                .cloned();
            Box::pin(async move { Ok(val) })
        }

        fn save(&self, id: u32, entity: String) -> BoxFuture<'_, Result<(), RepositoryError>> {
            self.store
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .insert(id, entity);
            Box::pin(async move { Ok(()) })
        }

        fn delete<'a>(&'a self, id: &'a u32) -> BoxFuture<'a, Result<bool, RepositoryError>> {
            let removed = self
                .store
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .remove(id)
                .is_some();
            Box::pin(async move { Ok(removed) })
        }

        fn list(&self) -> BoxFuture<'_, Result<Vec<String>, RepositoryError>> {
            let vals: Vec<_> = self
                .store
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .values()
                .cloned()
                .collect();
            Box::pin(async move { Ok(vals) })
        }
    }

    #[test]
    fn test_exists_saved_entity_returns_true_happy() {
        let repo = MapRepo::new();
        block_on(repo.save(1, "alpha".into())).unwrap_or_default();
        assert!(block_on(repo.exists(&1)).unwrap_or(false));
    }

    #[test]
    fn test_exists_missing_entity_returns_false_error() {
        let repo = MapRepo::new();
        assert!(!block_on(repo.exists(&99)).unwrap_or(true));
    }

    #[test]
    fn test_count_empty_repo_returns_zero_edge() {
        let repo = MapRepo::new();
        let n = block_on(repo.count()).unwrap_or(1);
        assert_eq!(n, 0);
    }
}
