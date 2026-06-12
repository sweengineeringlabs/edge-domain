//! `InMemoryRepository` — heap-backed repository for testing and prototyping.

use std::collections::HashMap;
use std::hash::Hash;

use parking_lot::RwLock;

/// An in-memory repository backed by a `HashMap` protected by a `RwLock`.
///
/// Suitable for tests and in-process prototyping. The `store` field is
/// `pub(crate)` so that `core/` implementations can access it directly
/// without exposing raw storage to consumers.
pub struct InMemoryRepository<T, Id>
where
    Id: Hash + Eq + Clone + Send + Sync + 'static,
    T: Clone + Send + Sync + 'static,
{
    pub(crate) store: RwLock<HashMap<Id, T>>,
}

impl<T, Id> InMemoryRepository<T, Id>
where
    Id: Hash + Eq + Clone + Send + Sync + 'static,
    T: Clone + Send + Sync + 'static,
{
    /// Creates a new, empty `InMemoryRepository`.
    pub fn new() -> Self {
        Self { store: RwLock::new(HashMap::new()) }
    }
}

impl<T, Id> Default for InMemoryRepository<T, Id>
where
    Id: Hash + Eq + Clone + Send + Sync + 'static,
    T: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_empty_store_happy() {
        let repo = InMemoryRepository::<String, String>::new();
        assert!(repo.store.read().is_empty());
    }

    #[test]
    fn test_default_creates_empty_store_edge() {
        let repo = InMemoryRepository::<u32, u32>::default();
        assert!(repo.store.read().is_empty());
    }

    #[test]
    fn test_new_independent_instances_are_isolated_error() {
        let a = InMemoryRepository::<u32, u32>::new();
        let b = InMemoryRepository::<u32, u32>::new();
        a.store.write().insert(1, 100);
        assert!(!b.store.read().contains_key(&1));
    }
}
