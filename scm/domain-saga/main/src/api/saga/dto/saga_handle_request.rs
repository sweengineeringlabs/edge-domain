//! [`SagaHandleRequest`] — request to apply an event to a saga.

/// Request to apply `event` to a [`Saga`](crate::api::saga::traits::Saga).
pub struct SagaHandleRequest<'a, E> {
    /// The event to apply.
    pub event: &'a E,
}
