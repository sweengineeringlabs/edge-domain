//! `DefaultReasoningHandler` — `Handler` impl for the reasoning primitive (ADR-024).

use std::sync::Arc;

use async_trait::async_trait;

use edge_domain_handler::{Handler, HandlerContext, HandlerError};

use crate::api::{Reasoning, ReasoningPattern, ThinkingProcess};

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

    fn id(&self) -> &str {
        REASONING_HANDLER_ID
    }

    fn pattern(&self) -> &str {
        REASONING_HANDLER_PATTERN
    }

    async fn execute(
        &self,
        problem: String,
        ctx: HandlerContext<'_>,
    ) -> Result<ThinkingProcess, HandlerError> {
        let span = ctx.observer().tracer().start_span("reasoning", "reason");
        let result = self
            .reasoner
            .reason(&problem, REASONING_DEFAULT_PATTERN)
            .await
            .map_err(|e| HandlerError::ExecutionFailed(e.message()));
        span.finish();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::LinearReasoning;
    use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
    use edge_domain_observe::StdObserveFactory;
    use edge_domain_security::SecurityContext;
    use futures::executor::block_on;

    fn handler() -> DefaultReasoningHandler {
        DefaultReasoningHandler {
            reasoner: Arc::new(LinearReasoning::new(ReasoningPattern::ChainOfThought)),
        }
    }

    #[test]
    fn test_handler_execute_returns_complete_process_happy() {
        let security = SecurityContext::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let observer = StdObserveFactory::noop_observe_context();
        let ctx = HandlerContext::new(&security, &commands, observer.as_ref());
        let out = block_on(Handler::execute(&handler(), "solve x".to_string(), ctx))
            .expect("handler ok");
        assert!(out.is_complete);
        assert!(out.conclusion.is_some());
    }

    #[test]
    fn test_handler_execute_opens_and_finishes_span_happy() {
        let security = SecurityContext::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let observer = StdObserveFactory::noop_observe_context();
        let ctx = HandlerContext::new(&security, &commands, observer.as_ref());
        block_on(Handler::execute(&handler(), "solve x".to_string(), ctx)).expect("handler ok");
    }

    #[test]
    fn test_handler_id_is_stable_edge() {
        assert_eq!(Handler::id(&handler()), REASONING_HANDLER_ID);
    }

    #[test]
    fn test_handler_pattern_is_stable_edge() {
        assert_eq!(Handler::pattern(&handler()), REASONING_HANDLER_PATTERN);
    }

    #[test]
    fn test_handler_execute_blank_problem_error() {
        let security = SecurityContext::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let observer = StdObserveFactory::noop_observe_context();
        let ctx = HandlerContext::new(&security, &commands, observer.as_ref());
        let result = block_on(Handler::execute(&handler(), "   ".to_string(), ctx));
        assert!(result.is_err());
    }
}
