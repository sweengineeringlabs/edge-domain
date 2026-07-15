//! [`RegisterRequest`] — request to register an entry under an id.

use std::sync::Arc;

/// Request to register `entry` under `id`, replacing any existing entry.
pub struct RegisterRequest<V: ?Sized + Send + Sync> {
    /// The id to register the entry under.
    pub id: String,
    /// The entry to store.
    pub entry: Arc<V>,
}
