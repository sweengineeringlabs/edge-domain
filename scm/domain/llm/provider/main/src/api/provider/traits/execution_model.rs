//! `ExecutionModel` — single-step reasoning-execution contract.

use async_trait::async_trait;

use crate::api::provider::errors::ExecutionError;
use crate::api::provider::types::{
    ExecutionConfigLookupRequest, ExecutionConfigResponse, ExecutionModeLookupRequest,
    ExecutionModeResponse, ExecutionReadinessRequest, StepExecutionRequest, StepExecutionResponse,
};

/// Pluggable LLM execution backend for individual reasoning steps.
#[async_trait]
pub trait ExecutionModel: Send + Sync {
    /// Execute a single reasoning step toward the request's goal.
    ///
    /// Returns [`ExecutionError`] when the backend rejects or fails the step.
    async fn execute_step(
        &self,
        req: StepExecutionRequest<'_>,
    ) -> Result<StepExecutionResponse, ExecutionError>;

    /// Check whether execution can proceed (budget, throttling, rate limits).
    fn can_execute(&self, req: ExecutionReadinessRequest) -> Result<(), ExecutionError>;

    /// Execution configuration (timeouts, token caps, mode).
    fn config(
        &self,
        req: ExecutionConfigLookupRequest,
    ) -> Result<ExecutionConfigResponse, ExecutionError>;

    /// Execution mode this model operates in.
    fn execution_mode(
        &self,
        req: ExecutionModeLookupRequest,
    ) -> Result<ExecutionModeResponse, ExecutionError>;
}
