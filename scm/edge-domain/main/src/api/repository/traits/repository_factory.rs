//! [`RepositoryFactory`] — constructor contract for repository infrastructure types.

use crate::api::repository::types::in_memory_repository::InMemoryRepository;

/// Factory trait for the standard repository-infrastructure types.
pub trait RepositoryFactory {
    /// Construct a default [`InMemoryRepository`] configuration.
    fn in_memory() -> InMemoryRepository {
        InMemoryRepository::default()
    }
}
