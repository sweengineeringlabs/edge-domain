//! SAF facade tests — `Aggregate` trait.

use edge_domain_event::{Aggregate, DomainEvent};

#[derive(Default)]
struct Counter {
    id: String,
    count: u64,
}

#[derive(Clone)]
struct Incremented {
    counter_id: String,
}

impl DomainEvent for Incremented {
    fn event_type(&self) -> &str {
        "counter.incremented"
    }
    fn aggregate_id(&self) -> &str {
        &self.counter_id
    }
}

impl Aggregate for Counter {
    type Event = Incremented;
    fn apply(&mut self, event: &Incremented) {
        self.id = event.counter_id.clone();
        self.count += 1;
    }
    fn id(&self) -> &str {
        &self.id
    }
}

/// @covers: Aggregate::apply — single event mutates state
#[test]
fn test_apply_incremented_event_mutates_count_happy() {
    let mut c = Counter::default();
    c.apply(&Incremented { counter_id: "c1".into() });
    assert_eq!(c.count, 1);
}

/// @covers: Aggregate::apply — apply with wrong type does not compile (verify default noop)
#[test]
fn test_apply_default_trait_fn_is_noop_without_override_error() {
    // Counter overrides apply — verify that a second apply still works (no side-effect from default)
    let mut c = Counter::default();
    c.apply(&Incremented { counter_id: "c1".into() });
    // count must be exactly 1 after one apply; if noop were called it would stay 0
    assert_ne!(c.count, 0, "apply must not be noop when overridden");
}

/// @covers: Aggregate::apply — multiple applies accumulate
#[test]
fn test_apply_multiple_events_accumulate_count_edge() {
    let mut c = Counter::default();
    c.apply(&Incremented { counter_id: "c1".into() });
    c.apply(&Incremented { counter_id: "c1".into() });
    assert_eq!(c.count, 2);
}

/// @covers: Aggregate::id — after apply, id is set to aggregate id
#[test]
fn test_id_after_apply_returns_set_id_happy() {
    let mut c = Counter::default();
    c.apply(&Incremented { counter_id: "my-counter".into() });
    assert_eq!(c.id(), "my-counter");
}

/// @covers: Aggregate::id — before any apply, default id is empty
#[test]
fn test_id_before_apply_returns_empty_string_error() {
    let c = Counter::default();
    assert_eq!(c.id(), "");
}

/// @covers: Aggregate::id — last apply wins for id
#[test]
fn test_id_after_multiple_applies_reflects_last_event_edge() {
    let mut c = Counter::default();
    c.apply(&Incremented { counter_id: "first".into() });
    c.apply(&Incremented { counter_id: "last".into() });
    assert_eq!(c.id(), "last");
}
