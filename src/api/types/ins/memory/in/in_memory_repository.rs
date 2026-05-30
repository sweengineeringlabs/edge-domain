//! API-layer configuration type for the in-memory repository.

/// Configuration for the in-memory repository implementation.
///
/// The concrete implementation lives in `core::repository::in_memory_repository`.
/// Pass this to infrastructure bootstrapping when you need to tune the
/// initial `HashMap` capacity without naming the `pub(crate)` core struct.
#[derive(Debug, Clone, Default)]
pub struct InMemoryRepository {
    /// Initial capacity of the underlying `HashMap`.
    ///
    /// Set this to the expected number of entities to avoid re-allocations
    /// during warmup.
    pub initial_capacity: usize,
}
