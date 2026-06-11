//! Event theme — neutral types and value objects.

pub mod closed_event_source;
pub mod event;
pub mod expected_version;
pub mod ins;
pub mod noop;
pub mod stage;

pub use closed_event_source::ClosedEventSource;
pub use event::EventBusConfig;
pub use event::EventEnvelope;
pub use event::EventReceiver;
pub use event::EventSource;
pub use expected_version::ExpectedVersion;
pub use ins::InMemoryEventStore;
pub use ins::InProcessEventBus;
pub use noop::NoopEventBus;
pub use noop::NoopEventPublisher;
pub use stage::StageCompleted;
pub use stage::StageCompletedBuilder;
pub use stage::StageFailed;
pub use stage::StageFailedBuilder;
pub use stage::StageSkipped;
pub use stage::StageStarted;
