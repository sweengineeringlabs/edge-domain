//! `DefaultToolCallStep` — bridges one `ToolOps::execute` call into `edge-domain-pipeline`'s
//! `Step` contract, one branch of a per-turn `ParallelStep` fan-out.
//!
//! The success path is exercised via `BoundedToolCallLoop::run()`'s own coverage
//! (externally in `tests/tool_call_loop_e2e_test.rs`) — no separate success-path test
//! double is declared here to avoid duplicating that coverage. The error path below
//! reuses the crate's existing `NoopCompleter` (its `ToolOps` always errors) rather
//! than declaring a new local double.

use std::sync::Arc;

use async_trait::async_trait;
use edge_domain_pipeline::{ContextMutationRequest, Step};

use crate::api::{
    CompleteError, Message, ToolCall, ToolExecutionRequest, ToolOps, ToolRecordRequest,
    ToolResultBatch,
};
use crate::core::complete::tool_call_batch::DefaultToolCallBatch;

pub(super) struct DefaultToolCallStep {
    pub(super) tool_ops: Arc<dyn ToolOps>,
    pub(super) call: ToolCall,
    pub(super) index: usize,
}

#[async_trait]
impl Step for DefaultToolCallStep {
    type Ctx = DefaultToolCallBatch;
    type ExecutionError = CompleteError;

    async fn execute(
        &self,
        req: ContextMutationRequest<'_, DefaultToolCallBatch>,
    ) -> Result<(), CompleteError> {
        let response = self
            .tool_ops
            .execute(ToolExecutionRequest { call: &self.call })?;
        req.ctx.record(ToolRecordRequest {
            index: self.index,
            message: Box::new(Message::tool(response.output, self.call.id.clone())),
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::NoopCompleter;
    use futures::executor::block_on;

    /// @covers: execute
    #[test]
    fn test_execute_propagates_tool_ops_error_error() {
        let step = DefaultToolCallStep {
            tool_ops: Arc::new(NoopCompleter),
            call: ToolCall::new(
                "id-1".to_string(),
                "get_weather".to_string(),
                "{}".to_string(),
            ),
            index: 0,
        };
        let mut batch = DefaultToolCallBatch::new(1);
        let result = block_on(step.execute(ContextMutationRequest { ctx: &mut batch }));
        assert!(matches!(result, Err(CompleteError::InvalidRequest(_))));
    }
}
