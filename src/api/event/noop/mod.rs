//! API interface surface for no-op event implementations.

#[allow(clippy::module_inception)]
pub(crate) mod noop_event_bus;
pub(crate) mod noop_event_publisher;

#[allow(unused_imports)]
pub use noop_event_bus::NoopEventBus;
#[allow(unused_imports)]
pub use noop_event_publisher::NoopEventPublisher;
