//! API interface surface for the in-process broadcast event bus.

/// Technology-neutral marker for the in-process broadcast event bus.
///
/// The concrete implementation is provided by an external-library adapter in
/// the `spi/` layer. Consumers obtain a bus via the SAF factory
/// [`Domain::in_process_event_bus`](crate::Domain::in_process_event_bus).
pub struct InProcessEventBus;
