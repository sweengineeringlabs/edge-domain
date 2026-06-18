//! `Handler` impl for `ProviderEndpoint` (ADR-024).

use async_trait::async_trait;

use edge_domain_handler::{Handler, HandlerContext, HandlerError};

use crate::api::{ExecutionStepResult, ProviderEndpoint};

/// Stable handler id under which the endpoint registers for dispatch.
const PROVIDER_HANDLER_ID: &str = "provider.execute_step";
/// Route pattern this endpoint matches in the dispatch table.
const PROVIDER_HANDLER_PATTERN: &str = "provider/execute_step";

#[async_trait]
impl Handler for ProviderEndpoint {
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
        _ctx: HandlerContext<'_>,
    ) -> Result<ExecutionStepResult, HandlerError> {
        self.model
            .execute_step("", &goal, "", Vec::new())
            .await
            .map_err(|e| HandlerError::ExecutionFailed(e.message()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::api::{EchoExecutionModel, ExecutionConfig, ExecutionMode};
    use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
    use edge_domain_security::SecurityContext;
    use futures::executor::block_on;

    fn endpoint() -> ProviderEndpoint {
        let config = ExecutionConfig::new(4096, 30_000, true, false, ExecutionMode::Async);
        ProviderEndpoint::new(Arc::new(EchoExecutionModel::new(config)))
    }

    #[test]
    fn test_handler_execute_returns_reasoning_containing_goal_happy() {
        let security = SecurityContext::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let ctx = HandlerContext { security: &security, commands: &commands };
        let out = block_on(Handler::execute(&endpoint(), "ship it".to_string(), ctx))
            .expect("handler ok");
        assert!(out.reasoning.contains("ship it"));
    }

    #[test]
    fn test_handler_id_is_stable_edge() {
        assert_eq!(Handler::id(&endpoint()), PROVIDER_HANDLER_ID);
    }

    #[test]
    fn test_handler_pattern_is_stable_edge() {
        assert_eq!(Handler::pattern(&endpoint()), PROVIDER_HANDLER_PATTERN);
    }
}
