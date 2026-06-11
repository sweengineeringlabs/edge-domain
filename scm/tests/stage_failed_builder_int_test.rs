//! Coverage for api/event/types/stage/stage_failed/stage/stage_failed_builder.rs
use edge_domain::{DomainEvent, StageFailedBuilder};

#[test]
fn test_stage_failed_builder_sets_stage_field() {
    let e = StageFailedBuilder::new()
        .stage("validate")
        .handler_id("h-1")
        .duration_ms(10)
        .error("bad input")
        .build();
    assert_eq!(e.stage(), "validate");
}

#[test]
fn test_stage_failed_builder_sets_error_field() {
    let e = StageFailedBuilder::new()
        .stage("s")
        .error("something went wrong")
        .build();
    assert_eq!(e.error(), "something went wrong");
}

#[test]
fn test_stage_failed_builder_default_produces_valid_event() {
    let e = StageFailedBuilder::new().build();
    assert_eq!(e.event_type(), "stage.failed");
}
