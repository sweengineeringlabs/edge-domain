//! Error type for [`EventPublisher`](super::EventPublisher) operations.

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
}
