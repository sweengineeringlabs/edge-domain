//! Integration tests for the event sourcing contracts.
//!
//! Uses only the public SAF surface — `new_in_memory_event_store` and
//! `reconstitute` — to verify the full append → load → reconstitute cycle.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::SystemTime;

use edge_domain::{
    Aggregate, AggregateApplyRequest, AggregateApplyResponse, AggregateIdentityRequest,
    AggregateIdentityResponse, Domain, DomainEvent, EventAggregateIdRequest,
    EventAggregateIdResponse, EventError, EventOccurredAtRequest, EventOccurredAtResponse,
    EventStoreAppendRequest, EventStoreError, EventStoreLoadFromRequest, EventStoreLoadRequest,
    EventTypeRequest, EventTypeResponse, ExpectedVersion,
};

// ── test fixtures ─────────────────────────────────────────────────────────────

#[derive(Clone)]
struct CounterIncremented {
    counter_id: String,
}

impl DomainEvent for CounterIncremented {
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
            aggregate_id: &self.counter_id,
        })
    }
    fn occurred_at(
        &self,
        _req: EventOccurredAtRequest,
    ) -> Result<EventOccurredAtResponse, EventError> {
        Ok(EventOccurredAtResponse {
            occurred_at: SystemTime::UNIX_EPOCH,
        })
    }
}

#[derive(Default)]
struct Counter {
    id: String,
    value: u64,
}

impl Aggregate for Counter {
    type Event = CounterIncremented;

    fn apply(
        &mut self,
        req: AggregateApplyRequest<'_, CounterIncremented>,
    ) -> Result<AggregateApplyResponse, EventError> {
        self.id = req.event.counter_id.clone();
        self.value += 1;
        Ok(AggregateApplyResponse)
    }

    fn id(
        &self,
        _req: AggregateIdentityRequest,
    ) -> Result<AggregateIdentityResponse<'_>, EventError> {
        Ok(AggregateIdentityResponse { id: &self.id })
    }
}

// ── append / load ─────────────────────────────────────────────────────────────

/// @covers: new_in_memory_event_store
#[test]
fn test_new_in_memory_event_store_is_constructible() {
    let store = Domain::new_in_memory_event_store::<CounterIncremented>();
    assert_ne!(std::mem::size_of_val(&store), 0);
}

/// @covers: EventStore::append — returns version 1 after first event.
#[tokio::test]
async fn test_append_returns_new_stream_version() {
    let store = Domain::new_in_memory_event_store::<CounterIncremented>();
    let ver = store
        .append(EventStoreAppendRequest {
            aggregate_id: "c1",
            events: vec![CounterIncremented {
                counter_id: "c1".into(),
            }],
            expected: ExpectedVersion::NoStream,
        })
        .await
        .expect("append must succeed")
        .sequence;
    assert_eq!(ver, 1);
}

/// @covers: EventStore::load — empty vec for unknown aggregate.
#[tokio::test]
async fn test_load_returns_empty_for_unknown_aggregate() {
    let store = Domain::new_in_memory_event_store::<CounterIncremented>();
    let events = store
        .load(EventStoreLoadRequest {
            aggregate_id: "unknown",
        })
        .await
        .expect("load must succeed");
    assert!(events.events.is_empty());
}

/// @covers: EventStore::append + load — events are stored and returned in order.
#[tokio::test]
async fn test_append_and_load_round_trip() {
    let store = Domain::new_in_memory_event_store::<CounterIncremented>();
    store
        .append(EventStoreAppendRequest {
            aggregate_id: "c1",
            events: vec![
                CounterIncremented {
                    counter_id: "c1".into(),
                },
                CounterIncremented {
                    counter_id: "c1".into(),
                },
            ],
            expected: ExpectedVersion::NoStream,
        })
        .await
        .expect("append");

    let events = store
        .load(EventStoreLoadRequest { aggregate_id: "c1" })
        .await
        .expect("load")
        .events;
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].sequence, 1);
    assert_eq!(events[1].sequence, 2);
}

/// @covers: EventStore::load_from — returns only events at or after given sequence.
#[tokio::test]
async fn test_load_from_returns_subset_from_sequence() {
    let store = Domain::new_in_memory_event_store::<CounterIncremented>();
    store
        .append(EventStoreAppendRequest {
            aggregate_id: "c1",
            events: vec![
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
            expected: ExpectedVersion::Any,
        })
        .await
        .expect("append");

    let events = store
        .load_from(EventStoreLoadFromRequest {
            aggregate_id: "c1",
            from_sequence: 2,
        })
        .await
        .expect("load_from")
        .events;
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].sequence, 2);
}

// ── optimistic concurrency ────────────────────────────────────────────────────

/// @covers: ExpectedVersion::NoStream — rejects append when stream already exists.
#[tokio::test]
async fn test_append_no_stream_conflicts_when_stream_exists() {
    let store = Domain::new_in_memory_event_store::<CounterIncremented>();
    store
        .append(EventStoreAppendRequest {
            aggregate_id: "c1",
            events: vec![CounterIncremented {
                counter_id: "c1".into(),
            }],
            expected: ExpectedVersion::NoStream,
        })
        .await
        .expect("first append");

    let err = store
        .append(EventStoreAppendRequest {
            aggregate_id: "c1",
            events: vec![CounterIncremented {
                counter_id: "c1".into(),
            }],
            expected: ExpectedVersion::NoStream,
        })
        .await
        .expect_err("second append must conflict");

    assert!(matches!(err, EventStoreError::Conflict { .. }));
}

/// @covers: ExpectedVersion::Exact — rejects append when version differs.
#[tokio::test]
async fn test_append_exact_version_conflicts_on_mismatch() {
    let store = Domain::new_in_memory_event_store::<CounterIncremented>();
    store
        .append(EventStoreAppendRequest {
            aggregate_id: "c1",
            events: vec![CounterIncremented {
                counter_id: "c1".into(),
            }],
            expected: ExpectedVersion::Any,
        })
        .await
        .expect("first append");

    let err = store
        .append(EventStoreAppendRequest {
            aggregate_id: "c1",
            events: vec![CounterIncremented {
                counter_id: "c1".into(),
            }],
            expected: ExpectedVersion::Exact(0),
        })
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
            .append(EventStoreAppendRequest {
                aggregate_id: "c1",
                events: vec![CounterIncremented {
                    counter_id: "c1".into(),
                }],
                expected: ExpectedVersion::Any,
            })
            .await
            .expect("Any must never conflict");
    }
    let events = store
        .load(EventStoreLoadRequest { aggregate_id: "c1" })
        .await
        .expect("load")
        .events;
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
        .append(EventStoreAppendRequest {
            aggregate_id: "c1",
            events: vec![
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
            expected: ExpectedVersion::NoStream,
        })
        .await
        .expect("append");

    let counter = Domain::reconstitute::<Counter>(&*store, "c1")
        .await
        .expect("reconstitute")
        .expect("must exist");

    assert_eq!(counter.value, 3);
    assert_eq!(counter.id(AggregateIdentityRequest).unwrap().id, "c1");
}

/// @covers: reconstitute — idempotent across multiple calls.
#[tokio::test]
async fn test_reconstitute_is_idempotent() {
    let store = Domain::new_in_memory_event_store::<CounterIncremented>();
    store
        .append(EventStoreAppendRequest {
            aggregate_id: "c1",
            events: vec![CounterIncremented {
                counter_id: "c1".into(),
            }],
            expected: ExpectedVersion::NoStream,
        })
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
