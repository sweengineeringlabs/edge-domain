//! [`SnapshotSaveRequest`] — request to persist a snapshot.

/// Request to persist `snapshot`, replacing any earlier snapshot for the same aggregate.
pub struct SnapshotSaveRequest<S> {
    /// The snapshot to persist.
    pub snapshot: S,
}
