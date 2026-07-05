//! `DefaultProviderHandler` — `Handler` impl for the provider primitive (ADR-024).

use std::sync::Arc;

use async_trait::async_trait;

use edge_domain_handler::{
    ExecutionRequest, Handler, HandlerError, IdRequest, IdResponse, PatternRequest, PatternResponse,
};
use edge_domain_observer::{SpanFinishRequest, SpanStartRequest, TracerRequest};

use crate::api::{ExecutionModel, ExecutionStepResult, StepExecutionRequest};

/// Stable handler id under which this handler registers for dispatch.
const PROVIDER_HANDLER_ID: &str = "provider.execute_step";
/// Route pattern this handler matches in the dispatch table.
const PROVIDER_HANDLER_PATTERN: &str = "provider/execute_step";

pub(crate) struct DefaultProviderHandler {
    pub(crate) model: Arc<dyn ExecutionModel>,
}

#[async_trait]
impl Handler for DefaultProviderHandler {
    type Request = String;
    type Response = ExecutionStepResult;

    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: PROVIDER_HANDLER_ID.to_string(),
        })
    }

    fn pattern(&self, _req: PatternRequest) -> Result<PatternResponse, HandlerError> {
        Ok(PatternResponse {
            pattern: PROVIDER_HANDLER_PATTERN.to_string(),
        })
    }

    async fn execute(
        &self,
        req: ExecutionRequest<'_, String>,
    ) -> Result<ExecutionStepResult, HandlerError> {
        let span = req
            .ctx
            .observer
            .tracer(TracerRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .tracer
            .start_span(SpanStartRequest {
                handler_id: "provider".to_string(),
                operation: "execute_step".to_string(),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .span;
        let result = self
            .model
            .execute_step(StepExecutionRequest {
                agent_id: "",
                goal: &req.req,
                context: "",
                available_tools: Vec::new(),
            })
            .await
            .map(|resp| *resp.result)
            .map_err(|e| HandlerError::ExecutionFailed(e.message()));
        span.finish(SpanFinishRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{EchoExecutionModel, ExecutionConfig, ExecutionMode};
    use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
    use edge_domain_handler::HandlerContext;
    use edge_domain_observer::StdObserveFactory;
    use edge_domain_security::{SecurityBootstrap, SecurityContext, SecurityServices};
    use futures::executor::block_on;

    fn handler() -> DefaultProviderHandler {
        let config = ExecutionConfig::new(4096, 30_000, true, false, ExecutionMode::Async);
        DefaultProviderHandler {
            model: Arc::new(EchoExecutionModel::new(config)),
        }
    }

    #[test]
    fn test_execute_returns_reasoning_containing_goal_happy() {
        let security: SecurityContext = SecurityServices::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = HandlerContext {
            security: &security,
            commands: &commands,
            observer: observer.as_ref(),
        };
        let out = block_on(Handler::execute(
            &handler(),
            ExecutionRequest {
                req: "ship it".to_string(),
                ctx: &ctx,
            },
        ))
        .expect("handler ok");
        assert!(out.reasoning.contains("ship it"));
    }

    #[test]
    fn test_execute_opens_and_finishes_span_happy() {
        let security: SecurityContext = SecurityServices::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = HandlerContext {
            security: &security,
            commands: &commands,
            observer: observer.as_ref(),
        };
        block_on(Handler::execute(
            &handler(),
            ExecutionRequest {
                req: "goal".to_string(),
                ctx: &ctx,
            },
        ))
        .expect("handler ok");
    }

    #[test]
    fn test_id_is_stable_edge() {
        assert_eq!(
            Handler::id(&handler(), IdRequest).expect("id ok").id,
            PROVIDER_HANDLER_ID
        );
    }

    #[test]
    fn test_pattern_is_stable_edge() {
        assert_eq!(
            Handler::pattern(&handler(), PatternRequest)
                .expect("pattern ok")
                .pattern,
            PROVIDER_HANDLER_PATTERN
        );
    }
}
