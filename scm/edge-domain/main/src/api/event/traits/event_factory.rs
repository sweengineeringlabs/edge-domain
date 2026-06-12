//! [`EventFactory`] — constructor contract for event infrastructure types.

use crate::api::event::types::closed_event_source::ClosedEventSource;
use crate::api::event::types::event_bus_config::EventBusConfig;
use crate::api::event::types::in_memory_event_store::InMemoryEventStore;
use crate::api::event::types::in_process_event_bus::InProcessEventBus;
use crate::api::event::types::noop_event_bus::NoopEventBus;
use crate::api::event::types::noop_event_publisher::NoopEventPublisher;

/// Factory trait for the standard event-infrastructure implementations.
pub trait EventFactory {
    /// Construct an [`InProcessEventBus`] configured by `config`.
    fn in_process_bus(config: EventBusConfig) -> InProcessEventBus {
        let _ = config;
        InProcessEventBus
    }

    /// Construct the [`NoopEventBus`] that discards all events.
    fn noop_bus() -> NoopEventBus {
        NoopEventBus
    }

    /// Construct the [`NoopEventPublisher`] that discards all events silently.
    fn noop_publisher() -> NoopEventPublisher {
        NoopEventPublisher
    }

    /// Construct the [`InMemoryEventStore`] marker type.
    fn in_memory_store() -> InMemoryEventStore {
        InMemoryEventStore
    }

    /// Construct the [`ClosedEventSource`] that immediately signals unavailability.
    fn closed_source() -> ClosedEventSource {
        ClosedEventSource
    }
}
