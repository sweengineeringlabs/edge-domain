//! `EventStoreError` — errors returned by [`EventStore`](crate::EventStore) operations.

use thiserror::Error;

/// Errors that can occur when reading from or writing to an event store.
#[derive(Debug, Error)]
pub enum EventStoreError {
    /// A version conflict was detected — the stream was modified concurrently.
    #[error("version conflict on '{aggregate_id}': expected version {expected}, found {actual}")]
    Conflict {
        /// The aggregate stream where the conflict occurred.
        aggregate_id: String,
        /// The version the caller expected.
        expected: u64,
        /// The version actually present in the store.
        actual: u64,
    },

    /// The event store is unavailable (network error, service down, etc.).
    #[error("event store unavailable: {0}")]
    Unavailable(String),

    /// An unexpected internal error occurred.
    #[error("internal error: {0}")]
    Internal(String),
}


