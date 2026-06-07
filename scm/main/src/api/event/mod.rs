//! `Event` theme — domain event contracts and event sourcing infrastructure.

pub mod error;
pub mod traits;
pub mod types;

pub use error::{EventError, EventStoreError};
pub use traits::{Aggregate, DomainEvent, EventBus, EventPublisher, EventStore};
pub use types::{
    EventBusConfig, EventEnvelope, EventReceiver, EventSource, ExpectedVersion, InMemoryEventStore,
    InProcessEventBus, NoopEventBus, NoopEventPublisher, StageCompleted, StageFailed, StageSkipped,
    StageStarted,
};
