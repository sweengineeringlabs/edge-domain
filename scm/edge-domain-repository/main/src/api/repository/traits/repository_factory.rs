//! `RepositoryFactory` — SAF factory trait for constructing repository instances.

use std::hash::Hash;

use crate::api::repository::types::InMemoryRepository;

/// Factory for constructing repository instances.
///
/// Implement this trait on a unit struct to gain access to the factory methods.
pub trait RepositoryFactory {
    /// Returns a new, empty `InMemoryRepository<T, Id>`.
    fn in_memory<T, Id>() -> InMemoryRepository<T, Id>
    where
        T: Clone + Send + Sync + 'static,
        Id: Hash + Eq + Clone + Send + Sync + 'static,
    {
        InMemoryRepository::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Repos;
    impl RepositoryFactory for Repos {}

    #[test]
    fn test_in_memory_returns_empty_store_happy() {
        let repo: InMemoryRepository<String, u32> = Repos::in_memory();
        assert!(repo.store.read().is_empty());
    }

    #[test]
    fn test_in_memory_independent_instances_isolated_error() {
        let a: InMemoryRepository<u32, u32> = Repos::in_memory();
        let b: InMemoryRepository<u32, u32> = Repos::in_memory();
        a.store.write().insert(1, 99);
        assert!(!b.store.read().contains_key(&1));
    }

    #[test]
    fn test_in_memory_default_creates_empty_store_edge() {
        let repo: InMemoryRepository<u32, u32> = Default::default();
        assert!(repo.store.read().is_empty());
    }
}
