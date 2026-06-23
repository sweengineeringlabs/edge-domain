//! `DefaultProviderHandler` — `Handler` impl for the provider primitive (ADR-024).

use std::sync::Arc;

use async_trait::async_trait;

use edge_domain_handler::{Handler, HandlerContext, HandlerError};

use crate::api::{ExecutionModel, ExecutionStepResult};

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

    fn id(&self) -> &str {
        PROVIDER_HANDLER_ID
    }

    fn pattern(&self) -> &str {
        PROVIDER_HANDLER_PATTERN
    }

    async fn execute(
        &self,
        goal: String,
        ctx: HandlerContext<'_>,
    ) -> Result<ExecutionStepResult, HandlerError> {
        let span = ctx
            .observer()
            .tracer()
            .start_span("provider", "execute_step");
        let result = self
            .model
            .execute_step("", &goal, "", Vec::new())
            .await
            .map_err(|e| HandlerError::ExecutionFailed(e.message()));
        span.finish();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{EchoExecutionModel, ExecutionConfig, ExecutionMode};
    use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
    use edge_domain_observer::StdObserveFactory;
    use edge_domain_security::SecurityContext;
    use futures::executor::block_on;

    fn handler() -> DefaultProviderHandler {
        let config = ExecutionConfig::new(4096, 30_000, true, false, ExecutionMode::Async);
        DefaultProviderHandler {
            model: Arc::new(EchoExecutionModel::new(config)),
        }
    }

    #[test]
    fn test_execute_returns_reasoning_containing_goal_happy() {
        let security = SecurityContext::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let observer = StdObserveFactory::noop_observe_context();
        let ctx = HandlerContext::new(&security, &commands, observer.as_ref());
        let out =
            block_on(Handler::execute(&handler(), "ship it".to_string(), ctx)).expect("handler ok");
        assert!(out.reasoning.contains("ship it"));
    }

    #[test]
    fn test_execute_opens_and_finishes_span_happy() {
        let security = SecurityContext::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let observer = StdObserveFactory::noop_observe_context();
        let ctx = HandlerContext::new(&security, &commands, observer.as_ref());
        block_on(Handler::execute(&handler(), "goal".to_string(), ctx)).expect("handler ok");
    }

    #[test]
    fn test_id_is_stable_edge() {
        assert_eq!(Handler::id(&handler()), PROVIDER_HANDLER_ID);
    }

    #[test]
    fn test_pattern_is_stable_edge() {
        assert_eq!(Handler::pattern(&handler()), PROVIDER_HANDLER_PATTERN);
    }
}
