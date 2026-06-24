//! `impl RegistryBootstrap for StdRegistryFactory`.

use crate::api::{InMemoryRegistry, RegistryBootstrap, StdRegistryFactory};

impl RegistryBootstrap for StdRegistryFactory {
    fn in_memory<V: ?Sized + Send + Sync>() -> InMemoryRegistry<V> {
        InMemoryRegistry::new()
    }
}
