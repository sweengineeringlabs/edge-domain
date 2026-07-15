//! [`SnapshotAggregateIdResponse`] — wrapper borrowing a snapshot's aggregate id.

/// Result of [`Snapshot::aggregate_id`](crate::api::Snapshot::aggregate_id).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SnapshotAggregateIdResponse<'a, Id> {
    /// The aggregate id this snapshot belongs to.
    pub aggregate_id: &'a Id,
}
