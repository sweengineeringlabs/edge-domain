//! # edge-domain-event
//!
//! The event port contracts — event sourcing, CQRS event bus, publish/subscribe.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use saf::Aggregate;
pub use saf::ClosedEventSource;
pub use saf::StdEventFactory;
pub use saf::DomainEvent;
pub use saf::EventBus;
pub use saf::EventBusConfig;
pub use saf::EventEnvelope;
pub use saf::EventError;
pub use saf::EventBootstrap;
pub use saf::EventPublisher;
pub use saf::EventReceiver;
pub use saf::EventSource;
pub use saf::EventStore;
pub use saf::EventStoreError;
pub use saf::ExpectedVersion;
pub use saf::InMemoryEventStore;
pub use saf::InProcessEventBus;
pub use saf::NoopAggregate;
pub use saf::NoopDomainEvent;
pub use saf::NoopEventBus;
pub use saf::NoopEventPublisher;
pub use saf::AGGREGATE_SVC;
pub use saf::DOMAIN_EVENT_SVC;
pub use saf::EVENT_BUS_SVC;
pub use saf::EVENT_FACTORY_SVC;
pub use saf::EVENT_PUBLISHER_SVC;
pub use saf::EVENT_SOURCE_SVC;
pub use saf::EVENT_STORE_SVC;
