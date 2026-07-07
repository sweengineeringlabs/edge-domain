//! `Event` theme — event sourcing and CQRS event bus contracts.

pub mod errors;
pub mod noop;
pub mod traits;
pub mod types;

// Rule 121 orphan mirrors — path-level files that exist alongside the sub-modules.
mod closed_event_source;
mod ins;

pub use errors::{EventError, EventStoreError};
pub use traits::{Aggregate, DomainEvent, EventBus, EventPublisher, EventSource, EventStore};
pub use types::{
    AggregateApplyRequest, AggregateApplyResponse, AggregateIdentityRequest,
    AggregateIdentityResponse, ClosedEventSource,
    EventAggregateIdRequest, EventAggregateIdResponse, EventBusConfig, EventBusPublishRequest,
    EventBusSubscribeRequest, EventBusSubscribeResponse, EventEnvelope, EventOccurredAtRequest,
    EventOccurredAtResponse, EventPublisherPublishRequest,
    EventSourceRecvNextRequest, EventSourceRecvNextResponse, EventStoreAppendRequest,
    EventStoreAppendResponse, EventStoreLoadFromRequest, EventStoreLoadFromResponse,
    EventStoreLoadRequest, EventStoreLoadResponse, EventTypeRequest, EventTypeResponse,
    ExpectedVersion, InMemoryEventStore, InProcessEventBus, NoopAggregate, NoopDomainEvent,
    NoopEventBus, NoopEventPublisher,
};
