//! Noop event bus — SEA structural counterpart.
//!
//! The real implementation lives in [`crate::core::event::noop_event_bus`].

/// Marker type for the noop event bus (rule 89 compliance).
///
/// Implementation is provided by [`crate::core::event::noop_event_bus::NoopEventBus`].
pub(crate) struct NoopEventBus;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_event_bus_noop_marker_is_constructible() {
        let _ = NoopEventBus;
    }
}
