//! `ExecutionModel` impl for `EchoExecutionModel`.

use async_trait::async_trait;

use crate::api::ExecutionError;
use crate::api::ExecutionModel;
use crate::api::{
    EchoExecutionModel, ExecutionConfig, ExecutionMode, ExecutionStepResult, TokenUsage,
};

#[async_trait]
impl ExecutionModel for EchoExecutionModel {
    async fn execute_step(
        &self,
        _agent_id: &str,
        goal: &str,
        _context: &str,
        available_tools: Vec<String>,
    ) -> Result<ExecutionStepResult, ExecutionError> {
        self.can_execute()?;
        let action = available_tools.into_iter().next();
        Ok(ExecutionStepResult::new(
            format!("reasoning toward: {goal}"),
            action,
            0.9,
            TokenUsage::new(0, 0, 0, 0),
        ))
    }

    fn can_execute(&self) -> Result<(), ExecutionError> {
        if self.config.max_tokens_per_call == 0 {
            return Err(ExecutionError::InvalidRequest(
                "max_tokens_per_call must be greater than zero".to_string(),
            ));
        }
        Ok(())
    }

    fn config(&self) -> ExecutionConfig {
        self.config.clone()
    }

    fn execution_mode(&self) -> ExecutionMode {
        self.config.execution_mode
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;

    fn model(max_tokens: u32) -> EchoExecutionModel {
        EchoExecutionModel::new(ExecutionConfig::new(
            max_tokens,
            30_000,
            true,
            false,
            ExecutionMode::Async,
        ))
    }

    #[test]
    fn test_execute_step_returns_result_with_first_tool() {
        let result =
            block_on(model(4096).execute_step("a1", "ship it", "ctx", vec!["search".to_string()]))
                .expect("step should succeed");
        assert_eq!(result.action, Some("search".to_string()));
    }

    #[test]
    fn test_can_execute_errors_on_zero_budget() {
        assert!(model(0).can_execute().is_err());
    }

    #[test]
    fn test_execution_mode_reflects_config() {
        assert_eq!(model(4096).execution_mode(), ExecutionMode::Async);
    }
}
