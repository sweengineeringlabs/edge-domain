//! API interface surface for the tokio event bus implementation.

/// API marker type for the tokio-backed event bus.
///
/// The implementation lives in `core::event::tokio_event_bus`.
/// Consumers obtain it via [`crate::tokio_event_bus`].
#[allow(dead_code)]
pub struct TokioEventBus;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokio_event_bus_api_marker_is_constructible() {
        let _ = TokioEventBus;
    }
}
