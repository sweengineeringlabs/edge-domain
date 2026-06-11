//! Coverage for api/event/types/stage/stage_started.rs
use edge_domain::{DomainEvent, StageStarted};

#[test]
fn test_stage_started_new_sets_stage_field() {
    let e = StageStarted::new("transform", "handler-3");
    assert_eq!(e.stage(), "transform");
}

#[test]
fn test_stage_started_new_sets_handler_id_field() {
    let e = StageStarted::new("transform", "handler-3");
    assert_eq!(e.handler_id(), "handler-3");
}

#[test]
fn test_stage_started_event_type_is_stage_started() {
    let e = StageStarted::new("s", "h");
    assert_eq!(e.event_type(), "stage.started");
}
