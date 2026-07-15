//! [`SnapshotLoadRequest`] — request identifying an aggregate to load the latest snapshot for.

/// Request to load the latest snapshot for `id`.
pub struct SnapshotLoadRequest<'a, Id> {
    /// The aggregate id to look up.
    pub id: &'a Id,
}
