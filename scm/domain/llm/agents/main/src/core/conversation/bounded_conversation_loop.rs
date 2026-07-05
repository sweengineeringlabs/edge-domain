//! `ConversationLoop` impl for [`BoundedConversationLoop`].

use std::sync::Arc;

use async_trait::async_trait;
use edge_pipeline::{
    ContextMutationRequest, PipelineBuilder, PipelineConfig, PipelineError, PipelineSvc, Step,
};

use crate::api::{
    AgentError, BoundedConversationLoop, ConversationLoop, ConversationRunRequest,
    ConversationRunResponse,
};
use crate::core::conversation::conversation_state::ConversationState;
use crate::core::conversation::conversation_turn_step::DefaultConversationTurnStep;

#[async_trait]
impl ConversationLoop for BoundedConversationLoop {
    async fn run(
        &self,
        req: ConversationRunRequest,
    ) -> Result<ConversationRunResponse, AgentError> {
        if req.max_turns == 0 {
            return Err(AgentError::InvalidState(
                "max_turns must be greater than zero".to_string(),
            ));
        }

        let step: Arc<dyn Step<Ctx = ConversationState, ExecutionError = AgentError>> =
            Arc::new(DefaultConversationTurnStep {
                agent: Arc::clone(&self.agent),
                handler_context: *req.handler_context,
            });
        let steps = vec![step; req.max_turns as usize];

        let pipeline = PipelineSvc::build(PipelineBuilder {
            steps,
            config: PipelineConfig::default(),
            event_bus: None,
        });

        let mut state = ConversationState::new(req.messages);
        pipeline
            .run(ContextMutationRequest { ctx: &mut state })
            .await
            .map_err(Self::map_pipeline_error)?;

        if !state.terminated {
            return Err(AgentError::TurnLimitExceeded {
                max_turns: req.max_turns,
            });
        }
        Ok(ConversationRunResponse {
            messages: state.messages,
            turns: state.turns_taken,
        })
    }
}

impl BoundedConversationLoop {
    fn map_pipeline_error(err: PipelineError<AgentError>) -> AgentError {
        match err {
            PipelineError::StepFailed(step_err) => step_err.cause,
            other => AgentError::ExecutionFailed(other.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_pipeline::StepError;

    /// @covers: map_pipeline_error
    #[test]
    fn test_map_pipeline_error_preserves_original_cause_when_step_failed_happy() {
        let err = PipelineError::StepFailed(StepError {
            step_name: "turn".to_string(),
            cause: AgentError::ExecutionFailed("boom".to_string()),
        });
        let mapped = BoundedConversationLoop::map_pipeline_error(err);
        assert!(matches!(mapped, AgentError::ExecutionFailed(msg) if msg == "boom"));
    }

    /// @covers: map_pipeline_error
    #[test]
    fn test_map_pipeline_error_wraps_non_step_failed_variants_edge() {
        let err: PipelineError<AgentError> = PipelineError::UnknownStep("mystery".to_string());
        let mapped = BoundedConversationLoop::map_pipeline_error(err);
        assert!(matches!(mapped, AgentError::ExecutionFailed(_)));
    }
}
