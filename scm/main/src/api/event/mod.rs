//! `Event` theme — domain event contracts and event sourcing infrastructure.

pub mod error;
pub mod traits;
pub mod types;
pub mod vo;

pub use error::{EventError, EventStoreError};
pub use traits::{Aggregate, DomainEvent, EventBus, EventPublisher, EventStore};
pub use types::{InMemoryEventStore, InProcessEventBus, NoopEventBus, NoopEventPublisher};
pub use vo::{
    EventBusConfig, EventEnvelope, EventReceiver, ExpectedVersion, StageCompleted, StageFailed,
    StageSkipped, StageStarted,
};
