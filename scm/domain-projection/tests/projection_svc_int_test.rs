//! SAF tests — `Projection` trait via `InMemoryProjection`.
// @allow: no_mocks_in_integration

use edge_domain_projection::{DomainEvent, InMemoryProjection, Projection, ProjectionFactory};

#[derive(Clone)]
struct BalanceEvt {
    delta: i64,
}

impl DomainEvent for BalanceEvt {
    fn aggregate_id(&self) -> &str {
        "account-1"
    }
}

struct Factories;
impl ProjectionFactory for Factories {}

fn make_balance(seed: i64) -> impl Projection<Event = BalanceEvt, ReadModel = i64> {
    Factories::in_memory(seed, |total: &mut i64, e: &BalanceEvt| *total += e.delta)
}

fn evt(delta: i64) -> BalanceEvt {
    BalanceEvt { delta }
}

/// @covers: apply
#[test]
fn test_apply_single_event_increments_read_model_happy() {
    let mut p = make_balance(0);
    p.apply(&evt(10));
    assert_eq!(*p.read_model(), 10);
}

/// @covers: apply
#[test]
fn test_apply_zero_delta_leaves_read_model_unchanged_error() {
    let mut p = make_balance(7);
    p.apply(&evt(0));
    assert_eq!(*p.read_model(), 7);
}

/// @covers: apply
#[test]
fn test_apply_many_events_accumulate_all_deltas_edge() {
    let mut p = make_balance(0);
    for i in 1..=10i64 {
        p.apply(&evt(i));
    }
    assert_eq!(*p.read_model(), 55);
}

/// @covers: read_model
#[test]
fn test_read_model_initial_value_matches_seed_happy() {
    let p = make_balance(42);
    assert_eq!(*p.read_model(), 42);
}

/// @covers: read_model
#[test]
fn test_read_model_before_events_is_seed_value_error() {
    let p = make_balance(0);
    assert_eq!(*p.read_model(), 0);
}

/// @covers: read_model
#[test]
fn test_read_model_reflects_all_applied_events_edge() {
    let mut p = make_balance(100);
    p.apply(&evt(-50));
    p.apply(&evt(20));
    assert_eq!(*p.read_model(), 70);
}

fn make_count(seed: u32) -> InMemoryProjection<BalanceEvt, u32, impl Fn(&mut u32, &BalanceEvt) + Send + Sync> {
    Factories::in_memory(seed, |count: &mut u32, _e: &BalanceEvt| *count += 1)
}

/// @covers: apply
#[test]
fn test_apply_counts_each_event_regardless_of_payload_edge() {
    let mut p = make_count(0);
    p.apply(&evt(0));
    p.apply(&evt(999));
    assert_eq!(*p.read_model(), 2);
}
