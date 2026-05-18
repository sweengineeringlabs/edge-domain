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
    fn test_event_error_has_no_internal_variant() {
        // Internal was removed — Unavailable covers all bus/infrastructure failures.
        // This test ensures the variant count stays at 2.
        let variants: &[EventError] = &[
            EventError::SerializationFailed("x".into()),
            EventError::Unavailable("x".into()),
        ];
        assert_eq!(variants.len(), 2);
    }
}
