//! Integration tests for the in-memory event store implementation.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::SystemTime;

use edge_domain::{
    Domain, DomainEvent, EventAggregateIdRequest, EventAggregateIdResponse, EventError,
    EventOccurredAtRequest, EventOccurredAtResponse, EventStoreAppendRequest,
    EventStoreLoadRequest, EventTypeRequest, EventTypeResponse, ExpectedVersion,
};

#[derive(Clone)]
struct Incremented {
    id: String,
}

impl DomainEvent for Incremented {
    fn event_type(&self, _req: EventTypeRequest) -> Result<EventTypeResponse<'_>, EventError> {
        Ok(EventTypeResponse {
            event_type: "counter.incremented",
        })
    }
    fn aggregate_id(
        &self,
        _req: EventAggregateIdRequest,
    ) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse {
            aggregate_id: &self.id,
        })
    }
    fn occurred_at(
        &self,
        _req: EventOccurredAtRequest,
    ) -> Result<EventOccurredAtResponse, EventError> {
        Ok(EventOccurredAtResponse {
            occurred_at: SystemTime::now(),
        })
    }
}

/// @covers: MemoryEventStore
#[tokio::test]
async fn test_in_memory_event_store_append_and_load_roundtrip() {
    let store = Domain.new_in_memory_event_store::<Incremented>();
    store
        .append(EventStoreAppendRequest {
            aggregate_id: "c1",
            events: vec![Incremented { id: "c1".into() }],
            expected: ExpectedVersion::NoStream,
        })
        .await
        .unwrap();
    let events = store
        .load(EventStoreLoadRequest { aggregate_id: "c1" })
        .await
        .unwrap()
        .events;
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].aggregate_id, "c1");
}

/// @covers: MemoryEventStore
#[tokio::test]
async fn test_in_memory_event_store_load_returns_empty_for_unknown_id() {
    let store = Domain.new_in_memory_event_store::<Incremented>();
    assert!(store
        .load(EventStoreLoadRequest {
            aggregate_id: "unknown"
        })
        .await
        .unwrap()
        .events
        .is_empty());
}
