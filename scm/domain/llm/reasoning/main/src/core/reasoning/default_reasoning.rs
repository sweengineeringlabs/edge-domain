//! `Handler` + `Service` impls for `DefaultReasoning` (ADR-037 connection).

use async_trait::async_trait;
use futures::future::BoxFuture;

use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
use edge_domain_handler::{Handler, HandlerContext, HandlerError};
use edge_domain_security::SecurityContext;
use edge_domain_service::{Service, ServiceError};

use crate::api::Reasoning;
use crate::api::{DefaultReasoning, ReasoningPattern, ThinkingProcess};

/// Stable handler id under which the endpoint registers for dispatch.
const REASONING_HANDLER_ID: &str = "reasoning.reason";
/// Route pattern this endpoint matches in the dispatch table.
const REASONING_HANDLER_PATTERN: &str = "reasoning/reason";
/// Stable service name under which consumers resolve this endpoint.
const REASONING_SERVICE_NAME: &str = "reasoning";
/// Default reasoning pattern applied when the pipeline carries no explicit one.
const REASONING_DEFAULT_PATTERN: ReasoningPattern = ReasoningPattern::ChainOfThought;

#[async_trait]
impl Handler for DefaultReasoning {
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

impl Service for DefaultReasoning {
    type Request = String;
    type Response = ThinkingProcess;

    fn name(&self) -> &str {
        REASONING_SERVICE_NAME
    }

    fn execute(&self, problem: String) -> BoxFuture<'_, Result<ThinkingProcess, ServiceError>> {
        // ADR-037: Service → Dispatch → Handler → core. This in-crate reference
        // builds a default request context; in production the dispatch runtime
        // supplies the per-request `HandlerContext` (security + command bus).
        Box::pin(async move {
            let security = SecurityContext::unauthenticated();
            let commands = StdCommandBusFactory::direct();
            let ctx = HandlerContext {
                security: &security,
                commands: &commands,
            };
            Handler::execute(self, problem, ctx)
                .await
                .map_err(|e| ServiceError::Internal(e.to_string()))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::LinearReasoning;
    use futures::executor::block_on;

    fn endpoint() -> DefaultReasoning {
        DefaultReasoning::new(LinearReasoning::new(ReasoningPattern::ChainOfThought))
    }

    #[test]
    fn test_service_delegates_to_handler() {
        let out = block_on(Service::execute(&endpoint(), "solve x".to_string()))
            .expect("service should succeed");
        assert!(out.is_complete);
        assert!(out.conclusion.is_some());
    }

    #[test]
    fn test_handler_id_is_stable() {
        assert_eq!(Handler::id(&endpoint()), REASONING_HANDLER_ID);
    }

    #[test]
    fn test_service_rejects_blank_problem() {
        assert!(block_on(Service::execute(&endpoint(), "   ".to_string())).is_err());
    }
}
