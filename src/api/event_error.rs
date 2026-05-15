//! Error type for [`EventPublisher`](super::event_publisher::EventPublisher) operations.

/// Error produced when publishing a domain event fails.
#[derive(Debug, thiserror::Error)]
pub enum EventError {
    /// The event could not be serialized.
    #[error("serialization failed: {0}")]
    SerializationFailed(String),
    /// The event bus or channel is unavailable.
    #[error("unavailable: {0}")]
    Unavailable(String),
    /// An unexpected internal error occurred.
    #[error("internal: {0}")]
    Internal(String),
}
