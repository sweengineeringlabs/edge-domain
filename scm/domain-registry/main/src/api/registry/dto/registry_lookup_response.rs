//! [`RegistryLookupResponse`] — wrapper for a registry lookup result.

use std::sync::Arc;

/// Result of [`Registry::get`](crate::api::Registry::get).
pub struct RegistryLookupResponse<V: ?Sized + Send + Sync> {
    /// The resolved entry, or `None` if no entry is registered under the requested id.
    pub entry: Option<Arc<V>>,
}
