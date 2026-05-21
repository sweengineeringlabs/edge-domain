//! `Event` module — domain event contracts and event sourcing infrastructure.

pub mod aggregate;
pub mod domain_event;
pub mod event_bus;
pub mod event_bus_config;
pub mod event_envelope;
pub mod event_error;
pub mod event_publisher;
pub mod event_receiver;
pub mod event_store;
pub mod event_store_error;
pub mod expected_version;
pub mod in_memory_event_store;
pub mod noop;

pub use aggregate::Aggregate;
pub use domain_event::DomainEvent;
pub use event_bus::EventBus;
pub use event_bus_config::EventBusConfig;
pub use event_envelope::EventEnvelope;
pub use event_error::EventError;
pub use event_publisher::EventPublisher;
pub use event_receiver::EventReceiver;
pub use event_store::EventStore;
pub use event_store_error::EventStoreError;
pub use expected_version::ExpectedVersion;
