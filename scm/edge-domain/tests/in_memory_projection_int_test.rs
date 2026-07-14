//! Integration tests for the in-memory projection and its SAF factory.
//!
//! The read model used here ([`Tally`]) deliberately surfaces a failure mode
//! (`u64` overflow) by recording it in the model rather than panicking — the
//! pattern the [`Projection`](edge_application::Projection) contract documents for
//! fallible folds.  This lets the `_error`-scenario tests exercise genuine
//! error-surfacing behaviour rather than a contrived assertion.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::SystemTime;

use edge_application::{
    Domain, DomainEvent, EventAggregateIdRequest, EventAggregateIdResponse, EventError,
    EventOccurredAtRequest, EventOccurredAtResponse, EventTypeRequest, EventTypeResponse,
    Projection, ProjectionApplyRequest, ProjectionReadModelRequest,
};

#[derive(Clone)]
struct AmountReceived {
    id: String,
    amount: u64,
}

impl DomainEvent for AmountReceived {
    fn event_type(&self, _req: EventTypeRequest) -> Result<EventTypeResponse<'_>, EventError> {
        Ok(EventTypeResponse {
            event_type: "ledger.amount_received",
        })
    }
    fn aggregate_id(
        &self,
        _req: EventAggregateIdRequest,
    ) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse {
            aggregate_id: &self.id,
        })
    }
    fn occurred_at(
        &self,
        _req: EventOccurredAtRequest,
    ) -> Result<EventOccurredAtResponse, EventError> {
        Ok(EventOccurredAtResponse {
            occurred_at: SystemTime::now(),
        })
    }
}

/// A read model that accumulates a running total and flags arithmetic overflow
/// instead of panicking.
#[derive(Default)]
struct Tally {
    total: u64,
    saturated: bool,
}

fn event(amount: u64) -> AmountReceived {
    AmountReceived {
        id: "ledger-1".to_string(),
        amount,
    }
}

/// Build a tally projection seeded at `initial`.
///
/// The reducer adds each event's amount, recording saturation rather than
/// overflowing.
fn tally_projection(
    initial: u64,
) -> Box<dyn Projection<Event = AmountReceived, ReadModel = Tally>> {
    Domain.new_in_memory_projection::<AmountReceived, Tally, _>(
        Tally {
            total: initial,
            saturated: false,
        },
        |tally, ev| match tally.total.checked_add(ev.amount) {
            Some(sum) => tally.total = sum,
            None => tally.saturated = true,
        },
    )
}

fn apply(p: &mut dyn Projection<Event = AmountReceived, ReadModel = Tally>, amount: u64) {
    p.apply(ProjectionApplyRequest {
        event: &event(amount),
    })
    .expect("apply should succeed");
}

fn total(p: &dyn Projection<Event = AmountReceived, ReadModel = Tally>) -> u64 {
    p.read_model(ProjectionReadModelRequest)
        .expect("read_model should succeed")
        .read_model
        .total
}

fn saturated(p: &dyn Projection<Event = AmountReceived, ReadModel = Tally>) -> bool {
    p.read_model(ProjectionReadModelRequest)
        .expect("read_model should succeed")
        .read_model
        .saturated
}

/// @covers: new_in_memory_projection
#[test]
fn test_new_in_memory_projection_folds_event_totals_happy() {
    let mut p = tally_projection(0);
    apply(&mut *p, 100);
    apply(&mut *p, 250);
    assert_eq!(total(&*p), 350);
    assert!(!saturated(&*p));
}

/// @covers: new_in_memory_projection
#[test]
fn test_new_in_memory_projection_seeds_initial_value_edge() {
    let p = tally_projection(7);
    assert_eq!(total(&*p), 7);
    assert!(!saturated(&*p));
}

/// @covers: new_in_memory_projection
#[test]
fn test_new_in_memory_projection_overflowing_event_records_saturation_error() {
    let mut p = tally_projection(u64::MAX);
    apply(&mut *p, 1);
    assert!(
        saturated(&*p),
        "overflow must be surfaced in the read model, not panic"
    );
    assert_eq!(total(&*p), u64::MAX);
}

/// @covers: Projection::apply
#[test]
fn test_apply_accumulates_across_multiple_events_happy() {
    let mut p = tally_projection(0);
    for _ in 1..=4 {
        apply(&mut *p, 10);
    }
    assert_eq!(total(&*p), 40);
}

/// @covers: Projection::apply
#[test]
fn test_apply_no_events_leaves_initial_state_edge() {
    let p = tally_projection(5);
    assert_eq!(total(&*p), 5);
}

/// @covers: Projection::apply
#[test]
fn test_apply_overflowing_event_sets_saturated_error() {
    let mut p = tally_projection(u64::MAX - 1);
    apply(&mut *p, 5);
    assert!(saturated(&*p));
}

/// @covers: Projection::read_model
#[test]
fn test_read_model_returns_accumulated_total_happy() {
    let mut p = tally_projection(0);
    apply(&mut *p, 42);
    assert_eq!(total(&*p), 42);
}

/// @covers: Projection::read_model
#[test]
fn test_read_model_returns_initial_before_any_apply_edge() {
    let p = tally_projection(9);
    assert_eq!(total(&*p), 9);
    assert!(!saturated(&*p));
}

/// @covers: Projection::read_model
#[test]
fn test_read_model_exposes_saturated_flag_after_overflow_error() {
    let mut p = tally_projection(u64::MAX);
    apply(&mut *p, 100);
    assert!(saturated(&*p));
}
