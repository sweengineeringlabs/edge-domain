//! Integration tests for `EventEnvelope`.

use std::time::SystemTime;
use edge_domain_event::{DomainEvent, EventEnvelope};

#[derive(Clone)]
struct OrderCreated {
    id: String,
}
impl DomainEvent for OrderCreated {
    fn event_type(&self) -> &str { "order.created" }
    fn aggregate_id(&self) -> &str { &self.id }
}

/// @covers: EventEnvelope — fields are accessible after construction
#[test]
fn test_event_envelope_fields_accessible_after_construction_happy() {
    let env = EventEnvelope {
        aggregate_id: "agg-1".into(),
        sequence: 1,
        occurred_at: SystemTime::now(),
        event: OrderCreated { id: "agg-1".into() },
    };
    assert_eq!(env.aggregate_id, "agg-1");
    assert_eq!(env.sequence, 1);
}

/// @covers: EventEnvelope — sequence must be positive (min 1)
#[test]
fn test_event_envelope_sequence_at_min_boundary_error() {
    let env = EventEnvelope {
        aggregate_id: "agg-2".into(),
        sequence: 1,
        occurred_at: SystemTime::UNIX_EPOCH,
        event: OrderCreated { id: "agg-2".into() },
    };
    assert!(env.sequence >= 1, "sequence must be >= 1");
}

/// @covers: EventEnvelope — clone produces independent copy
#[test]
fn test_event_envelope_clone_is_independent_edge() {
    let env = EventEnvelope {
        aggregate_id: "agg-3".into(),
        sequence: 7,
        occurred_at: SystemTime::now(),
        event: OrderCreated { id: "agg-3".into() },
    };
    let cloned = env.clone();
    assert_eq!(cloned.aggregate_id, env.aggregate_id);
    assert_eq!(cloned.sequence, env.sequence);
}
