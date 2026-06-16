use crate::api::types::{ExecutionStepResult, ExecutionError, ExecutionConfig};
use async_trait::async_trait;

/// Pluggable LLM execution backend for reasoning steps
#[async_trait]
pub trait ExecutionModel: Send + Sync {
    /// Execute a single reasoning step
    async fn execute_step(
        &self,
        agent_id: &str,
        goal: &str,
        context: &str,
        available_tools: Vec<String>,
    ) -> Result<ExecutionStepResult, ExecutionError>;

    /// Check if execution can proceed (budget, throttling, rate limits)
    fn can_execute(&self) -> Result<(), ExecutionError>;

    /// Get execution configuration (timeouts, costs, etc.)
    fn config(&self) -> ExecutionConfig;
}
