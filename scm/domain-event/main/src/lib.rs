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

// Re-export types used by other domain crates
pub use crate::api::EventBusConfig;
pub use crate::api::EventStoreError;
pub use crate::api::InMemoryEventStore;
pub use crate::api::NoopEventBus;
pub use crate::api::NoopEventPublisher;
pub use crate::api::InProcessEventBus;
