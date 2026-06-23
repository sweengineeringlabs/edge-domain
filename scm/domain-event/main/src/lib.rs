//! # edge-domain-event
//!
//! The event port contracts — event sourcing, CQRS event bus, publish/subscribe.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use api::ClosedEventSource;
pub use api::EventBusConfig;
pub use api::EventEnvelope;
pub use api::EventError;
pub use api::EventReceiver;
pub use api::EventStoreError;
pub use api::ExpectedVersion;
pub use api::InMemoryEventStore;
pub use api::InProcessEventBus;
pub use api::NoopEventBus;
pub use api::NoopEventPublisher;
pub use saf::Aggregate;
pub use saf::DomainEvent;
pub use saf::EventBus;
pub use saf::EventBootstrap;
pub use saf::EventPublisher;
pub use saf::EventSource;
pub use saf::EventStore;
pub use saf::AGGREGATE_SVC;
pub use saf::DOMAIN_EVENT_SVC;
pub use saf::EVENT_BUS_SVC;
pub use saf::EVENT_FACTORY_SVC;
pub use saf::EVENT_PUBLISHER_SVC;
pub use saf::EVENT_SOURCE_SVC;
pub use saf::EVENT_STORE_SVC;
