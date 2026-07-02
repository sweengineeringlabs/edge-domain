//! `ExecutionModel` impl for `EchoExecutionModel`.

use async_trait::async_trait;

use crate::api::ExecutionError;
use crate::api::ExecutionModel;
use crate::api::{
    EchoExecutionModel, ExecutionConfig, ExecutionConfigLookupRequest, ExecutionConfigResponse,
    ExecutionModeLookupRequest, ExecutionModeResponse, ExecutionReadinessRequest,
    ExecutionStepResult, StepExecutionRequest, StepExecutionResponse, TokenUsage,
};

impl EchoExecutionModel {
    /// Construct an execution model bound to the given config.
    pub fn new(config: ExecutionConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl ExecutionModel for EchoExecutionModel {
    async fn execute_step(
        &self,
        req: StepExecutionRequest<'_>,
    ) -> Result<StepExecutionResponse, ExecutionError> {
        self.can_execute(ExecutionReadinessRequest)?;
        let action = req.available_tools.into_iter().next();
        Ok(StepExecutionResponse {
            result: Box::new(ExecutionStepResult::new(
                format!("reasoning toward: {}", req.goal),
                action,
                0.9,
                Some(TokenUsage::new(0, 0, 0, 0)),
            )),
        })
    }

    fn can_execute(&self, _req: ExecutionReadinessRequest) -> Result<(), ExecutionError> {
        if self.config.max_tokens_per_call == 0 {
            return Err(ExecutionError::InvalidRequest(
                "max_tokens_per_call must be greater than zero".to_string(),
            ));
        }
        Ok(())
    }

    fn config(
        &self,
        _req: ExecutionConfigLookupRequest,
    ) -> Result<ExecutionConfigResponse, ExecutionError> {
        Ok(ExecutionConfigResponse {
            config: Box::new(self.config.clone()),
        })
    }

    fn execution_mode(
        &self,
        _req: ExecutionModeLookupRequest,
    ) -> Result<ExecutionModeResponse, ExecutionError> {
        Ok(ExecutionModeResponse {
            mode: self.config.execution_mode,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::ExecutionMode;
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
    fn test_new_binds_given_config() {
        let model = model(2048);
        assert_eq!(model.config.max_tokens_per_call, 2048);
    }

    #[test]
    fn test_execute_step_returns_result_with_first_tool() {
        let result = block_on(model(4096).execute_step(StepExecutionRequest {
            agent_id: "a1",
            goal: "ship it",
            context: "ctx",
            available_tools: vec!["search".to_string()],
        }))
        .expect("step should succeed");
        assert_eq!(result.result.action, Some("search".to_string()));
    }

    #[test]
    fn test_can_execute_errors_on_zero_budget() {
        assert!(model(0).can_execute(ExecutionReadinessRequest).is_err());
    }

    #[test]
    fn test_execution_mode_reflects_config() {
        assert_eq!(
            model(4096)
                .execution_mode(ExecutionModeLookupRequest)
                .expect("execution_mode ok")
                .mode,
            ExecutionMode::Async
        );
    }
}
