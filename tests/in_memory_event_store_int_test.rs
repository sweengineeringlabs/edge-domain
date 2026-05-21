//! Integration tests for the in-memory event store implementation.

use std::time::SystemTime;

use edge_domain::{new_in_memory_event_store, DomainEvent, ExpectedVersion};

#[derive(Clone)]
struct Incremented {
    id: String,
}

impl DomainEvent for Incremented {
    fn event_type(&self) -> &str {
        "counter.incremented"
    }
    fn aggregate_id(&self) -> &str {
        &self.id
    }
    fn occurred_at(&self) -> SystemTime {
        SystemTime::now()
    }
}

/// @covers: InMemoryEventStore
#[tokio::test]
async fn test_in_memory_event_store_append_and_load_roundtrip() {
    let store = new_in_memory_event_store::<Incremented>();
    store
        .append(
            "c1",
            vec![Incremented { id: "c1".into() }],
            ExpectedVersion::NoStream,
        )
        .await
        .unwrap();
    let events = store.load("c1").await.unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].aggregate_id, "c1");
}

/// @covers: InMemoryEventStore
#[tokio::test]
async fn test_in_memory_event_store_load_returns_empty_for_unknown_id() {
    let store = new_in_memory_event_store::<Incremented>();
    assert!(store.load("unknown").await.unwrap().is_empty());
}
