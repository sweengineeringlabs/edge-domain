//! `Event` theme — domain event contracts and event sourcing infrastructure.

pub mod closed_event_source;
pub mod errors;
pub mod noop;
pub mod traits;
pub mod types;

pub use errors::{EventError, EventStoreError};
pub use traits::{
    Aggregate, ClosedEventSource, DomainEvent, EventBus, EventBusConfig, EventFactory,
    EventPublisher, EventSource, EventStore, InMemoryEventStore, InProcessEventBus, NoopEventBus,
    NoopEventPublisher,
};
pub use types::{EventEnvelope, EventReceiver, ExpectedVersion};
