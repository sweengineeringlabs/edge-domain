//! No-op event implementations for development and testing.

pub mod noop_event_bus;
pub mod noop_event_publisher;

pub use noop_event_bus::NoopEventBus;
pub use noop_event_publisher::NoopEventPublisher;
