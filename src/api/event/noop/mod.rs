//! API interface surface for no-op event implementations.

pub mod noop_event_bus;
pub mod noop_event_publisher;

pub use crate::api::types::noop::NoopEventBus;
pub use crate::api::types::noop::NoopEventPublisher;
