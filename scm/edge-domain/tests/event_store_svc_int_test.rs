#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — EventStore is exported from the crate root.

use edge_domain::Domain;
use edge_domain::DomainEvent;
use edge_domain::EventAggregateIdRequest;
use edge_domain::EventAggregateIdResponse;
use edge_domain::EventError;
use edge_domain::EventStore;
use edge_domain::EventStoreAppendRequest;
use edge_domain::EventStoreLoadRequest;
use edge_domain::EventTypeRequest;
use edge_domain::EventTypeResponse;
use edge_domain::ExpectedVersion;
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
    let store: Arc<dyn EventStore<Event = E>> = Domain::new_in_memory_event_store();
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
    let store: Arc<dyn EventStore<Event = E>> = Domain::new_in_memory_event_store();
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
