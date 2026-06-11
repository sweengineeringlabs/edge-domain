//! Coverage for api/event/types/stage/stage_failed/stage/stage_failed.rs
use edge_domain::{DomainEvent, StageFailed};

#[test]
fn test_stage_failed_new_sets_stage_field() {
    let e = StageFailed::new("ingest", "handler-1", 99, "timeout");
    assert_eq!(e.stage(), "ingest");
}

#[test]
fn test_stage_failed_new_sets_error_field() {
    let e = StageFailed::new("s", "h", 0, "bad thing happened");
    assert_eq!(e.error(), "bad thing happened");
}

#[test]
fn test_stage_failed_event_type_is_stage_failed() {
    let e = StageFailed::new("s", "h", 0, "err");
    assert_eq!(e.event_type(), "stage.failed");
}

#[test]
fn test_stage_failed_duration_ms_is_set() {
    let e = StageFailed::new("s", "h", 50, "err");
    assert_eq!(e.duration_ms(), 50);
}
