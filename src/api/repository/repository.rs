//! `Repository` trait — data access contract for domain entities.

use futures::future::BoxFuture;

use crate::api::types::Page;
use crate::api::repository_error::RepositoryError;

/// Data access contract for a domain entity of type `T` keyed by `Id`.
///
/// Implementations live in infrastructure crates — never in `edge-domain`.
/// `edge-domain` owns only this contract.
///
/// ```rust,ignore
/// impl Repository<Order, OrderId> for PostgresOrderRepo {
///     fn find<'a>(&'a self, id: &'a OrderId) -> BoxFuture<'a, Result<Option<Order>, RepositoryError>> { ... }
///     fn save(&self, id: OrderId, entity: Order) -> BoxFuture<'_, Result<(), RepositoryError>> { ... }
///     fn delete<'a>(&'a self, id: &'a OrderId) -> BoxFuture<'a, Result<bool, RepositoryError>> { ... }
///     fn list(&self) -> BoxFuture<'_, Result<Vec<Order>, RepositoryError>> { ... }
/// }
/// ```
pub trait Repository<T, Id>: Send + Sync
where
    T: Send + 'static,
    Id: Send + Sync + 'static,
{
    /// Find an entity by its identity. Returns `None` when not found.
    fn find<'a>(&'a self, id: &'a Id) -> BoxFuture<'a, Result<Option<T>, RepositoryError>>;

    /// Persist an entity under the given identity, inserting or updating.
    fn save(&self, id: Id, entity: T) -> BoxFuture<'_, Result<(), RepositoryError>>;

    /// Remove the entity with the given identity.
    ///
    /// Returns `true` when an entity was removed, `false` when none existed.
    fn delete<'a>(&'a self, id: &'a Id) -> BoxFuture<'a, Result<bool, RepositoryError>>;

    /// Return all entities in the repository.
    fn list(&self) -> BoxFuture<'_, Result<Vec<T>, RepositoryError>>;

    /// Check whether an entity with `id` exists.
    ///
    /// Override in production backends — the default issues a `find`.
    fn exists<'a>(&'a self, id: &'a Id) -> BoxFuture<'a, Result<bool, RepositoryError>> {
        Box::pin(async move { self.find(id).await.map(|opt| opt.is_some()) })
    }

    /// Count all entities in the repository.
    ///
    /// Override in production backends — the default loads all entities.
    fn count(&self) -> BoxFuture<'_, Result<usize, RepositoryError>> {
        Box::pin(async move { self.list().await.map(|v| v.len()) })
    }

    /// Return a page of entities ordered by insertion.
    ///
    /// Override in production backends — the default loads all entities.
    fn list_page(
        &self,
        offset: usize,
        limit: usize,
    ) -> BoxFuture<'_, Result<Page<T>, RepositoryError>> {
        Box::pin(async move {
            let all = self.list().await?;
            let total = all.len();
            let items = all.into_iter().skip(offset).take(limit).collect();
            Ok(Page::new(items, total, offset, limit))
        })
    }
}


