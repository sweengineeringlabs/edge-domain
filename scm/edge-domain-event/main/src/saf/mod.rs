//! SAF — event service facade.

mod event;

pub use crate::api::event::traits::{
    Aggregate, DomainEvent, EventBus, EventFactory, EventPublisher, EventSource, EventStore,
};
pub use crate::api::event::types::{
    ClosedEventSource, EventBusConfig, EventEnvelope, EventReceiver, ExpectedVersion,
    InMemoryEventStore, InProcessEventBus, NoopEventBus, NoopEventPublisher,
};
pub use crate::api::event::errors::{EventError, EventStoreError};
