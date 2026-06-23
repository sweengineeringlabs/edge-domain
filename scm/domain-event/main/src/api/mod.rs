mod event;
pub use event::Aggregate;
pub use event::ClosedEventSource;
pub use event::DomainEvent;
pub use event::EventBus;
pub use event::EventEnvelope;
pub use event::EventError;
pub use event::EventBootstrap;
pub use event::EventPublisher;
pub use event::EventReceiver;
pub use event::EventSource;
pub use event::EventStore;
pub use event::EventStoreError;
pub use event::ExpectedVersion;
pub use event::InMemoryEventStore;
pub use event::InProcessEventBus;
pub use event::NoopAggregate;
pub use event::NoopDomainEvent;
pub use event::NoopEventBus;
pub use event::NoopEventPublisher;
pub use event::StdEventFactory;

// Re-export for public API (needed for trait signatures and test usage)
pub use event::EventBusConfig;
