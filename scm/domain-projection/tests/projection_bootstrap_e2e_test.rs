//! SAF tests — `ProjectionBootstrap` trait.
// @allow: no_mocks_in_integration
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_event::{EventAggregateIdRequest, EventAggregateIdResponse, EventError};
use edge_domain_projection::{
    BootstrapNameRequest, DomainEvent, Projection, ProjectionApplyRequest, ProjectionBootstrap,
    ProjectionError, ProjectionReadModelRequest, StdProjectionFactory,
};

#[derive(Clone)]
struct ScoreEvt {
    points: u32,
}

impl DomainEvent for ScoreEvt {
    fn aggregate_id(&self, _req: EventAggregateIdRequest) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse { aggregate_id: "game-1" })
    }
}

struct Factories;
impl ProjectionBootstrap for Factories {}

fn score_evt(points: u32) -> ScoreEvt {
    ScoreEvt { points }
}

fn read(p: &impl Projection<Event = ScoreEvt, ReadModel = u32>) -> u32 {
    *p.read_model(ProjectionReadModelRequest).expect("read_model should succeed").read_model
}

/// @covers: bootstrap_name
#[test]
fn test_bootstrap_name_returns_nonempty_string_happy() {
    let f = StdProjectionFactory;
    let response = f.bootstrap_name(BootstrapNameRequest).expect("bootstrap_name should succeed");
    assert!(!response.name.is_empty(), "bootstrap_name must return a non-empty identifier");
}

/// @covers: bootstrap_name
#[test]
fn test_bootstrap_name_is_idempotent_error() {
    let f = StdProjectionFactory;
    let name1 = f.bootstrap_name(BootstrapNameRequest).expect("bootstrap_name should succeed").name;
    let name2 = f.bootstrap_name(BootstrapNameRequest).expect("bootstrap_name should succeed").name;
    assert_eq!(name1, name2, "bootstrap_name must return the same value on repeated calls");
    assert_eq!(name1, "projection", "bootstrap_name must return expected identifier");
}

/// @covers: bootstrap_name
#[test]
fn test_bootstrap_name_is_callable_via_trait_object_edge() {
    let f: &dyn ProjectionBootstrap = &StdProjectionFactory;
    let _ = f.bootstrap_name(BootstrapNameRequest).expect("bootstrap_name should succeed");
}

/// @covers: in_memory
#[test]
fn test_in_memory_creates_projection_with_initial_seed_happy() {
    let p = Factories::in_memory(10u32, |total: &mut u32, e: &ScoreEvt| *total += e.points);
    assert_eq!(read(&p), 10);
}

/// @covers: in_memory
#[test]
fn test_in_memory_zero_initial_creates_zero_read_model_error() {
    let p = Factories::in_memory(0u32, |total: &mut u32, e: &ScoreEvt| *total += e.points);
    assert_eq!(read(&p), 0);
}

/// @covers: in_memory
#[test]
fn test_in_memory_multiple_instances_are_independent_edge() {
    let mut p1 = Factories::in_memory(0u32, |n: &mut u32, e: &ScoreEvt| *n += e.points);
    let p2 = Factories::in_memory(99u32, |n: &mut u32, _e: &ScoreEvt| *n += 1);
    p1.apply(ProjectionApplyRequest { event: &score_evt(5) }).expect("apply should succeed");
    assert_eq!(read(&p1), 5);
    assert_eq!(read(&p2), 99);
}

/// @covers: try_drain
#[test]
fn test_try_drain_events_slice_returns_count_happy() {
    let mut p = Factories::in_memory(0u32, |n: &mut u32, e: &ScoreEvt| *n += e.points);
    let events = vec![score_evt(3), score_evt(4)];
    let response = match Factories::try_drain(&mut p, &events) {
        Ok(r) => r,
        Err(e) => panic!("expected Ok, got: {e}"),
    };
    assert_eq!(response.count, 2);
    assert_eq!(read(&p), 7);
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
    let response = match Factories::try_drain(&mut p, &[score_evt(1)]) {
        Ok(r) => r,
        Err(e) => panic!("expected Ok, got: {e}"),
    };
    assert_eq!(response.count, 1);
    assert_eq!(read(&p), 1);
}
