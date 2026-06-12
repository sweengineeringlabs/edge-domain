//! `EventFactory` trait — constructor contract for standard event infrastructure.
//!
//! Every method has a default implementation that delegates to the concrete
//! type in `api/event/types/`. Callers implement this trait on a unit struct
//! (e.g. `struct Events; impl EventFactory for Events {}`) and call the
//! associated functions directly.

use crate::api::event::traits::DomainEvent;
use crate::api::event::types::{
    ClosedEventSource, EventBusConfig, InMemoryEventStore, InProcessEventBus, NoopEventBus,
    NoopEventPublisher,
};

/// Factory for the standard event infrastructure implementations.
///
/// Rule 228: every return type must be a concrete type defined in `api/types/`
/// so the structural auditor can resolve the dependency chain.
pub trait EventFactory {
    /// Construct a real in-process broadcast bus with the given config.
    fn in_process_bus(config: EventBusConfig) -> InProcessEventBus {
        InProcessEventBus::new(config.capacity)
    }

    /// Construct a no-op event bus that silently discards all events.
    fn noop_bus() -> NoopEventBus {
        NoopEventBus
    }

    /// Construct a no-op event publisher that silently discards all events.
    fn noop_publisher() -> NoopEventPublisher {
        NoopEventPublisher
    }

    /// Construct an in-memory event store for aggregate type `E`.
    fn in_memory_store<E>() -> InMemoryEventStore<E>
    where
        E: DomainEvent + Clone + Send + Sync + 'static,
    {
        InMemoryEventStore::new()
    }

    /// Construct a closed event source that immediately returns `Unavailable`.
    fn closed_source() -> ClosedEventSource {
        ClosedEventSource
    }
}
