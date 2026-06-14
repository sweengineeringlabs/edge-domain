#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — EventStore is exported from the crate root.

use edge_domain::Domain;
use edge_domain::DomainEvent;
use edge_domain::EventStore;
use edge_domain::ExpectedVersion;
use std::sync::Arc;

#[derive(Clone)]
struct E;
impl DomainEvent for E {
    fn event_type(&self) -> &str {
        "e"
    }
    fn aggregate_id(&self) -> &str {
        "id"
    }
}

#[tokio::test]
async fn test_event_store_svc_facade_load_returns_empty_for_unknown() {
    let store: Arc<dyn EventStore<Event = E>> = Domain::new_in_memory_event_store();
    let events = store.load("none").await.unwrap();
    assert!(events.is_empty());
}

#[tokio::test]
async fn test_event_store_svc_facade_append_and_load_roundtrip() {
    let store: Arc<dyn EventStore<Event = E>> = Domain::new_in_memory_event_store();
    store
        .append("agg", vec![E], ExpectedVersion::NoStream)
        .await
        .unwrap();
    let loaded = store.load("agg").await.unwrap();
    assert_eq!(loaded.len(), 1);
}
