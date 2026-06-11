//! `Event` theme — domain event contracts and event sourcing infrastructure.

pub mod closed_event_source;
pub mod errors;
pub mod noop;
pub mod traits;
pub mod types;

pub use errors::{EventError, EventStoreError};
pub use traits::{Aggregate, DomainEvent, EventBus, EventPublisher, EventSource, EventStore};
pub use types::{
    ClosedEventSource, EventBusConfig, EventEnvelope, EventReceiver, ExpectedVersion,
    InMemoryEventStore, InProcessEventBus, NoopEventBus, NoopEventPublisher, StageCompleted,
    StageCompletedBuilder, StageFailed, StageFailedBuilder, StageSkipped, StageStarted,
};
