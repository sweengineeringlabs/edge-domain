//! SAF facade tests — `ExecutionModel` trait via `EchoExecutionModel`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{
    ExecutionConfig, ExecutionConfigLookupRequest, ExecutionMode, ExecutionModeLookupRequest,
    ExecutionModel, ExecutionReadinessRequest, ProviderBootstrap, StdProviderFactory,
    StepExecutionRequest,
};
use futures::executor::block_on;

fn model(max_tokens: u32, mode: ExecutionMode) -> impl ExecutionModel {
    StdProviderFactory::execution_model(ExecutionConfig::new(max_tokens, 30_000, true, false, mode))
}

// --- execute_step ---

/// @covers: ExecutionModel::execute_step — returns a step result on success
#[test]
fn test_execute_step_returns_reasoning_happy() {
    let m = model(4096, ExecutionMode::Async);
    let req = StepExecutionRequest {
        agent_id: "a1",
        goal: "ship",
        context: "ctx",
        available_tools: vec!["search".to_string()],
    };
    let resp = block_on(m.execute_step(req)).expect("step should succeed");
    assert!(resp.result.reasoning.contains("ship"));
}

/// @covers: ExecutionModel::execute_step — errors when budget is zero
#[test]
fn test_execute_step_errors_on_zero_budget_error() {
    let m = model(0, ExecutionMode::Async);
    let req = StepExecutionRequest {
        agent_id: "a1",
        goal: "ship",
        context: "ctx",
        available_tools: vec![],
    };
    assert!(block_on(m.execute_step(req)).is_err());
}

/// @covers: ExecutionModel::execute_step — no tools yields no action
#[test]
fn test_execute_step_no_tools_has_no_action_edge() {
    let m = model(4096, ExecutionMode::Async);
    let req = StepExecutionRequest {
        agent_id: "a1",
        goal: "ship",
        context: "ctx",
        available_tools: vec![],
    };
    let resp = block_on(m.execute_step(req)).expect("step should succeed");
    assert!(!resp.result.has_action());
}

// --- can_execute ---

/// @covers: ExecutionModel::can_execute — ok with a positive budget
#[test]
fn test_can_execute_ok_with_budget_happy() {
    assert!(matches!(
        model(4096, ExecutionMode::Async).can_execute(ExecutionReadinessRequest),
        Ok(())
    ));
}

/// @covers: ExecutionModel::can_execute — errors with zero budget
#[test]
fn test_can_execute_errors_zero_budget_error() {
    assert!(model(0, ExecutionMode::Async)
        .can_execute(ExecutionReadinessRequest)
        .is_err());
}

/// @covers: ExecutionModel::can_execute — minimal budget of one is allowed
#[test]
fn test_can_execute_minimal_budget_ok_edge() {
    assert!(matches!(
        model(1, ExecutionMode::Async).can_execute(ExecutionReadinessRequest),
        Ok(())
    ));
}

// --- config ---

/// @covers: ExecutionModel::config — reports the configured token cap
#[test]
fn test_config_reports_token_cap_happy() {
    assert_eq!(
        model(4096, ExecutionMode::Async)
            .config(ExecutionConfigLookupRequest)
            .expect("config should succeed")
            .config
            .max_tokens_per_call,
        4096
    );
}

/// @covers: ExecutionModel::config — caching flag carried through
#[test]
fn test_config_carries_cache_flag_error() {
    assert!(
        model(4096, ExecutionMode::Async)
            .config(ExecutionConfigLookupRequest)
            .expect("config should succeed")
            .config
            .cache_enabled
    );
}

/// @covers: ExecutionModel::config — timeout carried through
#[test]
fn test_config_carries_timeout_edge() {
    assert_eq!(
        model(4096, ExecutionMode::Async)
            .config(ExecutionConfigLookupRequest)
            .expect("config should succeed")
            .config
            .timeout_per_step,
        30_000
    );
}

// --- execution_mode ---

/// @covers: ExecutionModel::execution_mode — reports async
#[test]
fn test_execution_mode_reports_async_happy() {
    assert_eq!(
        model(4096, ExecutionMode::Async)
            .execution_mode(ExecutionModeLookupRequest)
            .expect("execution_mode should succeed")
            .mode,
        ExecutionMode::Async
    );
}

/// @covers: ExecutionModel::execution_mode — reports streaming distinctly
#[test]
fn test_execution_mode_reports_streaming_error() {
    assert_eq!(
        model(4096, ExecutionMode::Streaming)
            .execution_mode(ExecutionModeLookupRequest)
            .expect("execution_mode should succeed")
            .mode,
        ExecutionMode::Streaming
    );
}

/// @covers: ExecutionModel::execution_mode — long-running mode is preserved
#[test]
fn test_execution_mode_long_running_edge() {
    assert_eq!(
        model(4096, ExecutionMode::LongRunning)
            .execution_mode(ExecutionModeLookupRequest)
            .expect("execution_mode should succeed")
            .mode,
        ExecutionMode::LongRunning
    );
}
