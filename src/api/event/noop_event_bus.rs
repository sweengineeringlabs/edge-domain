//! API interface surface for the no-op event bus implementation.

/// API marker type for the no-op event bus.
///
/// Consumers obtain the implementation via [`crate::noop_event_bus`].
#[allow(dead_code)]
pub struct NoopEventBus;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_event_bus_api_marker_is_constructible() {
        let _ = NoopEventBus;
    }
}
