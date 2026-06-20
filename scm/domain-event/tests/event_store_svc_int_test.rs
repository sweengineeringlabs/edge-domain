//! SAF facade tests — `EventStore` trait via `InMemoryEventStore`.

use edge_domain_event::{
    DomainEvent, EventBootstrap, EventStore, EventStoreError, ExpectedVersion,
};

struct Events;
impl EventBootstrap for Events {}

#[derive(Clone)]
struct OrderEvt(String);
impl DomainEvent for OrderEvt {
    fn event_type(&self) -> &str { "order.evt" }
    fn aggregate_id(&self) -> &str { &self.0 }
}

/// @covers: InMemoryEventStore::append — first write with Any succeeds
#[test]
fn test_append_any_first_write_returns_sequence_happy() {
    let store = Events::in_memory_store::<OrderEvt>();
    let seq = futures::executor::block_on(
        store.append("a1", vec![OrderEvt("a1".into())], ExpectedVersion::Any),
    )
    .expect("append");
    assert_eq!(seq, 1);
}

/// @covers: InMemoryEventStore::load — returns empty for unknown aggregate
#[test]
fn test_load_unknown_aggregate_returns_empty_error() {
    let store = Events::in_memory_store::<OrderEvt>();
    let events =
        futures::executor::block_on(store.load("unknown")).expect("load");
    assert!(events.is_empty());
}

/// @covers: InMemoryEventStore::append+load — round trip
#[test]
fn test_append_then_load_returns_appended_events_happy() {
    let store = Events::in_memory_store::<OrderEvt>();
    futures::executor::block_on(
        store.append("a2", vec![OrderEvt("a2".into()), OrderEvt("a2".into())], ExpectedVersion::Any),
    )
    .expect("append");
    let events = futures::executor::block_on(store.load("a2")).expect("load");
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].sequence, 1);
    assert_eq!(events[1].sequence, 2);
}

/// @covers: InMemoryEventStore::load_from — returns only events from sequence
#[test]
fn test_load_from_sequence_2_skips_first_event_edge() {
    let store = Events::in_memory_store::<OrderEvt>();
    futures::executor::block_on(
        store.append(
            "a3",
            vec![OrderEvt("a3".into()), OrderEvt("a3".into()), OrderEvt("a3".into())],
            ExpectedVersion::Any,
        ),
    )
    .expect("append");
    let events = futures::executor::block_on(store.load_from("a3", 2)).expect("load_from");
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].sequence, 2);
}

/// @covers: EventStoreError::Conflict — NoStream after stream exists
#[test]
fn test_append_no_stream_conflict_returns_err_error() {
    let store = Events::in_memory_store::<OrderEvt>();
    futures::executor::block_on(
        store.append("a4", vec![OrderEvt("a4".into())], ExpectedVersion::NoStream),
    )
    .expect("first");
    let err = futures::executor::block_on(
        store.append("a4", vec![OrderEvt("a4".into())], ExpectedVersion::NoStream),
    )
    .unwrap_err();
    assert!(matches!(err, EventStoreError::Conflict { .. }));
}

/// @covers: InMemoryEventStore::append — Exact version mismatch returns conflict
#[test]
fn test_append_exact_wrong_version_returns_conflict_edge() {
    let store = Events::in_memory_store::<OrderEvt>();
    let err = futures::executor::block_on(
        store.append("a5", vec![OrderEvt("a5".into())], ExpectedVersion::Exact(99)),
    )
    .unwrap_err();
    assert!(matches!(err, EventStoreError::Conflict { .. }));
}

/// @covers: InMemoryEventStore::load — returns appended events for known aggregate
#[test]
fn test_load_known_aggregate_returns_events_happy() {
    let store = Events::in_memory_store::<OrderEvt>();
    futures::executor::block_on(
        store.append("a6", vec![OrderEvt("a6".into())], ExpectedVersion::Any),
    )
    .expect("append");
    let events = futures::executor::block_on(store.load("a6")).expect("load");
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].aggregate_id, "a6");
}

/// @covers: InMemoryEventStore::load_from — returns events from sequence 1
#[test]
fn test_load_from_sequence_1_returns_all_events_happy() {
    let store = Events::in_memory_store::<OrderEvt>();
    futures::executor::block_on(
        store.append("a7", vec![OrderEvt("a7".into()), OrderEvt("a7".into())], ExpectedVersion::Any),
    )
    .expect("append");
    let events = futures::executor::block_on(store.load_from("a7", 1)).expect("load_from");
    assert_eq!(events.len(), 2);
}

/// @covers: InMemoryEventStore::load_from — unknown aggregate returns empty
#[test]
fn test_load_from_unknown_aggregate_returns_empty_error() {
    let store = Events::in_memory_store::<OrderEvt>();
    let events = futures::executor::block_on(store.load_from("missing", 1)).expect("load_from");
    assert!(events.is_empty());
}
