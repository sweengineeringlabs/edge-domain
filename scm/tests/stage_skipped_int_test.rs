//! Coverage for api/event/types/stage/stage_skipped.rs
use edge_domain::{DomainEvent, StageSkipped};

#[test]
fn test_stage_skipped_new_sets_stage_field() {
    let e = StageSkipped::new("enrich", "handler-2");
    assert_eq!(e.stage(), "enrich");
}

#[test]
fn test_stage_skipped_new_sets_handler_id_field() {
    let e = StageSkipped::new("enrich", "handler-2");
    assert_eq!(e.handler_id(), "handler-2");
}

#[test]
fn test_stage_skipped_event_type_is_stage_skipped() {
    let e = StageSkipped::new("s", "h");
    assert_eq!(e.event_type(), "stage.skipped");
}
