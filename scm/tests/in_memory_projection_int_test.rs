//! Integration tests for the in-memory projection and its SAF factory.
//!
//! The read model used here ([`Tally`]) deliberately surfaces a failure mode
//! (`u64` overflow) by recording it in the model rather than panicking — the
//! pattern the [`Projection`](edge_domain::Projection) contract documents for
//! fallible folds.  This lets the `_error`-scenario tests exercise genuine
//! error-surfacing behaviour rather than a contrived assertion.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::SystemTime;

use edge_domain::{Domain, DomainEvent, EventEnvelope, Projection};

#[derive(Clone)]
struct AmountReceived {
    id: String,
    amount: u64,
}

impl DomainEvent for AmountReceived {
    fn event_type(&self) -> &str {
        "ledger.amount_received"
    }
    fn aggregate_id(&self) -> &str {
        &self.id
    }
    fn occurred_at(&self) -> SystemTime {
        SystemTime::now()
    }
}

/// A read model that accumulates a running total and flags arithmetic overflow
/// instead of panicking.
#[derive(Default)]
struct Tally {
    total: u64,
    saturated: bool,
}

fn envelope(seq: u64, amount: u64) -> EventEnvelope<AmountReceived> {
    EventEnvelope {
        aggregate_id: "ledger-1".to_string(),
        sequence: seq,
        occurred_at: SystemTime::now(),
        event: AmountReceived {
            id: "ledger-1".to_string(),
            amount,
        },
    }
}

/// Build a tally projection seeded at `initial`.
///
/// The reducer adds each event's amount, recording saturation rather than
/// overflowing.
fn tally_projection(
    initial: u64,
) -> Box<dyn Projection<Event = AmountReceived, ReadModel = Tally>> {
    Domain::new_in_memory_projection::<AmountReceived, Tally, _>(
        Tally {
            total: initial,
            saturated: false,
        },
        |tally, env| match tally.total.checked_add(env.event.amount) {
            Some(sum) => tally.total = sum,
            None => tally.saturated = true,
        },
    )
}

/// @covers: new_in_memory_projection
#[test]
fn test_new_in_memory_projection_folds_event_totals_happy() {
    let mut p = tally_projection(0);
    p.apply(&envelope(1, 100));
    p.apply(&envelope(2, 250));
    assert_eq!(p.read_model().total, 350);
    assert!(!p.read_model().saturated);
}

/// @covers: new_in_memory_projection
#[test]
fn test_new_in_memory_projection_seeds_initial_value_edge() {
    let p = tally_projection(7);
    assert_eq!(p.read_model().total, 7);
    assert!(!p.read_model().saturated);
}

/// @covers: new_in_memory_projection
#[test]
fn test_new_in_memory_projection_overflowing_event_records_saturation_error() {
    let mut p = tally_projection(u64::MAX);
    p.apply(&envelope(1, 1));
    assert!(
        p.read_model().saturated,
        "overflow must be surfaced in the read model, not panic"
    );
    assert_eq!(p.read_model().total, u64::MAX);
}

/// @covers: Projection::apply
#[test]
fn test_apply_accumulates_across_multiple_events_happy() {
    let mut p = tally_projection(0);
    for seq in 1..=4 {
        p.apply(&envelope(seq, 10));
    }
    assert_eq!(p.read_model().total, 40);
}

/// @covers: Projection::apply
#[test]
fn test_apply_no_events_leaves_initial_state_edge() {
    let p = tally_projection(5);
    assert_eq!(p.read_model().total, 5);
}

/// @covers: Projection::apply
#[test]
fn test_apply_overflowing_event_sets_saturated_error() {
    let mut p = tally_projection(u64::MAX - 1);
    p.apply(&envelope(1, 5));
    assert!(p.read_model().saturated);
}

/// @covers: Projection::read_model
#[test]
fn test_read_model_returns_accumulated_total_happy() {
    let mut p = tally_projection(0);
    p.apply(&envelope(1, 42));
    assert_eq!(p.read_model().total, 42);
}

/// @covers: Projection::read_model
#[test]
fn test_read_model_returns_initial_before_any_apply_edge() {
    let p = tally_projection(9);
    assert_eq!(p.read_model().total, 9);
    assert!(!p.read_model().saturated);
}

/// @covers: Projection::read_model
#[test]
fn test_read_model_exposes_saturated_flag_after_overflow_error() {
    let mut p = tally_projection(u64::MAX);
    p.apply(&envelope(1, 100));
    assert!(p.read_model().saturated);
}
