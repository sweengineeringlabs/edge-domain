//! `Event` theme — domain event contracts and event sourcing infrastructure.

pub mod closed_event_source;
pub mod errors;
pub mod noop;
pub mod traits;
pub mod types;

pub use errors::{EventError, EventStoreError};
pub use traits::{Aggregate, DomainEvent, EventBus, EventPublisher, EventStore};
pub use types::ClosedEventSource;
pub use types::{
    EventBusConfig, EventEnvelope, EventReceiver, EventSource, ExpectedVersion, InMemoryEventStore,
    InProcessEventBus, NoopEventBus, NoopEventPublisher, StageCompleted, StageFailed,
    StageFailedBuilder, StageSkipped, StageStarted,
};
