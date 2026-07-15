//! [`AggregateApplyRequest`] — request to fold one event into an aggregate.

/// Request to apply `event` to an [`Aggregate`](crate::api::Aggregate)'s state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AggregateApplyRequest<'a, E> {
    /// The event to fold into the aggregate.
    pub event: &'a E,
}
