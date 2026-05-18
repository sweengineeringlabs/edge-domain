//! API-layer type for the no-op event publisher.

/// Marker type describing an `EventPublisher` that discards all events silently.
///
/// The concrete implementation lives in `core::event::noop_event_publisher`.
#[allow(dead_code)]
pub struct NoopEventPublisher;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_event_publisher_is_constructible() {
        let _: NoopEventPublisher = NoopEventPublisher;
    }
}
