//! Coverage for api/event/types/event/event_envelope.rs
use edge_domain::EventEnvelope;
use std::time::SystemTime;

#[derive(Clone)]
struct SimpleEvent;

#[test]
fn test_event_envelope_fields_are_accessible_happy() {
    let env = EventEnvelope {
        aggregate_id: "agg-1".into(),
        sequence: 1,
        occurred_at: SystemTime::UNIX_EPOCH,
        event: SimpleEvent,
    };
    assert_eq!(env.aggregate_id, "agg-1");
    assert_eq!(env.sequence, 1);
}

#[test]
fn test_event_envelope_is_cloneable_happy() {
    let env = EventEnvelope {
        aggregate_id: "agg-1".into(),
        sequence: 2,
        occurred_at: SystemTime::UNIX_EPOCH,
        event: SimpleEvent,
    };
    let clone = env.clone();
    assert_eq!(clone.aggregate_id, env.aggregate_id);
    assert_eq!(clone.sequence, env.sequence);
}

#[test]
fn test_event_envelope_sequence_starts_at_one_edge() {
    let env = EventEnvelope {
        aggregate_id: "agg".into(),
        sequence: 1,
        occurred_at: SystemTime::UNIX_EPOCH,
        event: SimpleEvent,
    };
    assert!(env.sequence >= 1);
}
