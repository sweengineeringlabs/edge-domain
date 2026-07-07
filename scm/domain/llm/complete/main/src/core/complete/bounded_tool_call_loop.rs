//! `ToolCallLoop` impl for [`BoundedToolCallLoop`].
//!
//! `run()` (the public trait method) is exercised externally in
//! `tests/tool_call_loop_e2e_test.rs`; only the private `map_parallel_error` helper
//! is tested inline here, per `core_impl_private_tests_inline`.

use std::sync::Arc;

use async_trait::async_trait;
use edge_pipeline::{
    ContextMutationRequest, ParallelBranchFailure, ParallelConfig, ParallelStepBuilder,
    ParallelStepError, ParallelStepSvc, Step,
};

use crate::api::{
    BoundedToolCallLoop, CompleteError, CompleteRequest, CompletionRequest, FinishReason, Message,
    MessageContent, Role, ToolCallLoop, ToolCallLoopRequest, ToolCallLoopResponse,
};
use crate::core::complete::tool_call_batch::DefaultToolCallBatch;
use crate::core::complete::tool_call_step::DefaultToolCallStep;

#[async_trait]
impl ToolCallLoop for BoundedToolCallLoop {
    async fn run(
        &self,
        req: ToolCallLoopRequest<'_>,
    ) -> Result<ToolCallLoopResponse, CompleteError> {
        if req.max_turns == 0 {
            return Err(CompleteError::InvalidRequest(
                "max_turns must be greater than zero".to_string(),
            ));
        }

        let mut messages = req.request.messages.clone();

        for turn in 0..req.max_turns {
            let current = CompletionRequest {
                messages: messages.clone(),
                ..req.request.clone()
            };
            let completion = self
                .completer
                .complete(CompleteRequest { request: &current })
                .await?;

            messages.push(Message {
                role: Role::Assistant,
                content: completion
                    .content
                    .clone()
                    .map(MessageContent::Text)
                    .unwrap_or(MessageContent::Empty),
                tool_calls: completion.tool_calls.clone(),
                ..Default::default()
            });

            if completion.finish_reason != FinishReason::ToolCalls
                || completion.tool_calls.is_empty()
            {
                return Ok(ToolCallLoopResponse {
                    response: completion,
                    messages,
                    turns: turn + 1,
                });
            }

            let steps: Vec<
                Arc<dyn Step<Ctx = DefaultToolCallBatch, ExecutionError = CompleteError>>,
            > = completion
                .tool_calls
                .iter()
                .enumerate()
                .map(|(index, call)| {
                    Arc::new(DefaultToolCallStep {
                        tool_ops: Arc::clone(&self.tool_ops),
                        call: call.clone(),
                        index,
                    })
                        as Arc<dyn Step<Ctx = DefaultToolCallBatch, ExecutionError = CompleteError>>
                })
                .collect();

            let mut batch = DefaultToolCallBatch::new(steps.len());
            let parallel_step = ParallelStepSvc::build(ParallelStepBuilder {
                steps,
                config: ParallelConfig::default(),
                event_bus: None,
            });
            parallel_step
                .execute(ContextMutationRequest { ctx: &mut batch })
                .await
                .map_err(Self::map_parallel_error)?;
            messages.extend(batch.into_messages());
        }

        Err(CompleteError::TurnLimitExceeded {
            max_turns: req.max_turns,
        })
    }
}

impl BoundedToolCallLoop {
    /// Compose one [`Completer`](crate::api::Completer) and one
    /// [`ToolOps`](crate::api::ToolOps) into a bounded tool-call loop.
    pub fn new(
        completer: Arc<dyn crate::api::Completer>,
        tool_ops: Arc<dyn crate::api::ToolOps>,
    ) -> Self {
        Self {
            completer,
            tool_ops,
        }
    }

    fn map_parallel_error(err: ParallelStepError<CompleteError>) -> CompleteError {
        for failure in err.failures {
            if let ParallelBranchFailure::Failed(step_err) = failure {
                return step_err.cause;
            }
        }
        CompleteError::ProviderError {
            provider: "tool_call_loop".to_string(),
            message: "one or more tool call branches timed out or panicked".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_pipeline::StepError;

    /// @covers: map_parallel_error
    #[test]
    fn test_map_parallel_error_preserves_original_cause_when_branch_failed_happy() {
        let err = ParallelStepError {
            failures: vec![ParallelBranchFailure::Failed(StepError {
                step_name: "tool".to_string(),
                cause: CompleteError::InvalidRequest("boom".to_string()),
            })],
        };
        let mapped = BoundedToolCallLoop::map_parallel_error(err);
        assert!(matches!(mapped, CompleteError::InvalidRequest(msg) if msg == "boom"));
    }

    /// @covers: map_parallel_error
    #[test]
    fn test_map_parallel_error_falls_back_when_only_timeouts_or_panics_edge() {
        let err: ParallelStepError<CompleteError> = ParallelStepError {
            failures: vec![ParallelBranchFailure::Panicked],
        };
        let mapped = BoundedToolCallLoop::map_parallel_error(err);
        assert!(matches!(mapped, CompleteError::ProviderError { .. }));
    }
}
