//! Layer-level coverage for small Request/Response marker and wrapper types in api/types/
//! that don't warrant a dedicated per-type test file (see `sea_layer_test_coverage`).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_domain_pipeline::{
    BuilderValidationRequest, ConfigValidationRequest, ContextMutationRequest, EnablementRequest,
    EnablementResponse, PipelineAssemblyRequest, PipelineAssemblyResponse, PipelineBuilder,
    PipelineConfig, PipelineConfigLookupRequest, PipelineConfigResponse, PipelineDefinition,
    PipelineEmptinessRequest, PipelineEmptinessResponse, PipelineSvc, Step, StepCountRequest,
    StepCountResponse, StepFailureRequest, StepFailureResponse, StepNameRequest, StepNameResponse,
    StepRegistrationRequest, StepSvc,
};

// ── ContextMutationRequest ──────────────────────────────────────────────────

#[test]
fn test_context_mutation_request_happy_wraps_mutable_reference() {
    let mut ctx = 5;
    let req = ContextMutationRequest { ctx: &mut ctx };
    *req.ctx += 1;
    assert_eq!(ctx, 6);
}

#[test]
fn test_context_mutation_request_edge_mutation_visible_after_scope() {
    let mut ctx = String::from("a");
    {
        let req = ContextMutationRequest { ctx: &mut ctx };
        req.ctx.push('b');
    }
    assert_eq!(ctx, "ab");
}

// ── StepCountRequest / StepCountResponse ────────────────────────────────────

#[test]
fn test_step_count_request_happy_is_zero_sized_marker() {
    assert_eq!(std::mem::size_of::<StepCountRequest>(), 0);
}

#[test]
fn test_step_count_response_happy_carries_count() {
    let response = StepCountResponse { count: 4 };
    assert_eq!(response.count, 4);
}

#[test]
fn test_step_count_response_edge_zero_count() {
    let response = StepCountResponse { count: 0 };
    assert_eq!(response.count, 0);
}

// ── PipelineEmptinessRequest / PipelineEmptinessResponse ────────────────────

#[test]
fn test_pipeline_emptiness_request_happy_is_zero_sized_marker() {
    assert_eq!(std::mem::size_of::<PipelineEmptinessRequest>(), 0);
}

#[test]
fn test_pipeline_emptiness_response_happy_true() {
    let response = PipelineEmptinessResponse { empty: true };
    assert!(response.empty);
}

#[test]
fn test_pipeline_emptiness_response_edge_false() {
    let response = PipelineEmptinessResponse { empty: false };
    assert!(!response.empty);
}

// ── PipelineConfigLookupRequest / PipelineConfigResponse ────────────────────

#[test]
fn test_pipeline_config_lookup_request_happy_is_zero_sized_marker() {
    assert_eq!(std::mem::size_of::<PipelineConfigLookupRequest>(), 0);
}

#[test]
fn test_pipeline_config_response_happy_carries_config() {
    let config = PipelineConfig {
        timeout_per_step: Some(std::time::Duration::from_secs(3)),
        emit_lifecycle_events: true,
        abort_on_error: false,
    };
    let response = PipelineConfigResponse {
        config: config.clone(),
    };
    assert_eq!(response.config.timeout_per_step, config.timeout_per_step);
    assert_eq!(response.config.abort_on_error, config.abort_on_error);
}

// ── StepNameRequest / StepNameResponse ──────────────────────────────────────

#[test]
fn test_step_name_request_happy_is_zero_sized_marker() {
    assert_eq!(std::mem::size_of::<StepNameRequest>(), 0);
}

#[test]
fn test_step_name_response_happy_carries_name() {
    let response = StepNameResponse {
        name: "validate".to_string(),
    };
    assert_eq!(response.name, "validate");
}

#[test]
fn test_step_name_response_edge_empty_name() {
    let response = StepNameResponse {
        name: String::new(),
    };
    assert!(response.name.is_empty());
}

// ── StepFailureRequest / StepFailureResponse ────────────────────────────────

#[test]
fn test_step_failure_request_happy_carries_name_and_cause() {
    let req = StepFailureRequest {
        step_name: "enrich".to_string(),
        cause: "timed out".to_string(),
    };
    assert_eq!(req.step_name, "enrich");
    assert_eq!(req.cause, "timed out");
}

#[test]
fn test_step_failure_response_happy_wraps_step_error() {
    let response = StepFailureResponse {
        error: edge_domain_pipeline::StepError {
            step_name: "enrich".to_string(),
            cause: "boom".to_string(),
        },
    };
    assert_eq!(response.error.step_name, "enrich");
    assert_eq!(response.error.cause, "boom");
}

// ── EnablementRequest / EnablementResponse ──────────────────────────────────

#[test]
fn test_enablement_request_happy_is_zero_sized_marker() {
    assert_eq!(std::mem::size_of::<EnablementRequest>(), 0);
}

#[test]
fn test_enablement_response_happy_true() {
    let response = EnablementResponse { enabled: true };
    assert!(response.enabled);
}

#[test]
fn test_enablement_response_edge_false() {
    let response = EnablementResponse { enabled: false };
    assert!(!response.enabled);
}

// ── ConfigValidationRequest / BuilderValidationRequest ──────────────────────

#[test]
fn test_config_validation_request_happy_carries_config() {
    let config = PipelineConfig::default();
    let req = ConfigValidationRequest {
        config: config.clone(),
    };
    assert_eq!(req.config.abort_on_error, config.abort_on_error);
}

#[test]
fn test_builder_validation_request_happy_borrows_builder() {
    let builder: PipelineBuilder<i32, String> = PipelineBuilder::new();
    let req = BuilderValidationRequest { builder: &builder };
    assert!(req.builder.steps.is_empty());
    assert!(req.builder.config.abort_on_error);
}

// ── StepRegistrationRequest ──────────────────────────────────────────────────

#[test]
fn test_step_registration_request_happy_carries_name_and_step() {
    let step = StepSvc::noop_shared::<i32, String>();
    let req = StepRegistrationRequest {
        name: "noop".to_string(),
        step,
    };
    assert_eq!(req.name, "noop");
    assert_eq!(Arc::strong_count(&req.step), 1);
}

// ── PipelineAssemblyRequest / PipelineAssemblyResponse ──────────────────────

#[test]
fn test_pipeline_assembly_request_happy_carries_definition() {
    let definition = PipelineDefinition {
        config: PipelineConfig::default(),
        steps: vec!["a".to_string(), "b".to_string()],
    };
    let req = PipelineAssemblyRequest {
        definition: Box::new(definition),
    };
    assert_eq!(req.definition.steps, vec!["a".to_string(), "b".to_string()]);
}

#[test]
fn test_pipeline_assembly_request_edge_empty_steps() {
    let req = PipelineAssemblyRequest {
        definition: Box::new(PipelineDefinition::default()),
    };
    assert!(req.definition.steps.is_empty());
}

#[test]
fn test_pipeline_assembly_response_happy_carries_runnable_pipeline() {
    let pipeline = PipelineSvc::build(PipelineBuilder::<i32, String>::new());
    let response = PipelineAssemblyResponse { pipeline };
    assert!(
        response
            .pipeline
            .step_count(StepCountRequest)
            .expect("step_count must succeed")
            .count
            == 0
    );
}

// ── PipelineBuilder inherent constructor sanity (only exercised here, not elsewhere in api_int_test) ──

#[test]
fn test_pipeline_builder_happy_with_steps_tracks_count() {
    struct NoopStep;
    #[async_trait::async_trait]
    impl Step for NoopStep {
        type Ctx = i32;
        type ExecutionError = String;

        async fn execute(&self, _req: ContextMutationRequest<'_, i32>) -> Result<(), String> {
            Ok(())
        }
    }
    let builder = PipelineBuilder::<i32, String>::new()
        .with(NoopStep)
        .with(NoopStep);
    assert_eq!(builder.steps.len(), 2);
}
