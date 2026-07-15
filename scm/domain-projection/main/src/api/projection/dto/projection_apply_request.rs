//! [`ProjectionApplyRequest`] — request to fold one event into a projection.

/// Request to apply `event` to a [`Projection`](crate::api::projection::traits::Projection).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProjectionApplyRequest<'a, E> {
    /// The event to fold into the read model.
    pub event: &'a E,
}
