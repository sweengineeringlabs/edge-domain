//! `ExecutionModel` — single-step reasoning-execution contract.

use async_trait::async_trait;

use crate::api::provider::errors::ExecutionError;
use crate::api::provider::types::{ExecutionConfig, ExecutionMode, ExecutionStepResult};

/// Pluggable LLM execution backend for individual reasoning steps.
#[async_trait]
pub trait ExecutionModel: Send + Sync {
    /// Execute a single reasoning step toward `goal` for `agent_id`.
    ///
    /// Returns [`ExecutionError`] when the backend rejects or fails the step.
    async fn execute_step(
        &self,
        agent_id: &str,
        goal: &str,
        context: &str,
        available_tools: Vec<String>,
    ) -> Result<ExecutionStepResult, ExecutionError>;

    /// Check whether execution can proceed (budget, throttling, rate limits).
    fn can_execute(&self) -> Result<(), ExecutionError>;

    /// Execution configuration (timeouts, token caps, mode).
    fn config(&self) -> ExecutionConfig;

    /// Execution mode this model operates in.
    fn execution_mode(&self) -> ExecutionMode;
}
