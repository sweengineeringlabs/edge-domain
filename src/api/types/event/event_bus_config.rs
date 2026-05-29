//! [`EventBusConfig`] — tuning knobs for the in-process event bus.

/// Configuration for [`crate::EventBus`] implementations.
///
/// Deserializes from the `[event_bus]` section of the application TOML config.
#[derive(Debug, Clone)]
pub struct EventBusConfig {
    /// Broadcast channel buffer capacity.
    ///
    /// Slow subscribers that fall behind by more than `capacity` events will
    /// receive a [`crate::EventError::BroadcastLagged`] error and miss messages.
    pub capacity: usize,
}

impl Default for EventBusConfig {
    fn default() -> Self {
        Self { capacity: 1024 }
    }
}
