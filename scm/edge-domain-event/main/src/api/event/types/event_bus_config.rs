//! [`EventBusConfig`] — configuration for the in-process event bus.

/// Configuration for the [`InProcessEventBus`](super::InProcessEventBus).
#[derive(Debug, Clone)]
pub struct EventBusConfig {
    /// Broadcast channel capacity (number of messages buffered before lagging).
    pub capacity: usize,
}

impl Default for EventBusConfig {
    fn default() -> Self {
        Self { capacity: 1024 }
    }
}
