//! API interface surface for the no-op event bus implementation.

/// API marker type for the no-op event bus.
///
/// The implementation lives in `core::event::noop::noop_event_bus`.
/// Consumers obtain it via [`crate::noop_event_bus`].
pub struct NoopEventBus;
