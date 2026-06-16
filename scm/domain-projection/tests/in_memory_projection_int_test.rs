//! Integration tests for `InMemoryProjection`.
// @allow: no_mocks_in_integration

use edge_domain_projection::{DomainEvent, InMemoryProjection, Projection};

#[derive(Clone)]
struct ItemEvt {
    count: usize,
}

impl DomainEvent for ItemEvt {
    fn aggregate_id(&self) -> &str {
        "cart"
    }
}

fn make(seed: usize) -> InMemoryProjection<ItemEvt, usize, impl Fn(&mut usize, &ItemEvt) + Send + Sync> {
    InMemoryProjection::new(seed, |total: &mut usize, e: &ItemEvt| *total += e.count)
}

#[test]
fn test_new_projection_read_model_matches_initial_happy() {
    let p = make(5);
    assert_eq!(*p.read_model(), 5);
}

#[test]
fn test_new_projection_with_zero_initial_is_zero_error() {
    let p = make(0);
    assert_eq!(*p.read_model(), 0);
}

#[test]
fn test_projection_reducers_are_independent_edge() {
    let mut p1 = make(0);
    let p2 = InMemoryProjection::new(0usize, |_n: &mut usize, _e: &ItemEvt| {});
    p1.apply(&ItemEvt { count: 10 });
    assert_eq!(*p1.read_model(), 10);
    assert_eq!(*p2.read_model(), 0);
}

#[test]
fn test_apply_then_read_model_returns_updated_value_happy() {
    let mut p = make(0);
    p.apply(&ItemEvt { count: 3 });
    p.apply(&ItemEvt { count: 7 });
    assert_eq!(*p.read_model(), 10);
}
