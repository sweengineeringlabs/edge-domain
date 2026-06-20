//! `RegistryBootstrap` — constructor contract for registry implementations.

use crate::api::registry::types::{InMemoryRegistry, StdRegistryFactory};

/// Bootstrap trait for the standard `Registry` implementation.
pub trait RegistryBootstrap {
    /// Identifies this bootstrap implementation.
    fn bootstrap_name(&self) -> &'static str {
        "registry"
    }

    /// Construct an empty in-memory registry of shared `V` entries.
    fn in_memory<V: ?Sized + Send + Sync>() -> InMemoryRegistry<V> where Self: Sized {
        InMemoryRegistry::new()
    }

    /// Return the standard registry-factory instance.
    fn std_factory() -> StdRegistryFactory where Self: Sized {
        StdRegistryFactory
    }
}
