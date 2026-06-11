//! Coverage for api/event/types/stage/stage_completed.rs
use edge_domain::{DomainEvent, StageCompleted};

#[test]
fn test_stage_completed_new_sets_stage_field() {
    let e = StageCompleted::new("ingest", "handler-1", 42);
    assert_eq!(e.stage(), "ingest");
}

#[test]
fn test_stage_completed_new_sets_handler_id_field() {
    let e = StageCompleted::new("ingest", "handler-1", 42);
    assert_eq!(e.handler_id(), "handler-1");
}

#[test]
fn test_stage_completed_new_sets_duration_ms_field() {
    let e = StageCompleted::new("ingest", "handler-1", 123);
    assert_eq!(e.duration_ms(), 123);
}

#[test]
fn test_stage_completed_event_type_is_stage_completed() {
    let e = StageCompleted::new("s", "h", 0);
    assert_eq!(e.event_type(), "stage.completed");
}

#[test]
fn test_stage_completed_aggregate_id_equals_stage() {
    let e = StageCompleted::new("my-stage", "h", 0);
    assert_eq!(e.aggregate_id(), "my-stage");
}
