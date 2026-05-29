//! Error type for [`EventPublisher`](super::EventPublisher) and [`crate::EventBus`] operations.

/// Error produced when publishing or receiving a domain event fails.
#[derive(Debug, thiserror::Error)]
pub enum EventError {
    /// The event could not be serialized.
    #[error("serialization failed: {0}")]
    SerializationFailed(String),
    /// The event bus or channel is unavailable.
    #[error("unavailable: {0}")]
    Unavailable(String),
    /// The subscriber fell behind; `{0}` messages were dropped by the broadcast channel.
    #[error("broadcast lagged: {0} messages dropped")]
    BroadcastLagged(u64),
}
