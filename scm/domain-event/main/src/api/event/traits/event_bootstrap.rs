//! `EventBootstrap` trait — constructor contract for standard event infrastructure.
//!
//! Every method has a default implementation that delegates to the concrete
//! type in `api/event/types/`. Callers implement this trait on a unit struct
//! (e.g. `struct Events; impl EventBootstrap for Events {}`) and call the
//! associated functions directly.

use crate::api::event::traits::DomainEvent;
use crate::api::event::types::{
    ClosedEventSource, DefaultEventFactory, EventBusConfig, InMemoryEventStore, InProcessEventBus,
    NoopAggregate, NoopDomainEvent, NoopEventBus, NoopEventPublisher, StdEventFactory,
};

/// Bootstrap namespace for the standard event infrastructure implementations.
///
/// Rule 228: every return type must be a concrete type defined in `api/types/`
/// so the structural auditor can resolve the dependency chain.
pub trait EventBootstrap {
    /// Identifies this bootstrap implementation.
    fn bootstrap_name(&self) -> &'static str {
        "event"
    }

    /// Construct a real in-process broadcast bus with the given config.
    fn in_process_bus(config: EventBusConfig) -> InProcessEventBus where Self: Sized {
        InProcessEventBus::new(config.capacity)
    }

    /// Construct a no-op event bus that silently discards all events.
    fn noop_bus() -> NoopEventBus where Self: Sized {
        NoopEventBus
    }

    /// Construct a no-op event publisher that silently discards all events.
    fn noop_publisher() -> NoopEventPublisher where Self: Sized {
        NoopEventPublisher
    }

    /// Construct an in-memory event store for aggregate type `E`.
    fn in_memory_store<E>() -> InMemoryEventStore<E>
    where
        E: DomainEvent + Clone + Send + Sync + 'static,
        Self: Sized,
    {
        InMemoryEventStore::new()
    }

    /// Construct a closed event source that immediately returns `Unavailable`.
    fn closed_source() -> ClosedEventSource where Self: Sized {
        ClosedEventSource
    }

    /// Construct the standard event factory itself.
    ///
    /// Returns a [`DefaultEventFactory`] (an alias for [`StdEventFactory`]) which
    /// can be used to call factory methods in a value-passing context.
    fn std() -> DefaultEventFactory where Self: Sized {
        StdEventFactory
    }

    /// Construct a no-op domain event that carries no state.
    ///
    /// Useful as a placeholder when a [`DomainEvent`] value is required but
    /// no meaningful event data exists (e.g. test fixtures, structural stubs).
    fn noop_event() -> NoopDomainEvent where Self: Sized {
        NoopDomainEvent
    }

    /// Construct a no-op aggregate root that holds no state.
    ///
    /// Useful as a placeholder when an [`Aggregate`] impl is required but
    /// no meaningful aggregate logic is needed.
    fn noop_aggregate() -> NoopAggregate where Self: Sized {
        NoopAggregate
    }
}
