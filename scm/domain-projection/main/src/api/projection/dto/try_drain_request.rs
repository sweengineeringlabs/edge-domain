//! [`TryDrainRequest`] — request to fold a slice of events into a projection.

/// Request to apply `events` to a [`Projection`](crate::api::projection::traits::Projection),
/// in order, via repeated [`apply`](crate::api::projection::traits::Projection::apply) calls.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TryDrainRequest<'a, E> {
    /// The events to fold into the read model, in order.
    pub events: &'a [E],
}
