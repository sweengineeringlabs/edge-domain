use edge_domain::StageCompletedBuilder;
use futures::executor::block_on;

#[test]
fn test_stage_completed_builder_build_with_all_fields_happy() {
    block_on(async {
        let event = StageCompletedBuilder::new()
            .stage("validation")
            .handler_id("handler-1")
            .duration_ms(42)
            .build();
        assert_eq!(event.stage(), "validation");
        assert_eq!(event.handler_id(), "handler-1");
        assert_eq!(event.duration_ms(), 42);
    });
}

#[test]
fn test_stage_completed_builder_build_with_empty_stage_edge() {
    block_on(async {
        let event = StageCompletedBuilder::new()
            .stage("")
            .handler_id("h")
            .duration_ms(0)
            .build();
        assert_eq!(event.stage(), "");
    });
}

#[test]
fn test_stage_completed_builder_build_with_zero_duration_edge() {
    block_on(async {
        let event = StageCompletedBuilder::new()
            .stage("s")
            .handler_id("h")
            .duration_ms(0)
            .build();
        assert_eq!(event.duration_ms(), 0);
    });
}

#[test]
fn test_stage_completed_builder_build_with_max_duration_edge() {
    block_on(async {
        let event = StageCompletedBuilder::new()
            .stage("s")
            .handler_id("h")
            .duration_ms(u64::MAX)
            .build();
        assert_eq!(event.duration_ms(), u64::MAX);
    });
}

#[test]
fn test_stage_completed_builder_new_returns_default_happy() {
    block_on(async {
        let b = StageCompletedBuilder::new();
        let event = b.build();
        assert_eq!(event.stage(), "");
        assert_eq!(event.handler_id(), "");
        assert_eq!(event.duration_ms(), 0);
    });
}

#[test]
fn test_stage_completed_builder_default_matches_new_edge() {
    block_on(async {
        let a = StageCompletedBuilder::default().build();
        let b = StageCompletedBuilder::new().build();
        assert_eq!(a.stage(), b.stage());
        assert_eq!(a.handler_id(), b.handler_id());
        assert_eq!(a.duration_ms(), b.duration_ms());
    });
}

#[test]
fn test_stage_completed_builder_chaining_preserves_all_fields_happy() {
    block_on(async {
        let event = StageCompletedBuilder::new()
            .handler_id("h-99")
            .duration_ms(999)
            .stage("processing")
            .build();
        assert_eq!(event.stage(), "processing");
        assert_eq!(event.handler_id(), "h-99");
        assert_eq!(event.duration_ms(), 999);
    });
}

#[test]
fn test_stage_completed_builder_overwrite_stage_error() {
    block_on(async {
        let event = StageCompletedBuilder::new()
            .stage("first")
            .stage("second")
            .handler_id("h")
            .duration_ms(1)
            .build();
        assert_eq!(event.stage(), "second");
    });
}
