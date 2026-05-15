//! API-layer configuration type for the in-memory repository.

/// Configuration for the in-memory repository implementation.
///
/// The concrete implementation lives in `core::repository::in_memory_repository`.
/// Pass this to infrastructure bootstrapping when you need to tune the
/// initial `HashMap` capacity without naming the `pub(crate)` core struct.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct InMemoryRepository {
    /// Initial capacity of the underlying `HashMap`.
    ///
    /// Set this to the expected number of entities to avoid re-allocations
    /// during warmup.
    pub initial_capacity: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_repository_default_initial_capacity_is_zero() {
        let cfg = InMemoryRepository::default();
        assert_eq!(cfg.initial_capacity, 0);
    }

    #[test]
    fn test_in_memory_repository_custom_capacity_is_stored() {
        let cfg = InMemoryRepository { initial_capacity: 64 };
        assert_eq!(cfg.initial_capacity, 64);
    }
}
