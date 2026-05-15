//! `Repository` trait — data access contract for domain entities.

use async_trait::async_trait;

use crate::api::page::Page;
use crate::api::repository_error::RepositoryError;

/// Data access contract for a domain entity of type `T` keyed by `Id`.
///
/// Implementations live in infrastructure crates — never in `edge-domain`.
/// `edge-domain` owns only this contract.
///
/// ```rust,ignore
/// #[async_trait]
/// impl Repository<Order, OrderId> for PostgresOrderRepo {
///     async fn find(&self, id: &OrderId) -> Result<Option<Order>, RepositoryError> { ... }
///     async fn save(&self, id: OrderId, entity: Order) -> Result<(), RepositoryError> { ... }
///     async fn delete(&self, id: &OrderId) -> Result<bool, RepositoryError> { ... }
///     async fn list(&self) -> Result<Vec<Order>, RepositoryError> { ... }
/// }
/// ```
#[async_trait]
pub trait Repository<T, Id>: Send + Sync
where
    T: Send + 'static,
    Id: Send + Sync + 'static,
{
    /// Find an entity by its identity. Returns `None` when not found.
    async fn find(&self, id: &Id) -> Result<Option<T>, RepositoryError>;

    /// Persist an entity under the given identity, inserting or updating.
    async fn save(&self, id: Id, entity: T) -> Result<(), RepositoryError>;

    /// Remove the entity with the given identity.
    ///
    /// Returns `true` when an entity was removed, `false` when none existed.
    async fn delete(&self, id: &Id) -> Result<bool, RepositoryError>;

    /// Return all entities in the repository.
    async fn list(&self) -> Result<Vec<T>, RepositoryError>;

    /// Check whether an entity with `id` exists.
    ///
    /// Override in production backends — the default issues a `find`.
    async fn exists(&self, id: &Id) -> Result<bool, RepositoryError> {
        self.find(id).await.map(|opt| opt.is_some())
    }

    /// Count all entities in the repository.
    ///
    /// Override in production backends — the default loads all entities.
    async fn count(&self) -> Result<usize, RepositoryError> {
        self.list().await.map(|v| v.len())
    }

    /// Return a page of entities ordered by insertion.
    ///
    /// Override in production backends — the default loads all entities.
    async fn list_page(&self, offset: usize, limit: usize) -> Result<Page<T>, RepositoryError> {
        let all = self.list().await?;
        let total = all.len();
        let items = all.into_iter().skip(offset).take(limit).collect();
        Ok(Page::new(items, total, offset, limit))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_trait_is_object_safe() {
        fn _assert(_: &dyn Repository<String, u64>) {}
    }
}
