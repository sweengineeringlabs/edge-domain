//! Tests for the `EchoExecutionModel` concrete implementation.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{EchoExecutionModel, ExecutionConfig, ExecutionMode, ExecutionModel};

fn build(max_tokens: u32) -> EchoExecutionModel {
    EchoExecutionModel::new(ExecutionConfig::new(
        max_tokens,
        30_000,
        true,
        false,
        ExecutionMode::Async,
    ))
}

/// @covers: EchoExecutionModel::new — reports the configured mode
#[test]
fn test_echo_execution_model_reports_mode() {
    assert_eq!(build(4096).execution_mode(), ExecutionMode::Async);
}

/// @covers: EchoExecutionModel — can execute with a budget
#[test]
fn test_echo_execution_model_can_execute_with_budget() {
    assert!(build(4096).can_execute().is_ok());
}

/// @covers: EchoExecutionModel — blocks execution without a budget
#[test]
fn test_echo_execution_model_blocks_without_budget() {
    assert!(build(0).can_execute().is_err());
}
