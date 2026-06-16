pub use crate::api::event::errors::EventStoreError;
pub use crate::api::event::traits::EventStore;
pub use crate::api::event::types::EventEnvelope;
pub use crate::api::event::types::ExpectedVersion;
pub use crate::api::event::types::InMemoryEventStore;

/// Service name token for the event store port contract.
pub const EVENT_STORE_SVC: &str = "event_store";
