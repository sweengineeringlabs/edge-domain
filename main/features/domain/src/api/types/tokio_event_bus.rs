//! API interface surface for the Tokio-based event bus implementation.

/// Tokio-based event bus implementation.
///
/// The implementation lives in `core::event::tokio_event_bus`.
/// Consumers obtain it via [`crate::tokio_event_bus`].
pub struct TokioEventBus;
