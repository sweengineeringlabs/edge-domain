//! `InMemoryRegistry` — in-process [`Registry`](crate::Registry) backed by a `RwLock<HashMap>`.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// In-process [`Registry`](crate::Registry) implementation storing shared
/// entries keyed by string id. Concurrent reads and writes are guarded by a
/// `RwLock`.
pub struct InMemoryRegistry<V: ?Sized + Send + Sync> {
    pub(crate) entries: RwLock<HashMap<String, Arc<V>>>,
}

impl<V: ?Sized + Send + Sync> InMemoryRegistry<V> {
    /// Construct an empty registry.
    pub fn new() -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
        }
    }
}

impl<V: ?Sized + Send + Sync> Default for InMemoryRegistry<V> {
    fn default() -> Self {
        Self::new()
    }
}
