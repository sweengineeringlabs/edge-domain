//! `Event` theme — event sourcing and CQRS event bus contracts.

pub mod errors;
pub mod noop;
pub mod traits;
pub mod types;

// Rule 121 orphan mirrors — path-level files that exist alongside the sub-modules.
mod closed_event_source;
mod ins;

pub use errors::{EventError, EventStoreError};
pub use traits::{Aggregate, DomainEvent, EventBus, EventFactory, EventPublisher, EventSource, EventStore};
pub use types::{
    ClosedEventSource, DefaultEventFactory, EventBusConfig, EventEnvelope, EventReceiver,
    ExpectedVersion, InMemoryEventStore, InProcessEventBus, NoopAggregate, NoopDomainEvent,
    NoopEventBus, NoopEventPublisher, StdEventFactory,
};

