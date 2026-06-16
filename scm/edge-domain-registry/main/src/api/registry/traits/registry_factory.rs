//! `RegistryFactory` — constructor contract for registry implementations.

use crate::api::registry::types::{InMemoryRegistry, StdRegistryFactory};

/// Factory trait for the standard `Registry` implementation.
pub trait RegistryFactory {
    /// Construct an empty in-memory registry of shared `V` entries.
    fn in_memory<V: ?Sized + Send + Sync>() -> InMemoryRegistry<V> {
        InMemoryRegistry::new()
    }

    /// Return the standard registry-factory instance.
    fn std_factory() -> StdRegistryFactory {
        StdRegistryFactory
    }
}
