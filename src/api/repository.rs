//! `Repository` trait — data access contract for domain entities.

use async_trait::async_trait;

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
///     async fn save(&self, entity: Order) -> Result<(), RepositoryError> { ... }
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

    /// Persist an entity, inserting or updating as appropriate.
    async fn save(&self, entity: T) -> Result<(), RepositoryError>;

    /// Remove the entity with the given identity.
    ///
    /// Returns `true` when an entity was removed, `false` when none existed.
    async fn delete(&self, id: &Id) -> Result<bool, RepositoryError>;

    /// Return all entities in the repository.
    async fn list(&self) -> Result<Vec<T>, RepositoryError>;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_trait_is_object_safe() {
        fn _assert(_: &dyn Repository<String, u64>) {}
    }
}
