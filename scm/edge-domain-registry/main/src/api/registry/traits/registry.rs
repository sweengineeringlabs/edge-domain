//! `Registry` — id-keyed resolution registry of shared entries.

use std::sync::Arc;

use crate::api::registry::errors::RegistryError;

/// An id-keyed registry of shared entries.
///
/// Generalizes the resolution-registry family — handlers, services, and live
/// task controllers: register a shared entry under a string id and resolve it
/// later by id. Concurrent: every method takes `&self`. The stored entry type
/// is the associated [`Value`](Registry::Value) (matching the `Repository` /
/// `ServiceRegistry` convention).
pub trait Registry: Send + Sync {
    /// The (possibly unsized) entry type stored in this registry.
    type Value: ?Sized + Send + Sync;

    /// Register `entry` under `id`, replacing any existing entry.
    fn register(&self, id: &str, entry: Arc<Self::Value>);

    /// Register `entry` under `id`, returning [`RegistryError::DuplicateId`]
    /// when an entry is already registered under `id` (the existing entry is
    /// left untouched).
    fn try_register(&self, id: &str, entry: Arc<Self::Value>) -> Result<(), RegistryError>;

    /// Remove the entry registered under `id`. Returns `true` if one existed.
    fn deregister(&self, id: &str) -> bool;

    /// Resolve the entry registered under `id`.
    fn get(&self, id: &str) -> Option<Arc<Self::Value>>;

    /// Return all registered ids.
    fn list_ids(&self) -> Vec<String>;

    /// Return the number of registered entries.
    fn len(&self) -> usize;

    /// Return `true` if no entries are registered.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
