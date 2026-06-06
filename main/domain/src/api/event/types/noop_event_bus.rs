//! API interface surface for the no-op event bus implementation.

/// API marker type for the no-op event bus.
///
/// The implementation lives in `core::event::noop_event_bus`.
/// Consumers obtain it via [`Domain::noop_event_bus`](crate::Domain::noop_event_bus).
pub struct NoopEventBus;
