//! Dispatchable-handler constructors for `StdReasoningFactory`.

use std::sync::Arc;

use edge_domain_handler::Handler;

use crate::api::{Reasoning, ReasoningPattern, StdReasoningFactory, ThinkingProcess};
use crate::core::reasoning::DefaultReasoningHandler;

impl StdReasoningFactory {
    /// Construct a dispatchable reasoning handler backed by the given reasoner.
    pub fn reasoning_handler(
        reasoner: Arc<dyn Reasoning>,
    ) -> impl Handler<Request = String, Response = ThinkingProcess> {
        Self::wrap_reasoner(reasoner)
    }

    /// Construct a dispatchable reasoning handler backed by the reference [`crate::api::LinearReasoning`].
    pub fn default_reasoning_handler(
        pattern: ReasoningPattern,
    ) -> impl Handler<Request = String, Response = ThinkingProcess> {
        DefaultReasoningHandler::with_pattern(pattern)
    }

    /// Wrap a reasoner in the dispatchable `DefaultReasoningHandler`, shared by both
    /// the caller-supplied-reasoner and default-pattern constructors above.
    fn wrap_reasoner(
        reasoner: Arc<dyn Reasoning>,
    ) -> impl Handler<Request = String, Response = ThinkingProcess> {
        DefaultReasoningHandler { reasoner }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{LinearReasoning, ReasonRequest};
    use edge_domain_command::DirectCommandBus;
    use edge_domain_handler::{ExecutionRequest, HandlerContext};
    use edge_domain_observer::StdObserveFactory;
    use edge_security_runtime::SecurityContext;
    use futures::executor::block_on;

    /// @covers: reasoning_handler
    #[test]
    fn test_reasoning_handler_executes_via_given_reasoner() {
        let reasoner: Arc<dyn Reasoning> =
            Arc::new(LinearReasoning::new(ReasoningPattern::ChainOfThought));
        let handler = StdReasoningFactory::reasoning_handler(reasoner);

        let security: SecurityContext = SecurityContext::unauthenticated();
        let commands = DirectCommandBus;
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = HandlerContext {
            security: &security,
            commands: &commands,
            observer: observer.as_ref(),
        };
        let out = block_on(Handler::execute(
            &handler,
            ExecutionRequest {
                req: "solve x".to_string(),
                ctx: &ctx,
            },
        ))
        .expect("handler ok");
        assert!(out.is_complete);
    }

    /// @covers: default_reasoning_handler
    #[test]
    fn test_default_reasoning_handler_has_stable_id() {
        let handler =
            StdReasoningFactory::default_reasoning_handler(ReasoningPattern::ChainOfThought);
        assert_eq!(
            Handler::id(&handler, edge_domain_handler::IdRequest)
                .expect("id ok")
                .id,
            "reasoning.reason"
        );
    }

    /// @covers: default_reasoning_handler
    #[test]
    fn test_default_reasoning_handler_uses_given_pattern() {
        let reasoner = LinearReasoning::new(ReasoningPattern::ChainOfThought);
        let response = block_on(reasoner.reason(ReasonRequest {
            problem: "solve x",
            pattern: ReasoningPattern::ChainOfThought,
        }))
        .expect("reason ok");
        assert!(response.process.is_complete);
    }

    /// @covers: wrap_reasoner
    #[test]
    fn test_wrap_reasoner_produces_handler_with_stable_id() {
        let reasoner: Arc<dyn Reasoning> =
            Arc::new(LinearReasoning::new(ReasoningPattern::ChainOfThought));
        let handler = StdReasoningFactory::wrap_reasoner(reasoner);
        assert_eq!(
            Handler::id(&handler, edge_domain_handler::IdRequest)
                .expect("id ok")
                .id,
            "reasoning.reason"
        );
    }
}
