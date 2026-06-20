//! Integration tests for `InMemoryEventStore`.

use edge_domain_event::{DomainEvent, EventBootstrap, EventStore, EventStoreError, ExpectedVersion};

struct Events;
impl EventBootstrap for Events {}

#[derive(Clone)]
struct ItemEvt(String);
impl DomainEvent for ItemEvt {
    fn event_type(&self) -> &str { "item.evt" }
    fn aggregate_id(&self) -> &str { &self.0 }
}

/// @covers: InMemoryEventStore::new — creates empty store (via factory)
#[test]
fn test_in_memory_event_store_new_creates_empty_store_happy() {
    let store = Events::in_memory_store::<ItemEvt>();
    let events = futures::executor::block_on(store.load("any")).expect("load");
    assert!(events.is_empty());
}

/// @covers: InMemoryEventStore::default — default store is usable
#[test]
fn test_in_memory_event_store_default_same_as_new_error() {
    let store = Events::in_memory_store::<ItemEvt>();
    let result = futures::executor::block_on(
        store.append("x", vec![ItemEvt("x".into())], ExpectedVersion::Any),
    );
    assert!(result.is_ok());
}

/// @covers: InMemoryEventStore — multiple aggregates are independent
#[test]
fn test_in_memory_event_store_multiple_aggregates_independent_edge() {
    let store = Events::in_memory_store::<ItemEvt>();
    futures::executor::block_on(
        store.append("agg-a", vec![ItemEvt("agg-a".into())], ExpectedVersion::Any),
    ).expect("append a");
    futures::executor::block_on(
        store.append("agg-b", vec![ItemEvt("agg-b".into())], ExpectedVersion::Any),
    ).expect("append b");
    let events_a = futures::executor::block_on(store.load("agg-a")).expect("load a");
    let events_b = futures::executor::block_on(store.load("agg-b")).expect("load b");
    assert_eq!(events_a.len(), 1);
    assert_eq!(events_b.len(), 1);
    assert_eq!(events_a[0].aggregate_id, "agg-a");
    assert_eq!(events_b[0].aggregate_id, "agg-b");
}

/// @covers: InMemoryEventStore::append — Conflict error has correct fields
#[test]
fn test_in_memory_event_store_conflict_error_fields_correct_happy() {
    let store = Events::in_memory_store::<ItemEvt>();
    futures::executor::block_on(
        store.append("x", vec![ItemEvt("x".into())], ExpectedVersion::NoStream),
    ).expect("first");
    let err = futures::executor::block_on(
        store.append("x", vec![ItemEvt("x".into())], ExpectedVersion::NoStream),
    ).unwrap_err();
    if let EventStoreError::Conflict { aggregate_id, expected, actual } = err {
        assert_eq!(aggregate_id, "x");
        assert_eq!(expected, 0);
        assert_eq!(actual, 1);
    } else {
        panic!("expected Conflict");
    }
}

/// @covers: InMemoryEventStore::load_from — returns empty for future sequence
#[test]
fn test_in_memory_event_store_load_from_future_seq_returns_empty_error() {
    let store = Events::in_memory_store::<ItemEvt>();
    futures::executor::block_on(
        store.append("x", vec![ItemEvt("x".into())], ExpectedVersion::Any),
    ).expect("append");
    let events = futures::executor::block_on(store.load_from("x", 999)).expect("load_from");
    assert!(events.is_empty());
}

/// @covers: InMemoryEventStore::load_from — returns events at exact sequence boundary
#[test]
fn test_in_memory_event_store_load_from_exact_boundary_edge() {
    let store = Events::in_memory_store::<ItemEvt>();
    futures::executor::block_on(
        store.append("x", vec![ItemEvt("x".into()), ItemEvt("x".into()), ItemEvt("x".into())], ExpectedVersion::Any),
    ).expect("append");
    let events = futures::executor::block_on(store.load_from("x", 3)).expect("load_from");
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].sequence, 3);
}
