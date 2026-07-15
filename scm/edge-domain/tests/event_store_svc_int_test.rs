#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — EventStore is exported from the crate root.
#![cfg(feature = "event")]

use edge_application::Domain;
use edge_application::DomainEvent;
use edge_application::EventAggregateIdRequest;
use edge_application::EventAggregateIdResponse;
use edge_application::EventError;
use edge_application::EventStore;
use edge_application::EventStoreAppendRequest;
use edge_application::EventStoreLoadRequest;
use edge_application::EventTypeRequest;
use edge_application::EventTypeResponse;
use edge_application::ExpectedVersion;
use std::sync::Arc;

#[derive(Clone)]
struct E;
impl DomainEvent for E {
    fn event_type(&self, _req: EventTypeRequest) -> Result<EventTypeResponse<'_>, EventError> {
        Ok(EventTypeResponse { event_type: "e" })
    }
    fn aggregate_id(
        &self,
        _req: EventAggregateIdRequest,
    ) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse { aggregate_id: "id" })
    }
}

#[tokio::test]
async fn test_event_store_svc_facade_load_returns_empty_for_unknown() {
    let store: Arc<dyn EventStore<Event = E>> = Domain.new_in_memory_event_store();
    let events = store
        .load(EventStoreLoadRequest {
            aggregate_id: "none",
        })
        .await
        .unwrap();
    assert!(events.events.is_empty());
}

#[tokio::test]
async fn test_event_store_svc_facade_append_and_load_roundtrip() {
    let store: Arc<dyn EventStore<Event = E>> = Domain.new_in_memory_event_store();
    store
        .append(EventStoreAppendRequest {
            aggregate_id: "agg",
            events: vec![E],
            expected: ExpectedVersion::NoStream,
        })
        .await
        .unwrap();
    let loaded = store
        .load(EventStoreLoadRequest {
            aggregate_id: "agg",
        })
        .await
        .unwrap();
    assert_eq!(loaded.events.len(), 1);
}
