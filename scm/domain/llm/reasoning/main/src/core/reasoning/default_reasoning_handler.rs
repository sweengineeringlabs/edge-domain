//! `DefaultReasoningHandler` — `Handler` impl for the reasoning primitive (ADR-024).

use std::sync::Arc;

use async_trait::async_trait;

use edge_domain_handler::{
    ExecutionRequest, Handler, HandlerError, IdRequest, IdResponse, PatternRequest, PatternResponse,
};
use edge_domain_observer::{SpanFinishRequest, SpanStartRequest, TracerRequest};

use crate::api::{LinearReasoning, ReasonRequest, Reasoning, ReasoningPattern, ThinkingProcess};

/// Stable handler id under which this handler registers for dispatch.
const REASONING_HANDLER_ID: &str = "reasoning.reason";
/// Route pattern this handler matches in the dispatch table.
const REASONING_HANDLER_PATTERN: &str = "reasoning/reason";
/// Default reasoning pattern applied when the pipeline carries no explicit one.
const REASONING_DEFAULT_PATTERN: ReasoningPattern = ReasoningPattern::ChainOfThought;

pub(crate) struct DefaultReasoningHandler {
    pub(crate) reasoner: Arc<dyn Reasoning>,
}

#[async_trait]
impl Handler for DefaultReasoningHandler {
    type Request = String;
    type Response = ThinkingProcess;

    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: REASONING_HANDLER_ID.to_string(),
        })
    }

    fn pattern(&self, _req: PatternRequest) -> Result<PatternResponse, HandlerError> {
        Ok(PatternResponse {
            pattern: REASONING_HANDLER_PATTERN.to_string(),
        })
    }

    async fn execute(
        &self,
        req: ExecutionRequest<'_, String>,
    ) -> Result<ThinkingProcess, HandlerError> {
        let span = req
            .ctx
            .observer
            .tracer(TracerRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .tracer
            .start_span(SpanStartRequest {
                handler_id: "reasoning".to_string(),
                operation: "reason".to_string(),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .span;
        let result = self
            .reasoner
            .reason(ReasonRequest {
                problem: &req.req,
                pattern: REASONING_DEFAULT_PATTERN,
            })
            .await
            .map(|resp| *resp.process)
            .map_err(|e| HandlerError::ExecutionFailed(e.message()));
        span.finish(SpanFinishRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        result
    }
}

impl DefaultReasoningHandler {
    /// Construct a handler backed by the reference [`LinearReasoning`] for the given pattern.
    pub(crate) fn with_pattern(pattern: ReasoningPattern) -> Self {
        DefaultReasoningHandler {
            reasoner: Arc::new(LinearReasoning::new(pattern)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
    use edge_domain_handler::HandlerContext;
    use edge_domain_observer::StdObserveFactory;
    use edge_domain_security::{SecurityBootstrap, SecurityContext, SecurityServices};
    use futures::executor::block_on;

    fn handler() -> DefaultReasoningHandler {
        DefaultReasoningHandler {
            reasoner: Arc::new(LinearReasoning::new(ReasoningPattern::ChainOfThought)),
        }
    }

    #[test]
    fn test_handler_execute_returns_complete_process_happy() {
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
                req: "solve x".to_string(),
                ctx: &ctx,
            },
        ))
        .expect("handler ok");
        assert!(out.is_complete);
        assert!(out.conclusion.is_some());
    }

    #[test]
    fn test_handler_id_is_stable_edge() {
        assert_eq!(
            Handler::id(&handler(), IdRequest).expect("id ok").id,
            REASONING_HANDLER_ID
        );
    }

    #[test]
    fn test_handler_pattern_is_stable_edge() {
        assert_eq!(
            Handler::pattern(&handler(), PatternRequest)
                .expect("pattern ok")
                .pattern,
            REASONING_HANDLER_PATTERN
        );
    }

    #[test]
    fn test_handler_execute_blank_problem_error() {
        let security: SecurityContext = SecurityServices::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = HandlerContext {
            security: &security,
            commands: &commands,
            observer: observer.as_ref(),
        };
        let result = block_on(Handler::execute(
            &handler(),
            ExecutionRequest {
                req: "   ".to_string(),
                ctx: &ctx,
            },
        ));
        assert!(result.is_err());
    }

    #[test]
    fn test_with_pattern_creates_handler_with_stable_id_happy() {
        let h = DefaultReasoningHandler::with_pattern(ReasoningPattern::ChainOfThought);
        assert_eq!(
            Handler::id(&h, IdRequest).expect("id ok").id,
            REASONING_HANDLER_ID
        );
    }
}
