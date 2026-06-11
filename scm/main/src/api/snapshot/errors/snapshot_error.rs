//! `SnapshotError` — errors returned by [`SnapshotStore`](crate::SnapshotStore) operations.

use thiserror::Error;

/// Errors that can occur when saving or loading snapshots.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SnapshotError {
    /// A snapshot was offered at a version that cannot be captured.
    ///
    /// Snapshots are only meaningful at version `>= 1`; version `0` means no
    /// events have been applied yet, so there is nothing to snapshot.
    #[error("snapshot for '{aggregate_id}' has invalid version {version} (must be >= 1)")]
    InvalidVersion {
        /// The aggregate the rejected snapshot was for.
        aggregate_id: String,
        /// The invalid version offered.
        version: u64,
    },

    /// The snapshot store is unavailable (network error, service down, etc.).
    #[error("snapshot store unavailable: {0}")]
    Unavailable(String),

    /// An unexpected internal error occurred.
    #[error("internal snapshot error: {0}")]
    Internal(String),
}
