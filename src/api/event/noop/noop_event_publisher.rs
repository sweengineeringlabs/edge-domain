//! API-layer type for the no-op event publisher.

/// Marker type describing an `EventPublisher` that discards all events silently.
///
/// The concrete implementation lives in `core::event::noop::noop_event_publisher`.
#[allow(dead_code)]
pub struct NoopEventPublisher;
