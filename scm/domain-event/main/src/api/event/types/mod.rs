pub mod closed_event_source;
pub mod event_bus_config;
pub mod event_envelope;
pub mod event_receiver;
pub mod expected_version;
pub mod in_memory_event_store;
pub mod in_process_event_bus;
pub mod noop_aggregate;
pub mod noop_domain_event;
pub mod noop_event_bus;
pub mod noop_event_publisher;
pub mod std_event_factory;

pub use closed_event_source::ClosedEventSource;
pub use event_bus_config::EventBusConfig;
pub use event_envelope::EventEnvelope;
pub use event_receiver::EventReceiver;
pub use expected_version::ExpectedVersion;
pub use in_memory_event_store::InMemoryEventStore;
pub use in_process_event_bus::InProcessEventBus;
pub use noop_aggregate::NoopAggregate;
pub use noop_domain_event::NoopDomainEvent;
pub use noop_event_bus::NoopEventBus;
pub use noop_event_publisher::NoopEventPublisher;
pub use std_event_factory::StdEventFactory;
/// [`DefaultEventFactory`] is a type alias for [`StdEventFactory`].
pub type DefaultEventFactory = StdEventFactory;
