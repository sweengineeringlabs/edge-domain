pub use crate::api::EventStoreError;
pub use crate::api::EventStore;
pub use crate::api::EventEnvelope;
pub use crate::api::ExpectedVersion;
pub use crate::api::InMemoryEventStore;

/// Service name token for the event store port contract.
pub const EVENT_STORE_SVC: &str = "event_store";
