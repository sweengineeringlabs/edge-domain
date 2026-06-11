//! `Event` theme — domain event contracts and event sourcing infrastructure.

pub mod closed_event_source;
pub mod errors;
pub mod ins;
pub mod noop;
pub mod stage;
pub mod traits;
pub mod types;

pub use errors::{EventError, EventStoreError};
pub use ins::{InMemoryEventStore, InProcessEventBus};
pub use noop::{NoopEventBus, NoopEventPublisher};
pub use stage::{
    StageCompleted, StageCompletedBuilder, StageFailed, StageFailedBuilder, StageSkipped,
    StageStarted,
};
pub use traits::{Aggregate, DomainEvent, EventBus, EventPublisher, EventStore};
pub use types::{
    ClosedEventSource, EventBusConfig, EventEnvelope, EventReceiver, EventSource, ExpectedVersion,
};
