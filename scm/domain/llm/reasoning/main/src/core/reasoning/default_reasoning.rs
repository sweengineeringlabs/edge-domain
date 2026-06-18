//! `Handler` impl for `ReasoningEndpoint` (ADR-024).

use async_trait::async_trait;

use edge_domain_handler::{Handler, HandlerContext, HandlerError};

use crate::api::{ReasoningEndpoint, ReasoningPattern, ThinkingProcess};

/// Stable handler id under which the endpoint registers for dispatch.
const REASONING_HANDLER_ID: &str = "reasoning.reason";
/// Route pattern this endpoint matches in the dispatch table.
const REASONING_HANDLER_PATTERN: &str = "reasoning/reason";
/// Default reasoning pattern applied when the pipeline carries no explicit one.
const REASONING_DEFAULT_PATTERN: ReasoningPattern = ReasoningPattern::ChainOfThought;

#[async_trait]
impl Handler for ReasoningEndpoint {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::api::LinearReasoning;
    use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
    use edge_domain_security::SecurityContext;
    use futures::executor::block_on;

    fn endpoint() -> ReasoningEndpoint {
        ReasoningEndpoint::new(Arc::new(LinearReasoning::new(ReasoningPattern::ChainOfThought)))
    }

    #[test]
    fn test_handler_execute_returns_complete_process_happy() {
        let security = SecurityContext::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let ctx = HandlerContext { security: &security, commands: &commands };
        let out = block_on(Handler::execute(&endpoint(), "solve x".to_string(), ctx))
            .expect("handler ok");
        assert!(out.is_complete);
        assert!(out.conclusion.is_some());
    }

    #[test]
    fn test_handler_id_is_stable_edge() {
        assert_eq!(Handler::id(&endpoint()), REASONING_HANDLER_ID);
    }

    #[test]
    fn test_handler_pattern_is_stable_edge() {
        assert_eq!(Handler::pattern(&endpoint()), REASONING_HANDLER_PATTERN);
    }

    #[test]
    fn test_handler_execute_blank_problem_error() {
        let security = SecurityContext::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let ctx = HandlerContext { security: &security, commands: &commands };
        let result = block_on(Handler::execute(&endpoint(), "   ".to_string(), ctx));
        assert!(result.is_err());
    }
}
