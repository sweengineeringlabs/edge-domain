//! SAF tests — `ProjectionBootstrap` trait.
// @allow: no_mocks_in_integration

use edge_domain_projection::{DomainEvent, Projection, ProjectionError, ProjectionBootstrap};

#[derive(Clone)]
struct ScoreEvt {
    points: u32,
}

impl DomainEvent for ScoreEvt {
    fn aggregate_id(&self) -> &str {
        "game-1"
    }
}

struct Factories;
impl ProjectionBootstrap for Factories {}

fn score_evt(points: u32) -> ScoreEvt {
    ScoreEvt { points }
}

/// @covers: in_memory
#[test]
fn test_in_memory_creates_projection_with_initial_seed_happy() {
    let p = Factories::in_memory(10u32, |total: &mut u32, e: &ScoreEvt| *total += e.points);
    assert_eq!(*p.read_model(), 10);
}

/// @covers: in_memory
#[test]
fn test_in_memory_zero_initial_creates_zero_read_model_error() {
    let p = Factories::in_memory(0u32, |total: &mut u32, e: &ScoreEvt| *total += e.points);
    assert_eq!(*p.read_model(), 0);
}

/// @covers: in_memory
#[test]
fn test_in_memory_multiple_instances_are_independent_edge() {
    let mut p1 = Factories::in_memory(0u32, |n: &mut u32, e: &ScoreEvt| *n += e.points);
    let p2 = Factories::in_memory(99u32, |n: &mut u32, _e: &ScoreEvt| *n += 1);
    p1.apply(&score_evt(5));
    assert_eq!(*p1.read_model(), 5);
    assert_eq!(*p2.read_model(), 99);
}

/// @covers: try_drain
#[test]
fn test_try_drain_events_slice_returns_count_happy() {
    let mut p = Factories::in_memory(0u32, |n: &mut u32, e: &ScoreEvt| *n += e.points);
    let events = vec![score_evt(3), score_evt(4)];
    let count = match Factories::try_drain(&mut p, &events) {
        Ok(n) => n,
        Err(e) => panic!("expected Ok, got: {e}"),
    };
    assert_eq!(count, 2);
    assert_eq!(*p.read_model(), 7);
}

/// @covers: try_drain
#[test]
fn test_try_drain_empty_slice_returns_empty_stream_error() {
    let mut p = Factories::in_memory(0u32, |n: &mut u32, e: &ScoreEvt| *n += e.points);
    let err = Factories::try_drain(&mut p, &[]).unwrap_err();
    assert_eq!(err, ProjectionError::EmptyStream);
}

/// @covers: try_drain
#[test]
fn test_try_drain_single_event_returns_one_edge() {
    let mut p = Factories::in_memory(0u32, |n: &mut u32, e: &ScoreEvt| *n += e.points);
    let count = match Factories::try_drain(&mut p, &[score_evt(1)]) {
        Ok(n) => n,
        Err(e) => panic!("expected Ok, got: {e}"),
    };
    assert_eq!(count, 1);
    assert_eq!(*p.read_model(), 1);
}
