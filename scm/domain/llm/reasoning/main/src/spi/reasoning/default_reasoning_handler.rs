//! `DefaultReasoningHandler` — self-contained SAF handler wiring for the reasoning primitive.

use std::sync::Arc;

use async_trait::async_trait;

use edge_domain_handler::{Handler, HandlerContext, HandlerError};

use crate::api::{LinearReasoning, Reasoning, ReasoningPattern, ThinkingProcess};

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
        _ctx: HandlerContext<'_>,
    ) -> Result<ThinkingProcess, HandlerError> {
        self.reasoner
            .reason(&problem, REASONING_DEFAULT_PATTERN)
            .await
            .map_err(|e| HandlerError::ExecutionFailed(e.message()))
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
    use edge_domain_observer::StdObserveFactory;
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

    #[test]
    fn test_with_pattern_creates_handler_with_stable_id_happy() {
        let h = DefaultReasoningHandler::with_pattern(ReasoningPattern::ChainOfThought);
        assert_eq!(Handler::id(&h), REASONING_HANDLER_ID);
    }
}
