//! Integration tests for the event sourcing contracts.
//!
//! Uses only the public SAF surface — `new_in_memory_event_store` and
//! `reconstitute` — to verify the full append → load → reconstitute cycle.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::SystemTime;

use edge_domain::{Aggregate, Domain, DomainEvent, EventStoreError, ExpectedVersion};

// ── test fixtures ─────────────────────────────────────────────────────────────

#[derive(Clone)]
struct CounterIncremented {
    counter_id: String,
}

impl DomainEvent for CounterIncremented {
    fn event_type(&self) -> &str {
        "counter.incremented"
    }
    fn aggregate_id(&self) -> &str {
        &self.counter_id
    }
    fn occurred_at(&self) -> SystemTime {
        SystemTime::UNIX_EPOCH
    }
}

#[derive(Default)]
struct Counter {
    id: String,
    value: u64,
}

impl Aggregate for Counter {
    type Event = CounterIncremented;

    fn apply(&mut self, event: &CounterIncremented) {
        self.id = event.counter_id.clone();
        self.value += 1;
    }

    fn id(&self) -> &str {
        &self.id
    }
}

// ── append / load ─────────────────────────────────────────────────────────────

/// @covers: new_in_memory_event_store
#[test]
fn test_new_in_memory_event_store_is_constructible() {
    let _store = Domain::new_in_memory_event_store::<CounterIncremented>();
}

/// @covers: EventStore::append — returns version 1 after first event.
#[tokio::test]
async fn test_append_returns_new_stream_version() {
    let store = Domain::new_in_memory_event_store::<CounterIncremented>();
    let ver = store
        .append(
            "c1",
            vec![CounterIncremented {
                counter_id: "c1".into(),
            }],
            ExpectedVersion::NoStream,
        )
        .await
        .expect("append must succeed");
    assert_eq!(ver, 1);
}

/// @covers: EventStore::load — empty vec for unknown aggregate.
#[tokio::test]
async fn test_load_returns_empty_for_unknown_aggregate() {
    let store = Domain::new_in_memory_event_store::<CounterIncremented>();
    let events = store.load("unknown").await.expect("load must succeed");
    assert!(events.is_empty());
}

/// @covers: EventStore::append + load — events are stored and returned in order.
#[tokio::test]
async fn test_append_and_load_round_trip() {
    let store = Domain::new_in_memory_event_store::<CounterIncremented>();
    store
        .append(
            "c1",
            vec![
                CounterIncremented {
                    counter_id: "c1".into(),
                },
                CounterIncremented {
                    counter_id: "c1".into(),
                },
            ],
            ExpectedVersion::NoStream,
        )
        .await
        .expect("append");

    let events = store.load("c1").await.expect("load");
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].sequence, 1);
    assert_eq!(events[1].sequence, 2);
}

/// @covers: EventStore::load_from — returns only events at or after given sequence.
#[tokio::test]
async fn test_load_from_returns_subset_from_sequence() {
    let store = Domain::new_in_memory_event_store::<CounterIncremented>();
    store
        .append(
            "c1",
            vec![
                CounterIncremented {
                    counter_id: "c1".into(),
                },
                CounterIncremented {
                    counter_id: "c1".into(),
                },
                CounterIncremented {
                    counter_id: "c1".into(),
                },
            ],
            ExpectedVersion::Any,
        )
        .await
        .expect("append");

    let events = store.load_from("c1", 2).await.expect("load_from");
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].sequence, 2);
}

// ── optimistic concurrency ────────────────────────────────────────────────────

/// @covers: ExpectedVersion::NoStream — rejects append when stream already exists.
#[tokio::test]
async fn test_append_no_stream_conflicts_when_stream_exists() {
    let store = Domain::new_in_memory_event_store::<CounterIncremented>();
    store
        .append(
            "c1",
            vec![CounterIncremented {
                counter_id: "c1".into(),
            }],
            ExpectedVersion::NoStream,
        )
        .await
        .expect("first append");

    let err = store
        .append(
            "c1",
            vec![CounterIncremented {
                counter_id: "c1".into(),
            }],
            ExpectedVersion::NoStream,
        )
        .await
        .expect_err("second append must conflict");

    assert!(matches!(err, EventStoreError::Conflict { .. }));
}

/// @covers: ExpectedVersion::Exact — rejects append when version differs.
#[tokio::test]
async fn test_append_exact_version_conflicts_on_mismatch() {
    let store = Domain::new_in_memory_event_store::<CounterIncremented>();
    store
        .append(
            "c1",
            vec![CounterIncremented {
                counter_id: "c1".into(),
            }],
            ExpectedVersion::Any,
        )
        .await
        .expect("first append");

    let err = store
        .append(
            "c1",
            vec![CounterIncremented {
                counter_id: "c1".into(),
            }],
            ExpectedVersion::Exact(0),
        )
        .await
        .expect_err("version mismatch must conflict");

    assert!(matches!(
        err,
        EventStoreError::Conflict {
            expected: 0,
            actual: 1,
            ..
        }
    ));
}

/// @covers: ExpectedVersion::Any — always succeeds regardless of version.
#[tokio::test]
async fn test_append_any_version_never_conflicts() {
    let store = Domain::new_in_memory_event_store::<CounterIncremented>();
    for _ in 0..3 {
        store
            .append(
                "c1",
                vec![CounterIncremented {
                    counter_id: "c1".into(),
                }],
                ExpectedVersion::Any,
            )
            .await
            .expect("Any must never conflict");
    }
    let events = store.load("c1").await.expect("load");
    assert_eq!(events.len(), 3);
}

// ── reconstitute ──────────────────────────────────────────────────────────────

/// @covers: reconstitute — returns None for aggregate that was never created.
#[tokio::test]
async fn test_reconstitute_returns_none_for_unknown_aggregate() {
    let store = Domain::new_in_memory_event_store::<CounterIncremented>();
    let result = Domain::reconstitute::<Counter>(&*store, "missing")
        .await
        .expect("reconstitute");
    assert!(result.is_none());
}

/// @covers: reconstitute — replays events and rebuilds correct aggregate state.
#[tokio::test]
async fn test_reconstitute_rebuilds_aggregate_from_events() {
    let store = Domain::new_in_memory_event_store::<CounterIncremented>();
    store
        .append(
            "c1",
            vec![
                CounterIncremented {
                    counter_id: "c1".into(),
                },
                CounterIncremented {
                    counter_id: "c1".into(),
                },
                CounterIncremented {
                    counter_id: "c1".into(),
                },
            ],
            ExpectedVersion::NoStream,
        )
        .await
        .expect("append");

    let counter = Domain::reconstitute::<Counter>(&*store, "c1")
        .await
        .expect("reconstitute")
        .expect("must exist");

    assert_eq!(counter.value, 3);
    assert_eq!(counter.id(), "c1");
}

/// @covers: reconstitute — idempotent across multiple calls.
#[tokio::test]
async fn test_reconstitute_is_idempotent() {
    let store = Domain::new_in_memory_event_store::<CounterIncremented>();
    store
        .append(
            "c1",
            vec![CounterIncremented {
                counter_id: "c1".into(),
            }],
            ExpectedVersion::NoStream,
        )
        .await
        .expect("append");

    let a = Domain::reconstitute::<Counter>(&*store, "c1")
        .await
        .expect("first")
        .expect("some");
    let b = Domain::reconstitute::<Counter>(&*store, "c1")
        .await
        .expect("second")
        .expect("some");
    assert_eq!(a.value, b.value);
}
