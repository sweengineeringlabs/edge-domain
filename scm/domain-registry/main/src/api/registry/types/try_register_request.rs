//! [`TryRegisterRequest`] — request to strictly register an entry under an id.

use std::sync::Arc;

/// Request to register `entry` under `id`, failing if `id` is already taken.
pub struct TryRegisterRequest<V: ?Sized + Send + Sync> {
    /// The id to register the entry under.
    pub id: String,
    /// The entry to store.
    pub entry: Arc<V>,
}
