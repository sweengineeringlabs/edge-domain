//! In-process implementations — in-memory event store and in-process event bus.

pub mod in_memory_event_store;
pub mod in_process_event_bus;

pub use in_memory_event_store::InMemoryEventStore;
pub use in_process_event_bus::InProcessEventBus;
