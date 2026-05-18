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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_capacity_is_1024() {
        assert_eq!(EventBusConfig::default().capacity, 1024);
    }

    #[test]
    fn test_custom_capacity_is_stored() {
        let cfg = EventBusConfig { capacity: 512 };
        assert_eq!(cfg.capacity, 512);
    }
}
