//! No-op (stub) implementations for event infrastructure.

pub mod noop_event_bus;
pub mod noop_event_publisher;

pub use noop_event_bus::NoopEventBus;
pub use noop_event_publisher::NoopEventPublisher;
