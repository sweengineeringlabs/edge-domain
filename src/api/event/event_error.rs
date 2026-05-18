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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_error_serialization_failed_message_is_actionable() {
        let e = EventError::SerializationFailed("missing field".into());
        assert!(e.to_string().contains("missing field"));
    }

    #[test]
    fn test_event_error_unavailable_message_is_actionable() {
        let e = EventError::Unavailable("broker down".into());
        assert!(e.to_string().contains("broker down"));
    }

    #[test]
    fn test_event_error_broadcast_lagged_includes_count() {
        let e = EventError::BroadcastLagged(42);
        assert!(e.to_string().contains("42"));
    }
}
