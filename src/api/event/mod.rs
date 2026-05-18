//! `Event` module — domain event contracts and event sourcing infrastructure.

pub mod aggregate;
pub mod domain_event;
pub mod event_envelope;
pub mod event_error;
pub mod event_publisher;
pub mod event_store;
pub mod event_store_error;
pub mod expected_version;
pub mod noop_event_publisher;

pub use aggregate::Aggregate;
pub use domain_event::DomainEvent;
pub use event_envelope::EventEnvelope;
pub use event_error::EventError;
pub use event_publisher::EventPublisher;
pub use event_store::EventStore;
pub use event_store_error::EventStoreError;
pub use expected_version::ExpectedVersion;
